//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks, scoped_threads)]

use std::io::Read;
use std::mem::size_of;
use std::net::{TcpListener, TcpStream};
use mempipe::RecvPipe;

/// Wrapper around [`Error`]
type Result<T> = std::result::Result<T, Error>;

/// Errors for this crate
#[derive(Debug)]
enum Error {
    /// Failed to bind to create the server, waiting for QEMU clients
    Bind(std::io::Error),

    /// Failed to set `nonblocking` on the stream
    SetNonblocking(std::io::Error),

    /// Failed to open IPC pipe with QEMU
    OpenPipe(mempipe::Error),

    /// Data consumed over the IPC pipe was invalid
    InvalidMessage,
}

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 16 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

/// Opcodes we send over the IPC
#[repr(u8)]
enum Opcode {
    /// A `u32` follows, which is the PC which started execution
    Exec32,

    /// A `u64` follows, which is the PC which started execution
    Exec64,

    Read8_32,
    Read16_32,
    Read32_32,
    Write8_32,
    Write16_32,
    Write32_32,

    Read8_64,
    Read16_64,
    Read32_64,
    Read64_64,
    Write8_64,
    Write16_64,
    Write32_64,
    Write64_64,
}

#[derive(Debug)]
enum Op {
    Exec32(u32),
    Exec64(u64),

    Read8_32  (u32, u8),
    Read16_32 (u32, u16),
    Read32_32 (u32, u32),
    Write8_32 (u32, u8),
    Write16_32(u32, u16),
    Write32_32(u32, u32),

    Read8_64  (u64, u8),
    Read16_64 (u64, u16),
    Read32_64 (u64, u32),
    Read64_64 (u64, u64),
    Write8_64 (u64, u8),
    Write16_64(u64, u16),
    Write32_64(u64, u32),
    Write64_64(u64, u64),
}

/// Handle a newly connected client
fn handle_client(mut stream: TcpStream, uid: u64) -> Result<()> {
    println!("{uid:016x} opened!");

    // Create the IPC connection to the UID we got
    let mut pipe = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open(uid)
        .map_err(Error::OpenPipe)?;

    // Make the stream nonblocking
    stream.set_nonblocking(true)
        .map_err(Error::SetNonblocking)?;

    // Scratch buffer to check for socket close
    let mut tmp = [0u8; 4096];

    while !matches!(stream.read(&mut tmp), Ok(0)) {
        for _ in 0..1000 {
            pipe.try_recv(|mut x| {
                macro_rules! consume {
                    ($ty:ty) => {
                        x.get(0..size_of::<$ty>()).map(|z| {
                            // Advance pointer
                            x = &x[size_of::<$ty>()..];

                            // Return value
                            <$ty>::from_le_bytes(z.try_into().unwrap())
                        }).ok_or(Error::InvalidMessage)
                    }
                }

                while !x.is_empty() {
                    let opcode = consume!(u8)?;
                    let op = if opcode == Opcode::Exec32 as u8 {
                        Op::Exec32(consume!(u32)?)
                    } else if opcode == Opcode::Exec64 as u8 {
                        Op::Exec64(consume!(u64)?)
                    } else if opcode == Opcode::Read8_32 as u8 {
                        Op::Read8_32(consume!(u32)?, consume!(u32)? as u8)
                    } else if opcode == Opcode::Read16_32 as u8 {
                        Op::Read16_32(consume!(u32)?, consume!(u32)? as u16)
                    } else if opcode == Opcode::Read32_32 as u8 {
                        Op::Read32_32(consume!(u32)?, consume!(u32)? as u32)
                    } else if opcode == Opcode::Write8_32 as u8 {
                        Op::Write8_32(consume!(u32)?, consume!(u32)? as u8)
                    } else if opcode == Opcode::Write16_32 as u8 {
                        Op::Write16_32(consume!(u32)?, consume!(u32)? as u16)
                    } else if opcode == Opcode::Write32_32 as u8 {
                        Op::Write32_32(consume!(u32)?, consume!(u32)? as u32)
                    } else if opcode == Opcode::Read8_64 as u8 {
                        Op::Read8_64(consume!(u64)?, consume!(u64)? as u8)
                    } else if opcode == Opcode::Read16_64 as u8 {
                        Op::Read16_64(consume!(u64)?, consume!(u64)? as u16)
                    } else if opcode == Opcode::Read32_64 as u8 {
                        Op::Read32_64(consume!(u64)?, consume!(u64)? as u32)
                    } else if opcode == Opcode::Read64_64 as u8 {
                        Op::Read64_64(consume!(u64)?, consume!(u64)? as u64)
                    } else if opcode == Opcode::Write8_64 as u8 {
                        Op::Write8_64(consume!(u64)?, consume!(u64)? as u8)
                    } else if opcode == Opcode::Write16_64 as u8 {
                        Op::Write16_64(consume!(u64)?, consume!(u64)? as u16)
                    } else if opcode == Opcode::Write32_64 as u8 {
                        Op::Write32_64(consume!(u64)?, consume!(u64)? as u32)
                    } else if opcode == Opcode::Write64_64 as u8 {
                        Op::Write64_64(consume!(u64)?, consume!(u64)? as u64)
                    } else {
                        panic!("Unexpected opcode {:#x}", opcode);
                    };

                    println!("{:x?}", op);
                }

                Ok(())
            })?;
        }
    }

    println!("{uid:016x} closed!");
    Ok(())
}

fn main() -> Result<()> {
    // Create socket, waiting for clients to connect and inform us about some
    // memory regions
    let listener = TcpListener::bind("127.0.0.1:11458")
        .map_err(Error::Bind)?;

    std::thread::scope(|scope| {
        for stream in listener.incoming() {
            scope.spawn(|| {
                // Get access to the stream
                let mut stream = stream.expect("Failed to get TCP stream");

                // Get the UID for this connection
                let mut uid = [0u8; 8];
                stream.read_exact(&mut uid)
                    .expect("Failed to get uid for IPC");

                handle_client(stream, u64::from_le_bytes(uid))
                    .expect("Failed to handle client");
            });
        }
    });

    Ok(())
}

