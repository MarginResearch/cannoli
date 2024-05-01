//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks, lazy_cell)]

use std::io::Read;
use std::any::Any;
use std::ffi::CStr;
use std::mem::{size_of, MaybeUninit};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, LazyLock};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use mempipe::RecvPipe;

pub use arch::{HostSyscallNum, TargetSyscallNum, SyscallParamType};

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

    /// Getting the path for mmap() did not contain valid UTF-8 characters
    PathEncoding(std::str::Utf8Error),
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

    pub fn bitness(&self) -> u8 {
        match self {
            Architecture::Aarch64     => 64,
            Architecture::Aarch64be   => 64,
            Architecture::Alpha       => 32,
            Architecture::Armv5teb    => 64,
            Architecture::Armv5tel    => 64,
            Architecture::Cris        => 32,
            Architecture::Hexagon     => 32,
            Architecture::I386        => 32,
            Architecture::I686        => 32,
            Architecture::M68k        => 32,
            Architecture::Microblaze  => 32,
            Architecture::Mips        => 32,
            Architecture::Mips64      => 64,
            Architecture::Nios2       => 32,
            Architecture::Openrisc    => 32,
            Architecture::Parisc      => 32,
            Architecture::Ppc         => 32,
            Architecture::Ppc64       => 64,
            Architecture::Ppc64le     => 64,
            Architecture::Riscv32     => 32,
            Architecture::Riscv64     => 64,
            Architecture::S390x       => 32,
            Architecture::Sh4         => 32,
            Architecture::Sparc       => 32,
            Architecture::Sparc64     => 64,
            Architecture::X86_64      => 64,
            Architecture::Xtensa      => 32,
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
            .ok_or_else(|| Error::BufferTruncated)?;

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

/// helper functions to handle consuming from buffer, since the consume macro
/// can throw an error which is not tolerated by functions which have a defined
/// return type (such as from<T>(t: T) -> S {}
fn get_u8(v: &mut &[u8]) -> Result<u8> {
    Ok(consume!(*v, u8).0)
}

fn get_u32(v: &mut &[u8]) -> Result<u32> {
    Ok(consume!(*v, u32).0)
}

fn get_u64(v: &mut &[u8]) -> Result<u64> {
    Ok(consume!(*v, u64).0)
}

/// types for syscall arguments and rets. Depending on the syscalls required,
/// this list should be updated as needed
#[derive(Debug)]
pub enum SyscallParam<'a> {
    Na,
    Int { val: u64 },   // ints are always u64 for simplicity
    Bool { val: bool }, // bools and chars are a single byte
    Char { val: char },

    /// the buffer format for RawPtr and CStr data types is as follows:
    ///    u64      u32       data len
    /// .______.__________.____________.
    /// | addr | data len |    data    |
    /// `------`----------`------------`
    ///
    RawPtr { addr: u64, data: Vec<u8> },
    CStr { addr: u64, data: &'a CStr },
}

impl From<&mut &[u8]> for SyscallParam<'_> {
    fn from(mut v: &mut &[u8]) -> Self {
        let t = SyscallParamType::from(
            get_u8(&mut v).unwrap());
        match t {
            SyscallParamType::Na => SyscallParam::Na,
            SyscallParamType::Bool => {
                let val = get_u8(&mut v).unwrap() != 0;
                SyscallParam::Bool { val }
            },
            SyscallParamType::Char => {
                let val = get_u8(&mut v).unwrap();
                SyscallParam::Char { val: val as char }
            },
            SyscallParamType::Int => {
                let val = get_u64(&mut v).unwrap();
                SyscallParam::Int { val }
            },
            SyscallParamType::RawPtr => {
                let addr = get_u64(&mut v).unwrap();
                let data_len = get_u32(&mut v).unwrap();
                let data = &v[..data_len as usize];
                *v = &v[data_len as usize..];
                SyscallParam::RawPtr {
                    addr,
                    data: data.to_vec(),
                }
            },
            _ => panic!("unmatched SyscallParamType {:?}", t),
        }
    }
}

/// handler functions invoked by the cannoli->syscall callback defined in
/// jitter/src/cannoli_internals.rs. These are defined separately to make the
/// source code in cannoli_internals.rs more concise
///
/// this is an evergreen list and each syscall of interest must implement a
/// unique function to handle its arguments and return value, since those vary
/// between syscalls
///
/// the buffer format for the return value and args is defined above
pub fn syscall_read(buf: &mut Vec<u8>, params: &Vec<u64>, guest_base: u64) {
    // params and types
    // ret: Int, fd: Int, buf: RawPtr, count: Int

    // return
    let ret = params[0];
    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&ret.to_le_bytes());

    // arguments
    buf.extend_from_slice(&(3u32).to_le_bytes()); // number of args

    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&(params[1]).to_le_bytes());

    buf.push(SyscallParamType::RawPtr as u8);
    // addr from syscall callback is target address (good). Push this not ptr,
    // which is the qemu host address where the actual memory is copied
    buf.extend_from_slice(&(params[2]).to_le_bytes());
    let ptr = (params[2] + guest_base) as *mut u8;
    buf.extend_from_slice(&(ret as u32).to_le_bytes()); // raw data len
    unsafe {
        for i in 0..ret {
            buf.extend_from_slice(
                &(*ptr.add(i.try_into().unwrap())).to_le_bytes());
        }
    }

    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&(params[3]).to_le_bytes());
}

pub fn syscall_getrandom(buf: &mut Vec<u8>, params: &mut Vec<u64>,
                         guest_base: u64) {
    // params and types
    // ret: Int, buf: RawPtr, buflen: Int, flags: Int

    // return
    let ret = params[0];
    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&ret.to_le_bytes());

    // arguments
    buf.extend_from_slice(&(3u32).to_le_bytes()); // number of args

    buf.push(SyscallParamType::RawPtr as u8);
    // addr from syscall callback is target address (good). Push this not ptr,
    // which is the qemu host address where the actual memory is copied
    buf.extend_from_slice(&(params[1]).to_le_bytes());
    let ptr = (params[1] + guest_base) as *mut u8;
    buf.extend_from_slice(&(ret as u32).to_le_bytes()); // raw data len
    unsafe {
        for i in 0..ret {
            buf.extend_from_slice(
                &(*ptr.add(i.try_into().unwrap())).to_le_bytes());
        }
    }

    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&(params[2]).to_le_bytes());

    buf.push(SyscallParamType::Int as u8);
    buf.extend_from_slice(&(params[3]).to_le_bytes());
}

/// Given a payload of bytes that came from the IPC channel, deserialize it and
/// invoke callbacks based on the payload
fn parse_payload<T: Cannoli>(pid: &T::PidContext, tid: &T::TidContext,
        trace: &mut Vec<T::Trace>, mut payload: &[u8]) -> Result<()> {
    // Clear the trace
    trace.clear();

    // Parse the payload while there's more data
    while !payload.is_empty() {
        // Get the opcode
        let op: u8 = consume!(payload, u8).0;

        // Handle each opcode
        match op {
            0x00 => { // Exec32
                T::exec(pid, tid, consume!(payload, u32).0 as u64, trace)
            },
            0x80 => { // Exec64
                T::exec(pid, tid, consume!(payload, u64).0, trace)
            },

            0x01 => { // Regs32
                let size = consume!(payload, u32).0;
                let pc   = consume!(payload, u32).0 as u64;
                let regs = &payload[..size as usize];
                payload = &payload[size as usize..];
                T::regs(pid, tid, pc, regs, trace)
            },
            0x81 => { // Regs64
                let size = consume!(payload, u32).0;
                let pc   = consume!(payload, u64).0;
                let regs = &payload[..size as usize];
                payload = &payload[size as usize..];
                T::regs(pid, tid, pc, regs, trace)
            },

            0x30 => { // Mmap32
                let (addr, len, anon, read, write, exec, path_len, offset) =
                    consume!(payload, u32, u32, u8, u8, u8, u8, u32, u32);
                let path = core::str::from_utf8(
                    payload.get(..path_len as usize)
                    .ok_or(Error::BufferTruncated)?)
                    .map_err(Error::PathEncoding)?;
                payload = &payload[path_len as usize..];

                T::mmap(pid, tid, addr as u64, len as u64, anon != 0, read != 0,
                    write != 0, exec != 0, path, offset as u64, trace)
            },
            0x31 => { // Munmap32
                let (addr, len) = consume!(payload, u32, u32);
                T::munmap(pid, tid, addr as u64, len as u64, trace)
            },
            0x50 => { // syscall32
                let mut params: Vec<SyscallParam> = Vec::new();
                let target_num = TargetSyscallNum::from(
                    consume!(payload, u32).0);
                let num = HostSyscallNum::from(target_num);

                // ensure syscall has data
                if num == HostSyscallNum::NotHandled { return Ok(()); }
                // parse ret
                let ret = SyscallParam::from(&mut payload);

                // parse arguments
                let arg_count = consume!(payload, u32).0;
                for _ in 0..arg_count {
                    let j = SyscallParam::from(&mut payload);
                    params.push(j);
                }
                T::syscall(pid, tid, num, ret, &params, trace)
            },
            0xb0 => { // Mmap64
                let (addr, len, anon, read, write, exec, path_len, offset) =
                    consume!(payload, u64, u64, u8, u8, u8, u8, u32, u64);
                let path = core::str::from_utf8(
                    payload.get(..path_len as usize)
                    .ok_or(Error::BufferTruncated)?)
                    .map_err(Error::PathEncoding)?;
                payload = &payload[path_len as usize..];

                T::mmap(pid, tid, addr, len, anon != 0, read != 0,
                    write != 0, exec != 0, path, offset, trace)
            },
            0xb1 => { // Munmap64
                let (addr, len) = consume!(payload, u64, u64);
                T::munmap(pid, tid, addr, len, trace)
            },
            0xd0 => { // syscall64
                let mut params: Vec<SyscallParam> = Vec::new();
                let target_num = TargetSyscallNum::from(
                    consume!(payload, u32).0);
                let num = HostSyscallNum::from(target_num);

                // ensure syscall has data
                if num == HostSyscallNum::NotHandled { return Ok(()); }
                // parse ret
                let ret = SyscallParam::from(&mut payload);

                // parse arguments
                let arg_count = consume!(payload, u32).0;
                for _ in 0..arg_count {
                    let j = SyscallParam::from(&mut payload);
                    params.push(j);
                }
                T::syscall(pid, tid, num, ret, &params, trace)
           },

            0x11 => { // Read8_32
                let (addr, val, pc) = consume!(payload, u32, u8, u32);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 1, trace)
            },
            0x12 => { // Read16_32
                let (addr, val, pc) = consume!(payload, u32, u16, u32);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 2, trace)
            },
            0x14 => { // Read32_32
                let (addr, val, pc) = consume!(payload, u32, u32, u32);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 4, trace)
            },
            0x18 => { // Read64_32
                let (addr, val, pc) = consume!(payload, u32, u64, u32);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 8, trace)
            },

            0x21 => { // Write8_32
                let (addr, val, pc) = consume!(payload, u32, u8, u32);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 1, trace)
            },
            0x22 => { // Write16_32
                let (addr, val, pc) = consume!(payload, u32, u16, u32);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 2, trace)
            },
            0x24 => { // Write32_32
                let (addr, val, pc) = consume!(payload, u32, u32, u32);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 4, trace)
            },
            0x28 => { // Write64_32
                let (addr, val, pc) = consume!(payload, u32, u64, u32);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 8, trace)
            },

            0x91 => { // Read8_64
                let (addr, val, pc) = consume!(payload, u64, u8, u64);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 1, trace)
            },
            0x92 => { // Read16_64
                let (addr, val, pc) = consume!(payload, u64, u16, u64);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 2, trace)
            },
            0x94 => { // Read32_64
                let (addr, val, pc) = consume!(payload, u64, u32, u64);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 4, trace)
            },
            0x98 => { // Read64_64
                let (addr, val, pc) = consume!(payload, u64, u64, u64);
                T::read(pid, tid, pc as u64, addr as u64, val as u64, 8, trace)
            },

            0xa1 => { // Write8_64
                let (addr, val, pc) = consume!(payload, u64, u8, u64);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 1, trace)
            },
            0xa2 => { // Write16_64
                let (addr, val, pc) = consume!(payload, u64, u16, u64);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 2, trace)
            },
            0xa4 => { // Write32_64
                let (addr, val, pc) = consume!(payload, u64, u32, u64);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 4, trace)
            },
            0xa8 => { // Write64_64
                let (addr, val, pc) = consume!(payload, u64, u64, u64);
                T::write(pid, tid, pc as u64, addr as u64,
                    val as u64, 8, trace)
            },
            0x40 => { // Branch32
                let size = consume!(payload, u32).0;
                let pc   = consume!(payload, u32).0 as u64;
                let branch = consume!(payload, u8).0 != 0;
                let regs = &payload[..size as usize];
                payload = &payload[size as usize..];
                T::branch(pid, tid, pc, branch, regs, trace)
            },
            0xc0 => { // Branch64
                let size = consume!(payload, u32).0;
                let pc   = consume!(payload, u64).0;
                let branch = consume!(payload, u8).0 != 0;
                let regs = &payload[..size as usize];
                payload = &payload[size as usize..];
                T::branch(pid, tid, pc, branch, regs, trace)
            },
            _ => {
                // Invalid opcode
                return Err(Error::InvalidOpcode(op));
            },
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
fn handle_client<T>(
        stream: TcpStream, num_threads: usize, ci: &ClientInfo) -> Result<()>
            where T: Cannoli + 'static,
                  T::PidContext: Send + Sync + 'static {
    /// Storage for PID contexts, keyed by target process ID
    static PID_CONTEXTS: LazyLock<Mutex<
            HashMap<i32, Arc<dyn Any + Send + Sync>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

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

    // Get the PID context
    let any_pid_context: Arc<dyn Any + Send + Sync> = {
        // Get the contexts
        let mut contexts = PID_CONTEXTS.lock().unwrap();

        // Either get the existing context or create a new one
        contexts.entry(ci.pid).or_insert_with(|| {
            T::init_pid(ci)
        }).clone()
    };

    // Get the PID context with the correct type
    let pid_context = any_pid_context.downcast_ref::<T::PidContext>().unwrap();

    // Create a new instance of the user's structure
    let (user_type, user_ctxt) = T::init_tid(&*pid_context, ci);
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
                            |x| parse_payload::<T>(
                                &*pid_context, user_ctxt, &mut trace, x));

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
                            let cap = trace.capacity();
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
                                state.user.trace(&*pid_context,
                                    user_ctxt, &trace);
                            }

                            // Drop the lock and re-allocate the trace buffer
                            std::mem::drop(state);
                            trace = Vec::with_capacity(cap);
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

        Ok(())
    })?;

    // Potentially delete the PID from the global database, we have to detect
    // when all threads are exited, this is kinda gross but whatever
    {
        // Drop the PID context
        drop(any_pid_context);

        // Get access to contexts to see if we've dropped all references to
        // this pid.
        let mut contexts = PID_CONTEXTS.lock().unwrap();
        if Arc::strong_count(&contexts[&ci.pid]) == 1 {
            contexts.remove(&ci.pid);
        }
    }

    // We did everything we wanted!
    Ok(())
}

/// Create a new Cannoli server and handle clients in parallel.
///
/// This will spin up the required processing needed to talk with QEMU and
/// deserialize messages, while dispatching callbacks to a user-defined `T`
/// which implements [`Cannoli`].
///
/// Create `threads` number of threads for every connection that comes in.
/// These threads will handle all Cannoli parsing and callbacks
///
/// See [`run`] to serve a single client.
pub fn run_forever<T>(threads: usize) -> Result<()>
        where T: Cannoli + 'static,
              T::PidContext: Send + Sync + 'static {
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
                handle_connection::<T>(stream.expect("Failed to get TCP stream"), threads);
            });
        }

        // All done!
        Ok(())
    })
}

/// Create a new Cannoli server and handle one client.
///
/// This will spin up the required processing needed to talk with QEMU and
/// deserialize messages, while dispatching callbacks to a user-defined `T`
/// which implements [`Cannoli`].
///
/// Create `threads` number of threads for the connection that comes in.
/// These threads will handle all Cannoli parsing and callbacks
///
/// See [`run_forever`] to serve multiple clients in parallel.
pub fn run<T>(threads: usize) -> Result<()>
        where T: Cannoli + 'static,
              T::PidContext: Send + Sync + 'static {
    // Create socket, waiting for clients to connect and inform us about some
    // memory regions
    let listener = TcpListener::bind("127.0.0.1:11458")
        .map_err(Error::Bind)?;

    // Wait for connection
    let (stream, _from) = listener.accept().unwrap();

    // Handle one connection then exit
    handle_connection::<T>(stream, threads);

    // All done!
    Ok(())
}

fn handle_connection<T>(mut stream: TcpStream, threads: usize) where T: Cannoli + 'static {
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

    // Handle the client
    handle_client::<T>(stream, threads, &ci)
        .expect("Failed to handle client");
}

/// Trait which must be implemented by a user to implement their hooks and
/// analysis of a given QEMU trace
pub trait Cannoli: Send + Sync {
    /// Type to use to convert generic operations into a user-defined
    /// sequential trace buffer
    type Trace: Send;

    /// Context which is shared between all threads in the target process and
    /// all trace processing threads.
    ///
    /// TL;DR: This context is target-PID specific
    type PidContext: Send + Sync;

    /// Context which is passed by shared reference to all functions which are
    /// executed in parallel
    ///
    /// This is shared between multiple threads which are processing the trace
    /// from a single QEMU thread. Thus, this is _not_ shared storage between
    /// target threads
    ///
    /// TL;DR: This context is target-TID specific
    type TidContext: Sync;

    /// Called when the first thread of a given target PID is connected, this
    /// creates the `PidContext` which is then shared with all threads
    /// for a given PID
    fn init_pid(ci: &ClientInfo) -> Arc<Self::PidContext> where Self: Sized;

    /// Create a new `Self` for a new IPC session. See [`ClientInfo`] for the
    /// information you are provided. This is created when a new thread is
    /// created in QEMU
    fn init_tid(pid: &Self::PidContext, ci: &ClientInfo)
        -> (Self, Self::TidContext) where Self: Sized;

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
    fn exec(_pid: &Self::PidContext, _tid: &Self::TidContext, _pc: u64,
            _trace: &mut Vec<Self::Trace>) {}

    /// Invoked when execution of an instruction with register tracing occurs
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
    fn regs(_pid: &Self::PidContext, _tid: &Self::TidContext,
            _pc: u64, _regs: &[u8],
            _trace: &mut Vec<Self::Trace>) {}

    /// Invoked when execution of an instruction with branch tracing occurs
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
    fn branch(_pid: &Self::PidContext, _tid: &Self::TidContext,
            _pc: u64, _branch: bool, _regs: &[u8],
            _trace: &mut Vec<Self::Trace>) {}


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
    fn read(_pid: &Self::PidContext, _tid: &Self::TidContext,
            _pc: u64, _addr: u64, _val: u64, _sz: u8,
            _trace: &mut Vec<Self::Trace>) {}

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
    fn write(_pid: &Self::PidContext, _tid: &Self::TidContext,
             _pc: u64, _addr: u64,
             _val: u64, _sz: u8,
             _trace: &mut Vec<Self::Trace>) {}

    /// When a new sequential chunk of traces is available, this is invoked.
    /// This is _always_ invoked sequentially, such that the traces could be
    /// concatenated together to get a trace of all execution in-order
    ///
    /// Executed serially. Maybe in different threads, but only one at a time
    /// (hence, mutable access to self)
    fn trace(&mut self, _pid: &Self::PidContext,
        _tid: &Self::TidContext, _trace: &[Self::Trace]) {}

    /// Invoked after a _successful_ mmap() in the target application, provides
    /// the base address, length, anon state, read, write, and exec flags
    fn mmap(_pid: &Self::PidContext, _tid: &Self::TidContext,
        _base: u64, _len: u64, _anon: bool,
        _read: bool, _write: bool, _exec: bool, _path: &str, _offset: u64,
        _trace: &mut Vec<Self::Trace>) {}

    /// Invoked when the guest is attempting to `munmap()` memory
    fn munmap(_pid: &Self::PidContext, _tid: &Self::TidContext,
              _base: u64, _len: u64,
              _trace: &mut Vec<Self::Trace>) {}

    /// Invoked when the guest syscall is passed to host
    fn syscall(_pid: &Self::PidContext, _tid: &Self::TidContext,
              _number: HostSyscallNum, _ret: SyscallParam,
              _args: &Vec<SyscallParam>, _trace: &mut Vec<Self::Trace>) {}
}

