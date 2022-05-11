//! We just use a disgusting assembly macro to auto-generate all of these
//! functions from a single template.

use std::ptr::addr_of;

extern {
    static cannoli_memhook_eax_eax:     u8;
    static cannoli_memhook_eax_eax_end: u8;
    static cannoli_memhook_ecx_eax:     u8;
    static cannoli_memhook_ecx_eax_end: u8;
    static cannoli_memhook_edx_eax:     u8;
    static cannoli_memhook_edx_eax_end: u8;
    static cannoli_memhook_ebx_eax:     u8;
    static cannoli_memhook_ebx_eax_end: u8;
    static cannoli_memhook_esp_eax:     u8;
    static cannoli_memhook_esp_eax_end: u8;
    static cannoli_memhook_ebp_eax:     u8;
    static cannoli_memhook_ebp_eax_end: u8;
    static cannoli_memhook_esi_eax:     u8;
    static cannoli_memhook_esi_eax_end: u8;
    static cannoli_memhook_edi_eax:     u8;
    static cannoli_memhook_edi_eax_end: u8;
    static cannoli_memhook_r8d_eax:     u8;
    static cannoli_memhook_r8d_eax_end: u8;
    static cannoli_memhook_r9d_eax:     u8;
    static cannoli_memhook_r9d_eax_end: u8;
    static cannoli_memhook_r10d_eax:     u8;
    static cannoli_memhook_r10d_eax_end: u8;
    static cannoli_memhook_r11d_eax:     u8;
    static cannoli_memhook_r11d_eax_end: u8;
    static cannoli_memhook_r12d_eax:     u8;
    static cannoli_memhook_r12d_eax_end: u8;
    static cannoli_memhook_r13d_eax:     u8;
    static cannoli_memhook_r13d_eax_end: u8;
    static cannoli_memhook_r14d_eax:     u8;
    static cannoli_memhook_r14d_eax_end: u8;
    static cannoli_memhook_r15d_eax:     u8;
    static cannoli_memhook_r15d_eax_end: u8;
    static cannoli_memhook_eax_ecx:     u8;
    static cannoli_memhook_eax_ecx_end: u8;
    static cannoli_memhook_ecx_ecx:     u8;
    static cannoli_memhook_ecx_ecx_end: u8;
    static cannoli_memhook_edx_ecx:     u8;
    static cannoli_memhook_edx_ecx_end: u8;
    static cannoli_memhook_ebx_ecx:     u8;
    static cannoli_memhook_ebx_ecx_end: u8;
    static cannoli_memhook_esp_ecx:     u8;
    static cannoli_memhook_esp_ecx_end: u8;
    static cannoli_memhook_ebp_ecx:     u8;
    static cannoli_memhook_ebp_ecx_end: u8;
    static cannoli_memhook_esi_ecx:     u8;
    static cannoli_memhook_esi_ecx_end: u8;
    static cannoli_memhook_edi_ecx:     u8;
    static cannoli_memhook_edi_ecx_end: u8;
    static cannoli_memhook_r8d_ecx:     u8;
    static cannoli_memhook_r8d_ecx_end: u8;
    static cannoli_memhook_r9d_ecx:     u8;
    static cannoli_memhook_r9d_ecx_end: u8;
    static cannoli_memhook_r10d_ecx:     u8;
    static cannoli_memhook_r10d_ecx_end: u8;
    static cannoli_memhook_r11d_ecx:     u8;
    static cannoli_memhook_r11d_ecx_end: u8;
    static cannoli_memhook_r12d_ecx:     u8;
    static cannoli_memhook_r12d_ecx_end: u8;
    static cannoli_memhook_r13d_ecx:     u8;
    static cannoli_memhook_r13d_ecx_end: u8;
    static cannoli_memhook_r14d_ecx:     u8;
    static cannoli_memhook_r14d_ecx_end: u8;
    static cannoli_memhook_r15d_ecx:     u8;
    static cannoli_memhook_r15d_ecx_end: u8;
    static cannoli_memhook_eax_edx:     u8;
    static cannoli_memhook_eax_edx_end: u8;
    static cannoli_memhook_ecx_edx:     u8;
    static cannoli_memhook_ecx_edx_end: u8;
    static cannoli_memhook_edx_edx:     u8;
    static cannoli_memhook_edx_edx_end: u8;
    static cannoli_memhook_ebx_edx:     u8;
    static cannoli_memhook_ebx_edx_end: u8;
    static cannoli_memhook_esp_edx:     u8;
    static cannoli_memhook_esp_edx_end: u8;
    static cannoli_memhook_ebp_edx:     u8;
    static cannoli_memhook_ebp_edx_end: u8;
    static cannoli_memhook_esi_edx:     u8;
    static cannoli_memhook_esi_edx_end: u8;
    static cannoli_memhook_edi_edx:     u8;
    static cannoli_memhook_edi_edx_end: u8;
    static cannoli_memhook_r8d_edx:     u8;
    static cannoli_memhook_r8d_edx_end: u8;
    static cannoli_memhook_r9d_edx:     u8;
    static cannoli_memhook_r9d_edx_end: u8;
    static cannoli_memhook_r10d_edx:     u8;
    static cannoli_memhook_r10d_edx_end: u8;
    static cannoli_memhook_r11d_edx:     u8;
    static cannoli_memhook_r11d_edx_end: u8;
    static cannoli_memhook_r12d_edx:     u8;
    static cannoli_memhook_r12d_edx_end: u8;
    static cannoli_memhook_r13d_edx:     u8;
    static cannoli_memhook_r13d_edx_end: u8;
    static cannoli_memhook_r14d_edx:     u8;
    static cannoli_memhook_r14d_edx_end: u8;
    static cannoli_memhook_r15d_edx:     u8;
    static cannoli_memhook_r15d_edx_end: u8;
    static cannoli_memhook_eax_ebx:     u8;
    static cannoli_memhook_eax_ebx_end: u8;
    static cannoli_memhook_ecx_ebx:     u8;
    static cannoli_memhook_ecx_ebx_end: u8;
    static cannoli_memhook_edx_ebx:     u8;
    static cannoli_memhook_edx_ebx_end: u8;
    static cannoli_memhook_ebx_ebx:     u8;
    static cannoli_memhook_ebx_ebx_end: u8;
    static cannoli_memhook_esp_ebx:     u8;
    static cannoli_memhook_esp_ebx_end: u8;
    static cannoli_memhook_ebp_ebx:     u8;
    static cannoli_memhook_ebp_ebx_end: u8;
    static cannoli_memhook_esi_ebx:     u8;
    static cannoli_memhook_esi_ebx_end: u8;
    static cannoli_memhook_edi_ebx:     u8;
    static cannoli_memhook_edi_ebx_end: u8;
    static cannoli_memhook_r8d_ebx:     u8;
    static cannoli_memhook_r8d_ebx_end: u8;
    static cannoli_memhook_r9d_ebx:     u8;
    static cannoli_memhook_r9d_ebx_end: u8;
    static cannoli_memhook_r10d_ebx:     u8;
    static cannoli_memhook_r10d_ebx_end: u8;
    static cannoli_memhook_r11d_ebx:     u8;
    static cannoli_memhook_r11d_ebx_end: u8;
    static cannoli_memhook_r12d_ebx:     u8;
    static cannoli_memhook_r12d_ebx_end: u8;
    static cannoli_memhook_r13d_ebx:     u8;
    static cannoli_memhook_r13d_ebx_end: u8;
    static cannoli_memhook_r14d_ebx:     u8;
    static cannoli_memhook_r14d_ebx_end: u8;
    static cannoli_memhook_r15d_ebx:     u8;
    static cannoli_memhook_r15d_ebx_end: u8;
    static cannoli_memhook_eax_esp:     u8;
    static cannoli_memhook_eax_esp_end: u8;
    static cannoli_memhook_ecx_esp:     u8;
    static cannoli_memhook_ecx_esp_end: u8;
    static cannoli_memhook_edx_esp:     u8;
    static cannoli_memhook_edx_esp_end: u8;
    static cannoli_memhook_ebx_esp:     u8;
    static cannoli_memhook_ebx_esp_end: u8;
    static cannoli_memhook_esp_esp:     u8;
    static cannoli_memhook_esp_esp_end: u8;
    static cannoli_memhook_ebp_esp:     u8;
    static cannoli_memhook_ebp_esp_end: u8;
    static cannoli_memhook_esi_esp:     u8;
    static cannoli_memhook_esi_esp_end: u8;
    static cannoli_memhook_edi_esp:     u8;
    static cannoli_memhook_edi_esp_end: u8;
    static cannoli_memhook_r8d_esp:     u8;
    static cannoli_memhook_r8d_esp_end: u8;
    static cannoli_memhook_r9d_esp:     u8;
    static cannoli_memhook_r9d_esp_end: u8;
    static cannoli_memhook_r10d_esp:     u8;
    static cannoli_memhook_r10d_esp_end: u8;
    static cannoli_memhook_r11d_esp:     u8;
    static cannoli_memhook_r11d_esp_end: u8;
    static cannoli_memhook_r12d_esp:     u8;
    static cannoli_memhook_r12d_esp_end: u8;
    static cannoli_memhook_r13d_esp:     u8;
    static cannoli_memhook_r13d_esp_end: u8;
    static cannoli_memhook_r14d_esp:     u8;
    static cannoli_memhook_r14d_esp_end: u8;
    static cannoli_memhook_r15d_esp:     u8;
    static cannoli_memhook_r15d_esp_end: u8;
    static cannoli_memhook_eax_ebp:     u8;
    static cannoli_memhook_eax_ebp_end: u8;
    static cannoli_memhook_ecx_ebp:     u8;
    static cannoli_memhook_ecx_ebp_end: u8;
    static cannoli_memhook_edx_ebp:     u8;
    static cannoli_memhook_edx_ebp_end: u8;
    static cannoli_memhook_ebx_ebp:     u8;
    static cannoli_memhook_ebx_ebp_end: u8;
    static cannoli_memhook_esp_ebp:     u8;
    static cannoli_memhook_esp_ebp_end: u8;
    static cannoli_memhook_ebp_ebp:     u8;
    static cannoli_memhook_ebp_ebp_end: u8;
    static cannoli_memhook_esi_ebp:     u8;
    static cannoli_memhook_esi_ebp_end: u8;
    static cannoli_memhook_edi_ebp:     u8;
    static cannoli_memhook_edi_ebp_end: u8;
    static cannoli_memhook_r8d_ebp:     u8;
    static cannoli_memhook_r8d_ebp_end: u8;
    static cannoli_memhook_r9d_ebp:     u8;
    static cannoli_memhook_r9d_ebp_end: u8;
    static cannoli_memhook_r10d_ebp:     u8;
    static cannoli_memhook_r10d_ebp_end: u8;
    static cannoli_memhook_r11d_ebp:     u8;
    static cannoli_memhook_r11d_ebp_end: u8;
    static cannoli_memhook_r12d_ebp:     u8;
    static cannoli_memhook_r12d_ebp_end: u8;
    static cannoli_memhook_r13d_ebp:     u8;
    static cannoli_memhook_r13d_ebp_end: u8;
    static cannoli_memhook_r14d_ebp:     u8;
    static cannoli_memhook_r14d_ebp_end: u8;
    static cannoli_memhook_r15d_ebp:     u8;
    static cannoli_memhook_r15d_ebp_end: u8;
    static cannoli_memhook_eax_esi:     u8;
    static cannoli_memhook_eax_esi_end: u8;
    static cannoli_memhook_ecx_esi:     u8;
    static cannoli_memhook_ecx_esi_end: u8;
    static cannoli_memhook_edx_esi:     u8;
    static cannoli_memhook_edx_esi_end: u8;
    static cannoli_memhook_ebx_esi:     u8;
    static cannoli_memhook_ebx_esi_end: u8;
    static cannoli_memhook_esp_esi:     u8;
    static cannoli_memhook_esp_esi_end: u8;
    static cannoli_memhook_ebp_esi:     u8;
    static cannoli_memhook_ebp_esi_end: u8;
    static cannoli_memhook_esi_esi:     u8;
    static cannoli_memhook_esi_esi_end: u8;
    static cannoli_memhook_edi_esi:     u8;
    static cannoli_memhook_edi_esi_end: u8;
    static cannoli_memhook_r8d_esi:     u8;
    static cannoli_memhook_r8d_esi_end: u8;
    static cannoli_memhook_r9d_esi:     u8;
    static cannoli_memhook_r9d_esi_end: u8;
    static cannoli_memhook_r10d_esi:     u8;
    static cannoli_memhook_r10d_esi_end: u8;
    static cannoli_memhook_r11d_esi:     u8;
    static cannoli_memhook_r11d_esi_end: u8;
    static cannoli_memhook_r12d_esi:     u8;
    static cannoli_memhook_r12d_esi_end: u8;
    static cannoli_memhook_r13d_esi:     u8;
    static cannoli_memhook_r13d_esi_end: u8;
    static cannoli_memhook_r14d_esi:     u8;
    static cannoli_memhook_r14d_esi_end: u8;
    static cannoli_memhook_r15d_esi:     u8;
    static cannoli_memhook_r15d_esi_end: u8;
    static cannoli_memhook_eax_edi:     u8;
    static cannoli_memhook_eax_edi_end: u8;
    static cannoli_memhook_ecx_edi:     u8;
    static cannoli_memhook_ecx_edi_end: u8;
    static cannoli_memhook_edx_edi:     u8;
    static cannoli_memhook_edx_edi_end: u8;
    static cannoli_memhook_ebx_edi:     u8;
    static cannoli_memhook_ebx_edi_end: u8;
    static cannoli_memhook_esp_edi:     u8;
    static cannoli_memhook_esp_edi_end: u8;
    static cannoli_memhook_ebp_edi:     u8;
    static cannoli_memhook_ebp_edi_end: u8;
    static cannoli_memhook_esi_edi:     u8;
    static cannoli_memhook_esi_edi_end: u8;
    static cannoli_memhook_edi_edi:     u8;
    static cannoli_memhook_edi_edi_end: u8;
    static cannoli_memhook_r8d_edi:     u8;
    static cannoli_memhook_r8d_edi_end: u8;
    static cannoli_memhook_r9d_edi:     u8;
    static cannoli_memhook_r9d_edi_end: u8;
    static cannoli_memhook_r10d_edi:     u8;
    static cannoli_memhook_r10d_edi_end: u8;
    static cannoli_memhook_r11d_edi:     u8;
    static cannoli_memhook_r11d_edi_end: u8;
    static cannoli_memhook_r12d_edi:     u8;
    static cannoli_memhook_r12d_edi_end: u8;
    static cannoli_memhook_r13d_edi:     u8;
    static cannoli_memhook_r13d_edi_end: u8;
    static cannoli_memhook_r14d_edi:     u8;
    static cannoli_memhook_r14d_edi_end: u8;
    static cannoli_memhook_r15d_edi:     u8;
    static cannoli_memhook_r15d_edi_end: u8;
    static cannoli_memhook_eax_r8d:     u8;
    static cannoli_memhook_eax_r8d_end: u8;
    static cannoli_memhook_ecx_r8d:     u8;
    static cannoli_memhook_ecx_r8d_end: u8;
    static cannoli_memhook_edx_r8d:     u8;
    static cannoli_memhook_edx_r8d_end: u8;
    static cannoli_memhook_ebx_r8d:     u8;
    static cannoli_memhook_ebx_r8d_end: u8;
    static cannoli_memhook_esp_r8d:     u8;
    static cannoli_memhook_esp_r8d_end: u8;
    static cannoli_memhook_ebp_r8d:     u8;
    static cannoli_memhook_ebp_r8d_end: u8;
    static cannoli_memhook_esi_r8d:     u8;
    static cannoli_memhook_esi_r8d_end: u8;
    static cannoli_memhook_edi_r8d:     u8;
    static cannoli_memhook_edi_r8d_end: u8;
    static cannoli_memhook_r8d_r8d:     u8;
    static cannoli_memhook_r8d_r8d_end: u8;
    static cannoli_memhook_r9d_r8d:     u8;
    static cannoli_memhook_r9d_r8d_end: u8;
    static cannoli_memhook_r10d_r8d:     u8;
    static cannoli_memhook_r10d_r8d_end: u8;
    static cannoli_memhook_r11d_r8d:     u8;
    static cannoli_memhook_r11d_r8d_end: u8;
    static cannoli_memhook_r12d_r8d:     u8;
    static cannoli_memhook_r12d_r8d_end: u8;
    static cannoli_memhook_r13d_r8d:     u8;
    static cannoli_memhook_r13d_r8d_end: u8;
    static cannoli_memhook_r14d_r8d:     u8;
    static cannoli_memhook_r14d_r8d_end: u8;
    static cannoli_memhook_r15d_r8d:     u8;
    static cannoli_memhook_r15d_r8d_end: u8;
    static cannoli_memhook_eax_r9d:     u8;
    static cannoli_memhook_eax_r9d_end: u8;
    static cannoli_memhook_ecx_r9d:     u8;
    static cannoli_memhook_ecx_r9d_end: u8;
    static cannoli_memhook_edx_r9d:     u8;
    static cannoli_memhook_edx_r9d_end: u8;
    static cannoli_memhook_ebx_r9d:     u8;
    static cannoli_memhook_ebx_r9d_end: u8;
    static cannoli_memhook_esp_r9d:     u8;
    static cannoli_memhook_esp_r9d_end: u8;
    static cannoli_memhook_ebp_r9d:     u8;
    static cannoli_memhook_ebp_r9d_end: u8;
    static cannoli_memhook_esi_r9d:     u8;
    static cannoli_memhook_esi_r9d_end: u8;
    static cannoli_memhook_edi_r9d:     u8;
    static cannoli_memhook_edi_r9d_end: u8;
    static cannoli_memhook_r8d_r9d:     u8;
    static cannoli_memhook_r8d_r9d_end: u8;
    static cannoli_memhook_r9d_r9d:     u8;
    static cannoli_memhook_r9d_r9d_end: u8;
    static cannoli_memhook_r10d_r9d:     u8;
    static cannoli_memhook_r10d_r9d_end: u8;
    static cannoli_memhook_r11d_r9d:     u8;
    static cannoli_memhook_r11d_r9d_end: u8;
    static cannoli_memhook_r12d_r9d:     u8;
    static cannoli_memhook_r12d_r9d_end: u8;
    static cannoli_memhook_r13d_r9d:     u8;
    static cannoli_memhook_r13d_r9d_end: u8;
    static cannoli_memhook_r14d_r9d:     u8;
    static cannoli_memhook_r14d_r9d_end: u8;
    static cannoli_memhook_r15d_r9d:     u8;
    static cannoli_memhook_r15d_r9d_end: u8;
    static cannoli_memhook_eax_r10d:     u8;
    static cannoli_memhook_eax_r10d_end: u8;
    static cannoli_memhook_ecx_r10d:     u8;
    static cannoli_memhook_ecx_r10d_end: u8;
    static cannoli_memhook_edx_r10d:     u8;
    static cannoli_memhook_edx_r10d_end: u8;
    static cannoli_memhook_ebx_r10d:     u8;
    static cannoli_memhook_ebx_r10d_end: u8;
    static cannoli_memhook_esp_r10d:     u8;
    static cannoli_memhook_esp_r10d_end: u8;
    static cannoli_memhook_ebp_r10d:     u8;
    static cannoli_memhook_ebp_r10d_end: u8;
    static cannoli_memhook_esi_r10d:     u8;
    static cannoli_memhook_esi_r10d_end: u8;
    static cannoli_memhook_edi_r10d:     u8;
    static cannoli_memhook_edi_r10d_end: u8;
    static cannoli_memhook_r8d_r10d:     u8;
    static cannoli_memhook_r8d_r10d_end: u8;
    static cannoli_memhook_r9d_r10d:     u8;
    static cannoli_memhook_r9d_r10d_end: u8;
    static cannoli_memhook_r10d_r10d:     u8;
    static cannoli_memhook_r10d_r10d_end: u8;
    static cannoli_memhook_r11d_r10d:     u8;
    static cannoli_memhook_r11d_r10d_end: u8;
    static cannoli_memhook_r12d_r10d:     u8;
    static cannoli_memhook_r12d_r10d_end: u8;
    static cannoli_memhook_r13d_r10d:     u8;
    static cannoli_memhook_r13d_r10d_end: u8;
    static cannoli_memhook_r14d_r10d:     u8;
    static cannoli_memhook_r14d_r10d_end: u8;
    static cannoli_memhook_r15d_r10d:     u8;
    static cannoli_memhook_r15d_r10d_end: u8;
    static cannoli_memhook_eax_r11d:     u8;
    static cannoli_memhook_eax_r11d_end: u8;
    static cannoli_memhook_ecx_r11d:     u8;
    static cannoli_memhook_ecx_r11d_end: u8;
    static cannoli_memhook_edx_r11d:     u8;
    static cannoli_memhook_edx_r11d_end: u8;
    static cannoli_memhook_ebx_r11d:     u8;
    static cannoli_memhook_ebx_r11d_end: u8;
    static cannoli_memhook_esp_r11d:     u8;
    static cannoli_memhook_esp_r11d_end: u8;
    static cannoli_memhook_ebp_r11d:     u8;
    static cannoli_memhook_ebp_r11d_end: u8;
    static cannoli_memhook_esi_r11d:     u8;
    static cannoli_memhook_esi_r11d_end: u8;
    static cannoli_memhook_edi_r11d:     u8;
    static cannoli_memhook_edi_r11d_end: u8;
    static cannoli_memhook_r8d_r11d:     u8;
    static cannoli_memhook_r8d_r11d_end: u8;
    static cannoli_memhook_r9d_r11d:     u8;
    static cannoli_memhook_r9d_r11d_end: u8;
    static cannoli_memhook_r10d_r11d:     u8;
    static cannoli_memhook_r10d_r11d_end: u8;
    static cannoli_memhook_r11d_r11d:     u8;
    static cannoli_memhook_r11d_r11d_end: u8;
    static cannoli_memhook_r12d_r11d:     u8;
    static cannoli_memhook_r12d_r11d_end: u8;
    static cannoli_memhook_r13d_r11d:     u8;
    static cannoli_memhook_r13d_r11d_end: u8;
    static cannoli_memhook_r14d_r11d:     u8;
    static cannoli_memhook_r14d_r11d_end: u8;
    static cannoli_memhook_r15d_r11d:     u8;
    static cannoli_memhook_r15d_r11d_end: u8;
    static cannoli_memhook_eax_r12d:     u8;
    static cannoli_memhook_eax_r12d_end: u8;
    static cannoli_memhook_ecx_r12d:     u8;
    static cannoli_memhook_ecx_r12d_end: u8;
    static cannoli_memhook_edx_r12d:     u8;
    static cannoli_memhook_edx_r12d_end: u8;
    static cannoli_memhook_ebx_r12d:     u8;
    static cannoli_memhook_ebx_r12d_end: u8;
    static cannoli_memhook_esp_r12d:     u8;
    static cannoli_memhook_esp_r12d_end: u8;
    static cannoli_memhook_ebp_r12d:     u8;
    static cannoli_memhook_ebp_r12d_end: u8;
    static cannoli_memhook_esi_r12d:     u8;
    static cannoli_memhook_esi_r12d_end: u8;
    static cannoli_memhook_edi_r12d:     u8;
    static cannoli_memhook_edi_r12d_end: u8;
    static cannoli_memhook_r8d_r12d:     u8;
    static cannoli_memhook_r8d_r12d_end: u8;
    static cannoli_memhook_r9d_r12d:     u8;
    static cannoli_memhook_r9d_r12d_end: u8;
    static cannoli_memhook_r10d_r12d:     u8;
    static cannoli_memhook_r10d_r12d_end: u8;
    static cannoli_memhook_r11d_r12d:     u8;
    static cannoli_memhook_r11d_r12d_end: u8;
    static cannoli_memhook_r12d_r12d:     u8;
    static cannoli_memhook_r12d_r12d_end: u8;
    static cannoli_memhook_r13d_r12d:     u8;
    static cannoli_memhook_r13d_r12d_end: u8;
    static cannoli_memhook_r14d_r12d:     u8;
    static cannoli_memhook_r14d_r12d_end: u8;
    static cannoli_memhook_r15d_r12d:     u8;
    static cannoli_memhook_r15d_r12d_end: u8;
    static cannoli_memhook_eax_r13d:     u8;
    static cannoli_memhook_eax_r13d_end: u8;
    static cannoli_memhook_ecx_r13d:     u8;
    static cannoli_memhook_ecx_r13d_end: u8;
    static cannoli_memhook_edx_r13d:     u8;
    static cannoli_memhook_edx_r13d_end: u8;
    static cannoli_memhook_ebx_r13d:     u8;
    static cannoli_memhook_ebx_r13d_end: u8;
    static cannoli_memhook_esp_r13d:     u8;
    static cannoli_memhook_esp_r13d_end: u8;
    static cannoli_memhook_ebp_r13d:     u8;
    static cannoli_memhook_ebp_r13d_end: u8;
    static cannoli_memhook_esi_r13d:     u8;
    static cannoli_memhook_esi_r13d_end: u8;
    static cannoli_memhook_edi_r13d:     u8;
    static cannoli_memhook_edi_r13d_end: u8;
    static cannoli_memhook_r8d_r13d:     u8;
    static cannoli_memhook_r8d_r13d_end: u8;
    static cannoli_memhook_r9d_r13d:     u8;
    static cannoli_memhook_r9d_r13d_end: u8;
    static cannoli_memhook_r10d_r13d:     u8;
    static cannoli_memhook_r10d_r13d_end: u8;
    static cannoli_memhook_r11d_r13d:     u8;
    static cannoli_memhook_r11d_r13d_end: u8;
    static cannoli_memhook_r12d_r13d:     u8;
    static cannoli_memhook_r12d_r13d_end: u8;
    static cannoli_memhook_r13d_r13d:     u8;
    static cannoli_memhook_r13d_r13d_end: u8;
    static cannoli_memhook_r14d_r13d:     u8;
    static cannoli_memhook_r14d_r13d_end: u8;
    static cannoli_memhook_r15d_r13d:     u8;
    static cannoli_memhook_r15d_r13d_end: u8;
    static cannoli_memhook_eax_r14d:     u8;
    static cannoli_memhook_eax_r14d_end: u8;
    static cannoli_memhook_ecx_r14d:     u8;
    static cannoli_memhook_ecx_r14d_end: u8;
    static cannoli_memhook_edx_r14d:     u8;
    static cannoli_memhook_edx_r14d_end: u8;
    static cannoli_memhook_ebx_r14d:     u8;
    static cannoli_memhook_ebx_r14d_end: u8;
    static cannoli_memhook_esp_r14d:     u8;
    static cannoli_memhook_esp_r14d_end: u8;
    static cannoli_memhook_ebp_r14d:     u8;
    static cannoli_memhook_ebp_r14d_end: u8;
    static cannoli_memhook_esi_r14d:     u8;
    static cannoli_memhook_esi_r14d_end: u8;
    static cannoli_memhook_edi_r14d:     u8;
    static cannoli_memhook_edi_r14d_end: u8;
    static cannoli_memhook_r8d_r14d:     u8;
    static cannoli_memhook_r8d_r14d_end: u8;
    static cannoli_memhook_r9d_r14d:     u8;
    static cannoli_memhook_r9d_r14d_end: u8;
    static cannoli_memhook_r10d_r14d:     u8;
    static cannoli_memhook_r10d_r14d_end: u8;
    static cannoli_memhook_r11d_r14d:     u8;
    static cannoli_memhook_r11d_r14d_end: u8;
    static cannoli_memhook_r12d_r14d:     u8;
    static cannoli_memhook_r12d_r14d_end: u8;
    static cannoli_memhook_r13d_r14d:     u8;
    static cannoli_memhook_r13d_r14d_end: u8;
    static cannoli_memhook_r14d_r14d:     u8;
    static cannoli_memhook_r14d_r14d_end: u8;
    static cannoli_memhook_r15d_r14d:     u8;
    static cannoli_memhook_r15d_r14d_end: u8;
    static cannoli_memhook_eax_r15d:     u8;
    static cannoli_memhook_eax_r15d_end: u8;
    static cannoli_memhook_ecx_r15d:     u8;
    static cannoli_memhook_ecx_r15d_end: u8;
    static cannoli_memhook_edx_r15d:     u8;
    static cannoli_memhook_edx_r15d_end: u8;
    static cannoli_memhook_ebx_r15d:     u8;
    static cannoli_memhook_ebx_r15d_end: u8;
    static cannoli_memhook_esp_r15d:     u8;
    static cannoli_memhook_esp_r15d_end: u8;
    static cannoli_memhook_ebp_r15d:     u8;
    static cannoli_memhook_ebp_r15d_end: u8;
    static cannoli_memhook_esi_r15d:     u8;
    static cannoli_memhook_esi_r15d_end: u8;
    static cannoli_memhook_edi_r15d:     u8;
    static cannoli_memhook_edi_r15d_end: u8;
    static cannoli_memhook_r8d_r15d:     u8;
    static cannoli_memhook_r8d_r15d_end: u8;
    static cannoli_memhook_r9d_r15d:     u8;
    static cannoli_memhook_r9d_r15d_end: u8;
    static cannoli_memhook_r10d_r15d:     u8;
    static cannoli_memhook_r10d_r15d_end: u8;
    static cannoli_memhook_r11d_r15d:     u8;
    static cannoli_memhook_r11d_r15d_end: u8;
    static cannoli_memhook_r12d_r15d:     u8;
    static cannoli_memhook_r12d_r15d_end: u8;
    static cannoli_memhook_r13d_r15d:     u8;
    static cannoli_memhook_r13d_r15d_end: u8;
    static cannoli_memhook_r14d_r15d:     u8;
    static cannoli_memhook_r14d_r15d_end: u8;
    static cannoli_memhook_r15d_r15d:     u8;
    static cannoli_memhook_r15d_r15d_end: u8;
    static cannoli_memhook_rax_rax:     u8;
    static cannoli_memhook_rax_rax_end: u8;
    static cannoli_memhook_rcx_rax:     u8;
    static cannoli_memhook_rcx_rax_end: u8;
    static cannoli_memhook_rdx_rax:     u8;
    static cannoli_memhook_rdx_rax_end: u8;
    static cannoli_memhook_rbx_rax:     u8;
    static cannoli_memhook_rbx_rax_end: u8;
    static cannoli_memhook_rsp_rax:     u8;
    static cannoli_memhook_rsp_rax_end: u8;
    static cannoli_memhook_rbp_rax:     u8;
    static cannoli_memhook_rbp_rax_end: u8;
    static cannoli_memhook_rsi_rax:     u8;
    static cannoli_memhook_rsi_rax_end: u8;
    static cannoli_memhook_rdi_rax:     u8;
    static cannoli_memhook_rdi_rax_end: u8;
    static cannoli_memhook_r8_rax:     u8;
    static cannoli_memhook_r8_rax_end: u8;
    static cannoli_memhook_r9_rax:     u8;
    static cannoli_memhook_r9_rax_end: u8;
    static cannoli_memhook_r10_rax:     u8;
    static cannoli_memhook_r10_rax_end: u8;
    static cannoli_memhook_r11_rax:     u8;
    static cannoli_memhook_r11_rax_end: u8;
    static cannoli_memhook_r12_rax:     u8;
    static cannoli_memhook_r12_rax_end: u8;
    static cannoli_memhook_r13_rax:     u8;
    static cannoli_memhook_r13_rax_end: u8;
    static cannoli_memhook_r14_rax:     u8;
    static cannoli_memhook_r14_rax_end: u8;
    static cannoli_memhook_r15_rax:     u8;
    static cannoli_memhook_r15_rax_end: u8;
    static cannoli_memhook_rax_rcx:     u8;
    static cannoli_memhook_rax_rcx_end: u8;
    static cannoli_memhook_rcx_rcx:     u8;
    static cannoli_memhook_rcx_rcx_end: u8;
    static cannoli_memhook_rdx_rcx:     u8;
    static cannoli_memhook_rdx_rcx_end: u8;
    static cannoli_memhook_rbx_rcx:     u8;
    static cannoli_memhook_rbx_rcx_end: u8;
    static cannoli_memhook_rsp_rcx:     u8;
    static cannoli_memhook_rsp_rcx_end: u8;
    static cannoli_memhook_rbp_rcx:     u8;
    static cannoli_memhook_rbp_rcx_end: u8;
    static cannoli_memhook_rsi_rcx:     u8;
    static cannoli_memhook_rsi_rcx_end: u8;
    static cannoli_memhook_rdi_rcx:     u8;
    static cannoli_memhook_rdi_rcx_end: u8;
    static cannoli_memhook_r8_rcx:     u8;
    static cannoli_memhook_r8_rcx_end: u8;
    static cannoli_memhook_r9_rcx:     u8;
    static cannoli_memhook_r9_rcx_end: u8;
    static cannoli_memhook_r10_rcx:     u8;
    static cannoli_memhook_r10_rcx_end: u8;
    static cannoli_memhook_r11_rcx:     u8;
    static cannoli_memhook_r11_rcx_end: u8;
    static cannoli_memhook_r12_rcx:     u8;
    static cannoli_memhook_r12_rcx_end: u8;
    static cannoli_memhook_r13_rcx:     u8;
    static cannoli_memhook_r13_rcx_end: u8;
    static cannoli_memhook_r14_rcx:     u8;
    static cannoli_memhook_r14_rcx_end: u8;
    static cannoli_memhook_r15_rcx:     u8;
    static cannoli_memhook_r15_rcx_end: u8;
    static cannoli_memhook_rax_rdx:     u8;
    static cannoli_memhook_rax_rdx_end: u8;
    static cannoli_memhook_rcx_rdx:     u8;
    static cannoli_memhook_rcx_rdx_end: u8;
    static cannoli_memhook_rdx_rdx:     u8;
    static cannoli_memhook_rdx_rdx_end: u8;
    static cannoli_memhook_rbx_rdx:     u8;
    static cannoli_memhook_rbx_rdx_end: u8;
    static cannoli_memhook_rsp_rdx:     u8;
    static cannoli_memhook_rsp_rdx_end: u8;
    static cannoli_memhook_rbp_rdx:     u8;
    static cannoli_memhook_rbp_rdx_end: u8;
    static cannoli_memhook_rsi_rdx:     u8;
    static cannoli_memhook_rsi_rdx_end: u8;
    static cannoli_memhook_rdi_rdx:     u8;
    static cannoli_memhook_rdi_rdx_end: u8;
    static cannoli_memhook_r8_rdx:     u8;
    static cannoli_memhook_r8_rdx_end: u8;
    static cannoli_memhook_r9_rdx:     u8;
    static cannoli_memhook_r9_rdx_end: u8;
    static cannoli_memhook_r10_rdx:     u8;
    static cannoli_memhook_r10_rdx_end: u8;
    static cannoli_memhook_r11_rdx:     u8;
    static cannoli_memhook_r11_rdx_end: u8;
    static cannoli_memhook_r12_rdx:     u8;
    static cannoli_memhook_r12_rdx_end: u8;
    static cannoli_memhook_r13_rdx:     u8;
    static cannoli_memhook_r13_rdx_end: u8;
    static cannoli_memhook_r14_rdx:     u8;
    static cannoli_memhook_r14_rdx_end: u8;
    static cannoli_memhook_r15_rdx:     u8;
    static cannoli_memhook_r15_rdx_end: u8;
    static cannoli_memhook_rax_rbx:     u8;
    static cannoli_memhook_rax_rbx_end: u8;
    static cannoli_memhook_rcx_rbx:     u8;
    static cannoli_memhook_rcx_rbx_end: u8;
    static cannoli_memhook_rdx_rbx:     u8;
    static cannoli_memhook_rdx_rbx_end: u8;
    static cannoli_memhook_rbx_rbx:     u8;
    static cannoli_memhook_rbx_rbx_end: u8;
    static cannoli_memhook_rsp_rbx:     u8;
    static cannoli_memhook_rsp_rbx_end: u8;
    static cannoli_memhook_rbp_rbx:     u8;
    static cannoli_memhook_rbp_rbx_end: u8;
    static cannoli_memhook_rsi_rbx:     u8;
    static cannoli_memhook_rsi_rbx_end: u8;
    static cannoli_memhook_rdi_rbx:     u8;
    static cannoli_memhook_rdi_rbx_end: u8;
    static cannoli_memhook_r8_rbx:     u8;
    static cannoli_memhook_r8_rbx_end: u8;
    static cannoli_memhook_r9_rbx:     u8;
    static cannoli_memhook_r9_rbx_end: u8;
    static cannoli_memhook_r10_rbx:     u8;
    static cannoli_memhook_r10_rbx_end: u8;
    static cannoli_memhook_r11_rbx:     u8;
    static cannoli_memhook_r11_rbx_end: u8;
    static cannoli_memhook_r12_rbx:     u8;
    static cannoli_memhook_r12_rbx_end: u8;
    static cannoli_memhook_r13_rbx:     u8;
    static cannoli_memhook_r13_rbx_end: u8;
    static cannoli_memhook_r14_rbx:     u8;
    static cannoli_memhook_r14_rbx_end: u8;
    static cannoli_memhook_r15_rbx:     u8;
    static cannoli_memhook_r15_rbx_end: u8;
    static cannoli_memhook_rax_rsp:     u8;
    static cannoli_memhook_rax_rsp_end: u8;
    static cannoli_memhook_rcx_rsp:     u8;
    static cannoli_memhook_rcx_rsp_end: u8;
    static cannoli_memhook_rdx_rsp:     u8;
    static cannoli_memhook_rdx_rsp_end: u8;
    static cannoli_memhook_rbx_rsp:     u8;
    static cannoli_memhook_rbx_rsp_end: u8;
    static cannoli_memhook_rsp_rsp:     u8;
    static cannoli_memhook_rsp_rsp_end: u8;
    static cannoli_memhook_rbp_rsp:     u8;
    static cannoli_memhook_rbp_rsp_end: u8;
    static cannoli_memhook_rsi_rsp:     u8;
    static cannoli_memhook_rsi_rsp_end: u8;
    static cannoli_memhook_rdi_rsp:     u8;
    static cannoli_memhook_rdi_rsp_end: u8;
    static cannoli_memhook_r8_rsp:     u8;
    static cannoli_memhook_r8_rsp_end: u8;
    static cannoli_memhook_r9_rsp:     u8;
    static cannoli_memhook_r9_rsp_end: u8;
    static cannoli_memhook_r10_rsp:     u8;
    static cannoli_memhook_r10_rsp_end: u8;
    static cannoli_memhook_r11_rsp:     u8;
    static cannoli_memhook_r11_rsp_end: u8;
    static cannoli_memhook_r12_rsp:     u8;
    static cannoli_memhook_r12_rsp_end: u8;
    static cannoli_memhook_r13_rsp:     u8;
    static cannoli_memhook_r13_rsp_end: u8;
    static cannoli_memhook_r14_rsp:     u8;
    static cannoli_memhook_r14_rsp_end: u8;
    static cannoli_memhook_r15_rsp:     u8;
    static cannoli_memhook_r15_rsp_end: u8;
    static cannoli_memhook_rax_rbp:     u8;
    static cannoli_memhook_rax_rbp_end: u8;
    static cannoli_memhook_rcx_rbp:     u8;
    static cannoli_memhook_rcx_rbp_end: u8;
    static cannoli_memhook_rdx_rbp:     u8;
    static cannoli_memhook_rdx_rbp_end: u8;
    static cannoli_memhook_rbx_rbp:     u8;
    static cannoli_memhook_rbx_rbp_end: u8;
    static cannoli_memhook_rsp_rbp:     u8;
    static cannoli_memhook_rsp_rbp_end: u8;
    static cannoli_memhook_rbp_rbp:     u8;
    static cannoli_memhook_rbp_rbp_end: u8;
    static cannoli_memhook_rsi_rbp:     u8;
    static cannoli_memhook_rsi_rbp_end: u8;
    static cannoli_memhook_rdi_rbp:     u8;
    static cannoli_memhook_rdi_rbp_end: u8;
    static cannoli_memhook_r8_rbp:     u8;
    static cannoli_memhook_r8_rbp_end: u8;
    static cannoli_memhook_r9_rbp:     u8;
    static cannoli_memhook_r9_rbp_end: u8;
    static cannoli_memhook_r10_rbp:     u8;
    static cannoli_memhook_r10_rbp_end: u8;
    static cannoli_memhook_r11_rbp:     u8;
    static cannoli_memhook_r11_rbp_end: u8;
    static cannoli_memhook_r12_rbp:     u8;
    static cannoli_memhook_r12_rbp_end: u8;
    static cannoli_memhook_r13_rbp:     u8;
    static cannoli_memhook_r13_rbp_end: u8;
    static cannoli_memhook_r14_rbp:     u8;
    static cannoli_memhook_r14_rbp_end: u8;
    static cannoli_memhook_r15_rbp:     u8;
    static cannoli_memhook_r15_rbp_end: u8;
    static cannoli_memhook_rax_rsi:     u8;
    static cannoli_memhook_rax_rsi_end: u8;
    static cannoli_memhook_rcx_rsi:     u8;
    static cannoli_memhook_rcx_rsi_end: u8;
    static cannoli_memhook_rdx_rsi:     u8;
    static cannoli_memhook_rdx_rsi_end: u8;
    static cannoli_memhook_rbx_rsi:     u8;
    static cannoli_memhook_rbx_rsi_end: u8;
    static cannoli_memhook_rsp_rsi:     u8;
    static cannoli_memhook_rsp_rsi_end: u8;
    static cannoli_memhook_rbp_rsi:     u8;
    static cannoli_memhook_rbp_rsi_end: u8;
    static cannoli_memhook_rsi_rsi:     u8;
    static cannoli_memhook_rsi_rsi_end: u8;
    static cannoli_memhook_rdi_rsi:     u8;
    static cannoli_memhook_rdi_rsi_end: u8;
    static cannoli_memhook_r8_rsi:     u8;
    static cannoli_memhook_r8_rsi_end: u8;
    static cannoli_memhook_r9_rsi:     u8;
    static cannoli_memhook_r9_rsi_end: u8;
    static cannoli_memhook_r10_rsi:     u8;
    static cannoli_memhook_r10_rsi_end: u8;
    static cannoli_memhook_r11_rsi:     u8;
    static cannoli_memhook_r11_rsi_end: u8;
    static cannoli_memhook_r12_rsi:     u8;
    static cannoli_memhook_r12_rsi_end: u8;
    static cannoli_memhook_r13_rsi:     u8;
    static cannoli_memhook_r13_rsi_end: u8;
    static cannoli_memhook_r14_rsi:     u8;
    static cannoli_memhook_r14_rsi_end: u8;
    static cannoli_memhook_r15_rsi:     u8;
    static cannoli_memhook_r15_rsi_end: u8;
    static cannoli_memhook_rax_rdi:     u8;
    static cannoli_memhook_rax_rdi_end: u8;
    static cannoli_memhook_rcx_rdi:     u8;
    static cannoli_memhook_rcx_rdi_end: u8;
    static cannoli_memhook_rdx_rdi:     u8;
    static cannoli_memhook_rdx_rdi_end: u8;
    static cannoli_memhook_rbx_rdi:     u8;
    static cannoli_memhook_rbx_rdi_end: u8;
    static cannoli_memhook_rsp_rdi:     u8;
    static cannoli_memhook_rsp_rdi_end: u8;
    static cannoli_memhook_rbp_rdi:     u8;
    static cannoli_memhook_rbp_rdi_end: u8;
    static cannoli_memhook_rsi_rdi:     u8;
    static cannoli_memhook_rsi_rdi_end: u8;
    static cannoli_memhook_rdi_rdi:     u8;
    static cannoli_memhook_rdi_rdi_end: u8;
    static cannoli_memhook_r8_rdi:     u8;
    static cannoli_memhook_r8_rdi_end: u8;
    static cannoli_memhook_r9_rdi:     u8;
    static cannoli_memhook_r9_rdi_end: u8;
    static cannoli_memhook_r10_rdi:     u8;
    static cannoli_memhook_r10_rdi_end: u8;
    static cannoli_memhook_r11_rdi:     u8;
    static cannoli_memhook_r11_rdi_end: u8;
    static cannoli_memhook_r12_rdi:     u8;
    static cannoli_memhook_r12_rdi_end: u8;
    static cannoli_memhook_r13_rdi:     u8;
    static cannoli_memhook_r13_rdi_end: u8;
    static cannoli_memhook_r14_rdi:     u8;
    static cannoli_memhook_r14_rdi_end: u8;
    static cannoli_memhook_r15_rdi:     u8;
    static cannoli_memhook_r15_rdi_end: u8;
    static cannoli_memhook_rax_r8:     u8;
    static cannoli_memhook_rax_r8_end: u8;
    static cannoli_memhook_rcx_r8:     u8;
    static cannoli_memhook_rcx_r8_end: u8;
    static cannoli_memhook_rdx_r8:     u8;
    static cannoli_memhook_rdx_r8_end: u8;
    static cannoli_memhook_rbx_r8:     u8;
    static cannoli_memhook_rbx_r8_end: u8;
    static cannoli_memhook_rsp_r8:     u8;
    static cannoli_memhook_rsp_r8_end: u8;
    static cannoli_memhook_rbp_r8:     u8;
    static cannoli_memhook_rbp_r8_end: u8;
    static cannoli_memhook_rsi_r8:     u8;
    static cannoli_memhook_rsi_r8_end: u8;
    static cannoli_memhook_rdi_r8:     u8;
    static cannoli_memhook_rdi_r8_end: u8;
    static cannoli_memhook_r8_r8:     u8;
    static cannoli_memhook_r8_r8_end: u8;
    static cannoli_memhook_r9_r8:     u8;
    static cannoli_memhook_r9_r8_end: u8;
    static cannoli_memhook_r10_r8:     u8;
    static cannoli_memhook_r10_r8_end: u8;
    static cannoli_memhook_r11_r8:     u8;
    static cannoli_memhook_r11_r8_end: u8;
    static cannoli_memhook_r12_r8:     u8;
    static cannoli_memhook_r12_r8_end: u8;
    static cannoli_memhook_r13_r8:     u8;
    static cannoli_memhook_r13_r8_end: u8;
    static cannoli_memhook_r14_r8:     u8;
    static cannoli_memhook_r14_r8_end: u8;
    static cannoli_memhook_r15_r8:     u8;
    static cannoli_memhook_r15_r8_end: u8;
    static cannoli_memhook_rax_r9:     u8;
    static cannoli_memhook_rax_r9_end: u8;
    static cannoli_memhook_rcx_r9:     u8;
    static cannoli_memhook_rcx_r9_end: u8;
    static cannoli_memhook_rdx_r9:     u8;
    static cannoli_memhook_rdx_r9_end: u8;
    static cannoli_memhook_rbx_r9:     u8;
    static cannoli_memhook_rbx_r9_end: u8;
    static cannoli_memhook_rsp_r9:     u8;
    static cannoli_memhook_rsp_r9_end: u8;
    static cannoli_memhook_rbp_r9:     u8;
    static cannoli_memhook_rbp_r9_end: u8;
    static cannoli_memhook_rsi_r9:     u8;
    static cannoli_memhook_rsi_r9_end: u8;
    static cannoli_memhook_rdi_r9:     u8;
    static cannoli_memhook_rdi_r9_end: u8;
    static cannoli_memhook_r8_r9:     u8;
    static cannoli_memhook_r8_r9_end: u8;
    static cannoli_memhook_r9_r9:     u8;
    static cannoli_memhook_r9_r9_end: u8;
    static cannoli_memhook_r10_r9:     u8;
    static cannoli_memhook_r10_r9_end: u8;
    static cannoli_memhook_r11_r9:     u8;
    static cannoli_memhook_r11_r9_end: u8;
    static cannoli_memhook_r12_r9:     u8;
    static cannoli_memhook_r12_r9_end: u8;
    static cannoli_memhook_r13_r9:     u8;
    static cannoli_memhook_r13_r9_end: u8;
    static cannoli_memhook_r14_r9:     u8;
    static cannoli_memhook_r14_r9_end: u8;
    static cannoli_memhook_r15_r9:     u8;
    static cannoli_memhook_r15_r9_end: u8;
    static cannoli_memhook_rax_r10:     u8;
    static cannoli_memhook_rax_r10_end: u8;
    static cannoli_memhook_rcx_r10:     u8;
    static cannoli_memhook_rcx_r10_end: u8;
    static cannoli_memhook_rdx_r10:     u8;
    static cannoli_memhook_rdx_r10_end: u8;
    static cannoli_memhook_rbx_r10:     u8;
    static cannoli_memhook_rbx_r10_end: u8;
    static cannoli_memhook_rsp_r10:     u8;
    static cannoli_memhook_rsp_r10_end: u8;
    static cannoli_memhook_rbp_r10:     u8;
    static cannoli_memhook_rbp_r10_end: u8;
    static cannoli_memhook_rsi_r10:     u8;
    static cannoli_memhook_rsi_r10_end: u8;
    static cannoli_memhook_rdi_r10:     u8;
    static cannoli_memhook_rdi_r10_end: u8;
    static cannoli_memhook_r8_r10:     u8;
    static cannoli_memhook_r8_r10_end: u8;
    static cannoli_memhook_r9_r10:     u8;
    static cannoli_memhook_r9_r10_end: u8;
    static cannoli_memhook_r10_r10:     u8;
    static cannoli_memhook_r10_r10_end: u8;
    static cannoli_memhook_r11_r10:     u8;
    static cannoli_memhook_r11_r10_end: u8;
    static cannoli_memhook_r12_r10:     u8;
    static cannoli_memhook_r12_r10_end: u8;
    static cannoli_memhook_r13_r10:     u8;
    static cannoli_memhook_r13_r10_end: u8;
    static cannoli_memhook_r14_r10:     u8;
    static cannoli_memhook_r14_r10_end: u8;
    static cannoli_memhook_r15_r10:     u8;
    static cannoli_memhook_r15_r10_end: u8;
    static cannoli_memhook_rax_r11:     u8;
    static cannoli_memhook_rax_r11_end: u8;
    static cannoli_memhook_rcx_r11:     u8;
    static cannoli_memhook_rcx_r11_end: u8;
    static cannoli_memhook_rdx_r11:     u8;
    static cannoli_memhook_rdx_r11_end: u8;
    static cannoli_memhook_rbx_r11:     u8;
    static cannoli_memhook_rbx_r11_end: u8;
    static cannoli_memhook_rsp_r11:     u8;
    static cannoli_memhook_rsp_r11_end: u8;
    static cannoli_memhook_rbp_r11:     u8;
    static cannoli_memhook_rbp_r11_end: u8;
    static cannoli_memhook_rsi_r11:     u8;
    static cannoli_memhook_rsi_r11_end: u8;
    static cannoli_memhook_rdi_r11:     u8;
    static cannoli_memhook_rdi_r11_end: u8;
    static cannoli_memhook_r8_r11:     u8;
    static cannoli_memhook_r8_r11_end: u8;
    static cannoli_memhook_r9_r11:     u8;
    static cannoli_memhook_r9_r11_end: u8;
    static cannoli_memhook_r10_r11:     u8;
    static cannoli_memhook_r10_r11_end: u8;
    static cannoli_memhook_r11_r11:     u8;
    static cannoli_memhook_r11_r11_end: u8;
    static cannoli_memhook_r12_r11:     u8;
    static cannoli_memhook_r12_r11_end: u8;
    static cannoli_memhook_r13_r11:     u8;
    static cannoli_memhook_r13_r11_end: u8;
    static cannoli_memhook_r14_r11:     u8;
    static cannoli_memhook_r14_r11_end: u8;
    static cannoli_memhook_r15_r11:     u8;
    static cannoli_memhook_r15_r11_end: u8;
    static cannoli_memhook_rax_r12:     u8;
    static cannoli_memhook_rax_r12_end: u8;
    static cannoli_memhook_rcx_r12:     u8;
    static cannoli_memhook_rcx_r12_end: u8;
    static cannoli_memhook_rdx_r12:     u8;
    static cannoli_memhook_rdx_r12_end: u8;
    static cannoli_memhook_rbx_r12:     u8;
    static cannoli_memhook_rbx_r12_end: u8;
    static cannoli_memhook_rsp_r12:     u8;
    static cannoli_memhook_rsp_r12_end: u8;
    static cannoli_memhook_rbp_r12:     u8;
    static cannoli_memhook_rbp_r12_end: u8;
    static cannoli_memhook_rsi_r12:     u8;
    static cannoli_memhook_rsi_r12_end: u8;
    static cannoli_memhook_rdi_r12:     u8;
    static cannoli_memhook_rdi_r12_end: u8;
    static cannoli_memhook_r8_r12:     u8;
    static cannoli_memhook_r8_r12_end: u8;
    static cannoli_memhook_r9_r12:     u8;
    static cannoli_memhook_r9_r12_end: u8;
    static cannoli_memhook_r10_r12:     u8;
    static cannoli_memhook_r10_r12_end: u8;
    static cannoli_memhook_r11_r12:     u8;
    static cannoli_memhook_r11_r12_end: u8;
    static cannoli_memhook_r12_r12:     u8;
    static cannoli_memhook_r12_r12_end: u8;
    static cannoli_memhook_r13_r12:     u8;
    static cannoli_memhook_r13_r12_end: u8;
    static cannoli_memhook_r14_r12:     u8;
    static cannoli_memhook_r14_r12_end: u8;
    static cannoli_memhook_r15_r12:     u8;
    static cannoli_memhook_r15_r12_end: u8;
    static cannoli_memhook_rax_r13:     u8;
    static cannoli_memhook_rax_r13_end: u8;
    static cannoli_memhook_rcx_r13:     u8;
    static cannoli_memhook_rcx_r13_end: u8;
    static cannoli_memhook_rdx_r13:     u8;
    static cannoli_memhook_rdx_r13_end: u8;
    static cannoli_memhook_rbx_r13:     u8;
    static cannoli_memhook_rbx_r13_end: u8;
    static cannoli_memhook_rsp_r13:     u8;
    static cannoli_memhook_rsp_r13_end: u8;
    static cannoli_memhook_rbp_r13:     u8;
    static cannoli_memhook_rbp_r13_end: u8;
    static cannoli_memhook_rsi_r13:     u8;
    static cannoli_memhook_rsi_r13_end: u8;
    static cannoli_memhook_rdi_r13:     u8;
    static cannoli_memhook_rdi_r13_end: u8;
    static cannoli_memhook_r8_r13:     u8;
    static cannoli_memhook_r8_r13_end: u8;
    static cannoli_memhook_r9_r13:     u8;
    static cannoli_memhook_r9_r13_end: u8;
    static cannoli_memhook_r10_r13:     u8;
    static cannoli_memhook_r10_r13_end: u8;
    static cannoli_memhook_r11_r13:     u8;
    static cannoli_memhook_r11_r13_end: u8;
    static cannoli_memhook_r12_r13:     u8;
    static cannoli_memhook_r12_r13_end: u8;
    static cannoli_memhook_r13_r13:     u8;
    static cannoli_memhook_r13_r13_end: u8;
    static cannoli_memhook_r14_r13:     u8;
    static cannoli_memhook_r14_r13_end: u8;
    static cannoli_memhook_r15_r13:     u8;
    static cannoli_memhook_r15_r13_end: u8;
    static cannoli_memhook_rax_r14:     u8;
    static cannoli_memhook_rax_r14_end: u8;
    static cannoli_memhook_rcx_r14:     u8;
    static cannoli_memhook_rcx_r14_end: u8;
    static cannoli_memhook_rdx_r14:     u8;
    static cannoli_memhook_rdx_r14_end: u8;
    static cannoli_memhook_rbx_r14:     u8;
    static cannoli_memhook_rbx_r14_end: u8;
    static cannoli_memhook_rsp_r14:     u8;
    static cannoli_memhook_rsp_r14_end: u8;
    static cannoli_memhook_rbp_r14:     u8;
    static cannoli_memhook_rbp_r14_end: u8;
    static cannoli_memhook_rsi_r14:     u8;
    static cannoli_memhook_rsi_r14_end: u8;
    static cannoli_memhook_rdi_r14:     u8;
    static cannoli_memhook_rdi_r14_end: u8;
    static cannoli_memhook_r8_r14:     u8;
    static cannoli_memhook_r8_r14_end: u8;
    static cannoli_memhook_r9_r14:     u8;
    static cannoli_memhook_r9_r14_end: u8;
    static cannoli_memhook_r10_r14:     u8;
    static cannoli_memhook_r10_r14_end: u8;
    static cannoli_memhook_r11_r14:     u8;
    static cannoli_memhook_r11_r14_end: u8;
    static cannoli_memhook_r12_r14:     u8;
    static cannoli_memhook_r12_r14_end: u8;
    static cannoli_memhook_r13_r14:     u8;
    static cannoli_memhook_r13_r14_end: u8;
    static cannoli_memhook_r14_r14:     u8;
    static cannoli_memhook_r14_r14_end: u8;
    static cannoli_memhook_r15_r14:     u8;
    static cannoli_memhook_r15_r14_end: u8;
    static cannoli_memhook_rax_r15:     u8;
    static cannoli_memhook_rax_r15_end: u8;
    static cannoli_memhook_rcx_r15:     u8;
    static cannoli_memhook_rcx_r15_end: u8;
    static cannoli_memhook_rdx_r15:     u8;
    static cannoli_memhook_rdx_r15_end: u8;
    static cannoli_memhook_rbx_r15:     u8;
    static cannoli_memhook_rbx_r15_end: u8;
    static cannoli_memhook_rsp_r15:     u8;
    static cannoli_memhook_rsp_r15_end: u8;
    static cannoli_memhook_rbp_r15:     u8;
    static cannoli_memhook_rbp_r15_end: u8;
    static cannoli_memhook_rsi_r15:     u8;
    static cannoli_memhook_rsi_r15_end: u8;
    static cannoli_memhook_rdi_r15:     u8;
    static cannoli_memhook_rdi_r15_end: u8;
    static cannoli_memhook_r8_r15:     u8;
    static cannoli_memhook_r8_r15_end: u8;
    static cannoli_memhook_r9_r15:     u8;
    static cannoli_memhook_r9_r15_end: u8;
    static cannoli_memhook_r10_r15:     u8;
    static cannoli_memhook_r10_r15_end: u8;
    static cannoli_memhook_r11_r15:     u8;
    static cannoli_memhook_r11_r15_end: u8;
    static cannoli_memhook_r12_r15:     u8;
    static cannoli_memhook_r12_r15_end: u8;
    static cannoli_memhook_r13_r15:     u8;
    static cannoli_memhook_r13_r15_end: u8;
    static cannoli_memhook_r14_r15:     u8;
    static cannoli_memhook_r14_r15_end: u8;
    static cannoli_memhook_r15_r15:     u8;
    static cannoli_memhook_r15_r15_end: u8;
}

/// Memory hook table, indexed by `MEMHOOK_TABLE[bitness][data][addr]` where
/// `bitness` is 0 for 32-bit and 1 for 64-bit, and `data` and `addr` being
/// the register indicies that currently holds those values
pub static MEMHOOK_TABLE: [[[(&u8, &u8); 16]; 16]; 2] = [
    [
        [
            unsafe { (&cannoli_memhook_eax_eax, &cannoli_memhook_eax_eax_end) },
            unsafe { (&cannoli_memhook_eax_ecx, &cannoli_memhook_eax_ecx_end) },
            unsafe { (&cannoli_memhook_eax_edx, &cannoli_memhook_eax_edx_end) },
            unsafe { (&cannoli_memhook_eax_ebx, &cannoli_memhook_eax_ebx_end) },
            unsafe { (&cannoli_memhook_eax_esp, &cannoli_memhook_eax_esp_end) },
            unsafe { (&cannoli_memhook_eax_ebp, &cannoli_memhook_eax_ebp_end) },
            unsafe { (&cannoli_memhook_eax_esi, &cannoli_memhook_eax_esi_end) },
            unsafe { (&cannoli_memhook_eax_edi, &cannoli_memhook_eax_edi_end) },
            unsafe { (&cannoli_memhook_eax_r8d, &cannoli_memhook_eax_r8d_end) },
            unsafe { (&cannoli_memhook_eax_r9d, &cannoli_memhook_eax_r9d_end) },
            unsafe { (&cannoli_memhook_eax_r10d, &cannoli_memhook_eax_r10d_end) },
            unsafe { (&cannoli_memhook_eax_r11d, &cannoli_memhook_eax_r11d_end) },
            unsafe { (&cannoli_memhook_eax_r12d, &cannoli_memhook_eax_r12d_end) },
            unsafe { (&cannoli_memhook_eax_r13d, &cannoli_memhook_eax_r13d_end) },
            unsafe { (&cannoli_memhook_eax_r14d, &cannoli_memhook_eax_r14d_end) },
            unsafe { (&cannoli_memhook_eax_r15d, &cannoli_memhook_eax_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_ecx_eax, &cannoli_memhook_ecx_eax_end) },
            unsafe { (&cannoli_memhook_ecx_ecx, &cannoli_memhook_ecx_ecx_end) },
            unsafe { (&cannoli_memhook_ecx_edx, &cannoli_memhook_ecx_edx_end) },
            unsafe { (&cannoli_memhook_ecx_ebx, &cannoli_memhook_ecx_ebx_end) },
            unsafe { (&cannoli_memhook_ecx_esp, &cannoli_memhook_ecx_esp_end) },
            unsafe { (&cannoli_memhook_ecx_ebp, &cannoli_memhook_ecx_ebp_end) },
            unsafe { (&cannoli_memhook_ecx_esi, &cannoli_memhook_ecx_esi_end) },
            unsafe { (&cannoli_memhook_ecx_edi, &cannoli_memhook_ecx_edi_end) },
            unsafe { (&cannoli_memhook_ecx_r8d, &cannoli_memhook_ecx_r8d_end) },
            unsafe { (&cannoli_memhook_ecx_r9d, &cannoli_memhook_ecx_r9d_end) },
            unsafe { (&cannoli_memhook_ecx_r10d, &cannoli_memhook_ecx_r10d_end) },
            unsafe { (&cannoli_memhook_ecx_r11d, &cannoli_memhook_ecx_r11d_end) },
            unsafe { (&cannoli_memhook_ecx_r12d, &cannoli_memhook_ecx_r12d_end) },
            unsafe { (&cannoli_memhook_ecx_r13d, &cannoli_memhook_ecx_r13d_end) },
            unsafe { (&cannoli_memhook_ecx_r14d, &cannoli_memhook_ecx_r14d_end) },
            unsafe { (&cannoli_memhook_ecx_r15d, &cannoli_memhook_ecx_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_edx_eax, &cannoli_memhook_edx_eax_end) },
            unsafe { (&cannoli_memhook_edx_ecx, &cannoli_memhook_edx_ecx_end) },
            unsafe { (&cannoli_memhook_edx_edx, &cannoli_memhook_edx_edx_end) },
            unsafe { (&cannoli_memhook_edx_ebx, &cannoli_memhook_edx_ebx_end) },
            unsafe { (&cannoli_memhook_edx_esp, &cannoli_memhook_edx_esp_end) },
            unsafe { (&cannoli_memhook_edx_ebp, &cannoli_memhook_edx_ebp_end) },
            unsafe { (&cannoli_memhook_edx_esi, &cannoli_memhook_edx_esi_end) },
            unsafe { (&cannoli_memhook_edx_edi, &cannoli_memhook_edx_edi_end) },
            unsafe { (&cannoli_memhook_edx_r8d, &cannoli_memhook_edx_r8d_end) },
            unsafe { (&cannoli_memhook_edx_r9d, &cannoli_memhook_edx_r9d_end) },
            unsafe { (&cannoli_memhook_edx_r10d, &cannoli_memhook_edx_r10d_end) },
            unsafe { (&cannoli_memhook_edx_r11d, &cannoli_memhook_edx_r11d_end) },
            unsafe { (&cannoli_memhook_edx_r12d, &cannoli_memhook_edx_r12d_end) },
            unsafe { (&cannoli_memhook_edx_r13d, &cannoli_memhook_edx_r13d_end) },
            unsafe { (&cannoli_memhook_edx_r14d, &cannoli_memhook_edx_r14d_end) },
            unsafe { (&cannoli_memhook_edx_r15d, &cannoli_memhook_edx_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_ebx_eax, &cannoli_memhook_ebx_eax_end) },
            unsafe { (&cannoli_memhook_ebx_ecx, &cannoli_memhook_ebx_ecx_end) },
            unsafe { (&cannoli_memhook_ebx_edx, &cannoli_memhook_ebx_edx_end) },
            unsafe { (&cannoli_memhook_ebx_ebx, &cannoli_memhook_ebx_ebx_end) },
            unsafe { (&cannoli_memhook_ebx_esp, &cannoli_memhook_ebx_esp_end) },
            unsafe { (&cannoli_memhook_ebx_ebp, &cannoli_memhook_ebx_ebp_end) },
            unsafe { (&cannoli_memhook_ebx_esi, &cannoli_memhook_ebx_esi_end) },
            unsafe { (&cannoli_memhook_ebx_edi, &cannoli_memhook_ebx_edi_end) },
            unsafe { (&cannoli_memhook_ebx_r8d, &cannoli_memhook_ebx_r8d_end) },
            unsafe { (&cannoli_memhook_ebx_r9d, &cannoli_memhook_ebx_r9d_end) },
            unsafe { (&cannoli_memhook_ebx_r10d, &cannoli_memhook_ebx_r10d_end) },
            unsafe { (&cannoli_memhook_ebx_r11d, &cannoli_memhook_ebx_r11d_end) },
            unsafe { (&cannoli_memhook_ebx_r12d, &cannoli_memhook_ebx_r12d_end) },
            unsafe { (&cannoli_memhook_ebx_r13d, &cannoli_memhook_ebx_r13d_end) },
            unsafe { (&cannoli_memhook_ebx_r14d, &cannoli_memhook_ebx_r14d_end) },
            unsafe { (&cannoli_memhook_ebx_r15d, &cannoli_memhook_ebx_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_esp_eax, &cannoli_memhook_esp_eax_end) },
            unsafe { (&cannoli_memhook_esp_ecx, &cannoli_memhook_esp_ecx_end) },
            unsafe { (&cannoli_memhook_esp_edx, &cannoli_memhook_esp_edx_end) },
            unsafe { (&cannoli_memhook_esp_ebx, &cannoli_memhook_esp_ebx_end) },
            unsafe { (&cannoli_memhook_esp_esp, &cannoli_memhook_esp_esp_end) },
            unsafe { (&cannoli_memhook_esp_ebp, &cannoli_memhook_esp_ebp_end) },
            unsafe { (&cannoli_memhook_esp_esi, &cannoli_memhook_esp_esi_end) },
            unsafe { (&cannoli_memhook_esp_edi, &cannoli_memhook_esp_edi_end) },
            unsafe { (&cannoli_memhook_esp_r8d, &cannoli_memhook_esp_r8d_end) },
            unsafe { (&cannoli_memhook_esp_r9d, &cannoli_memhook_esp_r9d_end) },
            unsafe { (&cannoli_memhook_esp_r10d, &cannoli_memhook_esp_r10d_end) },
            unsafe { (&cannoli_memhook_esp_r11d, &cannoli_memhook_esp_r11d_end) },
            unsafe { (&cannoli_memhook_esp_r12d, &cannoli_memhook_esp_r12d_end) },
            unsafe { (&cannoli_memhook_esp_r13d, &cannoli_memhook_esp_r13d_end) },
            unsafe { (&cannoli_memhook_esp_r14d, &cannoli_memhook_esp_r14d_end) },
            unsafe { (&cannoli_memhook_esp_r15d, &cannoli_memhook_esp_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_ebp_eax, &cannoli_memhook_ebp_eax_end) },
            unsafe { (&cannoli_memhook_ebp_ecx, &cannoli_memhook_ebp_ecx_end) },
            unsafe { (&cannoli_memhook_ebp_edx, &cannoli_memhook_ebp_edx_end) },
            unsafe { (&cannoli_memhook_ebp_ebx, &cannoli_memhook_ebp_ebx_end) },
            unsafe { (&cannoli_memhook_ebp_esp, &cannoli_memhook_ebp_esp_end) },
            unsafe { (&cannoli_memhook_ebp_ebp, &cannoli_memhook_ebp_ebp_end) },
            unsafe { (&cannoli_memhook_ebp_esi, &cannoli_memhook_ebp_esi_end) },
            unsafe { (&cannoli_memhook_ebp_edi, &cannoli_memhook_ebp_edi_end) },
            unsafe { (&cannoli_memhook_ebp_r8d, &cannoli_memhook_ebp_r8d_end) },
            unsafe { (&cannoli_memhook_ebp_r9d, &cannoli_memhook_ebp_r9d_end) },
            unsafe { (&cannoli_memhook_ebp_r10d, &cannoli_memhook_ebp_r10d_end) },
            unsafe { (&cannoli_memhook_ebp_r11d, &cannoli_memhook_ebp_r11d_end) },
            unsafe { (&cannoli_memhook_ebp_r12d, &cannoli_memhook_ebp_r12d_end) },
            unsafe { (&cannoli_memhook_ebp_r13d, &cannoli_memhook_ebp_r13d_end) },
            unsafe { (&cannoli_memhook_ebp_r14d, &cannoli_memhook_ebp_r14d_end) },
            unsafe { (&cannoli_memhook_ebp_r15d, &cannoli_memhook_ebp_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_esi_eax, &cannoli_memhook_esi_eax_end) },
            unsafe { (&cannoli_memhook_esi_ecx, &cannoli_memhook_esi_ecx_end) },
            unsafe { (&cannoli_memhook_esi_edx, &cannoli_memhook_esi_edx_end) },
            unsafe { (&cannoli_memhook_esi_ebx, &cannoli_memhook_esi_ebx_end) },
            unsafe { (&cannoli_memhook_esi_esp, &cannoli_memhook_esi_esp_end) },
            unsafe { (&cannoli_memhook_esi_ebp, &cannoli_memhook_esi_ebp_end) },
            unsafe { (&cannoli_memhook_esi_esi, &cannoli_memhook_esi_esi_end) },
            unsafe { (&cannoli_memhook_esi_edi, &cannoli_memhook_esi_edi_end) },
            unsafe { (&cannoli_memhook_esi_r8d, &cannoli_memhook_esi_r8d_end) },
            unsafe { (&cannoli_memhook_esi_r9d, &cannoli_memhook_esi_r9d_end) },
            unsafe { (&cannoli_memhook_esi_r10d, &cannoli_memhook_esi_r10d_end) },
            unsafe { (&cannoli_memhook_esi_r11d, &cannoli_memhook_esi_r11d_end) },
            unsafe { (&cannoli_memhook_esi_r12d, &cannoli_memhook_esi_r12d_end) },
            unsafe { (&cannoli_memhook_esi_r13d, &cannoli_memhook_esi_r13d_end) },
            unsafe { (&cannoli_memhook_esi_r14d, &cannoli_memhook_esi_r14d_end) },
            unsafe { (&cannoli_memhook_esi_r15d, &cannoli_memhook_esi_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_edi_eax, &cannoli_memhook_edi_eax_end) },
            unsafe { (&cannoli_memhook_edi_ecx, &cannoli_memhook_edi_ecx_end) },
            unsafe { (&cannoli_memhook_edi_edx, &cannoli_memhook_edi_edx_end) },
            unsafe { (&cannoli_memhook_edi_ebx, &cannoli_memhook_edi_ebx_end) },
            unsafe { (&cannoli_memhook_edi_esp, &cannoli_memhook_edi_esp_end) },
            unsafe { (&cannoli_memhook_edi_ebp, &cannoli_memhook_edi_ebp_end) },
            unsafe { (&cannoli_memhook_edi_esi, &cannoli_memhook_edi_esi_end) },
            unsafe { (&cannoli_memhook_edi_edi, &cannoli_memhook_edi_edi_end) },
            unsafe { (&cannoli_memhook_edi_r8d, &cannoli_memhook_edi_r8d_end) },
            unsafe { (&cannoli_memhook_edi_r9d, &cannoli_memhook_edi_r9d_end) },
            unsafe { (&cannoli_memhook_edi_r10d, &cannoli_memhook_edi_r10d_end) },
            unsafe { (&cannoli_memhook_edi_r11d, &cannoli_memhook_edi_r11d_end) },
            unsafe { (&cannoli_memhook_edi_r12d, &cannoli_memhook_edi_r12d_end) },
            unsafe { (&cannoli_memhook_edi_r13d, &cannoli_memhook_edi_r13d_end) },
            unsafe { (&cannoli_memhook_edi_r14d, &cannoli_memhook_edi_r14d_end) },
            unsafe { (&cannoli_memhook_edi_r15d, &cannoli_memhook_edi_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r8d_eax, &cannoli_memhook_r8d_eax_end) },
            unsafe { (&cannoli_memhook_r8d_ecx, &cannoli_memhook_r8d_ecx_end) },
            unsafe { (&cannoli_memhook_r8d_edx, &cannoli_memhook_r8d_edx_end) },
            unsafe { (&cannoli_memhook_r8d_ebx, &cannoli_memhook_r8d_ebx_end) },
            unsafe { (&cannoli_memhook_r8d_esp, &cannoli_memhook_r8d_esp_end) },
            unsafe { (&cannoli_memhook_r8d_ebp, &cannoli_memhook_r8d_ebp_end) },
            unsafe { (&cannoli_memhook_r8d_esi, &cannoli_memhook_r8d_esi_end) },
            unsafe { (&cannoli_memhook_r8d_edi, &cannoli_memhook_r8d_edi_end) },
            unsafe { (&cannoli_memhook_r8d_r8d, &cannoli_memhook_r8d_r8d_end) },
            unsafe { (&cannoli_memhook_r8d_r9d, &cannoli_memhook_r8d_r9d_end) },
            unsafe { (&cannoli_memhook_r8d_r10d, &cannoli_memhook_r8d_r10d_end) },
            unsafe { (&cannoli_memhook_r8d_r11d, &cannoli_memhook_r8d_r11d_end) },
            unsafe { (&cannoli_memhook_r8d_r12d, &cannoli_memhook_r8d_r12d_end) },
            unsafe { (&cannoli_memhook_r8d_r13d, &cannoli_memhook_r8d_r13d_end) },
            unsafe { (&cannoli_memhook_r8d_r14d, &cannoli_memhook_r8d_r14d_end) },
            unsafe { (&cannoli_memhook_r8d_r15d, &cannoli_memhook_r8d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r9d_eax, &cannoli_memhook_r9d_eax_end) },
            unsafe { (&cannoli_memhook_r9d_ecx, &cannoli_memhook_r9d_ecx_end) },
            unsafe { (&cannoli_memhook_r9d_edx, &cannoli_memhook_r9d_edx_end) },
            unsafe { (&cannoli_memhook_r9d_ebx, &cannoli_memhook_r9d_ebx_end) },
            unsafe { (&cannoli_memhook_r9d_esp, &cannoli_memhook_r9d_esp_end) },
            unsafe { (&cannoli_memhook_r9d_ebp, &cannoli_memhook_r9d_ebp_end) },
            unsafe { (&cannoli_memhook_r9d_esi, &cannoli_memhook_r9d_esi_end) },
            unsafe { (&cannoli_memhook_r9d_edi, &cannoli_memhook_r9d_edi_end) },
            unsafe { (&cannoli_memhook_r9d_r8d, &cannoli_memhook_r9d_r8d_end) },
            unsafe { (&cannoli_memhook_r9d_r9d, &cannoli_memhook_r9d_r9d_end) },
            unsafe { (&cannoli_memhook_r9d_r10d, &cannoli_memhook_r9d_r10d_end) },
            unsafe { (&cannoli_memhook_r9d_r11d, &cannoli_memhook_r9d_r11d_end) },
            unsafe { (&cannoli_memhook_r9d_r12d, &cannoli_memhook_r9d_r12d_end) },
            unsafe { (&cannoli_memhook_r9d_r13d, &cannoli_memhook_r9d_r13d_end) },
            unsafe { (&cannoli_memhook_r9d_r14d, &cannoli_memhook_r9d_r14d_end) },
            unsafe { (&cannoli_memhook_r9d_r15d, &cannoli_memhook_r9d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r10d_eax, &cannoli_memhook_r10d_eax_end) },
            unsafe { (&cannoli_memhook_r10d_ecx, &cannoli_memhook_r10d_ecx_end) },
            unsafe { (&cannoli_memhook_r10d_edx, &cannoli_memhook_r10d_edx_end) },
            unsafe { (&cannoli_memhook_r10d_ebx, &cannoli_memhook_r10d_ebx_end) },
            unsafe { (&cannoli_memhook_r10d_esp, &cannoli_memhook_r10d_esp_end) },
            unsafe { (&cannoli_memhook_r10d_ebp, &cannoli_memhook_r10d_ebp_end) },
            unsafe { (&cannoli_memhook_r10d_esi, &cannoli_memhook_r10d_esi_end) },
            unsafe { (&cannoli_memhook_r10d_edi, &cannoli_memhook_r10d_edi_end) },
            unsafe { (&cannoli_memhook_r10d_r8d, &cannoli_memhook_r10d_r8d_end) },
            unsafe { (&cannoli_memhook_r10d_r9d, &cannoli_memhook_r10d_r9d_end) },
            unsafe { (&cannoli_memhook_r10d_r10d, &cannoli_memhook_r10d_r10d_end) },
            unsafe { (&cannoli_memhook_r10d_r11d, &cannoli_memhook_r10d_r11d_end) },
            unsafe { (&cannoli_memhook_r10d_r12d, &cannoli_memhook_r10d_r12d_end) },
            unsafe { (&cannoli_memhook_r10d_r13d, &cannoli_memhook_r10d_r13d_end) },
            unsafe { (&cannoli_memhook_r10d_r14d, &cannoli_memhook_r10d_r14d_end) },
            unsafe { (&cannoli_memhook_r10d_r15d, &cannoli_memhook_r10d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r11d_eax, &cannoli_memhook_r11d_eax_end) },
            unsafe { (&cannoli_memhook_r11d_ecx, &cannoli_memhook_r11d_ecx_end) },
            unsafe { (&cannoli_memhook_r11d_edx, &cannoli_memhook_r11d_edx_end) },
            unsafe { (&cannoli_memhook_r11d_ebx, &cannoli_memhook_r11d_ebx_end) },
            unsafe { (&cannoli_memhook_r11d_esp, &cannoli_memhook_r11d_esp_end) },
            unsafe { (&cannoli_memhook_r11d_ebp, &cannoli_memhook_r11d_ebp_end) },
            unsafe { (&cannoli_memhook_r11d_esi, &cannoli_memhook_r11d_esi_end) },
            unsafe { (&cannoli_memhook_r11d_edi, &cannoli_memhook_r11d_edi_end) },
            unsafe { (&cannoli_memhook_r11d_r8d, &cannoli_memhook_r11d_r8d_end) },
            unsafe { (&cannoli_memhook_r11d_r9d, &cannoli_memhook_r11d_r9d_end) },
            unsafe { (&cannoli_memhook_r11d_r10d, &cannoli_memhook_r11d_r10d_end) },
            unsafe { (&cannoli_memhook_r11d_r11d, &cannoli_memhook_r11d_r11d_end) },
            unsafe { (&cannoli_memhook_r11d_r12d, &cannoli_memhook_r11d_r12d_end) },
            unsafe { (&cannoli_memhook_r11d_r13d, &cannoli_memhook_r11d_r13d_end) },
            unsafe { (&cannoli_memhook_r11d_r14d, &cannoli_memhook_r11d_r14d_end) },
            unsafe { (&cannoli_memhook_r11d_r15d, &cannoli_memhook_r11d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r12d_eax, &cannoli_memhook_r12d_eax_end) },
            unsafe { (&cannoli_memhook_r12d_ecx, &cannoli_memhook_r12d_ecx_end) },
            unsafe { (&cannoli_memhook_r12d_edx, &cannoli_memhook_r12d_edx_end) },
            unsafe { (&cannoli_memhook_r12d_ebx, &cannoli_memhook_r12d_ebx_end) },
            unsafe { (&cannoli_memhook_r12d_esp, &cannoli_memhook_r12d_esp_end) },
            unsafe { (&cannoli_memhook_r12d_ebp, &cannoli_memhook_r12d_ebp_end) },
            unsafe { (&cannoli_memhook_r12d_esi, &cannoli_memhook_r12d_esi_end) },
            unsafe { (&cannoli_memhook_r12d_edi, &cannoli_memhook_r12d_edi_end) },
            unsafe { (&cannoli_memhook_r12d_r8d, &cannoli_memhook_r12d_r8d_end) },
            unsafe { (&cannoli_memhook_r12d_r9d, &cannoli_memhook_r12d_r9d_end) },
            unsafe { (&cannoli_memhook_r12d_r10d, &cannoli_memhook_r12d_r10d_end) },
            unsafe { (&cannoli_memhook_r12d_r11d, &cannoli_memhook_r12d_r11d_end) },
            unsafe { (&cannoli_memhook_r12d_r12d, &cannoli_memhook_r12d_r12d_end) },
            unsafe { (&cannoli_memhook_r12d_r13d, &cannoli_memhook_r12d_r13d_end) },
            unsafe { (&cannoli_memhook_r12d_r14d, &cannoli_memhook_r12d_r14d_end) },
            unsafe { (&cannoli_memhook_r12d_r15d, &cannoli_memhook_r12d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r13d_eax, &cannoli_memhook_r13d_eax_end) },
            unsafe { (&cannoli_memhook_r13d_ecx, &cannoli_memhook_r13d_ecx_end) },
            unsafe { (&cannoli_memhook_r13d_edx, &cannoli_memhook_r13d_edx_end) },
            unsafe { (&cannoli_memhook_r13d_ebx, &cannoli_memhook_r13d_ebx_end) },
            unsafe { (&cannoli_memhook_r13d_esp, &cannoli_memhook_r13d_esp_end) },
            unsafe { (&cannoli_memhook_r13d_ebp, &cannoli_memhook_r13d_ebp_end) },
            unsafe { (&cannoli_memhook_r13d_esi, &cannoli_memhook_r13d_esi_end) },
            unsafe { (&cannoli_memhook_r13d_edi, &cannoli_memhook_r13d_edi_end) },
            unsafe { (&cannoli_memhook_r13d_r8d, &cannoli_memhook_r13d_r8d_end) },
            unsafe { (&cannoli_memhook_r13d_r9d, &cannoli_memhook_r13d_r9d_end) },
            unsafe { (&cannoli_memhook_r13d_r10d, &cannoli_memhook_r13d_r10d_end) },
            unsafe { (&cannoli_memhook_r13d_r11d, &cannoli_memhook_r13d_r11d_end) },
            unsafe { (&cannoli_memhook_r13d_r12d, &cannoli_memhook_r13d_r12d_end) },
            unsafe { (&cannoli_memhook_r13d_r13d, &cannoli_memhook_r13d_r13d_end) },
            unsafe { (&cannoli_memhook_r13d_r14d, &cannoli_memhook_r13d_r14d_end) },
            unsafe { (&cannoli_memhook_r13d_r15d, &cannoli_memhook_r13d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r14d_eax, &cannoli_memhook_r14d_eax_end) },
            unsafe { (&cannoli_memhook_r14d_ecx, &cannoli_memhook_r14d_ecx_end) },
            unsafe { (&cannoli_memhook_r14d_edx, &cannoli_memhook_r14d_edx_end) },
            unsafe { (&cannoli_memhook_r14d_ebx, &cannoli_memhook_r14d_ebx_end) },
            unsafe { (&cannoli_memhook_r14d_esp, &cannoli_memhook_r14d_esp_end) },
            unsafe { (&cannoli_memhook_r14d_ebp, &cannoli_memhook_r14d_ebp_end) },
            unsafe { (&cannoli_memhook_r14d_esi, &cannoli_memhook_r14d_esi_end) },
            unsafe { (&cannoli_memhook_r14d_edi, &cannoli_memhook_r14d_edi_end) },
            unsafe { (&cannoli_memhook_r14d_r8d, &cannoli_memhook_r14d_r8d_end) },
            unsafe { (&cannoli_memhook_r14d_r9d, &cannoli_memhook_r14d_r9d_end) },
            unsafe { (&cannoli_memhook_r14d_r10d, &cannoli_memhook_r14d_r10d_end) },
            unsafe { (&cannoli_memhook_r14d_r11d, &cannoli_memhook_r14d_r11d_end) },
            unsafe { (&cannoli_memhook_r14d_r12d, &cannoli_memhook_r14d_r12d_end) },
            unsafe { (&cannoli_memhook_r14d_r13d, &cannoli_memhook_r14d_r13d_end) },
            unsafe { (&cannoli_memhook_r14d_r14d, &cannoli_memhook_r14d_r14d_end) },
            unsafe { (&cannoli_memhook_r14d_r15d, &cannoli_memhook_r14d_r15d_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r15d_eax, &cannoli_memhook_r15d_eax_end) },
            unsafe { (&cannoli_memhook_r15d_ecx, &cannoli_memhook_r15d_ecx_end) },
            unsafe { (&cannoli_memhook_r15d_edx, &cannoli_memhook_r15d_edx_end) },
            unsafe { (&cannoli_memhook_r15d_ebx, &cannoli_memhook_r15d_ebx_end) },
            unsafe { (&cannoli_memhook_r15d_esp, &cannoli_memhook_r15d_esp_end) },
            unsafe { (&cannoli_memhook_r15d_ebp, &cannoli_memhook_r15d_ebp_end) },
            unsafe { (&cannoli_memhook_r15d_esi, &cannoli_memhook_r15d_esi_end) },
            unsafe { (&cannoli_memhook_r15d_edi, &cannoli_memhook_r15d_edi_end) },
            unsafe { (&cannoli_memhook_r15d_r8d, &cannoli_memhook_r15d_r8d_end) },
            unsafe { (&cannoli_memhook_r15d_r9d, &cannoli_memhook_r15d_r9d_end) },
            unsafe { (&cannoli_memhook_r15d_r10d, &cannoli_memhook_r15d_r10d_end) },
            unsafe { (&cannoli_memhook_r15d_r11d, &cannoli_memhook_r15d_r11d_end) },
            unsafe { (&cannoli_memhook_r15d_r12d, &cannoli_memhook_r15d_r12d_end) },
            unsafe { (&cannoli_memhook_r15d_r13d, &cannoli_memhook_r15d_r13d_end) },
            unsafe { (&cannoli_memhook_r15d_r14d, &cannoli_memhook_r15d_r14d_end) },
            unsafe { (&cannoli_memhook_r15d_r15d, &cannoli_memhook_r15d_r15d_end) },
        ],
    ],
    [
        [
            unsafe { (&cannoli_memhook_rax_rax, &cannoli_memhook_rax_rax_end) },
            unsafe { (&cannoli_memhook_rax_rcx, &cannoli_memhook_rax_rcx_end) },
            unsafe { (&cannoli_memhook_rax_rdx, &cannoli_memhook_rax_rdx_end) },
            unsafe { (&cannoli_memhook_rax_rbx, &cannoli_memhook_rax_rbx_end) },
            unsafe { (&cannoli_memhook_rax_rsp, &cannoli_memhook_rax_rsp_end) },
            unsafe { (&cannoli_memhook_rax_rbp, &cannoli_memhook_rax_rbp_end) },
            unsafe { (&cannoli_memhook_rax_rsi, &cannoli_memhook_rax_rsi_end) },
            unsafe { (&cannoli_memhook_rax_rdi, &cannoli_memhook_rax_rdi_end) },
            unsafe { (&cannoli_memhook_rax_r8, &cannoli_memhook_rax_r8_end) },
            unsafe { (&cannoli_memhook_rax_r9, &cannoli_memhook_rax_r9_end) },
            unsafe { (&cannoli_memhook_rax_r10, &cannoli_memhook_rax_r10_end) },
            unsafe { (&cannoli_memhook_rax_r11, &cannoli_memhook_rax_r11_end) },
            unsafe { (&cannoli_memhook_rax_r12, &cannoli_memhook_rax_r12_end) },
            unsafe { (&cannoli_memhook_rax_r13, &cannoli_memhook_rax_r13_end) },
            unsafe { (&cannoli_memhook_rax_r14, &cannoli_memhook_rax_r14_end) },
            unsafe { (&cannoli_memhook_rax_r15, &cannoli_memhook_rax_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rcx_rax, &cannoli_memhook_rcx_rax_end) },
            unsafe { (&cannoli_memhook_rcx_rcx, &cannoli_memhook_rcx_rcx_end) },
            unsafe { (&cannoli_memhook_rcx_rdx, &cannoli_memhook_rcx_rdx_end) },
            unsafe { (&cannoli_memhook_rcx_rbx, &cannoli_memhook_rcx_rbx_end) },
            unsafe { (&cannoli_memhook_rcx_rsp, &cannoli_memhook_rcx_rsp_end) },
            unsafe { (&cannoli_memhook_rcx_rbp, &cannoli_memhook_rcx_rbp_end) },
            unsafe { (&cannoli_memhook_rcx_rsi, &cannoli_memhook_rcx_rsi_end) },
            unsafe { (&cannoli_memhook_rcx_rdi, &cannoli_memhook_rcx_rdi_end) },
            unsafe { (&cannoli_memhook_rcx_r8, &cannoli_memhook_rcx_r8_end) },
            unsafe { (&cannoli_memhook_rcx_r9, &cannoli_memhook_rcx_r9_end) },
            unsafe { (&cannoli_memhook_rcx_r10, &cannoli_memhook_rcx_r10_end) },
            unsafe { (&cannoli_memhook_rcx_r11, &cannoli_memhook_rcx_r11_end) },
            unsafe { (&cannoli_memhook_rcx_r12, &cannoli_memhook_rcx_r12_end) },
            unsafe { (&cannoli_memhook_rcx_r13, &cannoli_memhook_rcx_r13_end) },
            unsafe { (&cannoli_memhook_rcx_r14, &cannoli_memhook_rcx_r14_end) },
            unsafe { (&cannoli_memhook_rcx_r15, &cannoli_memhook_rcx_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rdx_rax, &cannoli_memhook_rdx_rax_end) },
            unsafe { (&cannoli_memhook_rdx_rcx, &cannoli_memhook_rdx_rcx_end) },
            unsafe { (&cannoli_memhook_rdx_rdx, &cannoli_memhook_rdx_rdx_end) },
            unsafe { (&cannoli_memhook_rdx_rbx, &cannoli_memhook_rdx_rbx_end) },
            unsafe { (&cannoli_memhook_rdx_rsp, &cannoli_memhook_rdx_rsp_end) },
            unsafe { (&cannoli_memhook_rdx_rbp, &cannoli_memhook_rdx_rbp_end) },
            unsafe { (&cannoli_memhook_rdx_rsi, &cannoli_memhook_rdx_rsi_end) },
            unsafe { (&cannoli_memhook_rdx_rdi, &cannoli_memhook_rdx_rdi_end) },
            unsafe { (&cannoli_memhook_rdx_r8, &cannoli_memhook_rdx_r8_end) },
            unsafe { (&cannoli_memhook_rdx_r9, &cannoli_memhook_rdx_r9_end) },
            unsafe { (&cannoli_memhook_rdx_r10, &cannoli_memhook_rdx_r10_end) },
            unsafe { (&cannoli_memhook_rdx_r11, &cannoli_memhook_rdx_r11_end) },
            unsafe { (&cannoli_memhook_rdx_r12, &cannoli_memhook_rdx_r12_end) },
            unsafe { (&cannoli_memhook_rdx_r13, &cannoli_memhook_rdx_r13_end) },
            unsafe { (&cannoli_memhook_rdx_r14, &cannoli_memhook_rdx_r14_end) },
            unsafe { (&cannoli_memhook_rdx_r15, &cannoli_memhook_rdx_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rbx_rax, &cannoli_memhook_rbx_rax_end) },
            unsafe { (&cannoli_memhook_rbx_rcx, &cannoli_memhook_rbx_rcx_end) },
            unsafe { (&cannoli_memhook_rbx_rdx, &cannoli_memhook_rbx_rdx_end) },
            unsafe { (&cannoli_memhook_rbx_rbx, &cannoli_memhook_rbx_rbx_end) },
            unsafe { (&cannoli_memhook_rbx_rsp, &cannoli_memhook_rbx_rsp_end) },
            unsafe { (&cannoli_memhook_rbx_rbp, &cannoli_memhook_rbx_rbp_end) },
            unsafe { (&cannoli_memhook_rbx_rsi, &cannoli_memhook_rbx_rsi_end) },
            unsafe { (&cannoli_memhook_rbx_rdi, &cannoli_memhook_rbx_rdi_end) },
            unsafe { (&cannoli_memhook_rbx_r8, &cannoli_memhook_rbx_r8_end) },
            unsafe { (&cannoli_memhook_rbx_r9, &cannoli_memhook_rbx_r9_end) },
            unsafe { (&cannoli_memhook_rbx_r10, &cannoli_memhook_rbx_r10_end) },
            unsafe { (&cannoli_memhook_rbx_r11, &cannoli_memhook_rbx_r11_end) },
            unsafe { (&cannoli_memhook_rbx_r12, &cannoli_memhook_rbx_r12_end) },
            unsafe { (&cannoli_memhook_rbx_r13, &cannoli_memhook_rbx_r13_end) },
            unsafe { (&cannoli_memhook_rbx_r14, &cannoli_memhook_rbx_r14_end) },
            unsafe { (&cannoli_memhook_rbx_r15, &cannoli_memhook_rbx_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rsp_rax, &cannoli_memhook_rsp_rax_end) },
            unsafe { (&cannoli_memhook_rsp_rcx, &cannoli_memhook_rsp_rcx_end) },
            unsafe { (&cannoli_memhook_rsp_rdx, &cannoli_memhook_rsp_rdx_end) },
            unsafe { (&cannoli_memhook_rsp_rbx, &cannoli_memhook_rsp_rbx_end) },
            unsafe { (&cannoli_memhook_rsp_rsp, &cannoli_memhook_rsp_rsp_end) },
            unsafe { (&cannoli_memhook_rsp_rbp, &cannoli_memhook_rsp_rbp_end) },
            unsafe { (&cannoli_memhook_rsp_rsi, &cannoli_memhook_rsp_rsi_end) },
            unsafe { (&cannoli_memhook_rsp_rdi, &cannoli_memhook_rsp_rdi_end) },
            unsafe { (&cannoli_memhook_rsp_r8, &cannoli_memhook_rsp_r8_end) },
            unsafe { (&cannoli_memhook_rsp_r9, &cannoli_memhook_rsp_r9_end) },
            unsafe { (&cannoli_memhook_rsp_r10, &cannoli_memhook_rsp_r10_end) },
            unsafe { (&cannoli_memhook_rsp_r11, &cannoli_memhook_rsp_r11_end) },
            unsafe { (&cannoli_memhook_rsp_r12, &cannoli_memhook_rsp_r12_end) },
            unsafe { (&cannoli_memhook_rsp_r13, &cannoli_memhook_rsp_r13_end) },
            unsafe { (&cannoli_memhook_rsp_r14, &cannoli_memhook_rsp_r14_end) },
            unsafe { (&cannoli_memhook_rsp_r15, &cannoli_memhook_rsp_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rbp_rax, &cannoli_memhook_rbp_rax_end) },
            unsafe { (&cannoli_memhook_rbp_rcx, &cannoli_memhook_rbp_rcx_end) },
            unsafe { (&cannoli_memhook_rbp_rdx, &cannoli_memhook_rbp_rdx_end) },
            unsafe { (&cannoli_memhook_rbp_rbx, &cannoli_memhook_rbp_rbx_end) },
            unsafe { (&cannoli_memhook_rbp_rsp, &cannoli_memhook_rbp_rsp_end) },
            unsafe { (&cannoli_memhook_rbp_rbp, &cannoli_memhook_rbp_rbp_end) },
            unsafe { (&cannoli_memhook_rbp_rsi, &cannoli_memhook_rbp_rsi_end) },
            unsafe { (&cannoli_memhook_rbp_rdi, &cannoli_memhook_rbp_rdi_end) },
            unsafe { (&cannoli_memhook_rbp_r8, &cannoli_memhook_rbp_r8_end) },
            unsafe { (&cannoli_memhook_rbp_r9, &cannoli_memhook_rbp_r9_end) },
            unsafe { (&cannoli_memhook_rbp_r10, &cannoli_memhook_rbp_r10_end) },
            unsafe { (&cannoli_memhook_rbp_r11, &cannoli_memhook_rbp_r11_end) },
            unsafe { (&cannoli_memhook_rbp_r12, &cannoli_memhook_rbp_r12_end) },
            unsafe { (&cannoli_memhook_rbp_r13, &cannoli_memhook_rbp_r13_end) },
            unsafe { (&cannoli_memhook_rbp_r14, &cannoli_memhook_rbp_r14_end) },
            unsafe { (&cannoli_memhook_rbp_r15, &cannoli_memhook_rbp_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rsi_rax, &cannoli_memhook_rsi_rax_end) },
            unsafe { (&cannoli_memhook_rsi_rcx, &cannoli_memhook_rsi_rcx_end) },
            unsafe { (&cannoli_memhook_rsi_rdx, &cannoli_memhook_rsi_rdx_end) },
            unsafe { (&cannoli_memhook_rsi_rbx, &cannoli_memhook_rsi_rbx_end) },
            unsafe { (&cannoli_memhook_rsi_rsp, &cannoli_memhook_rsi_rsp_end) },
            unsafe { (&cannoli_memhook_rsi_rbp, &cannoli_memhook_rsi_rbp_end) },
            unsafe { (&cannoli_memhook_rsi_rsi, &cannoli_memhook_rsi_rsi_end) },
            unsafe { (&cannoli_memhook_rsi_rdi, &cannoli_memhook_rsi_rdi_end) },
            unsafe { (&cannoli_memhook_rsi_r8, &cannoli_memhook_rsi_r8_end) },
            unsafe { (&cannoli_memhook_rsi_r9, &cannoli_memhook_rsi_r9_end) },
            unsafe { (&cannoli_memhook_rsi_r10, &cannoli_memhook_rsi_r10_end) },
            unsafe { (&cannoli_memhook_rsi_r11, &cannoli_memhook_rsi_r11_end) },
            unsafe { (&cannoli_memhook_rsi_r12, &cannoli_memhook_rsi_r12_end) },
            unsafe { (&cannoli_memhook_rsi_r13, &cannoli_memhook_rsi_r13_end) },
            unsafe { (&cannoli_memhook_rsi_r14, &cannoli_memhook_rsi_r14_end) },
            unsafe { (&cannoli_memhook_rsi_r15, &cannoli_memhook_rsi_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_rdi_rax, &cannoli_memhook_rdi_rax_end) },
            unsafe { (&cannoli_memhook_rdi_rcx, &cannoli_memhook_rdi_rcx_end) },
            unsafe { (&cannoli_memhook_rdi_rdx, &cannoli_memhook_rdi_rdx_end) },
            unsafe { (&cannoli_memhook_rdi_rbx, &cannoli_memhook_rdi_rbx_end) },
            unsafe { (&cannoli_memhook_rdi_rsp, &cannoli_memhook_rdi_rsp_end) },
            unsafe { (&cannoli_memhook_rdi_rbp, &cannoli_memhook_rdi_rbp_end) },
            unsafe { (&cannoli_memhook_rdi_rsi, &cannoli_memhook_rdi_rsi_end) },
            unsafe { (&cannoli_memhook_rdi_rdi, &cannoli_memhook_rdi_rdi_end) },
            unsafe { (&cannoli_memhook_rdi_r8, &cannoli_memhook_rdi_r8_end) },
            unsafe { (&cannoli_memhook_rdi_r9, &cannoli_memhook_rdi_r9_end) },
            unsafe { (&cannoli_memhook_rdi_r10, &cannoli_memhook_rdi_r10_end) },
            unsafe { (&cannoli_memhook_rdi_r11, &cannoli_memhook_rdi_r11_end) },
            unsafe { (&cannoli_memhook_rdi_r12, &cannoli_memhook_rdi_r12_end) },
            unsafe { (&cannoli_memhook_rdi_r13, &cannoli_memhook_rdi_r13_end) },
            unsafe { (&cannoli_memhook_rdi_r14, &cannoli_memhook_rdi_r14_end) },
            unsafe { (&cannoli_memhook_rdi_r15, &cannoli_memhook_rdi_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r8_rax, &cannoli_memhook_r8_rax_end) },
            unsafe { (&cannoli_memhook_r8_rcx, &cannoli_memhook_r8_rcx_end) },
            unsafe { (&cannoli_memhook_r8_rdx, &cannoli_memhook_r8_rdx_end) },
            unsafe { (&cannoli_memhook_r8_rbx, &cannoli_memhook_r8_rbx_end) },
            unsafe { (&cannoli_memhook_r8_rsp, &cannoli_memhook_r8_rsp_end) },
            unsafe { (&cannoli_memhook_r8_rbp, &cannoli_memhook_r8_rbp_end) },
            unsafe { (&cannoli_memhook_r8_rsi, &cannoli_memhook_r8_rsi_end) },
            unsafe { (&cannoli_memhook_r8_rdi, &cannoli_memhook_r8_rdi_end) },
            unsafe { (&cannoli_memhook_r8_r8, &cannoli_memhook_r8_r8_end) },
            unsafe { (&cannoli_memhook_r8_r9, &cannoli_memhook_r8_r9_end) },
            unsafe { (&cannoli_memhook_r8_r10, &cannoli_memhook_r8_r10_end) },
            unsafe { (&cannoli_memhook_r8_r11, &cannoli_memhook_r8_r11_end) },
            unsafe { (&cannoli_memhook_r8_r12, &cannoli_memhook_r8_r12_end) },
            unsafe { (&cannoli_memhook_r8_r13, &cannoli_memhook_r8_r13_end) },
            unsafe { (&cannoli_memhook_r8_r14, &cannoli_memhook_r8_r14_end) },
            unsafe { (&cannoli_memhook_r8_r15, &cannoli_memhook_r8_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r9_rax, &cannoli_memhook_r9_rax_end) },
            unsafe { (&cannoli_memhook_r9_rcx, &cannoli_memhook_r9_rcx_end) },
            unsafe { (&cannoli_memhook_r9_rdx, &cannoli_memhook_r9_rdx_end) },
            unsafe { (&cannoli_memhook_r9_rbx, &cannoli_memhook_r9_rbx_end) },
            unsafe { (&cannoli_memhook_r9_rsp, &cannoli_memhook_r9_rsp_end) },
            unsafe { (&cannoli_memhook_r9_rbp, &cannoli_memhook_r9_rbp_end) },
            unsafe { (&cannoli_memhook_r9_rsi, &cannoli_memhook_r9_rsi_end) },
            unsafe { (&cannoli_memhook_r9_rdi, &cannoli_memhook_r9_rdi_end) },
            unsafe { (&cannoli_memhook_r9_r8, &cannoli_memhook_r9_r8_end) },
            unsafe { (&cannoli_memhook_r9_r9, &cannoli_memhook_r9_r9_end) },
            unsafe { (&cannoli_memhook_r9_r10, &cannoli_memhook_r9_r10_end) },
            unsafe { (&cannoli_memhook_r9_r11, &cannoli_memhook_r9_r11_end) },
            unsafe { (&cannoli_memhook_r9_r12, &cannoli_memhook_r9_r12_end) },
            unsafe { (&cannoli_memhook_r9_r13, &cannoli_memhook_r9_r13_end) },
            unsafe { (&cannoli_memhook_r9_r14, &cannoli_memhook_r9_r14_end) },
            unsafe { (&cannoli_memhook_r9_r15, &cannoli_memhook_r9_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r10_rax, &cannoli_memhook_r10_rax_end) },
            unsafe { (&cannoli_memhook_r10_rcx, &cannoli_memhook_r10_rcx_end) },
            unsafe { (&cannoli_memhook_r10_rdx, &cannoli_memhook_r10_rdx_end) },
            unsafe { (&cannoli_memhook_r10_rbx, &cannoli_memhook_r10_rbx_end) },
            unsafe { (&cannoli_memhook_r10_rsp, &cannoli_memhook_r10_rsp_end) },
            unsafe { (&cannoli_memhook_r10_rbp, &cannoli_memhook_r10_rbp_end) },
            unsafe { (&cannoli_memhook_r10_rsi, &cannoli_memhook_r10_rsi_end) },
            unsafe { (&cannoli_memhook_r10_rdi, &cannoli_memhook_r10_rdi_end) },
            unsafe { (&cannoli_memhook_r10_r8, &cannoli_memhook_r10_r8_end) },
            unsafe { (&cannoli_memhook_r10_r9, &cannoli_memhook_r10_r9_end) },
            unsafe { (&cannoli_memhook_r10_r10, &cannoli_memhook_r10_r10_end) },
            unsafe { (&cannoli_memhook_r10_r11, &cannoli_memhook_r10_r11_end) },
            unsafe { (&cannoli_memhook_r10_r12, &cannoli_memhook_r10_r12_end) },
            unsafe { (&cannoli_memhook_r10_r13, &cannoli_memhook_r10_r13_end) },
            unsafe { (&cannoli_memhook_r10_r14, &cannoli_memhook_r10_r14_end) },
            unsafe { (&cannoli_memhook_r10_r15, &cannoli_memhook_r10_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r11_rax, &cannoli_memhook_r11_rax_end) },
            unsafe { (&cannoli_memhook_r11_rcx, &cannoli_memhook_r11_rcx_end) },
            unsafe { (&cannoli_memhook_r11_rdx, &cannoli_memhook_r11_rdx_end) },
            unsafe { (&cannoli_memhook_r11_rbx, &cannoli_memhook_r11_rbx_end) },
            unsafe { (&cannoli_memhook_r11_rsp, &cannoli_memhook_r11_rsp_end) },
            unsafe { (&cannoli_memhook_r11_rbp, &cannoli_memhook_r11_rbp_end) },
            unsafe { (&cannoli_memhook_r11_rsi, &cannoli_memhook_r11_rsi_end) },
            unsafe { (&cannoli_memhook_r11_rdi, &cannoli_memhook_r11_rdi_end) },
            unsafe { (&cannoli_memhook_r11_r8, &cannoli_memhook_r11_r8_end) },
            unsafe { (&cannoli_memhook_r11_r9, &cannoli_memhook_r11_r9_end) },
            unsafe { (&cannoli_memhook_r11_r10, &cannoli_memhook_r11_r10_end) },
            unsafe { (&cannoli_memhook_r11_r11, &cannoli_memhook_r11_r11_end) },
            unsafe { (&cannoli_memhook_r11_r12, &cannoli_memhook_r11_r12_end) },
            unsafe { (&cannoli_memhook_r11_r13, &cannoli_memhook_r11_r13_end) },
            unsafe { (&cannoli_memhook_r11_r14, &cannoli_memhook_r11_r14_end) },
            unsafe { (&cannoli_memhook_r11_r15, &cannoli_memhook_r11_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r12_rax, &cannoli_memhook_r12_rax_end) },
            unsafe { (&cannoli_memhook_r12_rcx, &cannoli_memhook_r12_rcx_end) },
            unsafe { (&cannoli_memhook_r12_rdx, &cannoli_memhook_r12_rdx_end) },
            unsafe { (&cannoli_memhook_r12_rbx, &cannoli_memhook_r12_rbx_end) },
            unsafe { (&cannoli_memhook_r12_rsp, &cannoli_memhook_r12_rsp_end) },
            unsafe { (&cannoli_memhook_r12_rbp, &cannoli_memhook_r12_rbp_end) },
            unsafe { (&cannoli_memhook_r12_rsi, &cannoli_memhook_r12_rsi_end) },
            unsafe { (&cannoli_memhook_r12_rdi, &cannoli_memhook_r12_rdi_end) },
            unsafe { (&cannoli_memhook_r12_r8, &cannoli_memhook_r12_r8_end) },
            unsafe { (&cannoli_memhook_r12_r9, &cannoli_memhook_r12_r9_end) },
            unsafe { (&cannoli_memhook_r12_r10, &cannoli_memhook_r12_r10_end) },
            unsafe { (&cannoli_memhook_r12_r11, &cannoli_memhook_r12_r11_end) },
            unsafe { (&cannoli_memhook_r12_r12, &cannoli_memhook_r12_r12_end) },
            unsafe { (&cannoli_memhook_r12_r13, &cannoli_memhook_r12_r13_end) },
            unsafe { (&cannoli_memhook_r12_r14, &cannoli_memhook_r12_r14_end) },
            unsafe { (&cannoli_memhook_r12_r15, &cannoli_memhook_r12_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r13_rax, &cannoli_memhook_r13_rax_end) },
            unsafe { (&cannoli_memhook_r13_rcx, &cannoli_memhook_r13_rcx_end) },
            unsafe { (&cannoli_memhook_r13_rdx, &cannoli_memhook_r13_rdx_end) },
            unsafe { (&cannoli_memhook_r13_rbx, &cannoli_memhook_r13_rbx_end) },
            unsafe { (&cannoli_memhook_r13_rsp, &cannoli_memhook_r13_rsp_end) },
            unsafe { (&cannoli_memhook_r13_rbp, &cannoli_memhook_r13_rbp_end) },
            unsafe { (&cannoli_memhook_r13_rsi, &cannoli_memhook_r13_rsi_end) },
            unsafe { (&cannoli_memhook_r13_rdi, &cannoli_memhook_r13_rdi_end) },
            unsafe { (&cannoli_memhook_r13_r8, &cannoli_memhook_r13_r8_end) },
            unsafe { (&cannoli_memhook_r13_r9, &cannoli_memhook_r13_r9_end) },
            unsafe { (&cannoli_memhook_r13_r10, &cannoli_memhook_r13_r10_end) },
            unsafe { (&cannoli_memhook_r13_r11, &cannoli_memhook_r13_r11_end) },
            unsafe { (&cannoli_memhook_r13_r12, &cannoli_memhook_r13_r12_end) },
            unsafe { (&cannoli_memhook_r13_r13, &cannoli_memhook_r13_r13_end) },
            unsafe { (&cannoli_memhook_r13_r14, &cannoli_memhook_r13_r14_end) },
            unsafe { (&cannoli_memhook_r13_r15, &cannoli_memhook_r13_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r14_rax, &cannoli_memhook_r14_rax_end) },
            unsafe { (&cannoli_memhook_r14_rcx, &cannoli_memhook_r14_rcx_end) },
            unsafe { (&cannoli_memhook_r14_rdx, &cannoli_memhook_r14_rdx_end) },
            unsafe { (&cannoli_memhook_r14_rbx, &cannoli_memhook_r14_rbx_end) },
            unsafe { (&cannoli_memhook_r14_rsp, &cannoli_memhook_r14_rsp_end) },
            unsafe { (&cannoli_memhook_r14_rbp, &cannoli_memhook_r14_rbp_end) },
            unsafe { (&cannoli_memhook_r14_rsi, &cannoli_memhook_r14_rsi_end) },
            unsafe { (&cannoli_memhook_r14_rdi, &cannoli_memhook_r14_rdi_end) },
            unsafe { (&cannoli_memhook_r14_r8, &cannoli_memhook_r14_r8_end) },
            unsafe { (&cannoli_memhook_r14_r9, &cannoli_memhook_r14_r9_end) },
            unsafe { (&cannoli_memhook_r14_r10, &cannoli_memhook_r14_r10_end) },
            unsafe { (&cannoli_memhook_r14_r11, &cannoli_memhook_r14_r11_end) },
            unsafe { (&cannoli_memhook_r14_r12, &cannoli_memhook_r14_r12_end) },
            unsafe { (&cannoli_memhook_r14_r13, &cannoli_memhook_r14_r13_end) },
            unsafe { (&cannoli_memhook_r14_r14, &cannoli_memhook_r14_r14_end) },
            unsafe { (&cannoli_memhook_r14_r15, &cannoli_memhook_r14_r15_end) },
        ],
        [
            unsafe { (&cannoli_memhook_r15_rax, &cannoli_memhook_r15_rax_end) },
            unsafe { (&cannoli_memhook_r15_rcx, &cannoli_memhook_r15_rcx_end) },
            unsafe { (&cannoli_memhook_r15_rdx, &cannoli_memhook_r15_rdx_end) },
            unsafe { (&cannoli_memhook_r15_rbx, &cannoli_memhook_r15_rbx_end) },
            unsafe { (&cannoli_memhook_r15_rsp, &cannoli_memhook_r15_rsp_end) },
            unsafe { (&cannoli_memhook_r15_rbp, &cannoli_memhook_r15_rbp_end) },
            unsafe { (&cannoli_memhook_r15_rsi, &cannoli_memhook_r15_rsi_end) },
            unsafe { (&cannoli_memhook_r15_rdi, &cannoli_memhook_r15_rdi_end) },
            unsafe { (&cannoli_memhook_r15_r8, &cannoli_memhook_r15_r8_end) },
            unsafe { (&cannoli_memhook_r15_r9, &cannoli_memhook_r15_r9_end) },
            unsafe { (&cannoli_memhook_r15_r10, &cannoli_memhook_r15_r10_end) },
            unsafe { (&cannoli_memhook_r15_r11, &cannoli_memhook_r15_r11_end) },
            unsafe { (&cannoli_memhook_r15_r12, &cannoli_memhook_r15_r12_end) },
            unsafe { (&cannoli_memhook_r15_r13, &cannoli_memhook_r15_r13_end) },
            unsafe { (&cannoli_memhook_r15_r14, &cannoli_memhook_r15_r14_end) },
            unsafe { (&cannoli_memhook_r15_r15, &cannoli_memhook_r15_r15_end) },
        ],
    ],
];

