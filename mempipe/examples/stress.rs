//! Multiple readers sharing one `RecvPipe`, as cannoli uses it. Every message
//! must arrive exactly once with its own bytes, and the pipe must not hang.
//!
//! ```text
//! cargo +nightly-2024-03-01 run --release -p mempipe --example stress \
//!     -- READERS MESSAGES [BLOCKING]
//! ```
//!
//! Readers spin: keep READERS + 1 below the core count.

use mempipe::{RecvPipe, SendPipe};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::time::{Duration, Instant};

const CHUNK_SIZE: usize = 16;
const NUM_BUFFERS: usize = 4;

static DELIVERED: AtomicU64 = AtomicU64::new(0);

fn main() -> Result<(), mempipe::Error> {
    let args: Vec<String> = std::env::args().collect();
    let readers: usize = args.get(1).map_or(3, |a| a.parse().unwrap());
    let messages: u64 = args.get(2).map_or(1_000_000, |a| a.parse().unwrap());
    let blocking = args.get(3).is_some_and(|a| a == "blocking");

    let mut tx = SendPipe::<CHUNK_SIZE, NUM_BUFFERS>::create()?;
    let rx = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open(tx.uid())?;
    let seen: Vec<AtomicU8> = (0..messages).map(|_| AtomicU8::new(0)).collect();
    let stop = AtomicBool::new(false);

    let start = Instant::now();
    std::thread::scope(|s| {
        for _ in 0..readers {
            let (rx, seen, stop) = (&rx, &seen, &stop);
            s.spawn(move || {
                let mut ticket = rx.request_ticket();
                while !stop.load(Ordering::Relaxed) {
                    let (next, res) = rx.try_recv(ticket, |data| -> Result<u64, ()> {
                        let bytes: [u8; 8] = data.try_into()
                            .expect("length of another message");
                        Ok(u64::from_le_bytes(bytes))
                    });
                    ticket = next;
                    if let Some(Ok((seq, payload))) = res {
                        assert_eq!(seq, payload, "ticket {seq} got message {payload}");
                        assert_eq!(seen[seq as usize].fetch_add(1, Ordering::Relaxed), 0,
                            "message {seq} delivered twice");
                        DELIVERED.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }

        let watchdog = s.spawn(|| {
            let (mut last, mut since) = (0, Instant::now());
            loop {
                let now = DELIVERED.load(Ordering::Relaxed);
                if now == messages {
                    break;
                }
                if now != last {
                    (last, since) = (now, Instant::now());
                } else if since.elapsed() > Duration::from_secs(2) {
                    eprintln!("HANG: no progress for 2s at {now}/{messages} delivered");
                    std::process::exit(1);
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..messages {
            tx.alloc_buffer(blocking).send(i.to_le_bytes());
        }
        watchdog.join().unwrap();
        stop.store(true, Ordering::Relaxed);
    });

    println!("OK: {messages} messages, {readers} readers{} in {:?}",
        if blocking { ", blocking" } else { "" }, start.elapsed());
    Ok(())
}
