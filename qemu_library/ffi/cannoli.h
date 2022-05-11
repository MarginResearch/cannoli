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
    
    void (*jit_entry)(size_t *out);
    void (*jit_exit)(size_t reg_a, size_t reg_b, size_t reg_c);
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
    
    void (*jit_entry)(size_t *out);
    void (*jit_exit)(size_t reg_a, size_t reg_b, size_t reg_c);
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

