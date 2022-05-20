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

    /// Last time we printed benchmark times to the screen
    last_report: u64,
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
            last_report:  0,
        }, ())
    }

    fn exec(_ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace> {
        // Just return a marker indicating that we executed something
        Some(())
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        // Track number of instructions in the trace
        self.instructions += trace.len() as u64;

        if self.instructions - self.last_report > 1_000_000 {
            // Get cycles elapsed
            let elapsed_cycles = rdtsc() - self.start_cycles;

            // Get time elapsed
            let elapsed = self.start.elapsed().as_secs_f64();

            println!("{:14} {:12.6} {:14}",
                self.instructions, elapsed, elapsed_cycles);

            self.last_report = self.instructions;

            if elapsed > 5. {
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    create_cannoli::<Benchmark>(4).unwrap();
}

