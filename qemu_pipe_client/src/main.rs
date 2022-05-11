//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks, scoped_threads)]

use std::io::Read;
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
}

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 16 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

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
        for _ in 0..10 {
            pipe.try_recv(|x| {
                for _chunk in x.array_chunks::<4>() {
                    let _pc = u32::from_le_bytes(*_chunk);
                }
            });
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

