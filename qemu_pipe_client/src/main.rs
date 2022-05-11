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
            for chunk in x.array_chunks::<4>() {
                let pc = u32::from_le_bytes(*chunk);
            }
        });
    }
}

