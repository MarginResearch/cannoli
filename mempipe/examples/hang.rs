//! Reproducer for the stranded-buffer hang with several readers on one
//! `RecvPipe`.
//!
//! The race needs a reader to stall between its two loads in `try_recv`
//! while the sender is stalled between its `client_seq` and `client_owned`
//! stores. Natural preemption does that rarely, so this program injects
//! stalls: injector threads fire SIGUSR1 (readers) / SIGUSR2 (sender) at
//! random moments and the handler busy-waits. A signal lands at an arbitrary
//! instruction boundary, exactly like a timer preemption, so the library code
//! is exercised unmodified. Duty cycles stay well under 100% so a thread that
//! stops making progress is stuck in the library, not in the handler.
//!
//! ```text
//! cargo +nightly-2024-03-01 run --release -p mempipe --example hang -- \
//!     READERS MESSAGES [blocking]
//! ```
//!
//! Tunables (microseconds) via environment: `SENDER_GAP` / `SENDER_STALL`
//! (default 50 / 25) and `READER_GAP` / `READER_STALL` (default 25 / 50, the
//! gap is per signal, spread over all readers). `START_DELAY_MS` (default 2)
//! lets the readers spin before the first send.
//!
//! Settings that hit the stranding bug on a 16-core x86 box within seconds:
//!
//! ```text
//! SENDER_GAP=20 SENDER_STALL=8 READER_GAP=10 READER_STALL=20 \
//!     hang 6 10000000 blocking
//! ```
//!
//! Settings that hit the ticket-0 hole (a `try_recv` that checks the sequence
//! first but leaves `client_seq` initialised to 0) about 1 run in 10:
//!
//! ```text
//! SENDER_GAP=1000 SENDER_STALL=1 READER_GAP=30 READER_STALL=25 hang 1 50
//! ```
//!
//! Exit 0: every message arrived once. Exit 1: no progress for a while, with a
//! dump of the tickets still outstanding. Exit 2: a message was
//! duplicated or corrupted.

use mempipe::{RecvPipe, SendPipe};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::time::{Duration, Instant};

const CHUNK_SIZE: usize = 16;
const NUM_BUFFERS: usize = 4;

/// Watchdog: no delivery for this long counts as a hang
const STALL_LIMIT: Duration = Duration::from_millis(500);

/// Injected stall lengths, set from the environment before any signal fires
static READER_STALL_US: AtomicU64 = AtomicU64::new(50);
static SENDER_STALL_US: AtomicU64 = AtomicU64::new(25);

fn env_us(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().map_or(default, |v| v.parse().expect(name))
}

fn spin_for(d: Duration) {
    let t = Instant::now();
    while t.elapsed() < d {
        core::hint::spin_loop();
    }
}

extern "C" fn reader_stall(_: libc::c_int) {
    spin_for(Duration::from_micros(READER_STALL_US.load(Ordering::Relaxed)));
}
extern "C" fn sender_stall(_: libc::c_int) {
    spin_for(Duration::from_micros(SENDER_STALL_US.load(Ordering::Relaxed)));
}

fn install(sig: libc::c_int, handler: extern "C" fn(libc::c_int)) {
    unsafe {
        let mut sa: libc::sigaction = core::mem::zeroed();
        sa.sa_sigaction = handler as usize;
        sa.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut sa.sa_mask);
        assert_eq!(libc::sigaction(sig, &sa, core::ptr::null_mut()), 0);
    }
}

fn main() -> Result<(), mempipe::Error> {
    let args: Vec<String> = std::env::args().collect();
    let readers: usize = args.get(1).map_or(6, |a| a.parse().expect("READERS"));
    let messages: u64 = args.get(2).map_or(1_000_000, |a| a.parse().expect("MESSAGES"));
    let blocking = match args.get(3).map(String::as_str) {
        None => false,
        Some("blocking") => true,
        Some(other) => panic!("unknown mode {other:?}, expected `blocking`"),
    };
    assert!(readers > 0, "need at least one reader");

    let sender_gap = Duration::from_micros(env_us("SENDER_GAP", 50));
    let reader_gap = Duration::from_micros(env_us("READER_GAP", 25));
    let start_delay = Duration::from_millis(env_us("START_DELAY_MS", 2));
    SENDER_STALL_US.store(env_us("SENDER_STALL", 25), Ordering::Relaxed);
    READER_STALL_US.store(env_us("READER_STALL", 50), Ordering::Relaxed);
    install(libc::SIGUSR1, reader_stall);
    install(libc::SIGUSR2, sender_stall);

    let mut tx = SendPipe::<CHUNK_SIZE, NUM_BUFFERS>::create()?;
    let rx = RecvPipe::<CHUNK_SIZE, NUM_BUFFERS>::open(tx.uid())?;

    let seen: Vec<AtomicU8> = (0..messages).map(|_| AtomicU8::new(0)).collect();
    let threads: Vec<AtomicU64> =
        (0..readers).map(|_| AtomicU64::new(0)).collect();
    let sender_thread = unsafe { libc::pthread_self() } as u64;
    let delivered = AtomicU64::new(0);
    let sent = AtomicU64::new(0);
    let stop = AtomicBool::new(false);

    // A reader assertion (duplicate / wrong payload) must not be reported as
    // a hang, so exit straight from the panic hook.
    std::panic::set_hook(Box::new(|info| {
        eprintln!("FAIL: {info}");
        std::process::exit(2);
    }));

    let start = Instant::now();
    std::thread::scope(|s| {
        for r in 0..readers {
            let (rx, seen, threads, delivered, stop) =
                (&rx, &seen, &threads, &delivered, &stop);
            s.spawn(move || {
                threads[r].store(unsafe { libc::pthread_self() } as u64,
                    Ordering::Release);
                let mut ticket = rx.request_ticket();
                while !stop.load(Ordering::Relaxed) {
                    let (next, res) = rx.try_recv(ticket, |data| -> Result<u64, ()> {
                        let bytes: [u8; 8] = data.try_into().expect("payload length");
                        Ok(u64::from_le_bytes(bytes))
                    });
                    ticket = next;
                    if let Some(Ok((seq, payload))) = res {
                        assert_eq!(seq, payload, "ticket {seq} got message {payload}");
                        assert_eq!(
                            seen[seq as usize].fetch_add(1, Ordering::Relaxed), 0,
                            "message {seq} delivered twice");
                        delivered.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }

        // Stall injectors, one for the sender and one cycling over readers
        let sender_injector = s.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                unsafe {
                    libc::pthread_kill(sender_thread as libc::pthread_t,
                        libc::SIGUSR2);
                }
                spin_for(sender_gap);
            }
        });
        let reader_injector = s.spawn(|| {
            let mut x: u64 = 0x9e3779b97f4a7c15;
            while !stop.load(Ordering::Relaxed) {
                x ^= x << 13; x ^= x >> 7; x ^= x << 17;
                let tid = threads[(x as usize) % readers].load(Ordering::Acquire);
                if tid != 0 {
                    unsafe {
                        libc::pthread_kill(tid as libc::pthread_t, libc::SIGUSR1);
                    }
                }
                spin_for(reader_gap);
            }
        });

        let watchdog = s.spawn(|| {
            let (mut last, mut since) = (0, Instant::now());
            loop {
                let now = delivered.load(Ordering::Relaxed);
                if now == messages {
                    break;
                }
                if now != last {
                    (last, since) = (now, Instant::now());
                } else if since.elapsed() > STALL_LIMIT {
                    let sent = sent.load(Ordering::Relaxed);
                    eprintln!("HANG after {:?}: delivered {now}/{messages}, \
                        sender handed out {sent} buffers", start.elapsed());
                    // Tickets handed out so far are 0 .. readers + delivered
                    // and each delivery consumed the ticket equal to its
                    // sequence, so the outstanding ones are the undelivered
                    // sequences in that range
                    let handed = readers as u64 + now;
                    let outstanding: Vec<u64> = (0..handed.min(messages))
                        .filter(|&t| seen[t as usize].load(Ordering::Relaxed) == 0)
                        .collect();
                    eprintln!("  readers hold tickets {outstanding:?}");
                    if outstanding.iter().all(|&t| t >= sent) {
                        eprintln!("  every reader waits on a sequence the sender \
                            never handed out, and the sender (stalled at most \
                            half the time) has not handed out a buffer for \
                            {STALL_LIMIT:?}: it is waiting for a buffer that \
                            no reader will ever hand back (stranded)");
                    }
                    std::process::exit(1);
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        });

        // Let the readers reach their spin loops first
        std::thread::sleep(start_delay);
        for i in 0..messages {
            tx.alloc_buffer(blocking).send(i.to_le_bytes());
            sent.store(i + 1, Ordering::Relaxed);
        }
        watchdog.join().unwrap();
        stop.store(true, Ordering::Relaxed);
        sender_injector.join().unwrap();
        reader_injector.join().unwrap();
    });

    println!("OK: {messages} messages, {readers} readers{} in {:?}",
        if blocking { ", blocking" } else { "" }, start.elapsed());
    Ok(())
}
