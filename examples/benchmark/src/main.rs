//! An example user of Cannoli which focuses entirely on perf benchmarking

use std::sync::Arc;
use std::time::Instant;
use cannoli::Cannoli;

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
    }
}

fn main() {
    cannoli::run::<Benchmark>(4).unwrap();
}

