![Cannoli Mascot!](/logo.png)

## Cannoli

Cannoli is a high-performance tracing engine for qemu-user. It can record a
trace of both PCs executed, as well as memory operations. It consists of a
small patch to QEMU to expose locations to inject some code directly into the
JIT, a shared library which is loaded into QEMU to decide what and how to
instrument, and a final library which consumes the stream produced by QEMU in
another process, where analysis can be done on the trace.

Cannoli is designed to record this information with minimum interference of
QEMU's execution. In practice, this means that QEMU needs to produce a stream
of events, and hand them off (very quickly) to another process to handle more
complex analysis of them. Doing the analysis during execution of the QEMU JIT
itself would dramatically slow down execution.

Cannoli can handle billions of target instructions per second, can handle
multi-threaded qemu-user applications, and allows multiple threads to consume
the data from a single QEMU thread to parallelize processing of traces.

## TL;DR Getting it running

Build Cannoli

```
git clone https://github.com/MarginResearch/cannoli
cd cannoli
cargo build --release
```

Checkout QEMU

```
git clone https://github.com/qemu/QEMU
```

Apply patch from `qemu_patches.patch`

```
cd QEMU
git am </path/to/cannoli>/qemu_patches.patch
```

Build QEMU for your desired targets (example mipsel and riscv64)

```
mkdir build
cd build
../configure --target-list=mipsel-linux-user,riscv64-linux-user --extra-ldflags="-ldl" --with-cannoli=</path/to/cannoli>
make
```
Try out the example symbolizer

In one terminal, start the symbolizer

```
cd examples/symbolizer
cargo run --release
```

In another terminal, run the program in QEMU with Cannoli!

```
cd examples/symbolizer
</path/to/qemu>/build/qemu-mipsel -cannoli </path/to/cannoli>/target/release/libcannoli_server.so ./example_app
```

## What to do

1. Create an application using the `cannoli` library to process traces by
   implementing the `Cannoli` trait (see one of the `examples`)
2. Optionally modify `cannoli_server/src/lib.rs` as needed to reduce the number
   of hooks installed into the JIT (improves performance)
3. Build Cannoli
4. Run your trace-parsing application
5. Launch QEMU with the `-cannoli` argument, and a path to the compiled
   `libcannoli_server.so` that you built!

## User-experience

To start off, we should cover what you should expect as an end-user.

### QEMU patches

As a user you will have to apply a small patch set to QEMU, consisting of about
200 lines of additions. These are all gated with `#ifdef CANNOLI`, such that if
`CANNOLI` is not defined, QEMU will build identically to having none of the
patches in the first place.

The patches aren't too relevant to the user, other than understanding that they
add a `-cannoli` flag to QEMU which expects a path to a shared library. This
shared library is loaded into QEMU and is invoked at various points of the JIT.

To apply the patches, simply run something like:

`git am qemu_patches.patch`

### Cannoli "server"

The shared library which is loaded into QEMU is called the Cannoli server.

This library exposes two basic callbacks in `cannoli_server/src/lib.rs`.

```rust
/// Called before an instruction is lifted in QEMU. If this function returns
/// `true`, then the instrumentation is added and this PC will generate logs
/// in the traces.
///
/// This may be called from multiple threads
fn hook_inst(_pc: u64) -> bool {
    true
}

/// Called when a memory access is being lifted in QEMU. Returning `true` will
/// cause the memory access to generate events in the trace buffer.
///
/// This may be called from multiple threads
fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool {
    true
}
```

These hooks provide an opportunity for a user to decide whether or not a given
instruction or memory access should be hooked. Returning `true` (the default)
results in instrumenting the instruction. Returning `false` means that no
instrumentation is added to the JIT, and thus, QEMU runs with full speed
emulation.

This API is invoked when QEMU lifts target instructions. Lifting in this case,
is the core operation of an emulator, where it disassembles a target
instruction, and transforms it into an IL or JITs it to another architecture
for execution. Since QEMU caches instructions it has already lifted, these
functions are called "rarely" (with respect to how often the instructions
themselves execute), and thus this is the location where you should put in your
smart logic to filter what you hook.

If you hook a select few instructions, the performance overhead of this tool is
effectively zero. Cannoli is designed to provide very low overhead for full
tracing, however if you don't need full tracing you should filter at this
stage. This prevents the JIT from being instrumented in the first place, and
provides a filtering mechanism for an end-user.

### Cannoli "client"

Cannoli then has a client component. The client's goal is to process the massive
stream of data being produced by QEMU. Further, the API for Cannoli has been
designed with threading in mind, such that a single thread can be running
inside qemu-user, and complex analysis of that stream can be done by threading
the analysis while getting maximum single-core performance in QEMU itself.

Cannoli exposes a standard Rust trait-style interface, where you implement
`Cannoli` on your structure.

As an implementer of this trait, you must implement `init`. This is where you
create a structure for both a single-threaded mutable context (`Self`), as well
as a multi-threaded shared immutable context (`Self::Context`).

You then optionally can implement the following callbacks:

```rust
fn exec(_ctxt: &Self::Context, _pc: u64) -> Option<Self::Trace>;
fn read(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
    -> Option<Self::Trace>; 
fn write(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64);
    -> Option<Self::Trace>; 
fn trace(&mut self, _ctxt: &Self::Context, _trace: &[Self::Trace])
```

These callbacks are relatively self-explanatory, with the exception of the
threading aspects. The three main execution callbacks `exec`, `read`, and
`write` can be called from multiple threads in parallel. Thus, these are not
called sequentially. This is where stateless processing should be done. These
also only have immutable access to the `Self::Context`, as they run in
parallel. This is the correct location to do any processing which does not need
to know the ordering/sequence of instructions or memory accesses. For example,
applying symbols where you convert from a `pc` into a symbol + address should
be done here, such that you can symbolize the trace in parallel.

All of the main callbacks `exec`, `read`, and `write`, return an
`Option<Self::Trace>` type. This is a user-defined type which should be thought
of as a form of `filter_map`. If you return `None`, the event is not placed
into the trace, if you return `Some(your_value)` then `your_value` will be
placed sequentially into a trace buffer.

This trace is then exposed back to the user fully in-order via the `trace`
callback. The `trace` callback is called from various threads (eg. you might
run in a different TID), however, is it ensured to always be called
sequentially and in-order with respect to execution. Due to this, you get
mutable access to `self`, as well as a reference to the shared `Self::Context`.

I know this is a weird API, but it effectively allows parallelism of processing
the trace until you absolutely need it to be sequential. I hope it's not too
confusing for end users, but processing 2 billion instructions/second of data
kind of requires threading on the consumer side, otherwise you bottleneck QEMU!

