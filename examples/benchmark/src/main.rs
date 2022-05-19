//! An example user of Cannoli which focuses entirely on perf benchmarking

use std::time::Instant;
use cannoli::{Cannoli, create_cannoli};

/// Get the time stamp counter
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// Benchmark structure
struct Benchmark {
    /// Start time of the benchmark
    start: Instant,

    /// Start cycles
    start_cycles: u64,

    /// Number of instructions executed
    instructions: u64,
}

impl Drop for Benchmark {
    fn drop(&mut self) {
        // Get cycles elapsed
        let elapsed_cycles = rdtsc() - self.start_cycles;

        // Get time elapsed
        let elapsed = self.start.elapsed().as_secs_f64();

        // Get instructions/second
        let ips = self.instructions as f64 / elapsed;

        println!("Executed {:10} instructions in {:8.6} seconds \
                ({:10} cycles) - {:12.6} Minst/sec - {:8.6} cycles/inst",
            self.instructions, elapsed, elapsed_cycles, ips / 1e6,
            elapsed_cycles as f64 / self.instructions as f64);
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
            start_cycles: rdtsc(),
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

