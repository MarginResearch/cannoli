//! MemPipe! A high-performance, low-latency, high-bandwidth way of streaming
//! data from one core to another. It's effectively just a ring buffer of
//! buffers
//!
//! This crate is effectively a collection of buffers which are allocated
//! in shared memory. The creator of the memory, [`SendPipe`], defines the
//! size of each chunk, and the number of chunks (number of buffers) to use
//! for sending to the other core.
//!
//! When the sender wants to send data, they request a buffer via
//! `alloc_buffer`. This gives them a write-only accessor [`ChunkWriter`] that
//! allows writing to this buffer. When the buffer is dropped it is tagged
//! with a sequence number and flagged as owned by the client.
//!
//! This design is for super low latency, allowing very small transfers while
//! still getting high memory bandwidth. This is originally being designed as
//! a way to stream a log of control flow and register states from inside of
//! QEMU's JIT into another processes memory. The goal is to block QEMU as
//! little as possible, and thus latency and design of this structures memory
//! layout is critical.
//!
//! The way that we signal information between threads is on a separate cache
//! line from the raw backing data storage. This means that the producer may
//! be hammering on its data buffer, without causing cache coherency traffic
//! for the consumers that are polling metadata, waiting for data to be given
//! to them.

#![feature(maybe_uninit_uninit_array, array_from_fn)]
#![feature(inline_const)]

// This code should work on effectively any UNIX system. It uses `shm_*` APIs
// and `mmap`, and should be pretty portable between systems with these APIs
#[cfg(not(target_family = "unix"))]
compile_error!("Sorry, currently only UNIX-like systems are supported, you \
    need support for shm_* and mmap APIs");

use std::ptr::addr_of_mut;
use std::mem::{MaybeUninit, size_of, size_of_val};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU64, Ordering};
use std::ffi::{CString, NulError};

/// A wrapper around the [`Error`] type
type Result<T> = std::result::Result<T, Error>;

/// Error types for this module
#[derive(Debug)]
pub enum Error {
    /// An error occurred while converting Rust bytes into a [`CString`]
    CString(NulError),

    /// Failed to open shared memory file
    ShmOpen(std::io::Error),

    /// Failed to [`libc::ftruncate`] shared memory when it was created
    SetMemorySize(std::io::Error),

    /// Failed to [`libc::mmap`] memory
    MapMemory(std::io::Error),

    /// Attempted to connect to a pipe with a different configuration of
    /// chunk size or number of buffers
    PipeMismatch,

    /// The specified chunk size or number of buffers was invalid
    ///
    /// We expect non-zero number of buffers and size
    InvalidPipeConfiguration,
}

/// Wrapper around a chunk
#[repr(C, align(64))]
pub struct Chunk<const CHUNK_SIZE: usize>(
    [MaybeUninit<UnsafeCell<u8>>; CHUNK_SIZE]);

/// Magic value put at the header of memory pipe structures
const MEMPIPE_MAGIC: u64 = 0x91d021239b73bc57;

/// A memory pipe which uses `CHUNK_SIZE` byte chunks and `NUM_BUFFERS` for
/// transferring memory between processes.
///
/// This defines the actual shape of a memory pipe in memory, and this is what
/// is placed in shared memory at offset 0 of the allocated memory
#[repr(C)]
struct RawMemPipe<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> {
    /// Magic header value
    magic: u64,

    /// Size of a [`usize`] we expect, in bytes
    usize_size: u64,

    /// Size of this memory pipe. These are used to make sure both the sender
    /// and receiver are using the same constants
    chunk_size: u64,

    /// Number of buffers in this memory pipe. These are used to make sure both
    /// the sender and receiver are using the same constants
    num_buffers: u64,

    /// Unique identifier for the pipe
    uid: u64,

    /// Boolean tracking whether or not each corresponding buffer is owned by
    /// the client. In our case, the client is the program who opened the
    /// existing shared memory buffer.
    client_owned: [AtomicBool; NUM_BUFFERS],

    /// Holds the length of a transferred buffer. This must be populated prior
    /// to `client_owned` being set to `true`, and must be ordered correctly
    /// on the processor
    client_len: [AtomicUsize; NUM_BUFFERS],

    /// The sequence number for a given buffer, must be set prior to
    /// `client_owned` and ordered correctly on the processor
    client_seq: [AtomicU64; NUM_BUFFERS],

    /// Current sequence number, incremented by one to get a sequential ID to
    /// tag outbound chunks with
    cur_seq: AtomicU64,

    /// Chunks
    chunks: [Chunk<CHUNK_SIZE>; NUM_BUFFERS],
}

/// The sending side of a pipe. To create one, call [`SendPipe::create`] with
/// a name which will be used to access this pipe.
pub struct SendPipe<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> {
    /// UID for this pipe
    uid: u64,

    /// Reference to the memory pipe
    mem_pipe: *const RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>,
}

/// Get the filename for a given `uid`
fn filename_from_uid(uid: u64) -> Result<CString> {
    // Create a [`CString`] of the name to hexlify and null-terminate it
    CString::new(format!("cannoli_{:016x}", uid)).map_err(Error::CString)
}

/// Get the current `errno` and convert it into a [`std::io::Error`]
fn errno() -> std::io::Error {
    std::io::Error::from_raw_os_error(errno::errno().0)
}

impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize>
        SendPipe<CHUNK_SIZE, NUM_BUFFERS> {
    /// Create a pipe
    pub fn create() -> Result<Self> {
        // Make sure settings are sane
        if NUM_BUFFERS == 0 || CHUNK_SIZE == 0 {
            return Err(Error::InvalidPipeConfiguration);
        }

        // Generate a random name
        let uid = rand::random::<u64>();

        // Get the filename
        let cs = filename_from_uid(uid)?;

        // Delete the shared memory before we create it to make sure we
        // exclusively create it
        unsafe { libc::shm_unlink(cs.as_ptr()); }

        // Create new shared memory
        let shm = unsafe {
            libc::shm_open(cs.as_ptr(),
                libc::O_CREAT | libc::O_EXCL | libc::O_RDWR,
                libc::S_IRUSR | libc::S_IWUSR)
        };
        if shm == -1 { return Err(Error::ShmOpen(errno())); }

        // Set the shared memory size
        if unsafe { libc::ftruncate(shm,
                size_of::<RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>>() as i64) }
                    == -1 {
            // Save error
            let ret = Error::SetMemorySize(errno());

            // Close the file
            unsafe { libc::close(shm); }

            // Return the error
            return Err(ret);
        }

        // Map the shared memory
        let mapped = unsafe {
            libc::mmap(std::ptr::null_mut(),
                size_of::<RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>>(),
                libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED,
                shm, 0) as *mut RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>
        };
        let map_err = errno();

        // Close the FD as we no longer need it
        unsafe { libc::close(shm); }

        // Make sure mapping was successful
        if mapped as usize == libc::MAP_FAILED as usize {
            return Err(Error::MapMemory(map_err));
        }

        // Initialize the memory
        unsafe {
            addr_of_mut!((*mapped).magic).write(MEMPIPE_MAGIC);
            addr_of_mut!((*mapped).chunk_size).write(CHUNK_SIZE as u64);
            addr_of_mut!((*mapped).num_buffers).write(NUM_BUFFERS as u64);
            addr_of_mut!((*mapped).usize_size)
                .write(size_of::<usize>() as u64);
            addr_of_mut!((*mapped).uid).write(uid);
            addr_of_mut!((*mapped).client_owned)
                .write([const { AtomicBool::new(false) }; NUM_BUFFERS]);
            addr_of_mut!((*mapped).client_len)
                .write([const { AtomicUsize::new(0) }; NUM_BUFFERS]);
            addr_of_mut!((*mapped).client_seq)
                .write([const { AtomicU64::new(0) }; NUM_BUFFERS]);
            addr_of_mut!((*mapped).cur_seq).write(AtomicU64::new(0));

            // Chunks are left uninitialized, which is okay as they are marked
            // as [`MaybeUninit`]
        }

        Ok(Self { mem_pipe: mapped, uid })
    }

    /// Get the UID for the pipe
    pub fn uid(&self) -> u64 {
        self.uid
    }

    /// Allocate a buffer from the pipe
    ///
    /// This will spin on the available buffers `mem_pipe.client_owned` until
    /// one is available. This intentionally does not raise to a OS-level wait,
    /// to optimize for latency and our use case.
    ///
    /// Only one buffer can be issued at a time. Rust gives this safety since
    /// we take `&mut self` here, thus, only one [`ChunkWriter`] can exist for
    /// any given [`SendPipe`] at a given time
    #[inline]
    pub fn alloc_buffer(&mut self) -> ChunkWriter<CHUNK_SIZE, NUM_BUFFERS> {
        // Get the pipe
        let pipe = unsafe { &*self.mem_pipe };

        // Outer loop, look through buffers forever
        loop {
            // Check all buffers
            for ii in 0..NUM_BUFFERS {
                // See if this buffer is available for use (not client owned)
                if !pipe.client_owned[ii].load(Ordering::Acquire) {
                    // Woo, we own this buffer, return it!
                    return ChunkWriter {
                        mem_pipe: pipe,
                        idx:      ii,
                        written:  0,

                        // Construct a raw pointer to the first byte
                        bytes: UnsafeCell::raw_get(
                            pipe.chunks[ii].0[0].as_ptr()
                        ),
                    };
                }
            }
        }
    }
}

impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize>
        Drop for SendPipe<CHUNK_SIZE, NUM_BUFFERS> {
    fn drop(&mut self) {
        unsafe {
            // Delete the file we created
            let cs = filename_from_uid(self.uid)
                .expect("Failed to get filename for pipe");
            libc::shm_unlink(cs.as_ptr());

            // Unmap the memory we mapped
            assert!(libc::munmap(self.mem_pipe as *mut _,
                size_of::<RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>>()) == 0,
                "Failed to munmap() IPC for SendPipe : {:?}", errno());
        }
    }
}

/// A [`ChunkWriter`] is a wrapper around a [`Chunk`] which is currently. This
/// prevents all reading from the buffer, as buffers from `alloc_buffer` are
/// write-only.
///
/// When a chunk is dropped, it is sent over to the consumer immediately
pub struct ChunkWriter<'a, const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> {
    /// Reference to the `MemPipe` we came from
    mem_pipe: &'a RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>,

    /// Accessor to the raw underlying bytes, points to the first byte of a
    /// [`Chunk`]'s raw data
    bytes: *mut u8,

    /// Buffer index in the [`RawMemPipe`]
    idx: usize,

    /// Tracks the number of initialized bytes in the chunk
    written: usize,
}

impl<'a, const CHUNK_SIZE: usize, const NUM_BUFFERS: usize>
        ChunkWriter<'a, CHUNK_SIZE, NUM_BUFFERS> {
    /// Write data into the chunk and send it, consuming the `ChunkWriter` and
    /// returning the number of bytes sent
    pub fn send(mut self, data: impl AsRef<[u8]>) -> usize {
        // Compute the remaining size of the buffer
        let remain = CHUNK_SIZE - self.written;

        // Compute the number of bytes we can accept for this send
        let to_send = remain.min(data.as_ref().len());

        // Copy the data into the chunk
        unsafe {
            self.bytes.add(self.written)
                .copy_from_nonoverlapping(data.as_ref().as_ptr(), to_send);
        }

        // Update number of bytes written
        self.written += to_send;

        // Return number of copied and sent bytes
        to_send
    }

    /// Get a mutable reference to the backing memory
    #[inline]
    pub fn get_raw(&mut self) -> *mut u8 {
        self.bytes
    }

    /// Send raw data, this takes the length sent, and assumes it is in-bounds
    /// and the bytes referred to have been initialized
    ///
    /// # Safety
    ///
    /// The data pointed to by [`Self::get_raw`] needs to be initialized at
    /// offset 0 for at least `size` bytes for this to be safe. This assumes
    /// `size` bytes have been initialized by the caller
    ///
    #[inline]
    pub unsafe fn send_raw(mut self, size: usize) {
        // Update the number of bytes written, we'll send it on drop!
        self.written = size;
    }
}

impl<'a, const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> Drop for
        ChunkWriter<'a, CHUNK_SIZE, NUM_BUFFERS> {
    /// Drop a [`ChunkWriter`], this action sends ownership of the buffer to
    /// the client
    fn drop(&mut self) {
        // Send ownership of the buffer to the client

        // Populate the length
        self.mem_pipe.client_len[self.idx].store(self.written,
            Ordering::Relaxed);

        // Allocate a unique sequence ID for this buffer
        let seq_id = self.mem_pipe.cur_seq.fetch_add(1, Ordering::Relaxed);
        self.mem_pipe.client_seq[self.idx].store(seq_id, Ordering::Relaxed);

        // Flip ownership, using release semantics to make sure all writes have
        // become visible to the core we're sending to
        self.mem_pipe.client_owned[self.idx].store(true, Ordering::Release);
    }
}

/// A ticket, used to indicate your order in line for a IPC recv polling
/// operation
pub struct Ticket(u64);

/// The receiving side of a pipe, this will allow you to read sequenced data
/// as it was sent from a `SendPipe`
pub struct RecvPipe<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> {
    /// Reference to the memory pipe
    mem_pipe: *const RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>,

    /// Current sequence index we're looking for
    seq: AtomicU64,
}

unsafe impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> Send for
    RecvPipe<CHUNK_SIZE, NUM_BUFFERS> {}
unsafe impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize> Sync for
    RecvPipe<CHUNK_SIZE, NUM_BUFFERS> {}

impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize>
        RecvPipe<CHUNK_SIZE, NUM_BUFFERS> {
    /// Open a pipe with the given `uid`
    pub fn open(uid: u64) -> Result<Self> {
        // Make sure settings are sane
        if NUM_BUFFERS == 0 || CHUNK_SIZE == 0 {
            return Err(Error::InvalidPipeConfiguration);
        }

        // Get the filename
        let cs = filename_from_uid(uid)?;

        // Open shared memory, only if it already exists
        let shm = unsafe { libc::shm_open(cs.as_ptr(), libc::O_RDWR, 0) };
        if shm == -1 { return Err(Error::ShmOpen(errno())); }

        // Delete the file, it's SPSC. This isn't atomic or for safety, but
        // this is just the earliest we can delete the file so we don't have
        // to keep track of it anymore
        //
        // This also gives us the lowest possible chance of leaking the shm
        unsafe { libc::shm_unlink(cs.as_ptr()); }

        // Read the configuration of the chunk
        unsafe {
            let mut tmp = [0u64; 5];

            // Read the header, which we can use to verify this pipe matches
            // what we expect
            if libc::read(shm, tmp.as_mut_ptr() as *mut _,
                    size_of_val(&tmp)) as usize != size_of_val(&tmp) {
                libc::close(shm);
                return Err(Error::PipeMismatch);
            }

            // Make sure everything matches as we expect
            if tmp[0] != MEMPIPE_MAGIC ||
                    tmp[1] != size_of::<usize>() as u64 ||
                    tmp[2] != CHUNK_SIZE  as u64 ||
                    tmp[3] != NUM_BUFFERS as u64 ||
                    tmp[4] != uid {
                libc::close(shm);
                return Err(Error::PipeMismatch);
            }
        }

        // Map the shared memory
        let mapped = unsafe {
            libc::mmap(std::ptr::null_mut(),
                size_of::<RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>>(),
                libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED,
                shm, 0) as *mut RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>
        };
        let map_err = errno();

        // Close the FD we opened
        unsafe { libc::close(shm); }

        // Make sure mapping as successful
        if mapped as usize == libc::MAP_FAILED as usize {
            return Err(Error::MapMemory(map_err));
        }

        // Return a reference to the memory pipe
        Ok(RecvPipe {
            mem_pipe: mapped,
            seq:      AtomicU64::new(0),
        })
    }

    /// Requests a ticket. By taking a ticket you are saying that you are ready
    /// to process data and will be polling `try_recv`.
    ///
    /// This determines your order in line for processing data
    ///
    /// Dropping a ticket will permanently deadlock the pipe as nobody will
    /// be around to consume the data.
    pub fn request_ticket(&self) -> Ticket {
        // Get a sequence number
        Ticket(self.seq.fetch_add(1, Ordering::Relaxed))
    }

    /// Attempt to receive data from the pipe, invoking the closure only if
    /// data was ready
    ///
    /// Returns either the old ticket (if it didn't process the buffer), or a
    /// new ticket if it did
    ///
    /// Inside the `Ok()` result, there's a `u64` which is the zero-indexed,
    /// atomically incrementing sequence number for the buffer that was
    /// processed. This information allows a parallel user to reorder the
    /// traces until they are sequenced.
    #[allow(clippy::type_complexity)]
    pub fn try_recv<F, T, E>(&self, ticket: Ticket, mut func: F)
                -> (Ticket, Option<std::result::Result<(u64, T), E>>)
            where F: FnMut(&[u8]) -> std::result::Result<T, E> {
        // Get the pipe
        let pipe = unsafe { &*self.mem_pipe };

        // Look for a filled in buffer
        for ii in 0..NUM_BUFFERS {
            // If it's not client owned, skip it
            if !pipe.client_owned[ii].load(Ordering::Acquire) {
                continue;
            }

            // It's client owned, make sure it's the sequence we expect
            if ticket.0 != pipe.client_seq[ii].load(Ordering::Relaxed) {
                continue;
            }

            // Got the sequence we wanted, get the length
            let length = pipe.client_len[ii].load(Ordering::Relaxed);

            // Get a slice to the data
            let data = unsafe {
                std::slice::from_raw_parts(
                    UnsafeCell::raw_get(pipe.chunks[ii].0[0].as_ptr()),
                    length)
            };

            // Invoke the callback, giving the user access to the data
            // temporarily before we give it back to the sender
            match func(data) {
                Ok(resp) => {
                    // Move ownership back to the sender
                    pipe.client_owned[ii].store(false, Ordering::Release);

                    // Processed successfully, generate a new ticket
                    return (self.request_ticket(), Some(Ok((ticket.0, resp))));
                }
                Err(err) => {
                    // Failed to process data, return the ticket back to the
                    // user
                    return (ticket, Some(Err(err)));
                }
            }
        }

        // No buffer was available, give the ticket back
        (ticket, None)
    }
}

impl<const CHUNK_SIZE: usize, const NUM_BUFFERS: usize>
        Drop for RecvPipe<CHUNK_SIZE, NUM_BUFFERS> {
    fn drop(&mut self) {
        // Delete our file
        // Unmap the memory we mapped
        unsafe {
            assert!(libc::munmap(self.mem_pipe as *mut _,
                size_of::<RawMemPipe<CHUNK_SIZE, NUM_BUFFERS>>()) == 0,
                "Failed to munmap() IPC for RecvPipe");
        }
    }
}

#[test]
fn pipe_config() -> Result<()> {
    // We should fail to make a pipe with a mismatched size
    let pipe = SendPipe::<1, 2>::create()?;
    assert!(matches!(RecvPipe::<3, 2>::open(pipe.uid()),
        Err(Error::PipeMismatch)),
        "Whoa, was able to attach to a pipe with the wrong size");

    // We should fail to make a pipe with a mismatched number of buffers
    let pipe = SendPipe::<1, 2>::create()?;
    assert!(matches!(RecvPipe::<1, 3>::open(pipe.uid()),
        Err(Error::PipeMismatch)),
        "Whoa, was able to attach to a pipe with the wrong size");

    // The correct settings should work
    let pipe = SendPipe::<1, 2>::create()?;
    RecvPipe::<1, 2>::open(pipe.uid()).map(|_| ())
}

#[test]
fn large_stack_config() -> Result<()> {
    let _pipe = SendPipe::< { 1024 * 1024 * 1024 }, 2>::create()?;
    Ok(())
}

