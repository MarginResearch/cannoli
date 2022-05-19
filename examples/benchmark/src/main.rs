//! An example user of Cannoli which focuses entirely on perf benchmarking

use std::time::Instant;
use cannoli::{Cannoli, create_cannoli};

/// Benchmark structure
struct Benchmark {
    /// Start time of the benchmark
    start: Instant,

    /// Number of instructions executed
    instructions: u64,
}

impl Drop for Benchmark {
    fn drop(&mut self) {
        // Get time elapsed
        let elapsed = self.start.elapsed().as_secs_f64();

        // Get instructions/second
        let ips = self.instructions as f64 / elapsed;

        println!("Executed {:10} instructions in {:8.6} seconds - \
                {:12.6} Minst/sec",
            self.instructions, elapsed, ips / 1e6);
    }
}

impl Cannoli for Benchmark {
    /// Not tracing anything
    type Trace = ();

    /// Context structure
    type Context = ();

    /// Create contexts
    fn init(_: u64) -> (Self, Self::Context) {
        (Benchmark {
            start:        Instant::now(),
            instructions: 0,
        }, ())
    }

    fn exec(_ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace> {
        // Just return a marker indicating that we executed something
        Some(())
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        // Track number of instructions in the trace
        self.instructions += trace.len() as u64;
    }
}

fn main() {
    create_cannoli::<Benchmark>(2).unwrap();
}

