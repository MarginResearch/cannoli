//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks, scoped_threads)]

use std::io::Read;
use std::mem::size_of;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
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

/// Given a payload of bytes that came from the IPC channel, deserialize it and
/// invoke callbacks based on the payload
fn parse_payload<T: Cannoli>(user: &T::Context, trace: &mut Vec<T::Trace>,
        mut payload: &[u8]) -> Result<()> {
    /// Gross macro to deserialize multiple plain-old-data types into a tuple
    /// with only one length check.
    ///
    /// This is only safe if you pass in plain-old-data types.
    macro_rules! consume {
        ($($ty:ty),+) => {{
            // Get the size of the payload to deserialize, in bytes
            const OP_SIZE: usize = $(
                size_of::<$ty>() +
            )+ 0;

            // Get pointer to payload
            let ptr = payload.as_ptr();

            // Advance payload pointer, also performs the length check
            payload = payload.get(OP_SIZE..)
                .ok_or(Error::BufferTruncated)?;

            // Create a temporary value tracking how many bytes we've read so
            // far
            let mut _tmp = 0;

            // Ignore clippy here, I don't know if there's a better way to
            // express what we're doing to the compiler in this case
            #[allow(clippy::eval_order_dependence)]
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

    // Clear the trace
    trace.clear();

    // Parse the payload while there's more data
    while !payload.is_empty() {
        // Get the opcode
        let op: u8 = consume!(u8).0;

        // Handle each opcode
        let event = match op {
            0x00 => { // Exec32
                T::exec(user, consume!(u32).0 as u64)
            },
            0x80 => { // Exec64
                T::exec(user, consume!(u64).0)
            },

            0x11 => { // Read8_32
                let (pc, addr, val) = consume!(u32, u32, u8);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x12 => { // Read16_32
                let (pc, addr, val) = consume!(u32, u32, u16);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x14 => { // Read32_32
                let (pc, addr, val) = consume!(u32, u32, u32);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x18 => { // Read64_32
                let (pc, addr, val) = consume!(u32, u32, u64);
                T::read(user, pc as u64, addr as u64, val as u64)
            },

            0x21 => { // Write8_32
                let (pc, addr, val) = consume!(u32, u32, u8);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0x22 => { // Write16_32
                let (pc, addr, val) = consume!(u32, u32, u16);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0x24 => { // Write32_32
                let (pc, addr, val) = consume!(u32, u32, u32);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0x28 => { // Write64_32
                let (pc, addr, val) = consume!(u32, u32, u64);
                T::write(user, pc as u64, addr as u64, val as u64)
            },

            0x91 => { // Read8_64
                let (pc, addr, val) = consume!(u64, u64, u8);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x92 => { // Read16_64
                let (pc, addr, val) = consume!(u64, u64, u16);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x94 => { // Read32_64
                let (pc, addr, val) = consume!(u64, u64, u32);
                T::read(user, pc as u64, addr as u64, val as u64)
            },
            0x98 => { // Read64_64
                let (pc, addr, val) = consume!(u64, u64, u64);
                T::read(user, pc as u64, addr as u64, val as u64)
            },

            0xa1 => { // Write8_64
                let (pc, addr, val) = consume!(u64, u64, u8);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0xa2 => { // Write16_64
                let (pc, addr, val) = consume!(u64, u64, u16);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0xa4 => { // Write32_64
                let (pc, addr, val) = consume!(u64, u64, u32);
                T::write(user, pc as u64, addr as u64, val as u64)
            },
            0xa8 => { // Write64_64
                let (pc, addr, val) = consume!(u64, u64, u64);
                T::write(user, pc as u64, addr as u64, val as u64)
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

/// Handle a newly connected client. This is run on a new thread each time a
/// new TCP connection comes in.
fn handle_client<T: Cannoli + 'static>(
        stream: TcpStream, num_threads: usize, uid: u64) -> Result<()> {
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
    let pipe = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open(uid)
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
    let (user_type, user_ctxt) = T::init(uid);
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

                // Loop forever while the socket is open. This allows us to
                // check if the remote process died, our IPC mechanism doesn't
                // have a way of checking that
                while !matches!(stream.read(&mut scratch_buffer), Ok(0)) {
                    // Attempt to recv a few times using in-memory IPC. This
                    // is just looped to throttle how often we hit the OS by
                    // doing the socket receive above
                    for _ in 0..1000 {
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

                // Get the UID for this connection
                let mut uid = [0u8; size_of::<u64>()];
                stream.read_exact(&mut uid)
                    .expect("Failed to get uid for IPC");
                let uid = u64::from_le_bytes(uid);

                // Handle the client
                handle_client::<T>(stream, threads, uid)
                    .expect("Failed to handle client");
            });
        }

        // All done!
        Ok(())
    })
}

/// Trait which must be implemented by a user to implement their hooks and
/// analysis of a given QEMU trace
pub trait Cannoli<'a>: Send + Sync {
    /// Type to use to convert generic operations into a user-defined
    /// sequential trace buffer
    type Trace: Send;

    /// Context which is passed by shared reference to all functions which are
    /// executed in parallel
    type Context: Sync;

    /// Create a new `Self` for a new IPC session with `uid`. You probably
    /// don't need UID, so you can ignore it, but I provided it anyways.
    fn init(uid: u64) -> (Self, Self::Context) where Self: Sized;

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
    fn exec(_ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace>;

    /// Invoked when a memory load was lifted from the trace
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
    fn read(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
        -> Option<Self::Trace>;

    /// Invoked when a memory store was lifted from the trace
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
    fn write(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
        -> Option<Self::Trace>;

    /// When a new sequential chunk of traces is available, this is invoked.
    /// This is _always_ invoked sequentially, such that the traces could be
    /// concatenated together to get a trace of all execution in-order
    ///
    /// Executed serially. Maybe in different threads, but only one at a time
    /// (hence, mutable access to self)
    fn trace(&mut self, _ctxt: &Self::Context, _trace: &[Self::Trace]) {}
}

///asdf
struct Moose;

use std::sync::Arc;

impl<'a> Cannoli<'a> for Moose {
    type Trace = &'a str;

    type Context = String;

    fn init(_: u64) -> (Self, Self::Context) {
        println!("New moose!");
        (Moose, "asdf".into())
    }

    fn exec(ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace> {
        Some(ctxt.clone())
    }

    fn read(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
            -> Option<Self::Trace> {
        None
    }

    fn write(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
            -> Option<Self::Trace> {
        None
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
    }
}

fn main() -> Result<()> {
    create_cannoli::<Moose>(4)
}

