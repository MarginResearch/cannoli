//! FFI types used to communicate with QEMU

#ifndef CANNOLI_BINDGEN
#define CANNOLI_BINDGEN

typedef __UINT8_TYPE__  uint8_t;
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

