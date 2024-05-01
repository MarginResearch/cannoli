//! An example user of Cannoli which symbolizes a trace

#![feature(lazy_cell)]

use std::sync::{Mutex, LazyLock, Arc};
use std::process::Command;
use std::collections::{HashMap, BTreeMap, BTreeSet};
use cannoli::Cannoli;

struct CoverageDb {
    /// Symbols for the process
    symbols: Mutex<BTreeMap<u64, Arc<str>>>,

    /// All observed coverage for the process
    coverage: Mutex<BTreeSet<(Arc<str>, u64)>>,

    /// Unknown symbol arc, `Arc::new("<unknown>")`
    unknown_sym: Arc<str>,
}

/// Mapping of `pid`s to symbol tables
static SYMBOLS_BY_PID:
        LazyLock<Mutex<HashMap<i32, Arc<CoverageDb>>>> =
    LazyLock::new(|| {
        Default::default()
    });

/// The structure we implement [`Cannoli`] for!
struct Coverage;

enum Trace {
    /// Executed a PC
    Exec(u64),

    /// Mapped a new file with symbols
    AddSymbols(BTreeMap<u64, Arc<str>>),

    /// Removed symbols due to munmap()
    RemoveSymbols { base: u64, len: u64 },
}

/// Context shared between threads
struct Context {
    /// Mapping of addresses to symbol names for this process
    db: Arc<CoverageDb>,
}

impl Cannoli for Coverage {
    /// The type emit in the serialized trace
    type Trace = Trace;

    type PidContext = ();
    type TidContext = Context;
    
    fn init_pid(_: &cannoli::ClientInfo) -> Arc<Self::PidContext> {
        Arc::new(())
    }

    /// Load the symbol table
    fn init_tid(_pid: &Self::PidContext,
            ci: &cannoli::ClientInfo) -> (Self, Self::TidContext) {
        let db = SYMBOLS_BY_PID.lock().unwrap()
            .entry(ci.pid).or_insert_with(|| {
                Arc::new(CoverageDb {
                    symbols:     Default::default(),
                    coverage:    Default::default(),
                    unknown_sym: "<unknown>".into(),
                })
            }).clone();

        (Self, Context {
            db,
        })
    }

    // Look for dynamic code being loaded
    fn mmap(_pid: &Self::PidContext,
            _tid: &Self::TidContext, mmap_addr: u64, _len: u64,
            anon: bool, _read: bool, _write: bool, _exec: bool,
            path: &str, offset: u64, trace: &mut Vec<Self::Trace>) {
        // Ignore mappings that are not file mappings
        if anon || offset != 0 {
            return;
        }

        // First we have to figure out the base address of the ELF
        let output = Command::new("readelf").args(&[
            "-W", "-l", path,
        ]).output().unwrap();

        // Do nothing if we got an error running `readelf`
        if !output.status.success() {
            return;
        }
        
        // Parse the headers
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut base = None;
        for line in stdout.lines() {
            // Skip non-load sections
            if !line.trim().starts_with("LOAD ") {
                continue;
            }

            // Parse the offset and vaddr for each LOAD section
            let mut spl = line.trim().split_whitespace().skip(1);
            let offset = u64::from_str_radix(
                &spl.next().unwrap()[2..], 16).unwrap();
            let vaddr = u64::from_str_radix(
                &spl.next().unwrap()[2..], 16).unwrap();

            // Look for the 0th offset for the file
            if offset == 0 {
                base = Some(vaddr);
                break;
            }
        }

        // Make sure we got the base for the file
        let base = base.unwrap();

        println!("Resolved {path} loaded at {mmap_addr:#x}, based at {base:#x}");

        // Invoke nm to get the symbols for this file
        let output = Command::new("nm").args(&[
            "--demangle",
            path
        ]).output().unwrap();

        // Do nothing if we got an error running `nm`
        if !output.status.success() {
            return;
        }

        // Get stdout as a string
        let mut syms = BTreeMap::new();
        let stdout = String::from_utf8(output.stdout).unwrap();
        for line in stdout.lines() {
            let mut spl = line.splitn(3, " ");
            if let Ok(addr) = u64::from_str_radix(spl.next().unwrap(), 16) {
                let _typ = spl.next().unwrap();
                let name = spl.next().unwrap();

                // Change address to offset from base
                if let Some(offset) = addr.checked_sub(base) {
                    syms.insert(mmap_addr + offset,
                        format!("{path}!{name}").into());
                }
            }
        }

        println!("    Added {} symbols for {path}", syms.len());

        trace.push(Trace::AddSymbols(syms));
    }

    fn munmap(_pid: &Self::PidContext,
              _tid: &Self::TidContext, base: u64, len: u64,
              trace: &mut Vec<Self::Trace>) {
        trace.push(Trace::RemoveSymbols { base, len });
    }

    fn exec(_pid: &Self::PidContext, _tid: &Self::TidContext,
            pc: u64, trace: &mut Vec<Self::Trace>) {
        trace.push(Trace::Exec(pc));
    }

    fn trace(&mut self, _pid: &Self::PidContext,
            tid: &Self::TidContext, trace: &[Self::Trace]) {
        // Get access to the symbol and coverage databases
        let mut syms = tid.db.symbols.lock().unwrap();
        let mut cov  = tid.db.coverage.lock().unwrap();

        for trace in trace {
            match trace {
                Trace::Exec(pc) => {
                    // Find the symbol
                    let (sym, off) = syms.range(..=pc)
                        .next_back().map(|(sym_pc, sym)| {
                            (sym.clone(), pc - *sym_pc)
                        }).unwrap_or((tid.db.unknown_sym.clone(), *pc));

                    println!("cov {:10} | {sym}+{off:#x}", cov.len());
                    cov.insert((sym, off));
                }
                Trace::AddSymbols(new_syms) => {
                    // Add the symbols
                    syms.extend(new_syms.clone());
                }
                Trace::RemoveSymbols { base, len } => {
                    // Remove symbols that were in this region
                    syms.retain(|&addr, _| {
                        // Only keep things outside of this region
                        addr < *base || addr >= base + len
                    });
                }
            }
        }
    }
}

fn main() {
    cannoli::run::<Coverage>(4).unwrap();
}

