//! An example user of Cannoli which symbolizes a trace

#![feature(once_cell)]

use std::sync::{Mutex, LazyLock, Arc};
use std::process::Command;
use std::collections::{HashMap, BTreeMap, BTreeSet};
use cannoli::{Cannoli, create_cannoli};

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

    /// Context, the shared, immutable context shared between all threads doing
    /// processing. We stuff our symbol table here.
    type Context = Context;

    /// Load the symbol table
    fn init(ci: &cannoli::ClientInfo) -> (Self, Self::Context) {
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
    fn mmap(_ctxt: &Self::Context, mmap_addr: u64, _len: u64,
            anon: bool, _read: bool, _write: bool, _exec: bool,
            path: &str, offset: u64)
                -> Option<Self::Trace> {
        // Ignore mappings that are not file mappings
        if anon || offset != 0 {
            return None;
        }

        // First we have to figure out the base address of the ELF
        let output = Command::new("readelf").args(&[
            "-W", "-l", path,
        ]).output().ok()?;

        // Do nothing if we got an error running `readelf`
        if !output.status.success() {
            return None;
        }
        
        // Parse the headers
        let stdout = String::from_utf8(output.stdout).ok()?;
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
        let base = base?;

        println!("Resolved {path} loaded at {mmap_addr:#x}, based at {base:#x}");

        // Invoke nm to get the symbols for this file
        let output = Command::new("nm").args(&[
            "--demangle",
            path
        ]).output().ok()?;

        // Do nothing if we got an error running `nm`
        if !output.status.success() {
            return None;
        }

        // Get stdout as a string
        let mut syms = BTreeMap::new();
        let stdout = String::from_utf8(output.stdout).ok()?;
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

        Some(Trace::AddSymbols(syms))
    }

    fn munmap(_ctxt: &Self::Context, base: u64, len: u64)
            -> Option<Self::Trace> {
        Some(Trace::RemoveSymbols { base, len })
    }

    fn exec(_ctxt: &Self::Context, pc: u64) -> Option<Self::Trace> {
        Some(Trace::Exec(pc))
    }

    fn trace(&mut self, ctxt: &Self::Context, trace: Vec<Self::Trace>) {
        // Get access to the symbol and coverage databases
        let mut syms = ctxt.db.symbols.lock().unwrap();
        let mut cov  = ctxt.db.coverage.lock().unwrap();

        for trace in trace {
            match trace {
                Trace::Exec(pc) => {
                    // Find the symbol
                    let (sym, off) = syms.range(..=pc)
                        .next_back().map(|(sym_pc, sym)| {
                            (sym.clone(), pc - *sym_pc)
                        }).unwrap_or((ctxt.db.unknown_sym.clone(), pc));

                    println!("cov {:10} | {sym}+{off:#x}", cov.len());
                    cov.insert((sym, off));
                }
                Trace::AddSymbols(new_syms) => {
                    // Add the symbols
                    syms.extend(new_syms);
                }
                Trace::RemoveSymbols { base, len } => {
                    // Remove symbols that were in this region
                    syms.retain(|&addr, _| {
                        // Only keep things outside of this region
                        addr < base || addr >= base + len
                    });
                }
            }
        }
    }
}

fn main() {
    create_cannoli::<Coverage>(4).unwrap();
}

