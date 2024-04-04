// ============================================================================

// Macro invoked when creating an instruction hook. This is where we log the PC
// of the instruction being executed into the buffer!
//
// bits  - The bitness of the emulated target, either 32 or 64
// width - The bitness divided by eight (number of bytes per target usize)
// once  - Determines if this is a single-shot instruction hook, or an always
//         hook
.macro create_insthook bits, width, once

// This code is injected _directly_ into QEMUs JIT, we have to make sure we
// don't touch _any_ registers without preserving them
.global cannoli_insthook\bits\()\once\()
cannoli_insthook\bits\()\once\():
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - Scratch

.ifnb \once
    // Clear the zero flag
    xor r14d, r14d

    // Conditionally branch to the end of code
    jnz 10f

    // Replace branch above with a `jmp`
    mov byte ptr [rip - 9], 0xeb
.endif

    // Allocate room in the buffer
    lea r14, [r12 + \width + 1]

    // Make sure we didn't run out of buffer space
    cmp r14, r13
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.if \bits == 32
    // Opcode
    mov byte ptr [r12], 0x00

    // PC, directly put into memory from an immediate
    mov dword ptr [r12 + 1], {REPLACE_WITH_PC}
.elseif \bits == 64
    // Opcode
    mov byte ptr [r12], 0x80

    // Move PC into a register so we can use imm64 encoding
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1], r14
.else
.error "Invalid bitness passed to create_insthook"
.endif

    // Advance buffer
    add r12, \width + 1

    // End of hook
10:

.global cannoli_insthook\bits\()\once\()_end
cannoli_insthook\bits\()\once\()_end:

.endm // create_insthook

// Create both the 32-bit and 64-bit instruction hooks
create_insthook 32, 4
create_insthook 64, 8
create_insthook 32, 4, _once
create_insthook 64, 8, _once

// ============================================================================

// Okay. This macro is gnarly. This defines the shellcode we use for our memory
// hooks. Unlike the PC shellcode, we actually have 2 register inputs from
// QEMU's JIT. These registers could be "any" register that is scheduled to the
// JIT. Thus, we have to create different shellcode templates for every
// combination of registers.
//
// So, we've made this macro. The way this macro is invoked is disgusting, but
// the macro itself is pretty clean. Inside the macro you have a few different
// values you can access:
//
// \access    - Either 'read' or 'write' (no quotes) depending on the operation
//              type
// \datawidth - The size of the read/write being performed (1, 2, 4, or 8)
// \width     - The size of the target's usize, in bytes (4 or 8)
// \data      - The register name of the register which holds the data that
//              was read/written
// \addr      - The register name of the register which holds the address
.macro create_memhook access, datawidth, width, data, addr
.global cannoli_memhook_\access\()_\data\()_\addr\()
cannoli_memhook_\access\()_\data\()_\addr\():
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - For reads, this always holds the address, for writes, it's scratch

    // Allocate room in the buffer (we have to preserve r14 here)
    lea r12, [r12 + (\width * 2 + \datawidth + 1)]
    cmp r12, r13
    lea r12, [r12 - (\width * 2 + \datawidth + 1)]
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.ifc \access, read
    // Read opcode
    mov byte ptr [r12], (((\width / 4) - 1) << 7) | 0x10 | \datawidth
.endif
.ifc \access, write
    // Write opcode
    mov byte ptr [r12], (((\width / 4) - 1) << 7) | 0x20 | \datawidth
.endif

    // Address and data
    mov [r12          + 1], \addr
    mov [r12 + \width + 1], \data

    // Store the PC
.if \width == 4
    mov dword ptr [r12 + \width + 1 + \datawidth], {REPLACE_WITH_PC}
.elseif \width == 8
    mov r14, {REPLACE_WITH_PC}
    mov [r12 + \width + 1 + \datawidth], r14
.endif

    // Advance buffer
    add r12, \width * 2 + \datawidth + 1

.global cannoli_memhook_\access\()_\data\()_\addr\()_end
cannoli_memhook_\access\()_\data\()_\addr\()_end:
.endm // create_memhook

// Macro invoked when creating an register hook.
//
// bits  - The bitness of the emulated target, either 32 or 64
// width - The bitness divided by eight (number of bytes per target usize)
.macro create_reghook bits, width

.global cannoli_reghook\bits\()
cannoli_reghook\bits\():
    // Determine size required for register hook metadata
    lea r14, [r12 + 1 + 4 + \width]
    add r14, {REPLACE_WITH_REGHOOK_SIZE}

    // Make sure we're in bounds
    cmp r14, r13
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.if \bits == 32
    // Opcode
    mov byte ptr [r12], 0x01

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // PC, directly put into memory from an immediate
    mov dword ptr [r12 + 1 + 4], {REPLACE_WITH_PC}
.elseif \bits == 64
    // Opcode
    mov byte ptr [r12], 0x81

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // Move PC into a register so we can use imm64 encoding
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1 + 4], r14
.else
.error "Invalid bitness passed to cannoli_reghook"
.endif

    // Fill in the registers
    push rdi
    push rsi
    push rcx
    lea rdi, [r12 + \width + 1 + 4]
    lea rsi, [rbp + {REPLACE_WITH_REGHOOK_OFFSET}]
    mov ecx, {REPLACE_WITH_REGHOOK_SIZE}
    rep movsb
    pop rcx
    pop rsi
    pop rdi

    // Advance buffer
    add r12, \width + 1 + 4
    add r12, {REPLACE_WITH_REGHOOK_SIZE}

.global cannoli_reghook\bits\()_end
cannoli_reghook\bits\()_end:

.endm // create_reghook

create_reghook 32, 4
create_reghook 64, 8

// Macro invoked when creating an true branch hook.
//
// bits  - The bitness of the emulated target, either 32 or 64
// width - The bitness divided by eight (number of bytes per target usize)
// branch - Bool indicating if this is a branch or not
.macro create_branchhook bits, width, is_branch

.global cannoli_branchhook\is_branch\()\bits\()
cannoli_branchhook\is_branch\()\bits\():
    // Determine size required for register hook metadata
    // One more byte required (versus reghook) for branch boolean
    lea r14, [r12 + 1 + 4 + \width + 1]
    add r14, {REPLACE_WITH_REGHOOK_SIZE}

    // Make sure we're in bounds
    cmp r14, r13
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.if \bits == 32
    // Opcode
    mov byte ptr [r12], 0x40

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // PC, directly put into memory from an immediate
    mov dword ptr [r12 + 1 + 4], {REPLACE_WITH_PC}
    mov byte ptr [r12 + 1 + 4 + \width], \is_branch
.elseif \bits == 64
    // Opcode
    mov byte ptr [r12], 0xc0

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // Move PC into a register so we can use imm64 encoding
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1 + 4], r14
    mov byte ptr [r12 + 1 + 4 + \width], \is_branch
.else
.error "Invalid bitness passed to cannoli_branchhook"
.endif

    // Fill in the registers
    push rdi
    push rsi
    push rcx
    lea rdi, [r12 + \width + 1 + 4 + 1]
    lea rsi, [rbp + {REPLACE_WITH_REGHOOK_OFFSET}]
    mov ecx, {REPLACE_WITH_REGHOOK_SIZE}
    rep movsb
    pop rcx
    pop rsi
    pop rdi

    // Advance buffer
    add r12, \width + 1 + 4 + 1
    add r12, {REPLACE_WITH_REGHOOK_SIZE}

.global cannoli_branchhook\is_branch\()\bits\()_end
cannoli_branchhook\is_branch\()\bits\()_end:

.endm // create_branchhook

create_branchhook 32, 4, 1
create_branchhook 32, 4, 0
create_branchhook 64, 8, 1
create_branchhook 64, 8, 0

// ===========================================================================
// !!! WARNING !!!
//
// Don't look below, the code is disgusting. This code generates all of the
// possible combinations of the memory operations. Based on bitness, operation
// size, 2 register inputs, idk probably some other stuff.
//
// It might look gross, but honestly, I think it gives us a really cool
// environment above to write the memory hook shellcode. Thus, don't complain
// about it. Go away.
// ===========================================================================

// For each `addr` in `regs`, create the read and write memhooks. Using the
// `addr` as the address register name when creating the code
.macro multiple_create_memhook_int datawidth, width, data, addr, regs:vararg
    // Create the memhook
    create_memhook read,  \datawidth, \width, \data, \addr
    create_memhook write, \datawidth, \width, \data, \addr

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook_int \datawidth, \width, \data, \regs
    .endif
.endm // multiple_create_memhook_int

// For each `data` in `regs`, this extracts the register name to use for the
// data argument. This generates the memhooks for 32-bit usize targets
.macro multiple_create_memhook32 datawidth, reg, regs:vararg
    multiple_create_memhook_int \datawidth, 4, \reg, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook32 \datawidth, \regs
    .endif
.endm

// For each `data` in `regs`, this extracts the register name to use for the
// data argument. This generates the memhooks for 64-bit usize targets
.macro multiple_create_memhook64 datawidth, reg, regs:vararg
    multiple_create_memhook_int \datawidth, 8, \reg, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook64 \datawidth, \regs
    .endif
.endm

// Create all possible memhooks for 32-bit
multiple_create_memhook32 1, al, cl, dl, bl, spl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b, r13b, r14b, r15b
multiple_create_memhook32 2, ax, cx, dx, bx, sp, bp, si, di, r8w, r9w, r10w, r11w, r12w, r13w, r14w, r15w
multiple_create_memhook32 4, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
multiple_create_memhook32 8, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

// Create all possible memhooks for 64-bit
multiple_create_memhook64 1, al, cl, dl, bl, spl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b, r13b, r14b, r15b
multiple_create_memhook64 2, ax, cx, dx, bx, sp, bp, si, di, r8w, r9w, r10w, r11w, r12w, r13w, r14w, r15w
multiple_create_memhook64 4, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
multiple_create_memhook64 8, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

