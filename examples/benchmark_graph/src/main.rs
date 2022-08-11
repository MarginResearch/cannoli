//! An example user of Cannoli which focuses entirely on perf benchmarking

use std::sync::Arc;
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

    type PidContext = ();
    type TidContext = ();
    
    fn init_pid(_: &cannoli::ClientInfo) -> Arc<Self::PidContext> {
        Arc::new(())
    }

    /// Create contexts
    fn init_tid(_pid: &Self::PidContext,
            _: &cannoli::ClientInfo) -> (Self, Self::TidContext) {
        (Benchmark {
            start:        Instant::now(),
            start_cycles: rdtsc(),
            instructions: 0,
            last_report:  0,
        }, ())
    }

    fn exec(_pid: &Self::PidContext, _tid: &Self::TidContext,
            _pc: u64, trace: &mut Vec<Self::Trace>) {
        // Just return a marker indicating that we executed something
        trace.push(());
    }

    fn trace(&mut self, _pid: &Self::PidContext, _tid: &Self::TidContext,
            trace: &[Self::Trace]) {
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

