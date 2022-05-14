//! FFI types used to communicate with QEMU

#ifndef CANNOLI_BINDGEN
#define CANNOLI_BINDGEN

typedef __UINT8_TYPE__  uint8_t;
typedef __INT32_TYPE__  int32_t;
typedef __UINT32_TYPE__ uint32_t;
typedef __UINT64_TYPE__ uint64_t;
typedef __SIZE_TYPE__   size_t;

/// Random 64-bit integer defining this Cannoli version
static const uint64_t CANNOLI_VERSION = 0xf814f32b4628d058ULL;

/// Definition of the bindings defined in Cannoli, passed to QEMU so it knows
/// how to invoke us
struct Cannoli32 {
    /// Version number for this version of Canolli. Used to make sure the
    /// correct QEMU patches are applied for this shared library
    ///
    /// This must _always_ be the first field of this structure, this allows
    /// forwards compatibility with version checking
    uint64_t version;

    /// Output the host assembly for a given guest instruction at `pc`.
    ///
    /// QEMU gives a temporary buffer pointed to by `buf`, for `buf_size`
    /// bytes which is used as instruction storage.
    ///
    /// This function returns the number of bytes to insert into the JIT stream
    /// at this instruction start. If it is zero, no additional instructions
    /// are generated
    size_t (*lift_instruction)(uint32_t pc, uint8_t *buf, size_t buf_size);
    
    /// Invoked from QEMU when entering the JIT. This provides an opportunity
    /// for us to introduce some register state to the JIT.
    ///
    /// `out` points to an array of 3 `size_t`s which should be filled with the
    /// values for `r12`, `r13`, and `r14`, respectively
    void (*jit_entry)(size_t *out);

    /// Invoked from QEMU when exiting the JIT. This is then provided with the
    /// values of `r12`, `r13`, and `r14` upon exit of the JIT, giving the
    /// user an opportunity to observe the changes to the registers
    void (*jit_exit)(size_t r12, size_t r13, size_t r14);

    /// Invoked when QEMU is lifting a memory operation. Similar to the
    /// execution hook, this function is provided with a `buf` and a
    /// `buf_size` which are to be populated with shellcode to inject into the
    /// JIT stream. The return value from this function is the number of bytes
    /// written.
    ///
    /// This shellcode is injected prior to writes, and after reads, allowing
    /// the value that is being read/written to be observed
    ///
    /// This function is called with the parameters:
    ///
    /// - `pc`       - Target program counter associated with this operation
    /// - `is_write` - `0` if this is a read, `1` if this is a write
    /// - `data_reg` - x86_64 register index to the register which holds the
    ///                value that was read/written
    /// - `addr_reg` - x86_64 register index to the register which holds the
    ///                emulated guest's address that is being accessed
    /// - `memop`    - One of the QEMU memops from `MemOp` (eg. `MO_8`). This
    ///                tells us the size of the operation
    /// - `buf`      - Pointer to QEMU-allocated memory for where to copy
    ///                shellcode
    /// - `buf_size` - Size of `buf` in bytes
    size_t (*lift_memop)(uint32_t pc, int32_t is_write, size_t data_reg,
        size_t addr_reg, int32_t memop, uint8_t *buf, size_t buf_size);
};

/// Definition of the bindings defined in Cannoli, passed to QEMU so it knows
/// how to invoke us
struct Cannoli64 {
    /// Version number for this version of Cannoli. Used to make sure the
    /// correct QEMU patches are applied for this shared library
    ///
    /// This must _always_ be the first field of this structure, this allows
    /// forwards compatibility with version checking
    uint64_t version;

    /// Output the host assembly for a given guest instruction at `pc`.
    ///
    /// QEMU gives a temporary buffer pointed to by `buf`, for `buf_size`
    /// bytes which is used as instruction storage.
    ///
    /// This function returns the number of bytes to insert into the JIT stream
    /// at this instruction start. If it is zero, no additional instructions
    /// are generated
    size_t (*lift_instruction)(uint64_t pc, uint8_t *buf, size_t buf_size);
    
    /// Invoked from QEMU when entering the JIT. This provides an opportunity
    /// for us to introduce some register state to the JIT.
    ///
    /// `out` points to an array of 3 `size_t`s which should be filled with the
    /// values for `r12`, `r13`, and `r14`, respectively
    void (*jit_entry)(size_t *out);

    /// Invoked from QEMU when exiting the JIT. This is then provided with the
    /// values of `r12`, `r13`, and `r14` upon exit of the JIT, giving the
    /// user an opportunity to observe the changes to the registers
    void (*jit_exit)(size_t r12, size_t r13, size_t r14);
    
    /// Invoked when QEMU is lifting a memory operation. Similar to the
    /// execution hook, this function is provided with a `buf` and a
    /// `buf_size` which are to be populated with shellcode to inject into the
    /// JIT stream. The return value from this function is the number of bytes
    /// written.
    ///
    /// This shellcode is injected prior to writes, and after reads, allowing
    /// the value that is being read/written to be observed
    ///
    /// This function is called with the parameters:
    ///
    /// - `pc`       - Target program counter associated with this operation
    /// - `is_write` - `0` if this is a read, `1` if this is a write
    /// - `data_reg` - x86_64 register index to the register which holds the
    ///                value that was read/written
    /// - `addr_reg` - x86_64 register index to the register which holds the
    ///                emulated guest's address that is being accessed
    /// - `memop`    - One of the QEMU memops from `MemOp` (eg. `MO_8`). This
    ///                tells us the size of the operation
    /// - `buf`      - Pointer to QEMU-allocated memory for where to copy
    ///                shellcode
    /// - `buf_size` - Size of `buf` in bytes
    size_t (*lift_memop)(uint64_t pc, int32_t is_write, size_t data_reg,
        size_t addr_reg, int32_t memop, uint8_t *buf, size_t buf_size);
};

// If we're building in QEMU these will be defined and we'll make an alias for
// the correct bit-width for the target architecture
#if TARGET_LONG_BITS == 32
typedef struct Cannoli32 Cannoli;
#define CANNOLI_ENTRY "init_cannoli32"
#elif TARGET_LONG_BITS == 64
typedef struct Cannoli64 Cannoli;
#define CANNOLI_ENTRY "init_cannoli64"
#endif

#endif // CANNOLI_BINDGEN

