//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks, scoped_threads)]

use std::io::Read;
use std::ffi::CStr;
use std::mem::{size_of, MaybeUninit};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::time::{Instant, Duration};
use mempipe::RecvPipe;

/// Wrapper around [`Error`]
type Result<T> = std::result::Result<T, Error>;

/// Errors for this crate
#[derive(Debug)]
pub enum Error {
    /// Failed to bind to create the server, waiting for QEMU clients
    Bind(std::io::Error),

    /// Failed to set `nonblocking` on the stream
    SetNonblocking(std::io::Error),

    /// Failed to clone a socket for multiple threads
    CloneSocket(std::io::Error),

    /// Failed to open IPC pipe with QEMU
    OpenPipe(mempipe::Error),

    /// Failed to `join()` with a thread we created
    JoinThread,

    /// A buffer received through IPC didn't have as many bytes in it as were
    /// needed to deserialize it
    BufferTruncated,

    /// An invalid opcode was encountered in the data stream
    InvalidOpcode(u8),
}

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 256 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

/// Header sent when a client connects
///
/// Must only contain plain-old-data otherwise you will break the unsafe
/// serialization and deserialization we use :)
///
/// This also cannot have unaligned fields that cause padding bytes
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClientConn {
    /// UID for the pipe
    pub uid: u64,

    /// Architecture
    pub arch: i32,

    /// Big endian flag
    pub big_endian: i32,

    /// Parent process ID
    pub ppid: i32,

    /// Process ID
    pub pid: i32,

    /// Thread ID
    pub tid: i32,

    /// Length of the parent comm (in bytes)
    pub pcomm_len: u32,

    /// Length of the comm (in bytes)
    pub comm_len: u32,
}

/// Different QEMU target architectures
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Architecture {
    Aarch64,
    Aarch64be,
    Alpha,
    Armv5teb,
    Armv5tel,
    Cris,
    Hexagon,
    I386,
    I686,
    M68k,
    Microblaze,
    Mips,
    Mips64,
    Nios2,
    Openrisc,
    Parisc,
    Ppc,
    Ppc64,
    Ppc64le,
    Riscv32,
    Riscv64,
    S390x,
    Sh4,
    Sparc,
    Sparc64,
    X86_64,
    Xtensa,
}

impl From<i32> for Architecture {
    fn from(val: i32) -> Self {
        match val {
             0 => Self::Aarch64,
             1 => Self::Aarch64be,
             2 => Self::Alpha,
             3 => Self::Armv5teb,
             4 => Self::Armv5tel,
             5 => Self::Cris,
             6 => Self::Hexagon,
             7 => Self::I386,
             8 => Self::I686,
             9 => Self::M68k,
            10 => Self::Microblaze,
            11 => Self::Mips,
            12 => Self::Mips64,
            13 => Self::Nios2,
            14 => Self::Openrisc,
            15 => Self::Parisc,
            16 => Self::Ppc,
            17 => Self::Ppc64,
            18 => Self::Ppc64le,
            19 => Self::Riscv32,
            20 => Self::Riscv64,
            21 => Self::S390x,
            22 => Self::Sh4,
            23 => Self::Sparc,
            24 => Self::Sparc64,
            25 => Self::X86_64,
            26 => Self::Xtensa,
            _  => panic!("Cannoli: Unhandled architecture ID {}", val),
        }
    }
}

impl Architecture {
    /// Convert a `UNAME_MACHINE` C-string to an [`Architecture`]
    ///
    /// # Safety
    ///
    /// Expects a pointer to a valid null-terminatedf string
    pub unsafe fn from_cstr(arch: *const i8) -> Self {
        let arch = CStr::from_ptr(arch).to_str().expect(
            "Cannoli: Invalid string passed to Architecture::from_cstr()");
        match arch {
            "aarch64"    => Architecture::Aarch64,
            "aarch64_be" => Architecture::Aarch64be,
            "alpha"      => Architecture::Alpha,
            "armv5teb"   => Architecture::Armv5teb,
            "armv5tel"   => Architecture::Armv5tel,
            "cris"       => Architecture::Cris,
            "hexagon"    => Architecture::Hexagon,
            "i386"       => Architecture::I386,
            "i686"       => Architecture::I686,
            "m68k"       => Architecture::M68k,
            "microblaze" => Architecture::Microblaze,
            "mips"       => Architecture::Mips,
            "mips64"     => Architecture::Mips64,
            "nios2"      => Architecture::Nios2,
            "openrisc"   => Architecture::Openrisc,
            "parisc"     => Architecture::Parisc,
            "ppc"        => Architecture::Ppc,
            "ppc64"      => Architecture::Ppc64,
            "ppc64le"    => Architecture::Ppc64le,
            "riscv32"    => Architecture::Riscv32,
            "riscv64"    => Architecture::Riscv64,
            "s390x"      => Architecture::S390x,
            "sh4"        => Architecture::Sh4,
            "sparc"      => Architecture::Sparc,
            "sparc64"    => Architecture::Sparc64,
            "x86_64"     => Architecture::X86_64,
            "xtensa"     => Architecture::Xtensa,
            _ => panic!("Cannoli: Unhandled architecture name {}", arch),
        }
    }
}

/// Gross macro to deserialize multiple plain-old-data types into a tuple
/// with only one length check.
///
/// This is only safe if you pass in plain-old-data types.
macro_rules! consume {
    ($payload:expr, $($ty:ty),+) => {{
        // Get the size of the payload to deserialize, in bytes
        const OP_SIZE: usize = $(
            size_of::<$ty>() +
        )+ 0;

        // Get pointer to payload
        let ptr = $payload.as_ptr();

        // Advance payload pointer, also performs the length check
        $payload = $payload.get(OP_SIZE..)
            .ok_or(Error::BufferTruncated)?;

        // Create a temporary value tracking how many bytes we've read so
        // far
        let mut _tmp = 0;

        // Ignore clippy here, I don't know if there's a better way to
        // express what we're doing to the compiler in this case
        #[allow(clippy::mixed_read_write_in_expression)]
        unsafe {
            (
                // For each requested type, read it!
                $(
                    core::ptr::read_unaligned(
                        ptr.offset({
                            // Save off the current offset
                            let x = _tmp;

                            // Advance the index
                            _tmp += size_of::<$ty>() as isize;

                            // Return the offset to read from
                            x
                        }) as *const $ty),
                )+
            )
        }
    }}
}

/// Given a payload of bytes that came from the IPC channel, deserialize it and
/// invoke callbacks based on the payload
fn parse_payload<T: Cannoli>(user: &T::Context, trace: &mut Vec<T::Trace>,
        mut payload: &[u8]) -> Result<()> {
    // Clear the trace
    trace.clear();

    // Parse the payload while there's more data
    while !payload.is_empty() {
        // Get the opcode
        let op: u8 = consume!(payload, u8).0;

        // Handle each opcode
        let event = match op {
            0x00 => { // Exec32
                T::exec(user, consume!(payload, u32).0 as u64)
            },
            0x80 => { // Exec64
                T::exec(user, consume!(payload, u64).0)
            },

            0x11 => { // Read8_32
                let (addr, val, pc) = consume!(payload, u32, u8, u32);
                T::read(user, pc as u64, addr as u64, val as u64, 1)
            },
            0x12 => { // Read16_32
                let (addr, val, pc) = consume!(payload, u32, u16, u32);
                T::read(user, pc as u64, addr as u64, val as u64, 2)
            },
            0x14 => { // Read32_32
                let (addr, val, pc) = consume!(payload, u32, u32, u32);
                T::read(user, pc as u64, addr as u64, val as u64, 4)
            },
            0x18 => { // Read64_32
                let (addr, val, pc) = consume!(payload, u32, u64, u32);
                T::read(user, pc as u64, addr as u64, val as u64, 8)
            },

            0x21 => { // Write8_32
                let (addr, val, pc) = consume!(payload, u32, u8, u32);
                T::write(user, pc as u64, addr as u64, val as u64, 1)
            },
            0x22 => { // Write16_32
                let (addr, val, pc) = consume!(payload, u32, u16, u32);
                T::write(user, pc as u64, addr as u64, val as u64, 2)
            },
            0x24 => { // Write32_32
                let (addr, val, pc) = consume!(payload, u32, u32, u32);
                T::write(user, pc as u64, addr as u64, val as u64, 4)
            },
            0x28 => { // Write64_32
                let (addr, val, pc) = consume!(payload, u32, u64, u32);
                T::write(user, pc as u64, addr as u64, val as u64, 8)
            },

            0x91 => { // Read8_64
                let (addr, val, pc) = consume!(payload, u64, u8, u64);
                T::read(user, pc as u64, addr as u64, val as u64, 1)
            },
            0x92 => { // Read16_64
                let (addr, val, pc) = consume!(payload, u64, u16, u64);
                T::read(user, pc as u64, addr as u64, val as u64, 2)
            },
            0x94 => { // Read32_64
                let (addr, val, pc) = consume!(payload, u64, u32, u64);
                T::read(user, pc as u64, addr as u64, val as u64, 4)
            },
            0x98 => { // Read64_64
                let (addr, val, pc) = consume!(payload, u64, u64, u64);
                T::read(user, pc as u64, addr as u64, val as u64, 8)
            },

            0xa1 => { // Write8_64
                let (addr, val, pc) = consume!(payload, u64, u8, u64);
                T::write(user, pc as u64, addr as u64, val as u64, 1)
            },
            0xa2 => { // Write16_64
                let (addr, val, pc) = consume!(payload, u64, u16, u64);
                T::write(user, pc as u64, addr as u64, val as u64, 2)
            },
            0xa4 => { // Write32_64
                let (addr, val, pc) = consume!(payload, u64, u32, u64);
                T::write(user, pc as u64, addr as u64, val as u64, 4)
            },
            0xa8 => { // Write64_64
                let (addr, val, pc) = consume!(payload, u64, u64, u64);
                T::write(user, pc as u64, addr as u64, val as u64, 8)
            },

            _ => {
                // Invalid opcode
                return Err(Error::InvalidOpcode(op));
            },
        };

        // Log in the user's trace format as requested
        if let Some(event) = event {
            trace.push(event);
        }
    }

    Ok(())
}

/// Information about a newly connected client
#[derive(Debug, Clone)]
pub struct ClientInfo {
    /// UID of the communication stream
    pub uid: u64,

    /// Architecture of the target
    pub arch: Architecture,

    /// Is the target big endian?
    pub big_endian: bool,

    /// Parent process ID
    pub ppid: i32,

    /// Process ID
    pub pid: i32,

    /// Thread ID
    pub tid: i32,

    /// Parent comm, `/proc/ppid/comm`, this is the raw value read from `comm`
    /// and may include weird stuff like newlines
    pub pcomm: Option<String>,

    /// comm, `/proc/pid/comm`, this is the raw value read from `comm`
    /// and may include weird stuff like newlines
    pub comm: Option<String>,
}

/// Handle a newly connected client. This is run on a new thread each time a
/// new TCP connection comes in.
fn handle_client<T: Cannoli + 'static>(
        stream: TcpStream, num_threads: usize, ci: &ClientInfo) -> Result<()> {
    /// Storage for the mini state-machine we use to sequence traces
    struct State<T: Cannoli + 'static> {
        /// Next sequence number we are looking for to report traces
        next_seq: u64,

        /// Vector of traces, maintained sorted, with a sequence identifer in
        /// the first part of the tuple
        traces: Vec<(u64, Vec<T::Trace>)>,

        /// User's [`Cannoli`]-implementing type
        user: T,
    }

    // Create the IPC connection to the UID we got
    let pipe = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open(ci.uid)
        .map_err(Error::OpenPipe)?;

    // Make the stream nonblocking
    stream.set_nonblocking(true)
        .map_err(Error::SetNonblocking)?;

    // Scratch buffer to check for socket close
    let mut scratch_buffer = [0u8; 1];

    // Get a reference to the pipe so we can `move` the reference into the
    // threads we create
    let pipe = &pipe;

    // Create a new instance of the user's structure
    let (user_type, user_ctxt) = T::init(ci);
    let user_ctxt = &user_ctxt;

    // Create the sequencing state machine
    let state = Mutex::new(State {
        next_seq: 0,
        traces:   Vec::new(),
        user:     user_type,
    });
    let state = &state;

    // Create a thread scope
    std::thread::scope(|s| -> Result<()> {
        // Holds the handles to the threads we create
        let mut threads = Vec::new();

        // Create the number of threads requested
        for _ in 0..num_threads {
            // Clone the stream so we can checked for a closed socked on
            // multiple threads
            let mut stream = stream.try_clone().map_err(Error::CloneSocket)?;

            // Create the IPC reader thread!
            threads.push(s.spawn(move || -> Result<()> {
                // Buffer for trace results
                let mut trace = Vec::new();

                // Current ticket for getting a trace
                let mut ticket = Some(pipe.request_ticket());

                // The last time this thread read data
                let mut last_data = Instant::now();

                // Loop forever while the socket is open. This allows us to
                // check if the remote process died, our IPC mechanism doesn't
                // have a way of checking that
                while !matches!(stream.read(&mut scratch_buffer), Ok(0)) {
                    // If we haven't gotten any data recent, sleep a bit before
                    // hot polling. This prevents us completely eating 100% CPU
                    // when there are threads connected to us but not streaming
                    // us new data (eg, a client called `sleep()`)
                    if last_data.elapsed() >= Duration::from_millis(20) {
                        std::thread::sleep(Duration::from_millis(5));
                    }

                    // Poll via shared memory while we keep getting stuff
                    let mut hot_poll = 10000;
                    while hot_poll > 0 {
                        // Update that we did a hot poll
                        hot_poll -= 1;

                        // Attempt to get a payload from the pipe, parse it if
                        // there was one
                        let (new_ticket, payload) = pipe.try_recv(
                            ticket.take().unwrap(),
                            |x| parse_payload::<T>(user_ctxt, &mut trace, x));

                        // Replace the ticket with the new ticket
                        ticket = Some(new_ticket);

                        // Process the result if we parsed a payload
                        if let Some(payload) = payload {
                            // It's possible payload parsing failed, so check
                            // the error
                            let (seq, _) = payload?;

                            // Refresh hot polling
                            hot_poll = 10000;
                            last_data = Instant::now();

                            // Yay, we got a trace!
                            //
                            // This isn't super optimized, but due to the
                            // batching of chunks, the costs don't really
                            // matter too much, at least, not from my
                            // measurements. Just naively keep the buffers
                            // sorted, and report all of them in sequence when
                            // possible.
                            let mut state = state.lock().unwrap();

                            // Find the correct trace index
                            let idx = match state.traces
                                    .binary_search_by_key(&seq, |x| x.0) {
                                Ok(idx) | Err(idx) => idx,
                            };

                            // Insert the trace!
                            state.traces.insert(idx, (seq, trace));

                            // Report traces in order
                            while !state.traces.is_empty() &&
                                    state.next_seq == state.traces[0].0 {
                                // Update the reporting sequence
                                state.next_seq =
                                    state.next_seq.wrapping_add(1);

                                // Remove the entry from traces
                                let trace = state.traces.remove(0).1;

                                // Report the trace
                                state.user.trace(user_ctxt, &trace);
                            }

                            // Get a new trace buffer since we moved ours into
                            // the traces log
                            trace = Vec::new();
                        }
                    }
                }

                Ok(())
            }));
        }

        // Check threads for errors
        for thr in threads {
            thr.join().ok().ok_or(Error::JoinThread)??;
        }

        // We did everything we wanted!
        Ok(())
    })
}

/// Create a new Cannoli server. This will spin up the required processing
/// needed to talk with QEMU and deserialize messages, while dispatching
/// callbacks to a user-controlled `user_self`
///
/// Create `threads` number of threads for every connection that comes in.
/// These threads will handle all Cannoli parsing and callbacks
pub fn create_cannoli<T: Cannoli + 'static>(threads: usize) -> Result<()> {
    // Create socket, waiting for clients to connect and inform us about some
    // memory regions
    let listener = TcpListener::bind("127.0.0.1:11458")
        .map_err(Error::Bind)?;

    // Create a new thread scope for handling connections
    std::thread::scope(|scope| {
        // Wait for connections
        for stream in listener.incoming() {
            // Spawn a thread on new connections
            scope.spawn(move || {
                // Get access to the stream
                let mut stream = stream.expect("Failed to get TCP stream");

                // Get the header
                let mut header: MaybeUninit<ClientConn> =
                    MaybeUninit::uninit();
                stream.read_exact(unsafe {
                    core::slice::from_raw_parts_mut(
                        header.as_mut_ptr() as *mut u8,
                        core::mem::size_of_val(&header))
                }).expect("Failed to get client header");

                // Get the actual header now that it's initialized
                let header: ClientConn = unsafe { header.assume_init() };

                // Get the pcomm and comm
                let mut comm =
                    vec![0u8; header.pcomm_len as usize +
                              header.comm_len  as usize];
                stream.read_exact(&mut comm)
                    .expect("Failed to get client pcomm and comm");

                // Construct client information
                let ci = ClientInfo {
                    // IPC pipe UID
                    uid: header.uid,

                    // Architecture
                    arch: Architecture::from(header.arch),

                    // Endianness
                    big_endian: header.big_endian != 0,

                    // PIDs
                    ppid: header.ppid,
                    pid:  header.pid,
                    tid:  header.tid,

                    pcomm: std::str::from_utf8(
                        &comm[..header.pcomm_len as usize])
                        .ok().map(|x| x.to_string()),
                    comm: std::str::from_utf8(
                        &comm[header.pcomm_len as usize..])
                        .ok().map(|x| x.to_string()),
                };

                println!("New client {:#?}", ci);

                // Handle the client
                handle_client::<T>(stream, threads, &ci)
                    .expect("Failed to handle client");

                println!("Lost client {:#?}", ci);
            });
        }

        // All done!
        Ok(())
    })
}

/// Trait which must be implemented by a user to implement their hooks and
/// analysis of a given QEMU trace
pub trait Cannoli: Send + Sync {
    /// Type to use to convert generic operations into a user-defined
    /// sequential trace buffer
    type Trace: Send;

    /// Context which is passed by shared reference to all functions which are
    /// executed in parallel
    type Context: Sync;

    /// Create a new `Self` for a new IPC session. See [`ClientInfo`] for the
    /// information you are provided
    fn init(ci: &ClientInfo) -> (Self, Self::Context) where Self: Sized;

    /// Invoked when a PC execution opcode was lifted from the trace
    ///
    /// Executed on multiple threads
    ///
    /// This is a high-performance parallel callback, and is a prime location
    /// for adding code if you need to do processing unrelated to the flow of
    /// a trace itself. For example, symbolizing a trace makes the most sense
    /// here as it doesn't care about the instructions around it. Think of this
    /// like a `filter_map` where it applies a transformation in parallel,
    /// and potentially removing information from the trace
    ///
    /// Part of the parallel phase of trace processing. Since multiple threads
    /// are processing traces, the order of the events are not stable. This
    /// function is only meant to reason about `pc` in isolation, not with
    /// respect to previous operations.
    fn exec(_ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace> { None }

    /// Invoked when a memory load was lifted from the trace with a given
    /// access size in bytes
    ///
    /// Executed on multiple threads
    ///
    /// This is a high-performance parallel callback, and is a prime location
    /// for adding code if you need to do processing unrelated to the flow of
    /// a trace itself. For example, symbolizing a trace makes the most sense
    /// here as it doesn't care about the instructions around it. Think of this
    /// like a `filter_map` where it applies a transformation in parallel,
    /// and potentially removing information from the trace
    ///
    /// Part of the parallel phase of trace processing. Since multiple threads
    /// are processing traces, the order of the events are not stable. This
    /// function is only meant to reason about the arguments in isolation,
    /// not with respect to previous operations.
    fn read(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64, _sz: u8)
        -> Option<Self::Trace> { None }

    /// Invoked when a memory store was lifted from the trace with a given
    /// access size in bytes
    ///
    /// Executed on multiple threads
    ///
    /// This is a high-performance parallel callback, and is a prime location
    /// for adding code if you need to do processing unrelated to the flow of
    /// a trace itself. For example, symbolizing a trace makes the most sense
    /// here as it doesn't care about the instructions around it. Think of this
    /// like a `filter_map` where it applies a transformation in parallel,
    /// and potentially removing information from the trace
    ///
    /// Part of the parallel phase of trace processing. Since multiple threads
    /// are processing traces, the order of the events are not stable. This
    /// function is only meant to reason about the arguments in isolation,
    /// not with respect to previous operations.
    fn write(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64, _sz: u8)
        -> Option<Self::Trace> { None }

    /// When a new sequential chunk of traces is available, this is invoked.
    /// This is _always_ invoked sequentially, such that the traces could be
    /// concatenated together to get a trace of all execution in-order
    ///
    /// Executed serially. Maybe in different threads, but only one at a time
    /// (hence, mutable access to self)
    fn trace(&mut self, _ctxt: &Self::Context, _trace: &[Self::Trace]) {}
}

