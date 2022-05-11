//! Client for handling the IPC messages streamed from QEMU while it is
//! executing

#![feature(array_chunks)]

use mempipe::RecvPipe;

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 16 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

fn main() {
    let mut pipe = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open("scoop")
        .expect("Failed to open pipe");

    loop {
        pipe.recv(|x| {
            for _chunk in x.array_chunks::<4>() {
                let _pc = u32::from_le_bytes(*_chunk);
            }
        });
    }
}

