//! We just use a disgusting assembly macro to auto-generate all of these
//! functions from a single template.

extern {
    static cannoli_memhook_read_al_eax: u8;
    static cannoli_memhook_read_al_eax_end: u8;
    static cannoli_memhook_read_al_ecx: u8;
    static cannoli_memhook_read_al_ecx_end: u8;
    static cannoli_memhook_read_al_edx: u8;
    static cannoli_memhook_read_al_edx_end: u8;
    static cannoli_memhook_read_al_ebx: u8;
    static cannoli_memhook_read_al_ebx_end: u8;
    static cannoli_memhook_read_al_esp: u8;
    static cannoli_memhook_read_al_esp_end: u8;
    static cannoli_memhook_read_al_ebp: u8;
    static cannoli_memhook_read_al_ebp_end: u8;
    static cannoli_memhook_read_al_esi: u8;
    static cannoli_memhook_read_al_esi_end: u8;
    static cannoli_memhook_read_al_edi: u8;
    static cannoli_memhook_read_al_edi_end: u8;
    static cannoli_memhook_read_al_r8d: u8;
    static cannoli_memhook_read_al_r8d_end: u8;
    static cannoli_memhook_read_al_r9d: u8;
    static cannoli_memhook_read_al_r9d_end: u8;
    static cannoli_memhook_read_al_r10d: u8;
    static cannoli_memhook_read_al_r10d_end: u8;
    static cannoli_memhook_read_al_r11d: u8;
    static cannoli_memhook_read_al_r11d_end: u8;
    static cannoli_memhook_read_al_r12d: u8;
    static cannoli_memhook_read_al_r12d_end: u8;
    static cannoli_memhook_read_al_r13d: u8;
    static cannoli_memhook_read_al_r13d_end: u8;
    static cannoli_memhook_read_al_r14d: u8;
    static cannoli_memhook_read_al_r14d_end: u8;
    static cannoli_memhook_read_al_r15d: u8;
    static cannoli_memhook_read_al_r15d_end: u8;
    static cannoli_memhook_read_cl_eax: u8;
    static cannoli_memhook_read_cl_eax_end: u8;
    static cannoli_memhook_read_cl_ecx: u8;
    static cannoli_memhook_read_cl_ecx_end: u8;
    static cannoli_memhook_read_cl_edx: u8;
    static cannoli_memhook_read_cl_edx_end: u8;
    static cannoli_memhook_read_cl_ebx: u8;
    static cannoli_memhook_read_cl_ebx_end: u8;
    static cannoli_memhook_read_cl_esp: u8;
    static cannoli_memhook_read_cl_esp_end: u8;
    static cannoli_memhook_read_cl_ebp: u8;
    static cannoli_memhook_read_cl_ebp_end: u8;
    static cannoli_memhook_read_cl_esi: u8;
    static cannoli_memhook_read_cl_esi_end: u8;
    static cannoli_memhook_read_cl_edi: u8;
    static cannoli_memhook_read_cl_edi_end: u8;
    static cannoli_memhook_read_cl_r8d: u8;
    static cannoli_memhook_read_cl_r8d_end: u8;
    static cannoli_memhook_read_cl_r9d: u8;
    static cannoli_memhook_read_cl_r9d_end: u8;
    static cannoli_memhook_read_cl_r10d: u8;
    static cannoli_memhook_read_cl_r10d_end: u8;
    static cannoli_memhook_read_cl_r11d: u8;
    static cannoli_memhook_read_cl_r11d_end: u8;
    static cannoli_memhook_read_cl_r12d: u8;
    static cannoli_memhook_read_cl_r12d_end: u8;
    static cannoli_memhook_read_cl_r13d: u8;
    static cannoli_memhook_read_cl_r13d_end: u8;
    static cannoli_memhook_read_cl_r14d: u8;
    static cannoli_memhook_read_cl_r14d_end: u8;
    static cannoli_memhook_read_cl_r15d: u8;
    static cannoli_memhook_read_cl_r15d_end: u8;
    static cannoli_memhook_read_dl_eax: u8;
    static cannoli_memhook_read_dl_eax_end: u8;
    static cannoli_memhook_read_dl_ecx: u8;
    static cannoli_memhook_read_dl_ecx_end: u8;
    static cannoli_memhook_read_dl_edx: u8;
    static cannoli_memhook_read_dl_edx_end: u8;
    static cannoli_memhook_read_dl_ebx: u8;
    static cannoli_memhook_read_dl_ebx_end: u8;
    static cannoli_memhook_read_dl_esp: u8;
    static cannoli_memhook_read_dl_esp_end: u8;
    static cannoli_memhook_read_dl_ebp: u8;
    static cannoli_memhook_read_dl_ebp_end: u8;
    static cannoli_memhook_read_dl_esi: u8;
    static cannoli_memhook_read_dl_esi_end: u8;
    static cannoli_memhook_read_dl_edi: u8;
    static cannoli_memhook_read_dl_edi_end: u8;
    static cannoli_memhook_read_dl_r8d: u8;
    static cannoli_memhook_read_dl_r8d_end: u8;
    static cannoli_memhook_read_dl_r9d: u8;
    static cannoli_memhook_read_dl_r9d_end: u8;
    static cannoli_memhook_read_dl_r10d: u8;
    static cannoli_memhook_read_dl_r10d_end: u8;
    static cannoli_memhook_read_dl_r11d: u8;
    static cannoli_memhook_read_dl_r11d_end: u8;
    static cannoli_memhook_read_dl_r12d: u8;
    static cannoli_memhook_read_dl_r12d_end: u8;
    static cannoli_memhook_read_dl_r13d: u8;
    static cannoli_memhook_read_dl_r13d_end: u8;
    static cannoli_memhook_read_dl_r14d: u8;
    static cannoli_memhook_read_dl_r14d_end: u8;
    static cannoli_memhook_read_dl_r15d: u8;
    static cannoli_memhook_read_dl_r15d_end: u8;
    static cannoli_memhook_read_bl_eax: u8;
    static cannoli_memhook_read_bl_eax_end: u8;
    static cannoli_memhook_read_bl_ecx: u8;
    static cannoli_memhook_read_bl_ecx_end: u8;
    static cannoli_memhook_read_bl_edx: u8;
    static cannoli_memhook_read_bl_edx_end: u8;
    static cannoli_memhook_read_bl_ebx: u8;
    static cannoli_memhook_read_bl_ebx_end: u8;
    static cannoli_memhook_read_bl_esp: u8;
    static cannoli_memhook_read_bl_esp_end: u8;
    static cannoli_memhook_read_bl_ebp: u8;
    static cannoli_memhook_read_bl_ebp_end: u8;
    static cannoli_memhook_read_bl_esi: u8;
    static cannoli_memhook_read_bl_esi_end: u8;
    static cannoli_memhook_read_bl_edi: u8;
    static cannoli_memhook_read_bl_edi_end: u8;
    static cannoli_memhook_read_bl_r8d: u8;
    static cannoli_memhook_read_bl_r8d_end: u8;
    static cannoli_memhook_read_bl_r9d: u8;
    static cannoli_memhook_read_bl_r9d_end: u8;
    static cannoli_memhook_read_bl_r10d: u8;
    static cannoli_memhook_read_bl_r10d_end: u8;
    static cannoli_memhook_read_bl_r11d: u8;
    static cannoli_memhook_read_bl_r11d_end: u8;
    static cannoli_memhook_read_bl_r12d: u8;
    static cannoli_memhook_read_bl_r12d_end: u8;
    static cannoli_memhook_read_bl_r13d: u8;
    static cannoli_memhook_read_bl_r13d_end: u8;
    static cannoli_memhook_read_bl_r14d: u8;
    static cannoli_memhook_read_bl_r14d_end: u8;
    static cannoli_memhook_read_bl_r15d: u8;
    static cannoli_memhook_read_bl_r15d_end: u8;
    static cannoli_memhook_read_spl_eax: u8;
    static cannoli_memhook_read_spl_eax_end: u8;
    static cannoli_memhook_read_spl_ecx: u8;
    static cannoli_memhook_read_spl_ecx_end: u8;
    static cannoli_memhook_read_spl_edx: u8;
    static cannoli_memhook_read_spl_edx_end: u8;
    static cannoli_memhook_read_spl_ebx: u8;
    static cannoli_memhook_read_spl_ebx_end: u8;
    static cannoli_memhook_read_spl_esp: u8;
    static cannoli_memhook_read_spl_esp_end: u8;
    static cannoli_memhook_read_spl_ebp: u8;
    static cannoli_memhook_read_spl_ebp_end: u8;
    static cannoli_memhook_read_spl_esi: u8;
    static cannoli_memhook_read_spl_esi_end: u8;
    static cannoli_memhook_read_spl_edi: u8;
    static cannoli_memhook_read_spl_edi_end: u8;
    static cannoli_memhook_read_spl_r8d: u8;
    static cannoli_memhook_read_spl_r8d_end: u8;
    static cannoli_memhook_read_spl_r9d: u8;
    static cannoli_memhook_read_spl_r9d_end: u8;
    static cannoli_memhook_read_spl_r10d: u8;
    static cannoli_memhook_read_spl_r10d_end: u8;
    static cannoli_memhook_read_spl_r11d: u8;
    static cannoli_memhook_read_spl_r11d_end: u8;
    static cannoli_memhook_read_spl_r12d: u8;
    static cannoli_memhook_read_spl_r12d_end: u8;
    static cannoli_memhook_read_spl_r13d: u8;
    static cannoli_memhook_read_spl_r13d_end: u8;
    static cannoli_memhook_read_spl_r14d: u8;
    static cannoli_memhook_read_spl_r14d_end: u8;
    static cannoli_memhook_read_spl_r15d: u8;
    static cannoli_memhook_read_spl_r15d_end: u8;
    static cannoli_memhook_read_bpl_eax: u8;
    static cannoli_memhook_read_bpl_eax_end: u8;
    static cannoli_memhook_read_bpl_ecx: u8;
    static cannoli_memhook_read_bpl_ecx_end: u8;
    static cannoli_memhook_read_bpl_edx: u8;
    static cannoli_memhook_read_bpl_edx_end: u8;
    static cannoli_memhook_read_bpl_ebx: u8;
    static cannoli_memhook_read_bpl_ebx_end: u8;
    static cannoli_memhook_read_bpl_esp: u8;
    static cannoli_memhook_read_bpl_esp_end: u8;
    static cannoli_memhook_read_bpl_ebp: u8;
    static cannoli_memhook_read_bpl_ebp_end: u8;
    static cannoli_memhook_read_bpl_esi: u8;
    static cannoli_memhook_read_bpl_esi_end: u8;
    static cannoli_memhook_read_bpl_edi: u8;
    static cannoli_memhook_read_bpl_edi_end: u8;
    static cannoli_memhook_read_bpl_r8d: u8;
    static cannoli_memhook_read_bpl_r8d_end: u8;
    static cannoli_memhook_read_bpl_r9d: u8;
    static cannoli_memhook_read_bpl_r9d_end: u8;
    static cannoli_memhook_read_bpl_r10d: u8;
    static cannoli_memhook_read_bpl_r10d_end: u8;
    static cannoli_memhook_read_bpl_r11d: u8;
    static cannoli_memhook_read_bpl_r11d_end: u8;
    static cannoli_memhook_read_bpl_r12d: u8;
    static cannoli_memhook_read_bpl_r12d_end: u8;
    static cannoli_memhook_read_bpl_r13d: u8;
    static cannoli_memhook_read_bpl_r13d_end: u8;
    static cannoli_memhook_read_bpl_r14d: u8;
    static cannoli_memhook_read_bpl_r14d_end: u8;
    static cannoli_memhook_read_bpl_r15d: u8;
    static cannoli_memhook_read_bpl_r15d_end: u8;
    static cannoli_memhook_read_sil_eax: u8;
    static cannoli_memhook_read_sil_eax_end: u8;
    static cannoli_memhook_read_sil_ecx: u8;
    static cannoli_memhook_read_sil_ecx_end: u8;
    static cannoli_memhook_read_sil_edx: u8;
    static cannoli_memhook_read_sil_edx_end: u8;
    static cannoli_memhook_read_sil_ebx: u8;
    static cannoli_memhook_read_sil_ebx_end: u8;
    static cannoli_memhook_read_sil_esp: u8;
    static cannoli_memhook_read_sil_esp_end: u8;
    static cannoli_memhook_read_sil_ebp: u8;
    static cannoli_memhook_read_sil_ebp_end: u8;
    static cannoli_memhook_read_sil_esi: u8;
    static cannoli_memhook_read_sil_esi_end: u8;
    static cannoli_memhook_read_sil_edi: u8;
    static cannoli_memhook_read_sil_edi_end: u8;
    static cannoli_memhook_read_sil_r8d: u8;
    static cannoli_memhook_read_sil_r8d_end: u8;
    static cannoli_memhook_read_sil_r9d: u8;
    static cannoli_memhook_read_sil_r9d_end: u8;
    static cannoli_memhook_read_sil_r10d: u8;
    static cannoli_memhook_read_sil_r10d_end: u8;
    static cannoli_memhook_read_sil_r11d: u8;
    static cannoli_memhook_read_sil_r11d_end: u8;
    static cannoli_memhook_read_sil_r12d: u8;
    static cannoli_memhook_read_sil_r12d_end: u8;
    static cannoli_memhook_read_sil_r13d: u8;
    static cannoli_memhook_read_sil_r13d_end: u8;
    static cannoli_memhook_read_sil_r14d: u8;
    static cannoli_memhook_read_sil_r14d_end: u8;
    static cannoli_memhook_read_sil_r15d: u8;
    static cannoli_memhook_read_sil_r15d_end: u8;
    static cannoli_memhook_read_dil_eax: u8;
    static cannoli_memhook_read_dil_eax_end: u8;
    static cannoli_memhook_read_dil_ecx: u8;
    static cannoli_memhook_read_dil_ecx_end: u8;
    static cannoli_memhook_read_dil_edx: u8;
    static cannoli_memhook_read_dil_edx_end: u8;
    static cannoli_memhook_read_dil_ebx: u8;
    static cannoli_memhook_read_dil_ebx_end: u8;
    static cannoli_memhook_read_dil_esp: u8;
    static cannoli_memhook_read_dil_esp_end: u8;
    static cannoli_memhook_read_dil_ebp: u8;
    static cannoli_memhook_read_dil_ebp_end: u8;
    static cannoli_memhook_read_dil_esi: u8;
    static cannoli_memhook_read_dil_esi_end: u8;
    static cannoli_memhook_read_dil_edi: u8;
    static cannoli_memhook_read_dil_edi_end: u8;
    static cannoli_memhook_read_dil_r8d: u8;
    static cannoli_memhook_read_dil_r8d_end: u8;
    static cannoli_memhook_read_dil_r9d: u8;
    static cannoli_memhook_read_dil_r9d_end: u8;
    static cannoli_memhook_read_dil_r10d: u8;
    static cannoli_memhook_read_dil_r10d_end: u8;
    static cannoli_memhook_read_dil_r11d: u8;
    static cannoli_memhook_read_dil_r11d_end: u8;
    static cannoli_memhook_read_dil_r12d: u8;
    static cannoli_memhook_read_dil_r12d_end: u8;
    static cannoli_memhook_read_dil_r13d: u8;
    static cannoli_memhook_read_dil_r13d_end: u8;
    static cannoli_memhook_read_dil_r14d: u8;
    static cannoli_memhook_read_dil_r14d_end: u8;
    static cannoli_memhook_read_dil_r15d: u8;
    static cannoli_memhook_read_dil_r15d_end: u8;
    static cannoli_memhook_read_r8b_eax: u8;
    static cannoli_memhook_read_r8b_eax_end: u8;
    static cannoli_memhook_read_r8b_ecx: u8;
    static cannoli_memhook_read_r8b_ecx_end: u8;
    static cannoli_memhook_read_r8b_edx: u8;
    static cannoli_memhook_read_r8b_edx_end: u8;
    static cannoli_memhook_read_r8b_ebx: u8;
    static cannoli_memhook_read_r8b_ebx_end: u8;
    static cannoli_memhook_read_r8b_esp: u8;
    static cannoli_memhook_read_r8b_esp_end: u8;
    static cannoli_memhook_read_r8b_ebp: u8;
    static cannoli_memhook_read_r8b_ebp_end: u8;
    static cannoli_memhook_read_r8b_esi: u8;
    static cannoli_memhook_read_r8b_esi_end: u8;
    static cannoli_memhook_read_r8b_edi: u8;
    static cannoli_memhook_read_r8b_edi_end: u8;
    static cannoli_memhook_read_r8b_r8d: u8;
    static cannoli_memhook_read_r8b_r8d_end: u8;
    static cannoli_memhook_read_r8b_r9d: u8;
    static cannoli_memhook_read_r8b_r9d_end: u8;
    static cannoli_memhook_read_r8b_r10d: u8;
    static cannoli_memhook_read_r8b_r10d_end: u8;
    static cannoli_memhook_read_r8b_r11d: u8;
    static cannoli_memhook_read_r8b_r11d_end: u8;
    static cannoli_memhook_read_r8b_r12d: u8;
    static cannoli_memhook_read_r8b_r12d_end: u8;
    static cannoli_memhook_read_r8b_r13d: u8;
    static cannoli_memhook_read_r8b_r13d_end: u8;
    static cannoli_memhook_read_r8b_r14d: u8;
    static cannoli_memhook_read_r8b_r14d_end: u8;
    static cannoli_memhook_read_r8b_r15d: u8;
    static cannoli_memhook_read_r8b_r15d_end: u8;
    static cannoli_memhook_read_r9b_eax: u8;
    static cannoli_memhook_read_r9b_eax_end: u8;
    static cannoli_memhook_read_r9b_ecx: u8;
    static cannoli_memhook_read_r9b_ecx_end: u8;
    static cannoli_memhook_read_r9b_edx: u8;
    static cannoli_memhook_read_r9b_edx_end: u8;
    static cannoli_memhook_read_r9b_ebx: u8;
    static cannoli_memhook_read_r9b_ebx_end: u8;
    static cannoli_memhook_read_r9b_esp: u8;
    static cannoli_memhook_read_r9b_esp_end: u8;
    static cannoli_memhook_read_r9b_ebp: u8;
    static cannoli_memhook_read_r9b_ebp_end: u8;
    static cannoli_memhook_read_r9b_esi: u8;
    static cannoli_memhook_read_r9b_esi_end: u8;
    static cannoli_memhook_read_r9b_edi: u8;
    static cannoli_memhook_read_r9b_edi_end: u8;
    static cannoli_memhook_read_r9b_r8d: u8;
    static cannoli_memhook_read_r9b_r8d_end: u8;
    static cannoli_memhook_read_r9b_r9d: u8;
    static cannoli_memhook_read_r9b_r9d_end: u8;
    static cannoli_memhook_read_r9b_r10d: u8;
    static cannoli_memhook_read_r9b_r10d_end: u8;
    static cannoli_memhook_read_r9b_r11d: u8;
    static cannoli_memhook_read_r9b_r11d_end: u8;
    static cannoli_memhook_read_r9b_r12d: u8;
    static cannoli_memhook_read_r9b_r12d_end: u8;
    static cannoli_memhook_read_r9b_r13d: u8;
    static cannoli_memhook_read_r9b_r13d_end: u8;
    static cannoli_memhook_read_r9b_r14d: u8;
    static cannoli_memhook_read_r9b_r14d_end: u8;
    static cannoli_memhook_read_r9b_r15d: u8;
    static cannoli_memhook_read_r9b_r15d_end: u8;
    static cannoli_memhook_read_r10b_eax: u8;
    static cannoli_memhook_read_r10b_eax_end: u8;
    static cannoli_memhook_read_r10b_ecx: u8;
    static cannoli_memhook_read_r10b_ecx_end: u8;
    static cannoli_memhook_read_r10b_edx: u8;
    static cannoli_memhook_read_r10b_edx_end: u8;
    static cannoli_memhook_read_r10b_ebx: u8;
    static cannoli_memhook_read_r10b_ebx_end: u8;
    static cannoli_memhook_read_r10b_esp: u8;
    static cannoli_memhook_read_r10b_esp_end: u8;
    static cannoli_memhook_read_r10b_ebp: u8;
    static cannoli_memhook_read_r10b_ebp_end: u8;
    static cannoli_memhook_read_r10b_esi: u8;
    static cannoli_memhook_read_r10b_esi_end: u8;
    static cannoli_memhook_read_r10b_edi: u8;
    static cannoli_memhook_read_r10b_edi_end: u8;
    static cannoli_memhook_read_r10b_r8d: u8;
    static cannoli_memhook_read_r10b_r8d_end: u8;
    static cannoli_memhook_read_r10b_r9d: u8;
    static cannoli_memhook_read_r10b_r9d_end: u8;
    static cannoli_memhook_read_r10b_r10d: u8;
    static cannoli_memhook_read_r10b_r10d_end: u8;
    static cannoli_memhook_read_r10b_r11d: u8;
    static cannoli_memhook_read_r10b_r11d_end: u8;
    static cannoli_memhook_read_r10b_r12d: u8;
    static cannoli_memhook_read_r10b_r12d_end: u8;
    static cannoli_memhook_read_r10b_r13d: u8;
    static cannoli_memhook_read_r10b_r13d_end: u8;
    static cannoli_memhook_read_r10b_r14d: u8;
    static cannoli_memhook_read_r10b_r14d_end: u8;
    static cannoli_memhook_read_r10b_r15d: u8;
    static cannoli_memhook_read_r10b_r15d_end: u8;
    static cannoli_memhook_read_r11b_eax: u8;
    static cannoli_memhook_read_r11b_eax_end: u8;
    static cannoli_memhook_read_r11b_ecx: u8;
    static cannoli_memhook_read_r11b_ecx_end: u8;
    static cannoli_memhook_read_r11b_edx: u8;
    static cannoli_memhook_read_r11b_edx_end: u8;
    static cannoli_memhook_read_r11b_ebx: u8;
    static cannoli_memhook_read_r11b_ebx_end: u8;
    static cannoli_memhook_read_r11b_esp: u8;
    static cannoli_memhook_read_r11b_esp_end: u8;
    static cannoli_memhook_read_r11b_ebp: u8;
    static cannoli_memhook_read_r11b_ebp_end: u8;
    static cannoli_memhook_read_r11b_esi: u8;
    static cannoli_memhook_read_r11b_esi_end: u8;
    static cannoli_memhook_read_r11b_edi: u8;
    static cannoli_memhook_read_r11b_edi_end: u8;
    static cannoli_memhook_read_r11b_r8d: u8;
    static cannoli_memhook_read_r11b_r8d_end: u8;
    static cannoli_memhook_read_r11b_r9d: u8;
    static cannoli_memhook_read_r11b_r9d_end: u8;
    static cannoli_memhook_read_r11b_r10d: u8;
    static cannoli_memhook_read_r11b_r10d_end: u8;
    static cannoli_memhook_read_r11b_r11d: u8;
    static cannoli_memhook_read_r11b_r11d_end: u8;
    static cannoli_memhook_read_r11b_r12d: u8;
    static cannoli_memhook_read_r11b_r12d_end: u8;
    static cannoli_memhook_read_r11b_r13d: u8;
    static cannoli_memhook_read_r11b_r13d_end: u8;
    static cannoli_memhook_read_r11b_r14d: u8;
    static cannoli_memhook_read_r11b_r14d_end: u8;
    static cannoli_memhook_read_r11b_r15d: u8;
    static cannoli_memhook_read_r11b_r15d_end: u8;
    static cannoli_memhook_read_r12b_eax: u8;
    static cannoli_memhook_read_r12b_eax_end: u8;
    static cannoli_memhook_read_r12b_ecx: u8;
    static cannoli_memhook_read_r12b_ecx_end: u8;
    static cannoli_memhook_read_r12b_edx: u8;
    static cannoli_memhook_read_r12b_edx_end: u8;
    static cannoli_memhook_read_r12b_ebx: u8;
    static cannoli_memhook_read_r12b_ebx_end: u8;
    static cannoli_memhook_read_r12b_esp: u8;
    static cannoli_memhook_read_r12b_esp_end: u8;
    static cannoli_memhook_read_r12b_ebp: u8;
    static cannoli_memhook_read_r12b_ebp_end: u8;
    static cannoli_memhook_read_r12b_esi: u8;
    static cannoli_memhook_read_r12b_esi_end: u8;
    static cannoli_memhook_read_r12b_edi: u8;
    static cannoli_memhook_read_r12b_edi_end: u8;
    static cannoli_memhook_read_r12b_r8d: u8;
    static cannoli_memhook_read_r12b_r8d_end: u8;
    static cannoli_memhook_read_r12b_r9d: u8;
    static cannoli_memhook_read_r12b_r9d_end: u8;
    static cannoli_memhook_read_r12b_r10d: u8;
    static cannoli_memhook_read_r12b_r10d_end: u8;
    static cannoli_memhook_read_r12b_r11d: u8;
    static cannoli_memhook_read_r12b_r11d_end: u8;
    static cannoli_memhook_read_r12b_r12d: u8;
    static cannoli_memhook_read_r12b_r12d_end: u8;
    static cannoli_memhook_read_r12b_r13d: u8;
    static cannoli_memhook_read_r12b_r13d_end: u8;
    static cannoli_memhook_read_r12b_r14d: u8;
    static cannoli_memhook_read_r12b_r14d_end: u8;
    static cannoli_memhook_read_r12b_r15d: u8;
    static cannoli_memhook_read_r12b_r15d_end: u8;
    static cannoli_memhook_read_r13b_eax: u8;
    static cannoli_memhook_read_r13b_eax_end: u8;
    static cannoli_memhook_read_r13b_ecx: u8;
    static cannoli_memhook_read_r13b_ecx_end: u8;
    static cannoli_memhook_read_r13b_edx: u8;
    static cannoli_memhook_read_r13b_edx_end: u8;
    static cannoli_memhook_read_r13b_ebx: u8;
    static cannoli_memhook_read_r13b_ebx_end: u8;
    static cannoli_memhook_read_r13b_esp: u8;
    static cannoli_memhook_read_r13b_esp_end: u8;
    static cannoli_memhook_read_r13b_ebp: u8;
    static cannoli_memhook_read_r13b_ebp_end: u8;
    static cannoli_memhook_read_r13b_esi: u8;
    static cannoli_memhook_read_r13b_esi_end: u8;
    static cannoli_memhook_read_r13b_edi: u8;
    static cannoli_memhook_read_r13b_edi_end: u8;
    static cannoli_memhook_read_r13b_r8d: u8;
    static cannoli_memhook_read_r13b_r8d_end: u8;
    static cannoli_memhook_read_r13b_r9d: u8;
    static cannoli_memhook_read_r13b_r9d_end: u8;
    static cannoli_memhook_read_r13b_r10d: u8;
    static cannoli_memhook_read_r13b_r10d_end: u8;
    static cannoli_memhook_read_r13b_r11d: u8;
    static cannoli_memhook_read_r13b_r11d_end: u8;
    static cannoli_memhook_read_r13b_r12d: u8;
    static cannoli_memhook_read_r13b_r12d_end: u8;
    static cannoli_memhook_read_r13b_r13d: u8;
    static cannoli_memhook_read_r13b_r13d_end: u8;
    static cannoli_memhook_read_r13b_r14d: u8;
    static cannoli_memhook_read_r13b_r14d_end: u8;
    static cannoli_memhook_read_r13b_r15d: u8;
    static cannoli_memhook_read_r13b_r15d_end: u8;
    static cannoli_memhook_read_r14b_eax: u8;
    static cannoli_memhook_read_r14b_eax_end: u8;
    static cannoli_memhook_read_r14b_ecx: u8;
    static cannoli_memhook_read_r14b_ecx_end: u8;
    static cannoli_memhook_read_r14b_edx: u8;
    static cannoli_memhook_read_r14b_edx_end: u8;
    static cannoli_memhook_read_r14b_ebx: u8;
    static cannoli_memhook_read_r14b_ebx_end: u8;
    static cannoli_memhook_read_r14b_esp: u8;
    static cannoli_memhook_read_r14b_esp_end: u8;
    static cannoli_memhook_read_r14b_ebp: u8;
    static cannoli_memhook_read_r14b_ebp_end: u8;
    static cannoli_memhook_read_r14b_esi: u8;
    static cannoli_memhook_read_r14b_esi_end: u8;
    static cannoli_memhook_read_r14b_edi: u8;
    static cannoli_memhook_read_r14b_edi_end: u8;
    static cannoli_memhook_read_r14b_r8d: u8;
    static cannoli_memhook_read_r14b_r8d_end: u8;
    static cannoli_memhook_read_r14b_r9d: u8;
    static cannoli_memhook_read_r14b_r9d_end: u8;
    static cannoli_memhook_read_r14b_r10d: u8;
    static cannoli_memhook_read_r14b_r10d_end: u8;
    static cannoli_memhook_read_r14b_r11d: u8;
    static cannoli_memhook_read_r14b_r11d_end: u8;
    static cannoli_memhook_read_r14b_r12d: u8;
    static cannoli_memhook_read_r14b_r12d_end: u8;
    static cannoli_memhook_read_r14b_r13d: u8;
    static cannoli_memhook_read_r14b_r13d_end: u8;
    static cannoli_memhook_read_r14b_r14d: u8;
    static cannoli_memhook_read_r14b_r14d_end: u8;
    static cannoli_memhook_read_r14b_r15d: u8;
    static cannoli_memhook_read_r14b_r15d_end: u8;
    static cannoli_memhook_read_r15b_eax: u8;
    static cannoli_memhook_read_r15b_eax_end: u8;
    static cannoli_memhook_read_r15b_ecx: u8;
    static cannoli_memhook_read_r15b_ecx_end: u8;
    static cannoli_memhook_read_r15b_edx: u8;
    static cannoli_memhook_read_r15b_edx_end: u8;
    static cannoli_memhook_read_r15b_ebx: u8;
    static cannoli_memhook_read_r15b_ebx_end: u8;
    static cannoli_memhook_read_r15b_esp: u8;
    static cannoli_memhook_read_r15b_esp_end: u8;
    static cannoli_memhook_read_r15b_ebp: u8;
    static cannoli_memhook_read_r15b_ebp_end: u8;
    static cannoli_memhook_read_r15b_esi: u8;
    static cannoli_memhook_read_r15b_esi_end: u8;
    static cannoli_memhook_read_r15b_edi: u8;
    static cannoli_memhook_read_r15b_edi_end: u8;
    static cannoli_memhook_read_r15b_r8d: u8;
    static cannoli_memhook_read_r15b_r8d_end: u8;
    static cannoli_memhook_read_r15b_r9d: u8;
    static cannoli_memhook_read_r15b_r9d_end: u8;
    static cannoli_memhook_read_r15b_r10d: u8;
    static cannoli_memhook_read_r15b_r10d_end: u8;
    static cannoli_memhook_read_r15b_r11d: u8;
    static cannoli_memhook_read_r15b_r11d_end: u8;
    static cannoli_memhook_read_r15b_r12d: u8;
    static cannoli_memhook_read_r15b_r12d_end: u8;
    static cannoli_memhook_read_r15b_r13d: u8;
    static cannoli_memhook_read_r15b_r13d_end: u8;
    static cannoli_memhook_read_r15b_r14d: u8;
    static cannoli_memhook_read_r15b_r14d_end: u8;
    static cannoli_memhook_read_r15b_r15d: u8;
    static cannoli_memhook_read_r15b_r15d_end: u8;
    static cannoli_memhook_read_al_rax: u8;
    static cannoli_memhook_read_al_rax_end: u8;
    static cannoli_memhook_read_al_rcx: u8;
    static cannoli_memhook_read_al_rcx_end: u8;
    static cannoli_memhook_read_al_rdx: u8;
    static cannoli_memhook_read_al_rdx_end: u8;
    static cannoli_memhook_read_al_rbx: u8;
    static cannoli_memhook_read_al_rbx_end: u8;
    static cannoli_memhook_read_al_rsp: u8;
    static cannoli_memhook_read_al_rsp_end: u8;
    static cannoli_memhook_read_al_rbp: u8;
    static cannoli_memhook_read_al_rbp_end: u8;
    static cannoli_memhook_read_al_rsi: u8;
    static cannoli_memhook_read_al_rsi_end: u8;
    static cannoli_memhook_read_al_rdi: u8;
    static cannoli_memhook_read_al_rdi_end: u8;
    static cannoli_memhook_read_al_r8: u8;
    static cannoli_memhook_read_al_r8_end: u8;
    static cannoli_memhook_read_al_r9: u8;
    static cannoli_memhook_read_al_r9_end: u8;
    static cannoli_memhook_read_al_r10: u8;
    static cannoli_memhook_read_al_r10_end: u8;
    static cannoli_memhook_read_al_r11: u8;
    static cannoli_memhook_read_al_r11_end: u8;
    static cannoli_memhook_read_al_r12: u8;
    static cannoli_memhook_read_al_r12_end: u8;
    static cannoli_memhook_read_al_r13: u8;
    static cannoli_memhook_read_al_r13_end: u8;
    static cannoli_memhook_read_al_r14: u8;
    static cannoli_memhook_read_al_r14_end: u8;
    static cannoli_memhook_read_al_r15: u8;
    static cannoli_memhook_read_al_r15_end: u8;
    static cannoli_memhook_read_cl_rax: u8;
    static cannoli_memhook_read_cl_rax_end: u8;
    static cannoli_memhook_read_cl_rcx: u8;
    static cannoli_memhook_read_cl_rcx_end: u8;
    static cannoli_memhook_read_cl_rdx: u8;
    static cannoli_memhook_read_cl_rdx_end: u8;
    static cannoli_memhook_read_cl_rbx: u8;
    static cannoli_memhook_read_cl_rbx_end: u8;
    static cannoli_memhook_read_cl_rsp: u8;
    static cannoli_memhook_read_cl_rsp_end: u8;
    static cannoli_memhook_read_cl_rbp: u8;
    static cannoli_memhook_read_cl_rbp_end: u8;
    static cannoli_memhook_read_cl_rsi: u8;
    static cannoli_memhook_read_cl_rsi_end: u8;
    static cannoli_memhook_read_cl_rdi: u8;
    static cannoli_memhook_read_cl_rdi_end: u8;
    static cannoli_memhook_read_cl_r8: u8;
    static cannoli_memhook_read_cl_r8_end: u8;
    static cannoli_memhook_read_cl_r9: u8;
    static cannoli_memhook_read_cl_r9_end: u8;
    static cannoli_memhook_read_cl_r10: u8;
    static cannoli_memhook_read_cl_r10_end: u8;
    static cannoli_memhook_read_cl_r11: u8;
    static cannoli_memhook_read_cl_r11_end: u8;
    static cannoli_memhook_read_cl_r12: u8;
    static cannoli_memhook_read_cl_r12_end: u8;
    static cannoli_memhook_read_cl_r13: u8;
    static cannoli_memhook_read_cl_r13_end: u8;
    static cannoli_memhook_read_cl_r14: u8;
    static cannoli_memhook_read_cl_r14_end: u8;
    static cannoli_memhook_read_cl_r15: u8;
    static cannoli_memhook_read_cl_r15_end: u8;
    static cannoli_memhook_read_dl_rax: u8;
    static cannoli_memhook_read_dl_rax_end: u8;
    static cannoli_memhook_read_dl_rcx: u8;
    static cannoli_memhook_read_dl_rcx_end: u8;
    static cannoli_memhook_read_dl_rdx: u8;
    static cannoli_memhook_read_dl_rdx_end: u8;
    static cannoli_memhook_read_dl_rbx: u8;
    static cannoli_memhook_read_dl_rbx_end: u8;
    static cannoli_memhook_read_dl_rsp: u8;
    static cannoli_memhook_read_dl_rsp_end: u8;
    static cannoli_memhook_read_dl_rbp: u8;
    static cannoli_memhook_read_dl_rbp_end: u8;
    static cannoli_memhook_read_dl_rsi: u8;
    static cannoli_memhook_read_dl_rsi_end: u8;
    static cannoli_memhook_read_dl_rdi: u8;
    static cannoli_memhook_read_dl_rdi_end: u8;
    static cannoli_memhook_read_dl_r8: u8;
    static cannoli_memhook_read_dl_r8_end: u8;
    static cannoli_memhook_read_dl_r9: u8;
    static cannoli_memhook_read_dl_r9_end: u8;
    static cannoli_memhook_read_dl_r10: u8;
    static cannoli_memhook_read_dl_r10_end: u8;
    static cannoli_memhook_read_dl_r11: u8;
    static cannoli_memhook_read_dl_r11_end: u8;
    static cannoli_memhook_read_dl_r12: u8;
    static cannoli_memhook_read_dl_r12_end: u8;
    static cannoli_memhook_read_dl_r13: u8;
    static cannoli_memhook_read_dl_r13_end: u8;
    static cannoli_memhook_read_dl_r14: u8;
    static cannoli_memhook_read_dl_r14_end: u8;
    static cannoli_memhook_read_dl_r15: u8;
    static cannoli_memhook_read_dl_r15_end: u8;
    static cannoli_memhook_read_bl_rax: u8;
    static cannoli_memhook_read_bl_rax_end: u8;
    static cannoli_memhook_read_bl_rcx: u8;
    static cannoli_memhook_read_bl_rcx_end: u8;
    static cannoli_memhook_read_bl_rdx: u8;
    static cannoli_memhook_read_bl_rdx_end: u8;
    static cannoli_memhook_read_bl_rbx: u8;
    static cannoli_memhook_read_bl_rbx_end: u8;
    static cannoli_memhook_read_bl_rsp: u8;
    static cannoli_memhook_read_bl_rsp_end: u8;
    static cannoli_memhook_read_bl_rbp: u8;
    static cannoli_memhook_read_bl_rbp_end: u8;
    static cannoli_memhook_read_bl_rsi: u8;
    static cannoli_memhook_read_bl_rsi_end: u8;
    static cannoli_memhook_read_bl_rdi: u8;
    static cannoli_memhook_read_bl_rdi_end: u8;
    static cannoli_memhook_read_bl_r8: u8;
    static cannoli_memhook_read_bl_r8_end: u8;
    static cannoli_memhook_read_bl_r9: u8;
    static cannoli_memhook_read_bl_r9_end: u8;
    static cannoli_memhook_read_bl_r10: u8;
    static cannoli_memhook_read_bl_r10_end: u8;
    static cannoli_memhook_read_bl_r11: u8;
    static cannoli_memhook_read_bl_r11_end: u8;
    static cannoli_memhook_read_bl_r12: u8;
    static cannoli_memhook_read_bl_r12_end: u8;
    static cannoli_memhook_read_bl_r13: u8;
    static cannoli_memhook_read_bl_r13_end: u8;
    static cannoli_memhook_read_bl_r14: u8;
    static cannoli_memhook_read_bl_r14_end: u8;
    static cannoli_memhook_read_bl_r15: u8;
    static cannoli_memhook_read_bl_r15_end: u8;
    static cannoli_memhook_read_spl_rax: u8;
    static cannoli_memhook_read_spl_rax_end: u8;
    static cannoli_memhook_read_spl_rcx: u8;
    static cannoli_memhook_read_spl_rcx_end: u8;
    static cannoli_memhook_read_spl_rdx: u8;
    static cannoli_memhook_read_spl_rdx_end: u8;
    static cannoli_memhook_read_spl_rbx: u8;
    static cannoli_memhook_read_spl_rbx_end: u8;
    static cannoli_memhook_read_spl_rsp: u8;
    static cannoli_memhook_read_spl_rsp_end: u8;
    static cannoli_memhook_read_spl_rbp: u8;
    static cannoli_memhook_read_spl_rbp_end: u8;
    static cannoli_memhook_read_spl_rsi: u8;
    static cannoli_memhook_read_spl_rsi_end: u8;
    static cannoli_memhook_read_spl_rdi: u8;
    static cannoli_memhook_read_spl_rdi_end: u8;
    static cannoli_memhook_read_spl_r8: u8;
    static cannoli_memhook_read_spl_r8_end: u8;
    static cannoli_memhook_read_spl_r9: u8;
    static cannoli_memhook_read_spl_r9_end: u8;
    static cannoli_memhook_read_spl_r10: u8;
    static cannoli_memhook_read_spl_r10_end: u8;
    static cannoli_memhook_read_spl_r11: u8;
    static cannoli_memhook_read_spl_r11_end: u8;
    static cannoli_memhook_read_spl_r12: u8;
    static cannoli_memhook_read_spl_r12_end: u8;
    static cannoli_memhook_read_spl_r13: u8;
    static cannoli_memhook_read_spl_r13_end: u8;
    static cannoli_memhook_read_spl_r14: u8;
    static cannoli_memhook_read_spl_r14_end: u8;
    static cannoli_memhook_read_spl_r15: u8;
    static cannoli_memhook_read_spl_r15_end: u8;
    static cannoli_memhook_read_bpl_rax: u8;
    static cannoli_memhook_read_bpl_rax_end: u8;
    static cannoli_memhook_read_bpl_rcx: u8;
    static cannoli_memhook_read_bpl_rcx_end: u8;
    static cannoli_memhook_read_bpl_rdx: u8;
    static cannoli_memhook_read_bpl_rdx_end: u8;
    static cannoli_memhook_read_bpl_rbx: u8;
    static cannoli_memhook_read_bpl_rbx_end: u8;
    static cannoli_memhook_read_bpl_rsp: u8;
    static cannoli_memhook_read_bpl_rsp_end: u8;
    static cannoli_memhook_read_bpl_rbp: u8;
    static cannoli_memhook_read_bpl_rbp_end: u8;
    static cannoli_memhook_read_bpl_rsi: u8;
    static cannoli_memhook_read_bpl_rsi_end: u8;
    static cannoli_memhook_read_bpl_rdi: u8;
    static cannoli_memhook_read_bpl_rdi_end: u8;
    static cannoli_memhook_read_bpl_r8: u8;
    static cannoli_memhook_read_bpl_r8_end: u8;
    static cannoli_memhook_read_bpl_r9: u8;
    static cannoli_memhook_read_bpl_r9_end: u8;
    static cannoli_memhook_read_bpl_r10: u8;
    static cannoli_memhook_read_bpl_r10_end: u8;
    static cannoli_memhook_read_bpl_r11: u8;
    static cannoli_memhook_read_bpl_r11_end: u8;
    static cannoli_memhook_read_bpl_r12: u8;
    static cannoli_memhook_read_bpl_r12_end: u8;
    static cannoli_memhook_read_bpl_r13: u8;
    static cannoli_memhook_read_bpl_r13_end: u8;
    static cannoli_memhook_read_bpl_r14: u8;
    static cannoli_memhook_read_bpl_r14_end: u8;
    static cannoli_memhook_read_bpl_r15: u8;
    static cannoli_memhook_read_bpl_r15_end: u8;
    static cannoli_memhook_read_sil_rax: u8;
    static cannoli_memhook_read_sil_rax_end: u8;
    static cannoli_memhook_read_sil_rcx: u8;
    static cannoli_memhook_read_sil_rcx_end: u8;
    static cannoli_memhook_read_sil_rdx: u8;
    static cannoli_memhook_read_sil_rdx_end: u8;
    static cannoli_memhook_read_sil_rbx: u8;
    static cannoli_memhook_read_sil_rbx_end: u8;
    static cannoli_memhook_read_sil_rsp: u8;
    static cannoli_memhook_read_sil_rsp_end: u8;
    static cannoli_memhook_read_sil_rbp: u8;
    static cannoli_memhook_read_sil_rbp_end: u8;
    static cannoli_memhook_read_sil_rsi: u8;
    static cannoli_memhook_read_sil_rsi_end: u8;
    static cannoli_memhook_read_sil_rdi: u8;
    static cannoli_memhook_read_sil_rdi_end: u8;
    static cannoli_memhook_read_sil_r8: u8;
    static cannoli_memhook_read_sil_r8_end: u8;
    static cannoli_memhook_read_sil_r9: u8;
    static cannoli_memhook_read_sil_r9_end: u8;
    static cannoli_memhook_read_sil_r10: u8;
    static cannoli_memhook_read_sil_r10_end: u8;
    static cannoli_memhook_read_sil_r11: u8;
    static cannoli_memhook_read_sil_r11_end: u8;
    static cannoli_memhook_read_sil_r12: u8;
    static cannoli_memhook_read_sil_r12_end: u8;
    static cannoli_memhook_read_sil_r13: u8;
    static cannoli_memhook_read_sil_r13_end: u8;
    static cannoli_memhook_read_sil_r14: u8;
    static cannoli_memhook_read_sil_r14_end: u8;
    static cannoli_memhook_read_sil_r15: u8;
    static cannoli_memhook_read_sil_r15_end: u8;
    static cannoli_memhook_read_dil_rax: u8;
    static cannoli_memhook_read_dil_rax_end: u8;
    static cannoli_memhook_read_dil_rcx: u8;
    static cannoli_memhook_read_dil_rcx_end: u8;
    static cannoli_memhook_read_dil_rdx: u8;
    static cannoli_memhook_read_dil_rdx_end: u8;
    static cannoli_memhook_read_dil_rbx: u8;
    static cannoli_memhook_read_dil_rbx_end: u8;
    static cannoli_memhook_read_dil_rsp: u8;
    static cannoli_memhook_read_dil_rsp_end: u8;
    static cannoli_memhook_read_dil_rbp: u8;
    static cannoli_memhook_read_dil_rbp_end: u8;
    static cannoli_memhook_read_dil_rsi: u8;
    static cannoli_memhook_read_dil_rsi_end: u8;
    static cannoli_memhook_read_dil_rdi: u8;
    static cannoli_memhook_read_dil_rdi_end: u8;
    static cannoli_memhook_read_dil_r8: u8;
    static cannoli_memhook_read_dil_r8_end: u8;
    static cannoli_memhook_read_dil_r9: u8;
    static cannoli_memhook_read_dil_r9_end: u8;
    static cannoli_memhook_read_dil_r10: u8;
    static cannoli_memhook_read_dil_r10_end: u8;
    static cannoli_memhook_read_dil_r11: u8;
    static cannoli_memhook_read_dil_r11_end: u8;
    static cannoli_memhook_read_dil_r12: u8;
    static cannoli_memhook_read_dil_r12_end: u8;
    static cannoli_memhook_read_dil_r13: u8;
    static cannoli_memhook_read_dil_r13_end: u8;
    static cannoli_memhook_read_dil_r14: u8;
    static cannoli_memhook_read_dil_r14_end: u8;
    static cannoli_memhook_read_dil_r15: u8;
    static cannoli_memhook_read_dil_r15_end: u8;
    static cannoli_memhook_read_r8b_rax: u8;
    static cannoli_memhook_read_r8b_rax_end: u8;
    static cannoli_memhook_read_r8b_rcx: u8;
    static cannoli_memhook_read_r8b_rcx_end: u8;
    static cannoli_memhook_read_r8b_rdx: u8;
    static cannoli_memhook_read_r8b_rdx_end: u8;
    static cannoli_memhook_read_r8b_rbx: u8;
    static cannoli_memhook_read_r8b_rbx_end: u8;
    static cannoli_memhook_read_r8b_rsp: u8;
    static cannoli_memhook_read_r8b_rsp_end: u8;
    static cannoli_memhook_read_r8b_rbp: u8;
    static cannoli_memhook_read_r8b_rbp_end: u8;
    static cannoli_memhook_read_r8b_rsi: u8;
    static cannoli_memhook_read_r8b_rsi_end: u8;
    static cannoli_memhook_read_r8b_rdi: u8;
    static cannoli_memhook_read_r8b_rdi_end: u8;
    static cannoli_memhook_read_r8b_r8: u8;
    static cannoli_memhook_read_r8b_r8_end: u8;
    static cannoli_memhook_read_r8b_r9: u8;
    static cannoli_memhook_read_r8b_r9_end: u8;
    static cannoli_memhook_read_r8b_r10: u8;
    static cannoli_memhook_read_r8b_r10_end: u8;
    static cannoli_memhook_read_r8b_r11: u8;
    static cannoli_memhook_read_r8b_r11_end: u8;
    static cannoli_memhook_read_r8b_r12: u8;
    static cannoli_memhook_read_r8b_r12_end: u8;
    static cannoli_memhook_read_r8b_r13: u8;
    static cannoli_memhook_read_r8b_r13_end: u8;
    static cannoli_memhook_read_r8b_r14: u8;
    static cannoli_memhook_read_r8b_r14_end: u8;
    static cannoli_memhook_read_r8b_r15: u8;
    static cannoli_memhook_read_r8b_r15_end: u8;
    static cannoli_memhook_read_r9b_rax: u8;
    static cannoli_memhook_read_r9b_rax_end: u8;
    static cannoli_memhook_read_r9b_rcx: u8;
    static cannoli_memhook_read_r9b_rcx_end: u8;
    static cannoli_memhook_read_r9b_rdx: u8;
    static cannoli_memhook_read_r9b_rdx_end: u8;
    static cannoli_memhook_read_r9b_rbx: u8;
    static cannoli_memhook_read_r9b_rbx_end: u8;
    static cannoli_memhook_read_r9b_rsp: u8;
    static cannoli_memhook_read_r9b_rsp_end: u8;
    static cannoli_memhook_read_r9b_rbp: u8;
    static cannoli_memhook_read_r9b_rbp_end: u8;
    static cannoli_memhook_read_r9b_rsi: u8;
    static cannoli_memhook_read_r9b_rsi_end: u8;
    static cannoli_memhook_read_r9b_rdi: u8;
    static cannoli_memhook_read_r9b_rdi_end: u8;
    static cannoli_memhook_read_r9b_r8: u8;
    static cannoli_memhook_read_r9b_r8_end: u8;
    static cannoli_memhook_read_r9b_r9: u8;
    static cannoli_memhook_read_r9b_r9_end: u8;
    static cannoli_memhook_read_r9b_r10: u8;
    static cannoli_memhook_read_r9b_r10_end: u8;
    static cannoli_memhook_read_r9b_r11: u8;
    static cannoli_memhook_read_r9b_r11_end: u8;
    static cannoli_memhook_read_r9b_r12: u8;
    static cannoli_memhook_read_r9b_r12_end: u8;
    static cannoli_memhook_read_r9b_r13: u8;
    static cannoli_memhook_read_r9b_r13_end: u8;
    static cannoli_memhook_read_r9b_r14: u8;
    static cannoli_memhook_read_r9b_r14_end: u8;
    static cannoli_memhook_read_r9b_r15: u8;
    static cannoli_memhook_read_r9b_r15_end: u8;
    static cannoli_memhook_read_r10b_rax: u8;
    static cannoli_memhook_read_r10b_rax_end: u8;
    static cannoli_memhook_read_r10b_rcx: u8;
    static cannoli_memhook_read_r10b_rcx_end: u8;
    static cannoli_memhook_read_r10b_rdx: u8;
    static cannoli_memhook_read_r10b_rdx_end: u8;
    static cannoli_memhook_read_r10b_rbx: u8;
    static cannoli_memhook_read_r10b_rbx_end: u8;
    static cannoli_memhook_read_r10b_rsp: u8;
    static cannoli_memhook_read_r10b_rsp_end: u8;
    static cannoli_memhook_read_r10b_rbp: u8;
    static cannoli_memhook_read_r10b_rbp_end: u8;
    static cannoli_memhook_read_r10b_rsi: u8;
    static cannoli_memhook_read_r10b_rsi_end: u8;
    static cannoli_memhook_read_r10b_rdi: u8;
    static cannoli_memhook_read_r10b_rdi_end: u8;
    static cannoli_memhook_read_r10b_r8: u8;
    static cannoli_memhook_read_r10b_r8_end: u8;
    static cannoli_memhook_read_r10b_r9: u8;
    static cannoli_memhook_read_r10b_r9_end: u8;
    static cannoli_memhook_read_r10b_r10: u8;
    static cannoli_memhook_read_r10b_r10_end: u8;
    static cannoli_memhook_read_r10b_r11: u8;
    static cannoli_memhook_read_r10b_r11_end: u8;
    static cannoli_memhook_read_r10b_r12: u8;
    static cannoli_memhook_read_r10b_r12_end: u8;
    static cannoli_memhook_read_r10b_r13: u8;
    static cannoli_memhook_read_r10b_r13_end: u8;
    static cannoli_memhook_read_r10b_r14: u8;
    static cannoli_memhook_read_r10b_r14_end: u8;
    static cannoli_memhook_read_r10b_r15: u8;
    static cannoli_memhook_read_r10b_r15_end: u8;
    static cannoli_memhook_read_r11b_rax: u8;
    static cannoli_memhook_read_r11b_rax_end: u8;
    static cannoli_memhook_read_r11b_rcx: u8;
    static cannoli_memhook_read_r11b_rcx_end: u8;
    static cannoli_memhook_read_r11b_rdx: u8;
    static cannoli_memhook_read_r11b_rdx_end: u8;
    static cannoli_memhook_read_r11b_rbx: u8;
    static cannoli_memhook_read_r11b_rbx_end: u8;
    static cannoli_memhook_read_r11b_rsp: u8;
    static cannoli_memhook_read_r11b_rsp_end: u8;
    static cannoli_memhook_read_r11b_rbp: u8;
    static cannoli_memhook_read_r11b_rbp_end: u8;
    static cannoli_memhook_read_r11b_rsi: u8;
    static cannoli_memhook_read_r11b_rsi_end: u8;
    static cannoli_memhook_read_r11b_rdi: u8;
    static cannoli_memhook_read_r11b_rdi_end: u8;
    static cannoli_memhook_read_r11b_r8: u8;
    static cannoli_memhook_read_r11b_r8_end: u8;
    static cannoli_memhook_read_r11b_r9: u8;
    static cannoli_memhook_read_r11b_r9_end: u8;
    static cannoli_memhook_read_r11b_r10: u8;
    static cannoli_memhook_read_r11b_r10_end: u8;
    static cannoli_memhook_read_r11b_r11: u8;
    static cannoli_memhook_read_r11b_r11_end: u8;
    static cannoli_memhook_read_r11b_r12: u8;
    static cannoli_memhook_read_r11b_r12_end: u8;
    static cannoli_memhook_read_r11b_r13: u8;
    static cannoli_memhook_read_r11b_r13_end: u8;
    static cannoli_memhook_read_r11b_r14: u8;
    static cannoli_memhook_read_r11b_r14_end: u8;
    static cannoli_memhook_read_r11b_r15: u8;
    static cannoli_memhook_read_r11b_r15_end: u8;
    static cannoli_memhook_read_r12b_rax: u8;
    static cannoli_memhook_read_r12b_rax_end: u8;
    static cannoli_memhook_read_r12b_rcx: u8;
    static cannoli_memhook_read_r12b_rcx_end: u8;
    static cannoli_memhook_read_r12b_rdx: u8;
    static cannoli_memhook_read_r12b_rdx_end: u8;
    static cannoli_memhook_read_r12b_rbx: u8;
    static cannoli_memhook_read_r12b_rbx_end: u8;
    static cannoli_memhook_read_r12b_rsp: u8;
    static cannoli_memhook_read_r12b_rsp_end: u8;
    static cannoli_memhook_read_r12b_rbp: u8;
    static cannoli_memhook_read_r12b_rbp_end: u8;
    static cannoli_memhook_read_r12b_rsi: u8;
    static cannoli_memhook_read_r12b_rsi_end: u8;
    static cannoli_memhook_read_r12b_rdi: u8;
    static cannoli_memhook_read_r12b_rdi_end: u8;
    static cannoli_memhook_read_r12b_r8: u8;
    static cannoli_memhook_read_r12b_r8_end: u8;
    static cannoli_memhook_read_r12b_r9: u8;
    static cannoli_memhook_read_r12b_r9_end: u8;
    static cannoli_memhook_read_r12b_r10: u8;
    static cannoli_memhook_read_r12b_r10_end: u8;
    static cannoli_memhook_read_r12b_r11: u8;
    static cannoli_memhook_read_r12b_r11_end: u8;
    static cannoli_memhook_read_r12b_r12: u8;
    static cannoli_memhook_read_r12b_r12_end: u8;
    static cannoli_memhook_read_r12b_r13: u8;
    static cannoli_memhook_read_r12b_r13_end: u8;
    static cannoli_memhook_read_r12b_r14: u8;
    static cannoli_memhook_read_r12b_r14_end: u8;
    static cannoli_memhook_read_r12b_r15: u8;
    static cannoli_memhook_read_r12b_r15_end: u8;
    static cannoli_memhook_read_r13b_rax: u8;
    static cannoli_memhook_read_r13b_rax_end: u8;
    static cannoli_memhook_read_r13b_rcx: u8;
    static cannoli_memhook_read_r13b_rcx_end: u8;
    static cannoli_memhook_read_r13b_rdx: u8;
    static cannoli_memhook_read_r13b_rdx_end: u8;
    static cannoli_memhook_read_r13b_rbx: u8;
    static cannoli_memhook_read_r13b_rbx_end: u8;
    static cannoli_memhook_read_r13b_rsp: u8;
    static cannoli_memhook_read_r13b_rsp_end: u8;
    static cannoli_memhook_read_r13b_rbp: u8;
    static cannoli_memhook_read_r13b_rbp_end: u8;
    static cannoli_memhook_read_r13b_rsi: u8;
    static cannoli_memhook_read_r13b_rsi_end: u8;
    static cannoli_memhook_read_r13b_rdi: u8;
    static cannoli_memhook_read_r13b_rdi_end: u8;
    static cannoli_memhook_read_r13b_r8: u8;
    static cannoli_memhook_read_r13b_r8_end: u8;
    static cannoli_memhook_read_r13b_r9: u8;
    static cannoli_memhook_read_r13b_r9_end: u8;
    static cannoli_memhook_read_r13b_r10: u8;
    static cannoli_memhook_read_r13b_r10_end: u8;
    static cannoli_memhook_read_r13b_r11: u8;
    static cannoli_memhook_read_r13b_r11_end: u8;
    static cannoli_memhook_read_r13b_r12: u8;
    static cannoli_memhook_read_r13b_r12_end: u8;
    static cannoli_memhook_read_r13b_r13: u8;
    static cannoli_memhook_read_r13b_r13_end: u8;
    static cannoli_memhook_read_r13b_r14: u8;
    static cannoli_memhook_read_r13b_r14_end: u8;
    static cannoli_memhook_read_r13b_r15: u8;
    static cannoli_memhook_read_r13b_r15_end: u8;
    static cannoli_memhook_read_r14b_rax: u8;
    static cannoli_memhook_read_r14b_rax_end: u8;
    static cannoli_memhook_read_r14b_rcx: u8;
    static cannoli_memhook_read_r14b_rcx_end: u8;
    static cannoli_memhook_read_r14b_rdx: u8;
    static cannoli_memhook_read_r14b_rdx_end: u8;
    static cannoli_memhook_read_r14b_rbx: u8;
    static cannoli_memhook_read_r14b_rbx_end: u8;
    static cannoli_memhook_read_r14b_rsp: u8;
    static cannoli_memhook_read_r14b_rsp_end: u8;
    static cannoli_memhook_read_r14b_rbp: u8;
    static cannoli_memhook_read_r14b_rbp_end: u8;
    static cannoli_memhook_read_r14b_rsi: u8;
    static cannoli_memhook_read_r14b_rsi_end: u8;
    static cannoli_memhook_read_r14b_rdi: u8;
    static cannoli_memhook_read_r14b_rdi_end: u8;
    static cannoli_memhook_read_r14b_r8: u8;
    static cannoli_memhook_read_r14b_r8_end: u8;
    static cannoli_memhook_read_r14b_r9: u8;
    static cannoli_memhook_read_r14b_r9_end: u8;
    static cannoli_memhook_read_r14b_r10: u8;
    static cannoli_memhook_read_r14b_r10_end: u8;
    static cannoli_memhook_read_r14b_r11: u8;
    static cannoli_memhook_read_r14b_r11_end: u8;
    static cannoli_memhook_read_r14b_r12: u8;
    static cannoli_memhook_read_r14b_r12_end: u8;
    static cannoli_memhook_read_r14b_r13: u8;
    static cannoli_memhook_read_r14b_r13_end: u8;
    static cannoli_memhook_read_r14b_r14: u8;
    static cannoli_memhook_read_r14b_r14_end: u8;
    static cannoli_memhook_read_r14b_r15: u8;
    static cannoli_memhook_read_r14b_r15_end: u8;
    static cannoli_memhook_read_r15b_rax: u8;
    static cannoli_memhook_read_r15b_rax_end: u8;
    static cannoli_memhook_read_r15b_rcx: u8;
    static cannoli_memhook_read_r15b_rcx_end: u8;
    static cannoli_memhook_read_r15b_rdx: u8;
    static cannoli_memhook_read_r15b_rdx_end: u8;
    static cannoli_memhook_read_r15b_rbx: u8;
    static cannoli_memhook_read_r15b_rbx_end: u8;
    static cannoli_memhook_read_r15b_rsp: u8;
    static cannoli_memhook_read_r15b_rsp_end: u8;
    static cannoli_memhook_read_r15b_rbp: u8;
    static cannoli_memhook_read_r15b_rbp_end: u8;
    static cannoli_memhook_read_r15b_rsi: u8;
    static cannoli_memhook_read_r15b_rsi_end: u8;
    static cannoli_memhook_read_r15b_rdi: u8;
    static cannoli_memhook_read_r15b_rdi_end: u8;
    static cannoli_memhook_read_r15b_r8: u8;
    static cannoli_memhook_read_r15b_r8_end: u8;
    static cannoli_memhook_read_r15b_r9: u8;
    static cannoli_memhook_read_r15b_r9_end: u8;
    static cannoli_memhook_read_r15b_r10: u8;
    static cannoli_memhook_read_r15b_r10_end: u8;
    static cannoli_memhook_read_r15b_r11: u8;
    static cannoli_memhook_read_r15b_r11_end: u8;
    static cannoli_memhook_read_r15b_r12: u8;
    static cannoli_memhook_read_r15b_r12_end: u8;
    static cannoli_memhook_read_r15b_r13: u8;
    static cannoli_memhook_read_r15b_r13_end: u8;
    static cannoli_memhook_read_r15b_r14: u8;
    static cannoli_memhook_read_r15b_r14_end: u8;
    static cannoli_memhook_read_r15b_r15: u8;
    static cannoli_memhook_read_r15b_r15_end: u8;
    static cannoli_memhook_read_ax_eax: u8;
    static cannoli_memhook_read_ax_eax_end: u8;
    static cannoli_memhook_read_ax_ecx: u8;
    static cannoli_memhook_read_ax_ecx_end: u8;
    static cannoli_memhook_read_ax_edx: u8;
    static cannoli_memhook_read_ax_edx_end: u8;
    static cannoli_memhook_read_ax_ebx: u8;
    static cannoli_memhook_read_ax_ebx_end: u8;
    static cannoli_memhook_read_ax_esp: u8;
    static cannoli_memhook_read_ax_esp_end: u8;
    static cannoli_memhook_read_ax_ebp: u8;
    static cannoli_memhook_read_ax_ebp_end: u8;
    static cannoli_memhook_read_ax_esi: u8;
    static cannoli_memhook_read_ax_esi_end: u8;
    static cannoli_memhook_read_ax_edi: u8;
    static cannoli_memhook_read_ax_edi_end: u8;
    static cannoli_memhook_read_ax_r8d: u8;
    static cannoli_memhook_read_ax_r8d_end: u8;
    static cannoli_memhook_read_ax_r9d: u8;
    static cannoli_memhook_read_ax_r9d_end: u8;
    static cannoli_memhook_read_ax_r10d: u8;
    static cannoli_memhook_read_ax_r10d_end: u8;
    static cannoli_memhook_read_ax_r11d: u8;
    static cannoli_memhook_read_ax_r11d_end: u8;
    static cannoli_memhook_read_ax_r12d: u8;
    static cannoli_memhook_read_ax_r12d_end: u8;
    static cannoli_memhook_read_ax_r13d: u8;
    static cannoli_memhook_read_ax_r13d_end: u8;
    static cannoli_memhook_read_ax_r14d: u8;
    static cannoli_memhook_read_ax_r14d_end: u8;
    static cannoli_memhook_read_ax_r15d: u8;
    static cannoli_memhook_read_ax_r15d_end: u8;
    static cannoli_memhook_read_cx_eax: u8;
    static cannoli_memhook_read_cx_eax_end: u8;
    static cannoli_memhook_read_cx_ecx: u8;
    static cannoli_memhook_read_cx_ecx_end: u8;
    static cannoli_memhook_read_cx_edx: u8;
    static cannoli_memhook_read_cx_edx_end: u8;
    static cannoli_memhook_read_cx_ebx: u8;
    static cannoli_memhook_read_cx_ebx_end: u8;
    static cannoli_memhook_read_cx_esp: u8;
    static cannoli_memhook_read_cx_esp_end: u8;
    static cannoli_memhook_read_cx_ebp: u8;
    static cannoli_memhook_read_cx_ebp_end: u8;
    static cannoli_memhook_read_cx_esi: u8;
    static cannoli_memhook_read_cx_esi_end: u8;
    static cannoli_memhook_read_cx_edi: u8;
    static cannoli_memhook_read_cx_edi_end: u8;
    static cannoli_memhook_read_cx_r8d: u8;
    static cannoli_memhook_read_cx_r8d_end: u8;
    static cannoli_memhook_read_cx_r9d: u8;
    static cannoli_memhook_read_cx_r9d_end: u8;
    static cannoli_memhook_read_cx_r10d: u8;
    static cannoli_memhook_read_cx_r10d_end: u8;
    static cannoli_memhook_read_cx_r11d: u8;
    static cannoli_memhook_read_cx_r11d_end: u8;
    static cannoli_memhook_read_cx_r12d: u8;
    static cannoli_memhook_read_cx_r12d_end: u8;
    static cannoli_memhook_read_cx_r13d: u8;
    static cannoli_memhook_read_cx_r13d_end: u8;
    static cannoli_memhook_read_cx_r14d: u8;
    static cannoli_memhook_read_cx_r14d_end: u8;
    static cannoli_memhook_read_cx_r15d: u8;
    static cannoli_memhook_read_cx_r15d_end: u8;
    static cannoli_memhook_read_dx_eax: u8;
    static cannoli_memhook_read_dx_eax_end: u8;
    static cannoli_memhook_read_dx_ecx: u8;
    static cannoli_memhook_read_dx_ecx_end: u8;
    static cannoli_memhook_read_dx_edx: u8;
    static cannoli_memhook_read_dx_edx_end: u8;
    static cannoli_memhook_read_dx_ebx: u8;
    static cannoli_memhook_read_dx_ebx_end: u8;
    static cannoli_memhook_read_dx_esp: u8;
    static cannoli_memhook_read_dx_esp_end: u8;
    static cannoli_memhook_read_dx_ebp: u8;
    static cannoli_memhook_read_dx_ebp_end: u8;
    static cannoli_memhook_read_dx_esi: u8;
    static cannoli_memhook_read_dx_esi_end: u8;
    static cannoli_memhook_read_dx_edi: u8;
    static cannoli_memhook_read_dx_edi_end: u8;
    static cannoli_memhook_read_dx_r8d: u8;
    static cannoli_memhook_read_dx_r8d_end: u8;
    static cannoli_memhook_read_dx_r9d: u8;
    static cannoli_memhook_read_dx_r9d_end: u8;
    static cannoli_memhook_read_dx_r10d: u8;
    static cannoli_memhook_read_dx_r10d_end: u8;
    static cannoli_memhook_read_dx_r11d: u8;
    static cannoli_memhook_read_dx_r11d_end: u8;
    static cannoli_memhook_read_dx_r12d: u8;
    static cannoli_memhook_read_dx_r12d_end: u8;
    static cannoli_memhook_read_dx_r13d: u8;
    static cannoli_memhook_read_dx_r13d_end: u8;
    static cannoli_memhook_read_dx_r14d: u8;
    static cannoli_memhook_read_dx_r14d_end: u8;
    static cannoli_memhook_read_dx_r15d: u8;
    static cannoli_memhook_read_dx_r15d_end: u8;
    static cannoli_memhook_read_bx_eax: u8;
    static cannoli_memhook_read_bx_eax_end: u8;
    static cannoli_memhook_read_bx_ecx: u8;
    static cannoli_memhook_read_bx_ecx_end: u8;
    static cannoli_memhook_read_bx_edx: u8;
    static cannoli_memhook_read_bx_edx_end: u8;
    static cannoli_memhook_read_bx_ebx: u8;
    static cannoli_memhook_read_bx_ebx_end: u8;
    static cannoli_memhook_read_bx_esp: u8;
    static cannoli_memhook_read_bx_esp_end: u8;
    static cannoli_memhook_read_bx_ebp: u8;
    static cannoli_memhook_read_bx_ebp_end: u8;
    static cannoli_memhook_read_bx_esi: u8;
    static cannoli_memhook_read_bx_esi_end: u8;
    static cannoli_memhook_read_bx_edi: u8;
    static cannoli_memhook_read_bx_edi_end: u8;
    static cannoli_memhook_read_bx_r8d: u8;
    static cannoli_memhook_read_bx_r8d_end: u8;
    static cannoli_memhook_read_bx_r9d: u8;
    static cannoli_memhook_read_bx_r9d_end: u8;
    static cannoli_memhook_read_bx_r10d: u8;
    static cannoli_memhook_read_bx_r10d_end: u8;
    static cannoli_memhook_read_bx_r11d: u8;
    static cannoli_memhook_read_bx_r11d_end: u8;
    static cannoli_memhook_read_bx_r12d: u8;
    static cannoli_memhook_read_bx_r12d_end: u8;
    static cannoli_memhook_read_bx_r13d: u8;
    static cannoli_memhook_read_bx_r13d_end: u8;
    static cannoli_memhook_read_bx_r14d: u8;
    static cannoli_memhook_read_bx_r14d_end: u8;
    static cannoli_memhook_read_bx_r15d: u8;
    static cannoli_memhook_read_bx_r15d_end: u8;
    static cannoli_memhook_read_sp_eax: u8;
    static cannoli_memhook_read_sp_eax_end: u8;
    static cannoli_memhook_read_sp_ecx: u8;
    static cannoli_memhook_read_sp_ecx_end: u8;
    static cannoli_memhook_read_sp_edx: u8;
    static cannoli_memhook_read_sp_edx_end: u8;
    static cannoli_memhook_read_sp_ebx: u8;
    static cannoli_memhook_read_sp_ebx_end: u8;
    static cannoli_memhook_read_sp_esp: u8;
    static cannoli_memhook_read_sp_esp_end: u8;
    static cannoli_memhook_read_sp_ebp: u8;
    static cannoli_memhook_read_sp_ebp_end: u8;
    static cannoli_memhook_read_sp_esi: u8;
    static cannoli_memhook_read_sp_esi_end: u8;
    static cannoli_memhook_read_sp_edi: u8;
    static cannoli_memhook_read_sp_edi_end: u8;
    static cannoli_memhook_read_sp_r8d: u8;
    static cannoli_memhook_read_sp_r8d_end: u8;
    static cannoli_memhook_read_sp_r9d: u8;
    static cannoli_memhook_read_sp_r9d_end: u8;
    static cannoli_memhook_read_sp_r10d: u8;
    static cannoli_memhook_read_sp_r10d_end: u8;
    static cannoli_memhook_read_sp_r11d: u8;
    static cannoli_memhook_read_sp_r11d_end: u8;
    static cannoli_memhook_read_sp_r12d: u8;
    static cannoli_memhook_read_sp_r12d_end: u8;
    static cannoli_memhook_read_sp_r13d: u8;
    static cannoli_memhook_read_sp_r13d_end: u8;
    static cannoli_memhook_read_sp_r14d: u8;
    static cannoli_memhook_read_sp_r14d_end: u8;
    static cannoli_memhook_read_sp_r15d: u8;
    static cannoli_memhook_read_sp_r15d_end: u8;
    static cannoli_memhook_read_bp_eax: u8;
    static cannoli_memhook_read_bp_eax_end: u8;
    static cannoli_memhook_read_bp_ecx: u8;
    static cannoli_memhook_read_bp_ecx_end: u8;
    static cannoli_memhook_read_bp_edx: u8;
    static cannoli_memhook_read_bp_edx_end: u8;
    static cannoli_memhook_read_bp_ebx: u8;
    static cannoli_memhook_read_bp_ebx_end: u8;
    static cannoli_memhook_read_bp_esp: u8;
    static cannoli_memhook_read_bp_esp_end: u8;
    static cannoli_memhook_read_bp_ebp: u8;
    static cannoli_memhook_read_bp_ebp_end: u8;
    static cannoli_memhook_read_bp_esi: u8;
    static cannoli_memhook_read_bp_esi_end: u8;
    static cannoli_memhook_read_bp_edi: u8;
    static cannoli_memhook_read_bp_edi_end: u8;
    static cannoli_memhook_read_bp_r8d: u8;
    static cannoli_memhook_read_bp_r8d_end: u8;
    static cannoli_memhook_read_bp_r9d: u8;
    static cannoli_memhook_read_bp_r9d_end: u8;
    static cannoli_memhook_read_bp_r10d: u8;
    static cannoli_memhook_read_bp_r10d_end: u8;
    static cannoli_memhook_read_bp_r11d: u8;
    static cannoli_memhook_read_bp_r11d_end: u8;
    static cannoli_memhook_read_bp_r12d: u8;
    static cannoli_memhook_read_bp_r12d_end: u8;
    static cannoli_memhook_read_bp_r13d: u8;
    static cannoli_memhook_read_bp_r13d_end: u8;
    static cannoli_memhook_read_bp_r14d: u8;
    static cannoli_memhook_read_bp_r14d_end: u8;
    static cannoli_memhook_read_bp_r15d: u8;
    static cannoli_memhook_read_bp_r15d_end: u8;
    static cannoli_memhook_read_si_eax: u8;
    static cannoli_memhook_read_si_eax_end: u8;
    static cannoli_memhook_read_si_ecx: u8;
    static cannoli_memhook_read_si_ecx_end: u8;
    static cannoli_memhook_read_si_edx: u8;
    static cannoli_memhook_read_si_edx_end: u8;
    static cannoli_memhook_read_si_ebx: u8;
    static cannoli_memhook_read_si_ebx_end: u8;
    static cannoli_memhook_read_si_esp: u8;
    static cannoli_memhook_read_si_esp_end: u8;
    static cannoli_memhook_read_si_ebp: u8;
    static cannoli_memhook_read_si_ebp_end: u8;
    static cannoli_memhook_read_si_esi: u8;
    static cannoli_memhook_read_si_esi_end: u8;
    static cannoli_memhook_read_si_edi: u8;
    static cannoli_memhook_read_si_edi_end: u8;
    static cannoli_memhook_read_si_r8d: u8;
    static cannoli_memhook_read_si_r8d_end: u8;
    static cannoli_memhook_read_si_r9d: u8;
    static cannoli_memhook_read_si_r9d_end: u8;
    static cannoli_memhook_read_si_r10d: u8;
    static cannoli_memhook_read_si_r10d_end: u8;
    static cannoli_memhook_read_si_r11d: u8;
    static cannoli_memhook_read_si_r11d_end: u8;
    static cannoli_memhook_read_si_r12d: u8;
    static cannoli_memhook_read_si_r12d_end: u8;
    static cannoli_memhook_read_si_r13d: u8;
    static cannoli_memhook_read_si_r13d_end: u8;
    static cannoli_memhook_read_si_r14d: u8;
    static cannoli_memhook_read_si_r14d_end: u8;
    static cannoli_memhook_read_si_r15d: u8;
    static cannoli_memhook_read_si_r15d_end: u8;
    static cannoli_memhook_read_di_eax: u8;
    static cannoli_memhook_read_di_eax_end: u8;
    static cannoli_memhook_read_di_ecx: u8;
    static cannoli_memhook_read_di_ecx_end: u8;
    static cannoli_memhook_read_di_edx: u8;
    static cannoli_memhook_read_di_edx_end: u8;
    static cannoli_memhook_read_di_ebx: u8;
    static cannoli_memhook_read_di_ebx_end: u8;
    static cannoli_memhook_read_di_esp: u8;
    static cannoli_memhook_read_di_esp_end: u8;
    static cannoli_memhook_read_di_ebp: u8;
    static cannoli_memhook_read_di_ebp_end: u8;
    static cannoli_memhook_read_di_esi: u8;
    static cannoli_memhook_read_di_esi_end: u8;
    static cannoli_memhook_read_di_edi: u8;
    static cannoli_memhook_read_di_edi_end: u8;
    static cannoli_memhook_read_di_r8d: u8;
    static cannoli_memhook_read_di_r8d_end: u8;
    static cannoli_memhook_read_di_r9d: u8;
    static cannoli_memhook_read_di_r9d_end: u8;
    static cannoli_memhook_read_di_r10d: u8;
    static cannoli_memhook_read_di_r10d_end: u8;
    static cannoli_memhook_read_di_r11d: u8;
    static cannoli_memhook_read_di_r11d_end: u8;
    static cannoli_memhook_read_di_r12d: u8;
    static cannoli_memhook_read_di_r12d_end: u8;
    static cannoli_memhook_read_di_r13d: u8;
    static cannoli_memhook_read_di_r13d_end: u8;
    static cannoli_memhook_read_di_r14d: u8;
    static cannoli_memhook_read_di_r14d_end: u8;
    static cannoli_memhook_read_di_r15d: u8;
    static cannoli_memhook_read_di_r15d_end: u8;
    static cannoli_memhook_read_r8w_eax: u8;
    static cannoli_memhook_read_r8w_eax_end: u8;
    static cannoli_memhook_read_r8w_ecx: u8;
    static cannoli_memhook_read_r8w_ecx_end: u8;
    static cannoli_memhook_read_r8w_edx: u8;
    static cannoli_memhook_read_r8w_edx_end: u8;
    static cannoli_memhook_read_r8w_ebx: u8;
    static cannoli_memhook_read_r8w_ebx_end: u8;
    static cannoli_memhook_read_r8w_esp: u8;
    static cannoli_memhook_read_r8w_esp_end: u8;
    static cannoli_memhook_read_r8w_ebp: u8;
    static cannoli_memhook_read_r8w_ebp_end: u8;
    static cannoli_memhook_read_r8w_esi: u8;
    static cannoli_memhook_read_r8w_esi_end: u8;
    static cannoli_memhook_read_r8w_edi: u8;
    static cannoli_memhook_read_r8w_edi_end: u8;
    static cannoli_memhook_read_r8w_r8d: u8;
    static cannoli_memhook_read_r8w_r8d_end: u8;
    static cannoli_memhook_read_r8w_r9d: u8;
    static cannoli_memhook_read_r8w_r9d_end: u8;
    static cannoli_memhook_read_r8w_r10d: u8;
    static cannoli_memhook_read_r8w_r10d_end: u8;
    static cannoli_memhook_read_r8w_r11d: u8;
    static cannoli_memhook_read_r8w_r11d_end: u8;
    static cannoli_memhook_read_r8w_r12d: u8;
    static cannoli_memhook_read_r8w_r12d_end: u8;
    static cannoli_memhook_read_r8w_r13d: u8;
    static cannoli_memhook_read_r8w_r13d_end: u8;
    static cannoli_memhook_read_r8w_r14d: u8;
    static cannoli_memhook_read_r8w_r14d_end: u8;
    static cannoli_memhook_read_r8w_r15d: u8;
    static cannoli_memhook_read_r8w_r15d_end: u8;
    static cannoli_memhook_read_r9w_eax: u8;
    static cannoli_memhook_read_r9w_eax_end: u8;
    static cannoli_memhook_read_r9w_ecx: u8;
    static cannoli_memhook_read_r9w_ecx_end: u8;
    static cannoli_memhook_read_r9w_edx: u8;
    static cannoli_memhook_read_r9w_edx_end: u8;
    static cannoli_memhook_read_r9w_ebx: u8;
    static cannoli_memhook_read_r9w_ebx_end: u8;
    static cannoli_memhook_read_r9w_esp: u8;
    static cannoli_memhook_read_r9w_esp_end: u8;
    static cannoli_memhook_read_r9w_ebp: u8;
    static cannoli_memhook_read_r9w_ebp_end: u8;
    static cannoli_memhook_read_r9w_esi: u8;
    static cannoli_memhook_read_r9w_esi_end: u8;
    static cannoli_memhook_read_r9w_edi: u8;
    static cannoli_memhook_read_r9w_edi_end: u8;
    static cannoli_memhook_read_r9w_r8d: u8;
    static cannoli_memhook_read_r9w_r8d_end: u8;
    static cannoli_memhook_read_r9w_r9d: u8;
    static cannoli_memhook_read_r9w_r9d_end: u8;
    static cannoli_memhook_read_r9w_r10d: u8;
    static cannoli_memhook_read_r9w_r10d_end: u8;
    static cannoli_memhook_read_r9w_r11d: u8;
    static cannoli_memhook_read_r9w_r11d_end: u8;
    static cannoli_memhook_read_r9w_r12d: u8;
    static cannoli_memhook_read_r9w_r12d_end: u8;
    static cannoli_memhook_read_r9w_r13d: u8;
    static cannoli_memhook_read_r9w_r13d_end: u8;
    static cannoli_memhook_read_r9w_r14d: u8;
    static cannoli_memhook_read_r9w_r14d_end: u8;
    static cannoli_memhook_read_r9w_r15d: u8;
    static cannoli_memhook_read_r9w_r15d_end: u8;
    static cannoli_memhook_read_r10w_eax: u8;
    static cannoli_memhook_read_r10w_eax_end: u8;
    static cannoli_memhook_read_r10w_ecx: u8;
    static cannoli_memhook_read_r10w_ecx_end: u8;
    static cannoli_memhook_read_r10w_edx: u8;
    static cannoli_memhook_read_r10w_edx_end: u8;
    static cannoli_memhook_read_r10w_ebx: u8;
    static cannoli_memhook_read_r10w_ebx_end: u8;
    static cannoli_memhook_read_r10w_esp: u8;
    static cannoli_memhook_read_r10w_esp_end: u8;
    static cannoli_memhook_read_r10w_ebp: u8;
    static cannoli_memhook_read_r10w_ebp_end: u8;
    static cannoli_memhook_read_r10w_esi: u8;
    static cannoli_memhook_read_r10w_esi_end: u8;
    static cannoli_memhook_read_r10w_edi: u8;
    static cannoli_memhook_read_r10w_edi_end: u8;
    static cannoli_memhook_read_r10w_r8d: u8;
    static cannoli_memhook_read_r10w_r8d_end: u8;
    static cannoli_memhook_read_r10w_r9d: u8;
    static cannoli_memhook_read_r10w_r9d_end: u8;
    static cannoli_memhook_read_r10w_r10d: u8;
    static cannoli_memhook_read_r10w_r10d_end: u8;
    static cannoli_memhook_read_r10w_r11d: u8;
    static cannoli_memhook_read_r10w_r11d_end: u8;
    static cannoli_memhook_read_r10w_r12d: u8;
    static cannoli_memhook_read_r10w_r12d_end: u8;
    static cannoli_memhook_read_r10w_r13d: u8;
    static cannoli_memhook_read_r10w_r13d_end: u8;
    static cannoli_memhook_read_r10w_r14d: u8;
    static cannoli_memhook_read_r10w_r14d_end: u8;
    static cannoli_memhook_read_r10w_r15d: u8;
    static cannoli_memhook_read_r10w_r15d_end: u8;
    static cannoli_memhook_read_r11w_eax: u8;
    static cannoli_memhook_read_r11w_eax_end: u8;
    static cannoli_memhook_read_r11w_ecx: u8;
    static cannoli_memhook_read_r11w_ecx_end: u8;
    static cannoli_memhook_read_r11w_edx: u8;
    static cannoli_memhook_read_r11w_edx_end: u8;
    static cannoli_memhook_read_r11w_ebx: u8;
    static cannoli_memhook_read_r11w_ebx_end: u8;
    static cannoli_memhook_read_r11w_esp: u8;
    static cannoli_memhook_read_r11w_esp_end: u8;
    static cannoli_memhook_read_r11w_ebp: u8;
    static cannoli_memhook_read_r11w_ebp_end: u8;
    static cannoli_memhook_read_r11w_esi: u8;
    static cannoli_memhook_read_r11w_esi_end: u8;
    static cannoli_memhook_read_r11w_edi: u8;
    static cannoli_memhook_read_r11w_edi_end: u8;
    static cannoli_memhook_read_r11w_r8d: u8;
    static cannoli_memhook_read_r11w_r8d_end: u8;
    static cannoli_memhook_read_r11w_r9d: u8;
    static cannoli_memhook_read_r11w_r9d_end: u8;
    static cannoli_memhook_read_r11w_r10d: u8;
    static cannoli_memhook_read_r11w_r10d_end: u8;
    static cannoli_memhook_read_r11w_r11d: u8;
    static cannoli_memhook_read_r11w_r11d_end: u8;
    static cannoli_memhook_read_r11w_r12d: u8;
    static cannoli_memhook_read_r11w_r12d_end: u8;
    static cannoli_memhook_read_r11w_r13d: u8;
    static cannoli_memhook_read_r11w_r13d_end: u8;
    static cannoli_memhook_read_r11w_r14d: u8;
    static cannoli_memhook_read_r11w_r14d_end: u8;
    static cannoli_memhook_read_r11w_r15d: u8;
    static cannoli_memhook_read_r11w_r15d_end: u8;
    static cannoli_memhook_read_r12w_eax: u8;
    static cannoli_memhook_read_r12w_eax_end: u8;
    static cannoli_memhook_read_r12w_ecx: u8;
    static cannoli_memhook_read_r12w_ecx_end: u8;
    static cannoli_memhook_read_r12w_edx: u8;
    static cannoli_memhook_read_r12w_edx_end: u8;
    static cannoli_memhook_read_r12w_ebx: u8;
    static cannoli_memhook_read_r12w_ebx_end: u8;
    static cannoli_memhook_read_r12w_esp: u8;
    static cannoli_memhook_read_r12w_esp_end: u8;
    static cannoli_memhook_read_r12w_ebp: u8;
    static cannoli_memhook_read_r12w_ebp_end: u8;
    static cannoli_memhook_read_r12w_esi: u8;
    static cannoli_memhook_read_r12w_esi_end: u8;
    static cannoli_memhook_read_r12w_edi: u8;
    static cannoli_memhook_read_r12w_edi_end: u8;
    static cannoli_memhook_read_r12w_r8d: u8;
    static cannoli_memhook_read_r12w_r8d_end: u8;
    static cannoli_memhook_read_r12w_r9d: u8;
    static cannoli_memhook_read_r12w_r9d_end: u8;
    static cannoli_memhook_read_r12w_r10d: u8;
    static cannoli_memhook_read_r12w_r10d_end: u8;
    static cannoli_memhook_read_r12w_r11d: u8;
    static cannoli_memhook_read_r12w_r11d_end: u8;
    static cannoli_memhook_read_r12w_r12d: u8;
    static cannoli_memhook_read_r12w_r12d_end: u8;
    static cannoli_memhook_read_r12w_r13d: u8;
    static cannoli_memhook_read_r12w_r13d_end: u8;
    static cannoli_memhook_read_r12w_r14d: u8;
    static cannoli_memhook_read_r12w_r14d_end: u8;
    static cannoli_memhook_read_r12w_r15d: u8;
    static cannoli_memhook_read_r12w_r15d_end: u8;
    static cannoli_memhook_read_r13w_eax: u8;
    static cannoli_memhook_read_r13w_eax_end: u8;
    static cannoli_memhook_read_r13w_ecx: u8;
    static cannoli_memhook_read_r13w_ecx_end: u8;
    static cannoli_memhook_read_r13w_edx: u8;
    static cannoli_memhook_read_r13w_edx_end: u8;
    static cannoli_memhook_read_r13w_ebx: u8;
    static cannoli_memhook_read_r13w_ebx_end: u8;
    static cannoli_memhook_read_r13w_esp: u8;
    static cannoli_memhook_read_r13w_esp_end: u8;
    static cannoli_memhook_read_r13w_ebp: u8;
    static cannoli_memhook_read_r13w_ebp_end: u8;
    static cannoli_memhook_read_r13w_esi: u8;
    static cannoli_memhook_read_r13w_esi_end: u8;
    static cannoli_memhook_read_r13w_edi: u8;
    static cannoli_memhook_read_r13w_edi_end: u8;
    static cannoli_memhook_read_r13w_r8d: u8;
    static cannoli_memhook_read_r13w_r8d_end: u8;
    static cannoli_memhook_read_r13w_r9d: u8;
    static cannoli_memhook_read_r13w_r9d_end: u8;
    static cannoli_memhook_read_r13w_r10d: u8;
    static cannoli_memhook_read_r13w_r10d_end: u8;
    static cannoli_memhook_read_r13w_r11d: u8;
    static cannoli_memhook_read_r13w_r11d_end: u8;
    static cannoli_memhook_read_r13w_r12d: u8;
    static cannoli_memhook_read_r13w_r12d_end: u8;
    static cannoli_memhook_read_r13w_r13d: u8;
    static cannoli_memhook_read_r13w_r13d_end: u8;
    static cannoli_memhook_read_r13w_r14d: u8;
    static cannoli_memhook_read_r13w_r14d_end: u8;
    static cannoli_memhook_read_r13w_r15d: u8;
    static cannoli_memhook_read_r13w_r15d_end: u8;
    static cannoli_memhook_read_r14w_eax: u8;
    static cannoli_memhook_read_r14w_eax_end: u8;
    static cannoli_memhook_read_r14w_ecx: u8;
    static cannoli_memhook_read_r14w_ecx_end: u8;
    static cannoli_memhook_read_r14w_edx: u8;
    static cannoli_memhook_read_r14w_edx_end: u8;
    static cannoli_memhook_read_r14w_ebx: u8;
    static cannoli_memhook_read_r14w_ebx_end: u8;
    static cannoli_memhook_read_r14w_esp: u8;
    static cannoli_memhook_read_r14w_esp_end: u8;
    static cannoli_memhook_read_r14w_ebp: u8;
    static cannoli_memhook_read_r14w_ebp_end: u8;
    static cannoli_memhook_read_r14w_esi: u8;
    static cannoli_memhook_read_r14w_esi_end: u8;
    static cannoli_memhook_read_r14w_edi: u8;
    static cannoli_memhook_read_r14w_edi_end: u8;
    static cannoli_memhook_read_r14w_r8d: u8;
    static cannoli_memhook_read_r14w_r8d_end: u8;
    static cannoli_memhook_read_r14w_r9d: u8;
    static cannoli_memhook_read_r14w_r9d_end: u8;
    static cannoli_memhook_read_r14w_r10d: u8;
    static cannoli_memhook_read_r14w_r10d_end: u8;
    static cannoli_memhook_read_r14w_r11d: u8;
    static cannoli_memhook_read_r14w_r11d_end: u8;
    static cannoli_memhook_read_r14w_r12d: u8;
    static cannoli_memhook_read_r14w_r12d_end: u8;
    static cannoli_memhook_read_r14w_r13d: u8;
    static cannoli_memhook_read_r14w_r13d_end: u8;
    static cannoli_memhook_read_r14w_r14d: u8;
    static cannoli_memhook_read_r14w_r14d_end: u8;
    static cannoli_memhook_read_r14w_r15d: u8;
    static cannoli_memhook_read_r14w_r15d_end: u8;
    static cannoli_memhook_read_r15w_eax: u8;
    static cannoli_memhook_read_r15w_eax_end: u8;
    static cannoli_memhook_read_r15w_ecx: u8;
    static cannoli_memhook_read_r15w_ecx_end: u8;
    static cannoli_memhook_read_r15w_edx: u8;
    static cannoli_memhook_read_r15w_edx_end: u8;
    static cannoli_memhook_read_r15w_ebx: u8;
    static cannoli_memhook_read_r15w_ebx_end: u8;
    static cannoli_memhook_read_r15w_esp: u8;
    static cannoli_memhook_read_r15w_esp_end: u8;
    static cannoli_memhook_read_r15w_ebp: u8;
    static cannoli_memhook_read_r15w_ebp_end: u8;
    static cannoli_memhook_read_r15w_esi: u8;
    static cannoli_memhook_read_r15w_esi_end: u8;
    static cannoli_memhook_read_r15w_edi: u8;
    static cannoli_memhook_read_r15w_edi_end: u8;
    static cannoli_memhook_read_r15w_r8d: u8;
    static cannoli_memhook_read_r15w_r8d_end: u8;
    static cannoli_memhook_read_r15w_r9d: u8;
    static cannoli_memhook_read_r15w_r9d_end: u8;
    static cannoli_memhook_read_r15w_r10d: u8;
    static cannoli_memhook_read_r15w_r10d_end: u8;
    static cannoli_memhook_read_r15w_r11d: u8;
    static cannoli_memhook_read_r15w_r11d_end: u8;
    static cannoli_memhook_read_r15w_r12d: u8;
    static cannoli_memhook_read_r15w_r12d_end: u8;
    static cannoli_memhook_read_r15w_r13d: u8;
    static cannoli_memhook_read_r15w_r13d_end: u8;
    static cannoli_memhook_read_r15w_r14d: u8;
    static cannoli_memhook_read_r15w_r14d_end: u8;
    static cannoli_memhook_read_r15w_r15d: u8;
    static cannoli_memhook_read_r15w_r15d_end: u8;
    static cannoli_memhook_read_ax_rax: u8;
    static cannoli_memhook_read_ax_rax_end: u8;
    static cannoli_memhook_read_ax_rcx: u8;
    static cannoli_memhook_read_ax_rcx_end: u8;
    static cannoli_memhook_read_ax_rdx: u8;
    static cannoli_memhook_read_ax_rdx_end: u8;
    static cannoli_memhook_read_ax_rbx: u8;
    static cannoli_memhook_read_ax_rbx_end: u8;
    static cannoli_memhook_read_ax_rsp: u8;
    static cannoli_memhook_read_ax_rsp_end: u8;
    static cannoli_memhook_read_ax_rbp: u8;
    static cannoli_memhook_read_ax_rbp_end: u8;
    static cannoli_memhook_read_ax_rsi: u8;
    static cannoli_memhook_read_ax_rsi_end: u8;
    static cannoli_memhook_read_ax_rdi: u8;
    static cannoli_memhook_read_ax_rdi_end: u8;
    static cannoli_memhook_read_ax_r8: u8;
    static cannoli_memhook_read_ax_r8_end: u8;
    static cannoli_memhook_read_ax_r9: u8;
    static cannoli_memhook_read_ax_r9_end: u8;
    static cannoli_memhook_read_ax_r10: u8;
    static cannoli_memhook_read_ax_r10_end: u8;
    static cannoli_memhook_read_ax_r11: u8;
    static cannoli_memhook_read_ax_r11_end: u8;
    static cannoli_memhook_read_ax_r12: u8;
    static cannoli_memhook_read_ax_r12_end: u8;
    static cannoli_memhook_read_ax_r13: u8;
    static cannoli_memhook_read_ax_r13_end: u8;
    static cannoli_memhook_read_ax_r14: u8;
    static cannoli_memhook_read_ax_r14_end: u8;
    static cannoli_memhook_read_ax_r15: u8;
    static cannoli_memhook_read_ax_r15_end: u8;
    static cannoli_memhook_read_cx_rax: u8;
    static cannoli_memhook_read_cx_rax_end: u8;
    static cannoli_memhook_read_cx_rcx: u8;
    static cannoli_memhook_read_cx_rcx_end: u8;
    static cannoli_memhook_read_cx_rdx: u8;
    static cannoli_memhook_read_cx_rdx_end: u8;
    static cannoli_memhook_read_cx_rbx: u8;
    static cannoli_memhook_read_cx_rbx_end: u8;
    static cannoli_memhook_read_cx_rsp: u8;
    static cannoli_memhook_read_cx_rsp_end: u8;
    static cannoli_memhook_read_cx_rbp: u8;
    static cannoli_memhook_read_cx_rbp_end: u8;
    static cannoli_memhook_read_cx_rsi: u8;
    static cannoli_memhook_read_cx_rsi_end: u8;
    static cannoli_memhook_read_cx_rdi: u8;
    static cannoli_memhook_read_cx_rdi_end: u8;
    static cannoli_memhook_read_cx_r8: u8;
    static cannoli_memhook_read_cx_r8_end: u8;
    static cannoli_memhook_read_cx_r9: u8;
    static cannoli_memhook_read_cx_r9_end: u8;
    static cannoli_memhook_read_cx_r10: u8;
    static cannoli_memhook_read_cx_r10_end: u8;
    static cannoli_memhook_read_cx_r11: u8;
    static cannoli_memhook_read_cx_r11_end: u8;
    static cannoli_memhook_read_cx_r12: u8;
    static cannoli_memhook_read_cx_r12_end: u8;
    static cannoli_memhook_read_cx_r13: u8;
    static cannoli_memhook_read_cx_r13_end: u8;
    static cannoli_memhook_read_cx_r14: u8;
    static cannoli_memhook_read_cx_r14_end: u8;
    static cannoli_memhook_read_cx_r15: u8;
    static cannoli_memhook_read_cx_r15_end: u8;
    static cannoli_memhook_read_dx_rax: u8;
    static cannoli_memhook_read_dx_rax_end: u8;
    static cannoli_memhook_read_dx_rcx: u8;
    static cannoli_memhook_read_dx_rcx_end: u8;
    static cannoli_memhook_read_dx_rdx: u8;
    static cannoli_memhook_read_dx_rdx_end: u8;
    static cannoli_memhook_read_dx_rbx: u8;
    static cannoli_memhook_read_dx_rbx_end: u8;
    static cannoli_memhook_read_dx_rsp: u8;
    static cannoli_memhook_read_dx_rsp_end: u8;
    static cannoli_memhook_read_dx_rbp: u8;
    static cannoli_memhook_read_dx_rbp_end: u8;
    static cannoli_memhook_read_dx_rsi: u8;
    static cannoli_memhook_read_dx_rsi_end: u8;
    static cannoli_memhook_read_dx_rdi: u8;
    static cannoli_memhook_read_dx_rdi_end: u8;
    static cannoli_memhook_read_dx_r8: u8;
    static cannoli_memhook_read_dx_r8_end: u8;
    static cannoli_memhook_read_dx_r9: u8;
    static cannoli_memhook_read_dx_r9_end: u8;
    static cannoli_memhook_read_dx_r10: u8;
    static cannoli_memhook_read_dx_r10_end: u8;
    static cannoli_memhook_read_dx_r11: u8;
    static cannoli_memhook_read_dx_r11_end: u8;
    static cannoli_memhook_read_dx_r12: u8;
    static cannoli_memhook_read_dx_r12_end: u8;
    static cannoli_memhook_read_dx_r13: u8;
    static cannoli_memhook_read_dx_r13_end: u8;
    static cannoli_memhook_read_dx_r14: u8;
    static cannoli_memhook_read_dx_r14_end: u8;
    static cannoli_memhook_read_dx_r15: u8;
    static cannoli_memhook_read_dx_r15_end: u8;
    static cannoli_memhook_read_bx_rax: u8;
    static cannoli_memhook_read_bx_rax_end: u8;
    static cannoli_memhook_read_bx_rcx: u8;
    static cannoli_memhook_read_bx_rcx_end: u8;
    static cannoli_memhook_read_bx_rdx: u8;
    static cannoli_memhook_read_bx_rdx_end: u8;
    static cannoli_memhook_read_bx_rbx: u8;
    static cannoli_memhook_read_bx_rbx_end: u8;
    static cannoli_memhook_read_bx_rsp: u8;
    static cannoli_memhook_read_bx_rsp_end: u8;
    static cannoli_memhook_read_bx_rbp: u8;
    static cannoli_memhook_read_bx_rbp_end: u8;
    static cannoli_memhook_read_bx_rsi: u8;
    static cannoli_memhook_read_bx_rsi_end: u8;
    static cannoli_memhook_read_bx_rdi: u8;
    static cannoli_memhook_read_bx_rdi_end: u8;
    static cannoli_memhook_read_bx_r8: u8;
    static cannoli_memhook_read_bx_r8_end: u8;
    static cannoli_memhook_read_bx_r9: u8;
    static cannoli_memhook_read_bx_r9_end: u8;
    static cannoli_memhook_read_bx_r10: u8;
    static cannoli_memhook_read_bx_r10_end: u8;
    static cannoli_memhook_read_bx_r11: u8;
    static cannoli_memhook_read_bx_r11_end: u8;
    static cannoli_memhook_read_bx_r12: u8;
    static cannoli_memhook_read_bx_r12_end: u8;
    static cannoli_memhook_read_bx_r13: u8;
    static cannoli_memhook_read_bx_r13_end: u8;
    static cannoli_memhook_read_bx_r14: u8;
    static cannoli_memhook_read_bx_r14_end: u8;
    static cannoli_memhook_read_bx_r15: u8;
    static cannoli_memhook_read_bx_r15_end: u8;
    static cannoli_memhook_read_sp_rax: u8;
    static cannoli_memhook_read_sp_rax_end: u8;
    static cannoli_memhook_read_sp_rcx: u8;
    static cannoli_memhook_read_sp_rcx_end: u8;
    static cannoli_memhook_read_sp_rdx: u8;
    static cannoli_memhook_read_sp_rdx_end: u8;
    static cannoli_memhook_read_sp_rbx: u8;
    static cannoli_memhook_read_sp_rbx_end: u8;
    static cannoli_memhook_read_sp_rsp: u8;
    static cannoli_memhook_read_sp_rsp_end: u8;
    static cannoli_memhook_read_sp_rbp: u8;
    static cannoli_memhook_read_sp_rbp_end: u8;
    static cannoli_memhook_read_sp_rsi: u8;
    static cannoli_memhook_read_sp_rsi_end: u8;
    static cannoli_memhook_read_sp_rdi: u8;
    static cannoli_memhook_read_sp_rdi_end: u8;
    static cannoli_memhook_read_sp_r8: u8;
    static cannoli_memhook_read_sp_r8_end: u8;
    static cannoli_memhook_read_sp_r9: u8;
    static cannoli_memhook_read_sp_r9_end: u8;
    static cannoli_memhook_read_sp_r10: u8;
    static cannoli_memhook_read_sp_r10_end: u8;
    static cannoli_memhook_read_sp_r11: u8;
    static cannoli_memhook_read_sp_r11_end: u8;
    static cannoli_memhook_read_sp_r12: u8;
    static cannoli_memhook_read_sp_r12_end: u8;
    static cannoli_memhook_read_sp_r13: u8;
    static cannoli_memhook_read_sp_r13_end: u8;
    static cannoli_memhook_read_sp_r14: u8;
    static cannoli_memhook_read_sp_r14_end: u8;
    static cannoli_memhook_read_sp_r15: u8;
    static cannoli_memhook_read_sp_r15_end: u8;
    static cannoli_memhook_read_bp_rax: u8;
    static cannoli_memhook_read_bp_rax_end: u8;
    static cannoli_memhook_read_bp_rcx: u8;
    static cannoli_memhook_read_bp_rcx_end: u8;
    static cannoli_memhook_read_bp_rdx: u8;
    static cannoli_memhook_read_bp_rdx_end: u8;
    static cannoli_memhook_read_bp_rbx: u8;
    static cannoli_memhook_read_bp_rbx_end: u8;
    static cannoli_memhook_read_bp_rsp: u8;
    static cannoli_memhook_read_bp_rsp_end: u8;
    static cannoli_memhook_read_bp_rbp: u8;
    static cannoli_memhook_read_bp_rbp_end: u8;
    static cannoli_memhook_read_bp_rsi: u8;
    static cannoli_memhook_read_bp_rsi_end: u8;
    static cannoli_memhook_read_bp_rdi: u8;
    static cannoli_memhook_read_bp_rdi_end: u8;
    static cannoli_memhook_read_bp_r8: u8;
    static cannoli_memhook_read_bp_r8_end: u8;
    static cannoli_memhook_read_bp_r9: u8;
    static cannoli_memhook_read_bp_r9_end: u8;
    static cannoli_memhook_read_bp_r10: u8;
    static cannoli_memhook_read_bp_r10_end: u8;
    static cannoli_memhook_read_bp_r11: u8;
    static cannoli_memhook_read_bp_r11_end: u8;
    static cannoli_memhook_read_bp_r12: u8;
    static cannoli_memhook_read_bp_r12_end: u8;
    static cannoli_memhook_read_bp_r13: u8;
    static cannoli_memhook_read_bp_r13_end: u8;
    static cannoli_memhook_read_bp_r14: u8;
    static cannoli_memhook_read_bp_r14_end: u8;
    static cannoli_memhook_read_bp_r15: u8;
    static cannoli_memhook_read_bp_r15_end: u8;
    static cannoli_memhook_read_si_rax: u8;
    static cannoli_memhook_read_si_rax_end: u8;
    static cannoli_memhook_read_si_rcx: u8;
    static cannoli_memhook_read_si_rcx_end: u8;
    static cannoli_memhook_read_si_rdx: u8;
    static cannoli_memhook_read_si_rdx_end: u8;
    static cannoli_memhook_read_si_rbx: u8;
    static cannoli_memhook_read_si_rbx_end: u8;
    static cannoli_memhook_read_si_rsp: u8;
    static cannoli_memhook_read_si_rsp_end: u8;
    static cannoli_memhook_read_si_rbp: u8;
    static cannoli_memhook_read_si_rbp_end: u8;
    static cannoli_memhook_read_si_rsi: u8;
    static cannoli_memhook_read_si_rsi_end: u8;
    static cannoli_memhook_read_si_rdi: u8;
    static cannoli_memhook_read_si_rdi_end: u8;
    static cannoli_memhook_read_si_r8: u8;
    static cannoli_memhook_read_si_r8_end: u8;
    static cannoli_memhook_read_si_r9: u8;
    static cannoli_memhook_read_si_r9_end: u8;
    static cannoli_memhook_read_si_r10: u8;
    static cannoli_memhook_read_si_r10_end: u8;
    static cannoli_memhook_read_si_r11: u8;
    static cannoli_memhook_read_si_r11_end: u8;
    static cannoli_memhook_read_si_r12: u8;
    static cannoli_memhook_read_si_r12_end: u8;
    static cannoli_memhook_read_si_r13: u8;
    static cannoli_memhook_read_si_r13_end: u8;
    static cannoli_memhook_read_si_r14: u8;
    static cannoli_memhook_read_si_r14_end: u8;
    static cannoli_memhook_read_si_r15: u8;
    static cannoli_memhook_read_si_r15_end: u8;
    static cannoli_memhook_read_di_rax: u8;
    static cannoli_memhook_read_di_rax_end: u8;
    static cannoli_memhook_read_di_rcx: u8;
    static cannoli_memhook_read_di_rcx_end: u8;
    static cannoli_memhook_read_di_rdx: u8;
    static cannoli_memhook_read_di_rdx_end: u8;
    static cannoli_memhook_read_di_rbx: u8;
    static cannoli_memhook_read_di_rbx_end: u8;
    static cannoli_memhook_read_di_rsp: u8;
    static cannoli_memhook_read_di_rsp_end: u8;
    static cannoli_memhook_read_di_rbp: u8;
    static cannoli_memhook_read_di_rbp_end: u8;
    static cannoli_memhook_read_di_rsi: u8;
    static cannoli_memhook_read_di_rsi_end: u8;
    static cannoli_memhook_read_di_rdi: u8;
    static cannoli_memhook_read_di_rdi_end: u8;
    static cannoli_memhook_read_di_r8: u8;
    static cannoli_memhook_read_di_r8_end: u8;
    static cannoli_memhook_read_di_r9: u8;
    static cannoli_memhook_read_di_r9_end: u8;
    static cannoli_memhook_read_di_r10: u8;
    static cannoli_memhook_read_di_r10_end: u8;
    static cannoli_memhook_read_di_r11: u8;
    static cannoli_memhook_read_di_r11_end: u8;
    static cannoli_memhook_read_di_r12: u8;
    static cannoli_memhook_read_di_r12_end: u8;
    static cannoli_memhook_read_di_r13: u8;
    static cannoli_memhook_read_di_r13_end: u8;
    static cannoli_memhook_read_di_r14: u8;
    static cannoli_memhook_read_di_r14_end: u8;
    static cannoli_memhook_read_di_r15: u8;
    static cannoli_memhook_read_di_r15_end: u8;
    static cannoli_memhook_read_r8w_rax: u8;
    static cannoli_memhook_read_r8w_rax_end: u8;
    static cannoli_memhook_read_r8w_rcx: u8;
    static cannoli_memhook_read_r8w_rcx_end: u8;
    static cannoli_memhook_read_r8w_rdx: u8;
    static cannoli_memhook_read_r8w_rdx_end: u8;
    static cannoli_memhook_read_r8w_rbx: u8;
    static cannoli_memhook_read_r8w_rbx_end: u8;
    static cannoli_memhook_read_r8w_rsp: u8;
    static cannoli_memhook_read_r8w_rsp_end: u8;
    static cannoli_memhook_read_r8w_rbp: u8;
    static cannoli_memhook_read_r8w_rbp_end: u8;
    static cannoli_memhook_read_r8w_rsi: u8;
    static cannoli_memhook_read_r8w_rsi_end: u8;
    static cannoli_memhook_read_r8w_rdi: u8;
    static cannoli_memhook_read_r8w_rdi_end: u8;
    static cannoli_memhook_read_r8w_r8: u8;
    static cannoli_memhook_read_r8w_r8_end: u8;
    static cannoli_memhook_read_r8w_r9: u8;
    static cannoli_memhook_read_r8w_r9_end: u8;
    static cannoli_memhook_read_r8w_r10: u8;
    static cannoli_memhook_read_r8w_r10_end: u8;
    static cannoli_memhook_read_r8w_r11: u8;
    static cannoli_memhook_read_r8w_r11_end: u8;
    static cannoli_memhook_read_r8w_r12: u8;
    static cannoli_memhook_read_r8w_r12_end: u8;
    static cannoli_memhook_read_r8w_r13: u8;
    static cannoli_memhook_read_r8w_r13_end: u8;
    static cannoli_memhook_read_r8w_r14: u8;
    static cannoli_memhook_read_r8w_r14_end: u8;
    static cannoli_memhook_read_r8w_r15: u8;
    static cannoli_memhook_read_r8w_r15_end: u8;
    static cannoli_memhook_read_r9w_rax: u8;
    static cannoli_memhook_read_r9w_rax_end: u8;
    static cannoli_memhook_read_r9w_rcx: u8;
    static cannoli_memhook_read_r9w_rcx_end: u8;
    static cannoli_memhook_read_r9w_rdx: u8;
    static cannoli_memhook_read_r9w_rdx_end: u8;
    static cannoli_memhook_read_r9w_rbx: u8;
    static cannoli_memhook_read_r9w_rbx_end: u8;
    static cannoli_memhook_read_r9w_rsp: u8;
    static cannoli_memhook_read_r9w_rsp_end: u8;
    static cannoli_memhook_read_r9w_rbp: u8;
    static cannoli_memhook_read_r9w_rbp_end: u8;
    static cannoli_memhook_read_r9w_rsi: u8;
    static cannoli_memhook_read_r9w_rsi_end: u8;
    static cannoli_memhook_read_r9w_rdi: u8;
    static cannoli_memhook_read_r9w_rdi_end: u8;
    static cannoli_memhook_read_r9w_r8: u8;
    static cannoli_memhook_read_r9w_r8_end: u8;
    static cannoli_memhook_read_r9w_r9: u8;
    static cannoli_memhook_read_r9w_r9_end: u8;
    static cannoli_memhook_read_r9w_r10: u8;
    static cannoli_memhook_read_r9w_r10_end: u8;
    static cannoli_memhook_read_r9w_r11: u8;
    static cannoli_memhook_read_r9w_r11_end: u8;
    static cannoli_memhook_read_r9w_r12: u8;
    static cannoli_memhook_read_r9w_r12_end: u8;
    static cannoli_memhook_read_r9w_r13: u8;
    static cannoli_memhook_read_r9w_r13_end: u8;
    static cannoli_memhook_read_r9w_r14: u8;
    static cannoli_memhook_read_r9w_r14_end: u8;
    static cannoli_memhook_read_r9w_r15: u8;
    static cannoli_memhook_read_r9w_r15_end: u8;
    static cannoli_memhook_read_r10w_rax: u8;
    static cannoli_memhook_read_r10w_rax_end: u8;
    static cannoli_memhook_read_r10w_rcx: u8;
    static cannoli_memhook_read_r10w_rcx_end: u8;
    static cannoli_memhook_read_r10w_rdx: u8;
    static cannoli_memhook_read_r10w_rdx_end: u8;
    static cannoli_memhook_read_r10w_rbx: u8;
    static cannoli_memhook_read_r10w_rbx_end: u8;
    static cannoli_memhook_read_r10w_rsp: u8;
    static cannoli_memhook_read_r10w_rsp_end: u8;
    static cannoli_memhook_read_r10w_rbp: u8;
    static cannoli_memhook_read_r10w_rbp_end: u8;
    static cannoli_memhook_read_r10w_rsi: u8;
    static cannoli_memhook_read_r10w_rsi_end: u8;
    static cannoli_memhook_read_r10w_rdi: u8;
    static cannoli_memhook_read_r10w_rdi_end: u8;
    static cannoli_memhook_read_r10w_r8: u8;
    static cannoli_memhook_read_r10w_r8_end: u8;
    static cannoli_memhook_read_r10w_r9: u8;
    static cannoli_memhook_read_r10w_r9_end: u8;
    static cannoli_memhook_read_r10w_r10: u8;
    static cannoli_memhook_read_r10w_r10_end: u8;
    static cannoli_memhook_read_r10w_r11: u8;
    static cannoli_memhook_read_r10w_r11_end: u8;
    static cannoli_memhook_read_r10w_r12: u8;
    static cannoli_memhook_read_r10w_r12_end: u8;
    static cannoli_memhook_read_r10w_r13: u8;
    static cannoli_memhook_read_r10w_r13_end: u8;
    static cannoli_memhook_read_r10w_r14: u8;
    static cannoli_memhook_read_r10w_r14_end: u8;
    static cannoli_memhook_read_r10w_r15: u8;
    static cannoli_memhook_read_r10w_r15_end: u8;
    static cannoli_memhook_read_r11w_rax: u8;
    static cannoli_memhook_read_r11w_rax_end: u8;
    static cannoli_memhook_read_r11w_rcx: u8;
    static cannoli_memhook_read_r11w_rcx_end: u8;
    static cannoli_memhook_read_r11w_rdx: u8;
    static cannoli_memhook_read_r11w_rdx_end: u8;
    static cannoli_memhook_read_r11w_rbx: u8;
    static cannoli_memhook_read_r11w_rbx_end: u8;
    static cannoli_memhook_read_r11w_rsp: u8;
    static cannoli_memhook_read_r11w_rsp_end: u8;
    static cannoli_memhook_read_r11w_rbp: u8;
    static cannoli_memhook_read_r11w_rbp_end: u8;
    static cannoli_memhook_read_r11w_rsi: u8;
    static cannoli_memhook_read_r11w_rsi_end: u8;
    static cannoli_memhook_read_r11w_rdi: u8;
    static cannoli_memhook_read_r11w_rdi_end: u8;
    static cannoli_memhook_read_r11w_r8: u8;
    static cannoli_memhook_read_r11w_r8_end: u8;
    static cannoli_memhook_read_r11w_r9: u8;
    static cannoli_memhook_read_r11w_r9_end: u8;
    static cannoli_memhook_read_r11w_r10: u8;
    static cannoli_memhook_read_r11w_r10_end: u8;
    static cannoli_memhook_read_r11w_r11: u8;
    static cannoli_memhook_read_r11w_r11_end: u8;
    static cannoli_memhook_read_r11w_r12: u8;
    static cannoli_memhook_read_r11w_r12_end: u8;
    static cannoli_memhook_read_r11w_r13: u8;
    static cannoli_memhook_read_r11w_r13_end: u8;
    static cannoli_memhook_read_r11w_r14: u8;
    static cannoli_memhook_read_r11w_r14_end: u8;
    static cannoli_memhook_read_r11w_r15: u8;
    static cannoli_memhook_read_r11w_r15_end: u8;
    static cannoli_memhook_read_r12w_rax: u8;
    static cannoli_memhook_read_r12w_rax_end: u8;
    static cannoli_memhook_read_r12w_rcx: u8;
    static cannoli_memhook_read_r12w_rcx_end: u8;
    static cannoli_memhook_read_r12w_rdx: u8;
    static cannoli_memhook_read_r12w_rdx_end: u8;
    static cannoli_memhook_read_r12w_rbx: u8;
    static cannoli_memhook_read_r12w_rbx_end: u8;
    static cannoli_memhook_read_r12w_rsp: u8;
    static cannoli_memhook_read_r12w_rsp_end: u8;
    static cannoli_memhook_read_r12w_rbp: u8;
    static cannoli_memhook_read_r12w_rbp_end: u8;
    static cannoli_memhook_read_r12w_rsi: u8;
    static cannoli_memhook_read_r12w_rsi_end: u8;
    static cannoli_memhook_read_r12w_rdi: u8;
    static cannoli_memhook_read_r12w_rdi_end: u8;
    static cannoli_memhook_read_r12w_r8: u8;
    static cannoli_memhook_read_r12w_r8_end: u8;
    static cannoli_memhook_read_r12w_r9: u8;
    static cannoli_memhook_read_r12w_r9_end: u8;
    static cannoli_memhook_read_r12w_r10: u8;
    static cannoli_memhook_read_r12w_r10_end: u8;
    static cannoli_memhook_read_r12w_r11: u8;
    static cannoli_memhook_read_r12w_r11_end: u8;
    static cannoli_memhook_read_r12w_r12: u8;
    static cannoli_memhook_read_r12w_r12_end: u8;
    static cannoli_memhook_read_r12w_r13: u8;
    static cannoli_memhook_read_r12w_r13_end: u8;
    static cannoli_memhook_read_r12w_r14: u8;
    static cannoli_memhook_read_r12w_r14_end: u8;
    static cannoli_memhook_read_r12w_r15: u8;
    static cannoli_memhook_read_r12w_r15_end: u8;
    static cannoli_memhook_read_r13w_rax: u8;
    static cannoli_memhook_read_r13w_rax_end: u8;
    static cannoli_memhook_read_r13w_rcx: u8;
    static cannoli_memhook_read_r13w_rcx_end: u8;
    static cannoli_memhook_read_r13w_rdx: u8;
    static cannoli_memhook_read_r13w_rdx_end: u8;
    static cannoli_memhook_read_r13w_rbx: u8;
    static cannoli_memhook_read_r13w_rbx_end: u8;
    static cannoli_memhook_read_r13w_rsp: u8;
    static cannoli_memhook_read_r13w_rsp_end: u8;
    static cannoli_memhook_read_r13w_rbp: u8;
    static cannoli_memhook_read_r13w_rbp_end: u8;
    static cannoli_memhook_read_r13w_rsi: u8;
    static cannoli_memhook_read_r13w_rsi_end: u8;
    static cannoli_memhook_read_r13w_rdi: u8;
    static cannoli_memhook_read_r13w_rdi_end: u8;
    static cannoli_memhook_read_r13w_r8: u8;
    static cannoli_memhook_read_r13w_r8_end: u8;
    static cannoli_memhook_read_r13w_r9: u8;
    static cannoli_memhook_read_r13w_r9_end: u8;
    static cannoli_memhook_read_r13w_r10: u8;
    static cannoli_memhook_read_r13w_r10_end: u8;
    static cannoli_memhook_read_r13w_r11: u8;
    static cannoli_memhook_read_r13w_r11_end: u8;
    static cannoli_memhook_read_r13w_r12: u8;
    static cannoli_memhook_read_r13w_r12_end: u8;
    static cannoli_memhook_read_r13w_r13: u8;
    static cannoli_memhook_read_r13w_r13_end: u8;
    static cannoli_memhook_read_r13w_r14: u8;
    static cannoli_memhook_read_r13w_r14_end: u8;
    static cannoli_memhook_read_r13w_r15: u8;
    static cannoli_memhook_read_r13w_r15_end: u8;
    static cannoli_memhook_read_r14w_rax: u8;
    static cannoli_memhook_read_r14w_rax_end: u8;
    static cannoli_memhook_read_r14w_rcx: u8;
    static cannoli_memhook_read_r14w_rcx_end: u8;
    static cannoli_memhook_read_r14w_rdx: u8;
    static cannoli_memhook_read_r14w_rdx_end: u8;
    static cannoli_memhook_read_r14w_rbx: u8;
    static cannoli_memhook_read_r14w_rbx_end: u8;
    static cannoli_memhook_read_r14w_rsp: u8;
    static cannoli_memhook_read_r14w_rsp_end: u8;
    static cannoli_memhook_read_r14w_rbp: u8;
    static cannoli_memhook_read_r14w_rbp_end: u8;
    static cannoli_memhook_read_r14w_rsi: u8;
    static cannoli_memhook_read_r14w_rsi_end: u8;
    static cannoli_memhook_read_r14w_rdi: u8;
    static cannoli_memhook_read_r14w_rdi_end: u8;
    static cannoli_memhook_read_r14w_r8: u8;
    static cannoli_memhook_read_r14w_r8_end: u8;
    static cannoli_memhook_read_r14w_r9: u8;
    static cannoli_memhook_read_r14w_r9_end: u8;
    static cannoli_memhook_read_r14w_r10: u8;
    static cannoli_memhook_read_r14w_r10_end: u8;
    static cannoli_memhook_read_r14w_r11: u8;
    static cannoli_memhook_read_r14w_r11_end: u8;
    static cannoli_memhook_read_r14w_r12: u8;
    static cannoli_memhook_read_r14w_r12_end: u8;
    static cannoli_memhook_read_r14w_r13: u8;
    static cannoli_memhook_read_r14w_r13_end: u8;
    static cannoli_memhook_read_r14w_r14: u8;
    static cannoli_memhook_read_r14w_r14_end: u8;
    static cannoli_memhook_read_r14w_r15: u8;
    static cannoli_memhook_read_r14w_r15_end: u8;
    static cannoli_memhook_read_r15w_rax: u8;
    static cannoli_memhook_read_r15w_rax_end: u8;
    static cannoli_memhook_read_r15w_rcx: u8;
    static cannoli_memhook_read_r15w_rcx_end: u8;
    static cannoli_memhook_read_r15w_rdx: u8;
    static cannoli_memhook_read_r15w_rdx_end: u8;
    static cannoli_memhook_read_r15w_rbx: u8;
    static cannoli_memhook_read_r15w_rbx_end: u8;
    static cannoli_memhook_read_r15w_rsp: u8;
    static cannoli_memhook_read_r15w_rsp_end: u8;
    static cannoli_memhook_read_r15w_rbp: u8;
    static cannoli_memhook_read_r15w_rbp_end: u8;
    static cannoli_memhook_read_r15w_rsi: u8;
    static cannoli_memhook_read_r15w_rsi_end: u8;
    static cannoli_memhook_read_r15w_rdi: u8;
    static cannoli_memhook_read_r15w_rdi_end: u8;
    static cannoli_memhook_read_r15w_r8: u8;
    static cannoli_memhook_read_r15w_r8_end: u8;
    static cannoli_memhook_read_r15w_r9: u8;
    static cannoli_memhook_read_r15w_r9_end: u8;
    static cannoli_memhook_read_r15w_r10: u8;
    static cannoli_memhook_read_r15w_r10_end: u8;
    static cannoli_memhook_read_r15w_r11: u8;
    static cannoli_memhook_read_r15w_r11_end: u8;
    static cannoli_memhook_read_r15w_r12: u8;
    static cannoli_memhook_read_r15w_r12_end: u8;
    static cannoli_memhook_read_r15w_r13: u8;
    static cannoli_memhook_read_r15w_r13_end: u8;
    static cannoli_memhook_read_r15w_r14: u8;
    static cannoli_memhook_read_r15w_r14_end: u8;
    static cannoli_memhook_read_r15w_r15: u8;
    static cannoli_memhook_read_r15w_r15_end: u8;
    static cannoli_memhook_read_eax_eax: u8;
    static cannoli_memhook_read_eax_eax_end: u8;
    static cannoli_memhook_read_eax_ecx: u8;
    static cannoli_memhook_read_eax_ecx_end: u8;
    static cannoli_memhook_read_eax_edx: u8;
    static cannoli_memhook_read_eax_edx_end: u8;
    static cannoli_memhook_read_eax_ebx: u8;
    static cannoli_memhook_read_eax_ebx_end: u8;
    static cannoli_memhook_read_eax_esp: u8;
    static cannoli_memhook_read_eax_esp_end: u8;
    static cannoli_memhook_read_eax_ebp: u8;
    static cannoli_memhook_read_eax_ebp_end: u8;
    static cannoli_memhook_read_eax_esi: u8;
    static cannoli_memhook_read_eax_esi_end: u8;
    static cannoli_memhook_read_eax_edi: u8;
    static cannoli_memhook_read_eax_edi_end: u8;
    static cannoli_memhook_read_eax_r8d: u8;
    static cannoli_memhook_read_eax_r8d_end: u8;
    static cannoli_memhook_read_eax_r9d: u8;
    static cannoli_memhook_read_eax_r9d_end: u8;
    static cannoli_memhook_read_eax_r10d: u8;
    static cannoli_memhook_read_eax_r10d_end: u8;
    static cannoli_memhook_read_eax_r11d: u8;
    static cannoli_memhook_read_eax_r11d_end: u8;
    static cannoli_memhook_read_eax_r12d: u8;
    static cannoli_memhook_read_eax_r12d_end: u8;
    static cannoli_memhook_read_eax_r13d: u8;
    static cannoli_memhook_read_eax_r13d_end: u8;
    static cannoli_memhook_read_eax_r14d: u8;
    static cannoli_memhook_read_eax_r14d_end: u8;
    static cannoli_memhook_read_eax_r15d: u8;
    static cannoli_memhook_read_eax_r15d_end: u8;
    static cannoli_memhook_read_ecx_eax: u8;
    static cannoli_memhook_read_ecx_eax_end: u8;
    static cannoli_memhook_read_ecx_ecx: u8;
    static cannoli_memhook_read_ecx_ecx_end: u8;
    static cannoli_memhook_read_ecx_edx: u8;
    static cannoli_memhook_read_ecx_edx_end: u8;
    static cannoli_memhook_read_ecx_ebx: u8;
    static cannoli_memhook_read_ecx_ebx_end: u8;
    static cannoli_memhook_read_ecx_esp: u8;
    static cannoli_memhook_read_ecx_esp_end: u8;
    static cannoli_memhook_read_ecx_ebp: u8;
    static cannoli_memhook_read_ecx_ebp_end: u8;
    static cannoli_memhook_read_ecx_esi: u8;
    static cannoli_memhook_read_ecx_esi_end: u8;
    static cannoli_memhook_read_ecx_edi: u8;
    static cannoli_memhook_read_ecx_edi_end: u8;
    static cannoli_memhook_read_ecx_r8d: u8;
    static cannoli_memhook_read_ecx_r8d_end: u8;
    static cannoli_memhook_read_ecx_r9d: u8;
    static cannoli_memhook_read_ecx_r9d_end: u8;
    static cannoli_memhook_read_ecx_r10d: u8;
    static cannoli_memhook_read_ecx_r10d_end: u8;
    static cannoli_memhook_read_ecx_r11d: u8;
    static cannoli_memhook_read_ecx_r11d_end: u8;
    static cannoli_memhook_read_ecx_r12d: u8;
    static cannoli_memhook_read_ecx_r12d_end: u8;
    static cannoli_memhook_read_ecx_r13d: u8;
    static cannoli_memhook_read_ecx_r13d_end: u8;
    static cannoli_memhook_read_ecx_r14d: u8;
    static cannoli_memhook_read_ecx_r14d_end: u8;
    static cannoli_memhook_read_ecx_r15d: u8;
    static cannoli_memhook_read_ecx_r15d_end: u8;
    static cannoli_memhook_read_edx_eax: u8;
    static cannoli_memhook_read_edx_eax_end: u8;
    static cannoli_memhook_read_edx_ecx: u8;
    static cannoli_memhook_read_edx_ecx_end: u8;
    static cannoli_memhook_read_edx_edx: u8;
    static cannoli_memhook_read_edx_edx_end: u8;
    static cannoli_memhook_read_edx_ebx: u8;
    static cannoli_memhook_read_edx_ebx_end: u8;
    static cannoli_memhook_read_edx_esp: u8;
    static cannoli_memhook_read_edx_esp_end: u8;
    static cannoli_memhook_read_edx_ebp: u8;
    static cannoli_memhook_read_edx_ebp_end: u8;
    static cannoli_memhook_read_edx_esi: u8;
    static cannoli_memhook_read_edx_esi_end: u8;
    static cannoli_memhook_read_edx_edi: u8;
    static cannoli_memhook_read_edx_edi_end: u8;
    static cannoli_memhook_read_edx_r8d: u8;
    static cannoli_memhook_read_edx_r8d_end: u8;
    static cannoli_memhook_read_edx_r9d: u8;
    static cannoli_memhook_read_edx_r9d_end: u8;
    static cannoli_memhook_read_edx_r10d: u8;
    static cannoli_memhook_read_edx_r10d_end: u8;
    static cannoli_memhook_read_edx_r11d: u8;
    static cannoli_memhook_read_edx_r11d_end: u8;
    static cannoli_memhook_read_edx_r12d: u8;
    static cannoli_memhook_read_edx_r12d_end: u8;
    static cannoli_memhook_read_edx_r13d: u8;
    static cannoli_memhook_read_edx_r13d_end: u8;
    static cannoli_memhook_read_edx_r14d: u8;
    static cannoli_memhook_read_edx_r14d_end: u8;
    static cannoli_memhook_read_edx_r15d: u8;
    static cannoli_memhook_read_edx_r15d_end: u8;
    static cannoli_memhook_read_ebx_eax: u8;
    static cannoli_memhook_read_ebx_eax_end: u8;
    static cannoli_memhook_read_ebx_ecx: u8;
    static cannoli_memhook_read_ebx_ecx_end: u8;
    static cannoli_memhook_read_ebx_edx: u8;
    static cannoli_memhook_read_ebx_edx_end: u8;
    static cannoli_memhook_read_ebx_ebx: u8;
    static cannoli_memhook_read_ebx_ebx_end: u8;
    static cannoli_memhook_read_ebx_esp: u8;
    static cannoli_memhook_read_ebx_esp_end: u8;
    static cannoli_memhook_read_ebx_ebp: u8;
    static cannoli_memhook_read_ebx_ebp_end: u8;
    static cannoli_memhook_read_ebx_esi: u8;
    static cannoli_memhook_read_ebx_esi_end: u8;
    static cannoli_memhook_read_ebx_edi: u8;
    static cannoli_memhook_read_ebx_edi_end: u8;
    static cannoli_memhook_read_ebx_r8d: u8;
    static cannoli_memhook_read_ebx_r8d_end: u8;
    static cannoli_memhook_read_ebx_r9d: u8;
    static cannoli_memhook_read_ebx_r9d_end: u8;
    static cannoli_memhook_read_ebx_r10d: u8;
    static cannoli_memhook_read_ebx_r10d_end: u8;
    static cannoli_memhook_read_ebx_r11d: u8;
    static cannoli_memhook_read_ebx_r11d_end: u8;
    static cannoli_memhook_read_ebx_r12d: u8;
    static cannoli_memhook_read_ebx_r12d_end: u8;
    static cannoli_memhook_read_ebx_r13d: u8;
    static cannoli_memhook_read_ebx_r13d_end: u8;
    static cannoli_memhook_read_ebx_r14d: u8;
    static cannoli_memhook_read_ebx_r14d_end: u8;
    static cannoli_memhook_read_ebx_r15d: u8;
    static cannoli_memhook_read_ebx_r15d_end: u8;
    static cannoli_memhook_read_esp_eax: u8;
    static cannoli_memhook_read_esp_eax_end: u8;
    static cannoli_memhook_read_esp_ecx: u8;
    static cannoli_memhook_read_esp_ecx_end: u8;
    static cannoli_memhook_read_esp_edx: u8;
    static cannoli_memhook_read_esp_edx_end: u8;
    static cannoli_memhook_read_esp_ebx: u8;
    static cannoli_memhook_read_esp_ebx_end: u8;
    static cannoli_memhook_read_esp_esp: u8;
    static cannoli_memhook_read_esp_esp_end: u8;
    static cannoli_memhook_read_esp_ebp: u8;
    static cannoli_memhook_read_esp_ebp_end: u8;
    static cannoli_memhook_read_esp_esi: u8;
    static cannoli_memhook_read_esp_esi_end: u8;
    static cannoli_memhook_read_esp_edi: u8;
    static cannoli_memhook_read_esp_edi_end: u8;
    static cannoli_memhook_read_esp_r8d: u8;
    static cannoli_memhook_read_esp_r8d_end: u8;
    static cannoli_memhook_read_esp_r9d: u8;
    static cannoli_memhook_read_esp_r9d_end: u8;
    static cannoli_memhook_read_esp_r10d: u8;
    static cannoli_memhook_read_esp_r10d_end: u8;
    static cannoli_memhook_read_esp_r11d: u8;
    static cannoli_memhook_read_esp_r11d_end: u8;
    static cannoli_memhook_read_esp_r12d: u8;
    static cannoli_memhook_read_esp_r12d_end: u8;
    static cannoli_memhook_read_esp_r13d: u8;
    static cannoli_memhook_read_esp_r13d_end: u8;
    static cannoli_memhook_read_esp_r14d: u8;
    static cannoli_memhook_read_esp_r14d_end: u8;
    static cannoli_memhook_read_esp_r15d: u8;
    static cannoli_memhook_read_esp_r15d_end: u8;
    static cannoli_memhook_read_ebp_eax: u8;
    static cannoli_memhook_read_ebp_eax_end: u8;
    static cannoli_memhook_read_ebp_ecx: u8;
    static cannoli_memhook_read_ebp_ecx_end: u8;
    static cannoli_memhook_read_ebp_edx: u8;
    static cannoli_memhook_read_ebp_edx_end: u8;
    static cannoli_memhook_read_ebp_ebx: u8;
    static cannoli_memhook_read_ebp_ebx_end: u8;
    static cannoli_memhook_read_ebp_esp: u8;
    static cannoli_memhook_read_ebp_esp_end: u8;
    static cannoli_memhook_read_ebp_ebp: u8;
    static cannoli_memhook_read_ebp_ebp_end: u8;
    static cannoli_memhook_read_ebp_esi: u8;
    static cannoli_memhook_read_ebp_esi_end: u8;
    static cannoli_memhook_read_ebp_edi: u8;
    static cannoli_memhook_read_ebp_edi_end: u8;
    static cannoli_memhook_read_ebp_r8d: u8;
    static cannoli_memhook_read_ebp_r8d_end: u8;
    static cannoli_memhook_read_ebp_r9d: u8;
    static cannoli_memhook_read_ebp_r9d_end: u8;
    static cannoli_memhook_read_ebp_r10d: u8;
    static cannoli_memhook_read_ebp_r10d_end: u8;
    static cannoli_memhook_read_ebp_r11d: u8;
    static cannoli_memhook_read_ebp_r11d_end: u8;
    static cannoli_memhook_read_ebp_r12d: u8;
    static cannoli_memhook_read_ebp_r12d_end: u8;
    static cannoli_memhook_read_ebp_r13d: u8;
    static cannoli_memhook_read_ebp_r13d_end: u8;
    static cannoli_memhook_read_ebp_r14d: u8;
    static cannoli_memhook_read_ebp_r14d_end: u8;
    static cannoli_memhook_read_ebp_r15d: u8;
    static cannoli_memhook_read_ebp_r15d_end: u8;
    static cannoli_memhook_read_esi_eax: u8;
    static cannoli_memhook_read_esi_eax_end: u8;
    static cannoli_memhook_read_esi_ecx: u8;
    static cannoli_memhook_read_esi_ecx_end: u8;
    static cannoli_memhook_read_esi_edx: u8;
    static cannoli_memhook_read_esi_edx_end: u8;
    static cannoli_memhook_read_esi_ebx: u8;
    static cannoli_memhook_read_esi_ebx_end: u8;
    static cannoli_memhook_read_esi_esp: u8;
    static cannoli_memhook_read_esi_esp_end: u8;
    static cannoli_memhook_read_esi_ebp: u8;
    static cannoli_memhook_read_esi_ebp_end: u8;
    static cannoli_memhook_read_esi_esi: u8;
    static cannoli_memhook_read_esi_esi_end: u8;
    static cannoli_memhook_read_esi_edi: u8;
    static cannoli_memhook_read_esi_edi_end: u8;
    static cannoli_memhook_read_esi_r8d: u8;
    static cannoli_memhook_read_esi_r8d_end: u8;
    static cannoli_memhook_read_esi_r9d: u8;
    static cannoli_memhook_read_esi_r9d_end: u8;
    static cannoli_memhook_read_esi_r10d: u8;
    static cannoli_memhook_read_esi_r10d_end: u8;
    static cannoli_memhook_read_esi_r11d: u8;
    static cannoli_memhook_read_esi_r11d_end: u8;
    static cannoli_memhook_read_esi_r12d: u8;
    static cannoli_memhook_read_esi_r12d_end: u8;
    static cannoli_memhook_read_esi_r13d: u8;
    static cannoli_memhook_read_esi_r13d_end: u8;
    static cannoli_memhook_read_esi_r14d: u8;
    static cannoli_memhook_read_esi_r14d_end: u8;
    static cannoli_memhook_read_esi_r15d: u8;
    static cannoli_memhook_read_esi_r15d_end: u8;
    static cannoli_memhook_read_edi_eax: u8;
    static cannoli_memhook_read_edi_eax_end: u8;
    static cannoli_memhook_read_edi_ecx: u8;
    static cannoli_memhook_read_edi_ecx_end: u8;
    static cannoli_memhook_read_edi_edx: u8;
    static cannoli_memhook_read_edi_edx_end: u8;
    static cannoli_memhook_read_edi_ebx: u8;
    static cannoli_memhook_read_edi_ebx_end: u8;
    static cannoli_memhook_read_edi_esp: u8;
    static cannoli_memhook_read_edi_esp_end: u8;
    static cannoli_memhook_read_edi_ebp: u8;
    static cannoli_memhook_read_edi_ebp_end: u8;
    static cannoli_memhook_read_edi_esi: u8;
    static cannoli_memhook_read_edi_esi_end: u8;
    static cannoli_memhook_read_edi_edi: u8;
    static cannoli_memhook_read_edi_edi_end: u8;
    static cannoli_memhook_read_edi_r8d: u8;
    static cannoli_memhook_read_edi_r8d_end: u8;
    static cannoli_memhook_read_edi_r9d: u8;
    static cannoli_memhook_read_edi_r9d_end: u8;
    static cannoli_memhook_read_edi_r10d: u8;
    static cannoli_memhook_read_edi_r10d_end: u8;
    static cannoli_memhook_read_edi_r11d: u8;
    static cannoli_memhook_read_edi_r11d_end: u8;
    static cannoli_memhook_read_edi_r12d: u8;
    static cannoli_memhook_read_edi_r12d_end: u8;
    static cannoli_memhook_read_edi_r13d: u8;
    static cannoli_memhook_read_edi_r13d_end: u8;
    static cannoli_memhook_read_edi_r14d: u8;
    static cannoli_memhook_read_edi_r14d_end: u8;
    static cannoli_memhook_read_edi_r15d: u8;
    static cannoli_memhook_read_edi_r15d_end: u8;
    static cannoli_memhook_read_r8d_eax: u8;
    static cannoli_memhook_read_r8d_eax_end: u8;
    static cannoli_memhook_read_r8d_ecx: u8;
    static cannoli_memhook_read_r8d_ecx_end: u8;
    static cannoli_memhook_read_r8d_edx: u8;
    static cannoli_memhook_read_r8d_edx_end: u8;
    static cannoli_memhook_read_r8d_ebx: u8;
    static cannoli_memhook_read_r8d_ebx_end: u8;
    static cannoli_memhook_read_r8d_esp: u8;
    static cannoli_memhook_read_r8d_esp_end: u8;
    static cannoli_memhook_read_r8d_ebp: u8;
    static cannoli_memhook_read_r8d_ebp_end: u8;
    static cannoli_memhook_read_r8d_esi: u8;
    static cannoli_memhook_read_r8d_esi_end: u8;
    static cannoli_memhook_read_r8d_edi: u8;
    static cannoli_memhook_read_r8d_edi_end: u8;
    static cannoli_memhook_read_r8d_r8d: u8;
    static cannoli_memhook_read_r8d_r8d_end: u8;
    static cannoli_memhook_read_r8d_r9d: u8;
    static cannoli_memhook_read_r8d_r9d_end: u8;
    static cannoli_memhook_read_r8d_r10d: u8;
    static cannoli_memhook_read_r8d_r10d_end: u8;
    static cannoli_memhook_read_r8d_r11d: u8;
    static cannoli_memhook_read_r8d_r11d_end: u8;
    static cannoli_memhook_read_r8d_r12d: u8;
    static cannoli_memhook_read_r8d_r12d_end: u8;
    static cannoli_memhook_read_r8d_r13d: u8;
    static cannoli_memhook_read_r8d_r13d_end: u8;
    static cannoli_memhook_read_r8d_r14d: u8;
    static cannoli_memhook_read_r8d_r14d_end: u8;
    static cannoli_memhook_read_r8d_r15d: u8;
    static cannoli_memhook_read_r8d_r15d_end: u8;
    static cannoli_memhook_read_r9d_eax: u8;
    static cannoli_memhook_read_r9d_eax_end: u8;
    static cannoli_memhook_read_r9d_ecx: u8;
    static cannoli_memhook_read_r9d_ecx_end: u8;
    static cannoli_memhook_read_r9d_edx: u8;
    static cannoli_memhook_read_r9d_edx_end: u8;
    static cannoli_memhook_read_r9d_ebx: u8;
    static cannoli_memhook_read_r9d_ebx_end: u8;
    static cannoli_memhook_read_r9d_esp: u8;
    static cannoli_memhook_read_r9d_esp_end: u8;
    static cannoli_memhook_read_r9d_ebp: u8;
    static cannoli_memhook_read_r9d_ebp_end: u8;
    static cannoli_memhook_read_r9d_esi: u8;
    static cannoli_memhook_read_r9d_esi_end: u8;
    static cannoli_memhook_read_r9d_edi: u8;
    static cannoli_memhook_read_r9d_edi_end: u8;
    static cannoli_memhook_read_r9d_r8d: u8;
    static cannoli_memhook_read_r9d_r8d_end: u8;
    static cannoli_memhook_read_r9d_r9d: u8;
    static cannoli_memhook_read_r9d_r9d_end: u8;
    static cannoli_memhook_read_r9d_r10d: u8;
    static cannoli_memhook_read_r9d_r10d_end: u8;
    static cannoli_memhook_read_r9d_r11d: u8;
    static cannoli_memhook_read_r9d_r11d_end: u8;
    static cannoli_memhook_read_r9d_r12d: u8;
    static cannoli_memhook_read_r9d_r12d_end: u8;
    static cannoli_memhook_read_r9d_r13d: u8;
    static cannoli_memhook_read_r9d_r13d_end: u8;
    static cannoli_memhook_read_r9d_r14d: u8;
    static cannoli_memhook_read_r9d_r14d_end: u8;
    static cannoli_memhook_read_r9d_r15d: u8;
    static cannoli_memhook_read_r9d_r15d_end: u8;
    static cannoli_memhook_read_r10d_eax: u8;
    static cannoli_memhook_read_r10d_eax_end: u8;
    static cannoli_memhook_read_r10d_ecx: u8;
    static cannoli_memhook_read_r10d_ecx_end: u8;
    static cannoli_memhook_read_r10d_edx: u8;
    static cannoli_memhook_read_r10d_edx_end: u8;
    static cannoli_memhook_read_r10d_ebx: u8;
    static cannoli_memhook_read_r10d_ebx_end: u8;
    static cannoli_memhook_read_r10d_esp: u8;
    static cannoli_memhook_read_r10d_esp_end: u8;
    static cannoli_memhook_read_r10d_ebp: u8;
    static cannoli_memhook_read_r10d_ebp_end: u8;
    static cannoli_memhook_read_r10d_esi: u8;
    static cannoli_memhook_read_r10d_esi_end: u8;
    static cannoli_memhook_read_r10d_edi: u8;
    static cannoli_memhook_read_r10d_edi_end: u8;
    static cannoli_memhook_read_r10d_r8d: u8;
    static cannoli_memhook_read_r10d_r8d_end: u8;
    static cannoli_memhook_read_r10d_r9d: u8;
    static cannoli_memhook_read_r10d_r9d_end: u8;
    static cannoli_memhook_read_r10d_r10d: u8;
    static cannoli_memhook_read_r10d_r10d_end: u8;
    static cannoli_memhook_read_r10d_r11d: u8;
    static cannoli_memhook_read_r10d_r11d_end: u8;
    static cannoli_memhook_read_r10d_r12d: u8;
    static cannoli_memhook_read_r10d_r12d_end: u8;
    static cannoli_memhook_read_r10d_r13d: u8;
    static cannoli_memhook_read_r10d_r13d_end: u8;
    static cannoli_memhook_read_r10d_r14d: u8;
    static cannoli_memhook_read_r10d_r14d_end: u8;
    static cannoli_memhook_read_r10d_r15d: u8;
    static cannoli_memhook_read_r10d_r15d_end: u8;
    static cannoli_memhook_read_r11d_eax: u8;
    static cannoli_memhook_read_r11d_eax_end: u8;
    static cannoli_memhook_read_r11d_ecx: u8;
    static cannoli_memhook_read_r11d_ecx_end: u8;
    static cannoli_memhook_read_r11d_edx: u8;
    static cannoli_memhook_read_r11d_edx_end: u8;
    static cannoli_memhook_read_r11d_ebx: u8;
    static cannoli_memhook_read_r11d_ebx_end: u8;
    static cannoli_memhook_read_r11d_esp: u8;
    static cannoli_memhook_read_r11d_esp_end: u8;
    static cannoli_memhook_read_r11d_ebp: u8;
    static cannoli_memhook_read_r11d_ebp_end: u8;
    static cannoli_memhook_read_r11d_esi: u8;
    static cannoli_memhook_read_r11d_esi_end: u8;
    static cannoli_memhook_read_r11d_edi: u8;
    static cannoli_memhook_read_r11d_edi_end: u8;
    static cannoli_memhook_read_r11d_r8d: u8;
    static cannoli_memhook_read_r11d_r8d_end: u8;
    static cannoli_memhook_read_r11d_r9d: u8;
    static cannoli_memhook_read_r11d_r9d_end: u8;
    static cannoli_memhook_read_r11d_r10d: u8;
    static cannoli_memhook_read_r11d_r10d_end: u8;
    static cannoli_memhook_read_r11d_r11d: u8;
    static cannoli_memhook_read_r11d_r11d_end: u8;
    static cannoli_memhook_read_r11d_r12d: u8;
    static cannoli_memhook_read_r11d_r12d_end: u8;
    static cannoli_memhook_read_r11d_r13d: u8;
    static cannoli_memhook_read_r11d_r13d_end: u8;
    static cannoli_memhook_read_r11d_r14d: u8;
    static cannoli_memhook_read_r11d_r14d_end: u8;
    static cannoli_memhook_read_r11d_r15d: u8;
    static cannoli_memhook_read_r11d_r15d_end: u8;
    static cannoli_memhook_read_r12d_eax: u8;
    static cannoli_memhook_read_r12d_eax_end: u8;
    static cannoli_memhook_read_r12d_ecx: u8;
    static cannoli_memhook_read_r12d_ecx_end: u8;
    static cannoli_memhook_read_r12d_edx: u8;
    static cannoli_memhook_read_r12d_edx_end: u8;
    static cannoli_memhook_read_r12d_ebx: u8;
    static cannoli_memhook_read_r12d_ebx_end: u8;
    static cannoli_memhook_read_r12d_esp: u8;
    static cannoli_memhook_read_r12d_esp_end: u8;
    static cannoli_memhook_read_r12d_ebp: u8;
    static cannoli_memhook_read_r12d_ebp_end: u8;
    static cannoli_memhook_read_r12d_esi: u8;
    static cannoli_memhook_read_r12d_esi_end: u8;
    static cannoli_memhook_read_r12d_edi: u8;
    static cannoli_memhook_read_r12d_edi_end: u8;
    static cannoli_memhook_read_r12d_r8d: u8;
    static cannoli_memhook_read_r12d_r8d_end: u8;
    static cannoli_memhook_read_r12d_r9d: u8;
    static cannoli_memhook_read_r12d_r9d_end: u8;
    static cannoli_memhook_read_r12d_r10d: u8;
    static cannoli_memhook_read_r12d_r10d_end: u8;
    static cannoli_memhook_read_r12d_r11d: u8;
    static cannoli_memhook_read_r12d_r11d_end: u8;
    static cannoli_memhook_read_r12d_r12d: u8;
    static cannoli_memhook_read_r12d_r12d_end: u8;
    static cannoli_memhook_read_r12d_r13d: u8;
    static cannoli_memhook_read_r12d_r13d_end: u8;
    static cannoli_memhook_read_r12d_r14d: u8;
    static cannoli_memhook_read_r12d_r14d_end: u8;
    static cannoli_memhook_read_r12d_r15d: u8;
    static cannoli_memhook_read_r12d_r15d_end: u8;
    static cannoli_memhook_read_r13d_eax: u8;
    static cannoli_memhook_read_r13d_eax_end: u8;
    static cannoli_memhook_read_r13d_ecx: u8;
    static cannoli_memhook_read_r13d_ecx_end: u8;
    static cannoli_memhook_read_r13d_edx: u8;
    static cannoli_memhook_read_r13d_edx_end: u8;
    static cannoli_memhook_read_r13d_ebx: u8;
    static cannoli_memhook_read_r13d_ebx_end: u8;
    static cannoli_memhook_read_r13d_esp: u8;
    static cannoli_memhook_read_r13d_esp_end: u8;
    static cannoli_memhook_read_r13d_ebp: u8;
    static cannoli_memhook_read_r13d_ebp_end: u8;
    static cannoli_memhook_read_r13d_esi: u8;
    static cannoli_memhook_read_r13d_esi_end: u8;
    static cannoli_memhook_read_r13d_edi: u8;
    static cannoli_memhook_read_r13d_edi_end: u8;
    static cannoli_memhook_read_r13d_r8d: u8;
    static cannoli_memhook_read_r13d_r8d_end: u8;
    static cannoli_memhook_read_r13d_r9d: u8;
    static cannoli_memhook_read_r13d_r9d_end: u8;
    static cannoli_memhook_read_r13d_r10d: u8;
    static cannoli_memhook_read_r13d_r10d_end: u8;
    static cannoli_memhook_read_r13d_r11d: u8;
    static cannoli_memhook_read_r13d_r11d_end: u8;
    static cannoli_memhook_read_r13d_r12d: u8;
    static cannoli_memhook_read_r13d_r12d_end: u8;
    static cannoli_memhook_read_r13d_r13d: u8;
    static cannoli_memhook_read_r13d_r13d_end: u8;
    static cannoli_memhook_read_r13d_r14d: u8;
    static cannoli_memhook_read_r13d_r14d_end: u8;
    static cannoli_memhook_read_r13d_r15d: u8;
    static cannoli_memhook_read_r13d_r15d_end: u8;
    static cannoli_memhook_read_r14d_eax: u8;
    static cannoli_memhook_read_r14d_eax_end: u8;
    static cannoli_memhook_read_r14d_ecx: u8;
    static cannoli_memhook_read_r14d_ecx_end: u8;
    static cannoli_memhook_read_r14d_edx: u8;
    static cannoli_memhook_read_r14d_edx_end: u8;
    static cannoli_memhook_read_r14d_ebx: u8;
    static cannoli_memhook_read_r14d_ebx_end: u8;
    static cannoli_memhook_read_r14d_esp: u8;
    static cannoli_memhook_read_r14d_esp_end: u8;
    static cannoli_memhook_read_r14d_ebp: u8;
    static cannoli_memhook_read_r14d_ebp_end: u8;
    static cannoli_memhook_read_r14d_esi: u8;
    static cannoli_memhook_read_r14d_esi_end: u8;
    static cannoli_memhook_read_r14d_edi: u8;
    static cannoli_memhook_read_r14d_edi_end: u8;
    static cannoli_memhook_read_r14d_r8d: u8;
    static cannoli_memhook_read_r14d_r8d_end: u8;
    static cannoli_memhook_read_r14d_r9d: u8;
    static cannoli_memhook_read_r14d_r9d_end: u8;
    static cannoli_memhook_read_r14d_r10d: u8;
    static cannoli_memhook_read_r14d_r10d_end: u8;
    static cannoli_memhook_read_r14d_r11d: u8;
    static cannoli_memhook_read_r14d_r11d_end: u8;
    static cannoli_memhook_read_r14d_r12d: u8;
    static cannoli_memhook_read_r14d_r12d_end: u8;
    static cannoli_memhook_read_r14d_r13d: u8;
    static cannoli_memhook_read_r14d_r13d_end: u8;
    static cannoli_memhook_read_r14d_r14d: u8;
    static cannoli_memhook_read_r14d_r14d_end: u8;
    static cannoli_memhook_read_r14d_r15d: u8;
    static cannoli_memhook_read_r14d_r15d_end: u8;
    static cannoli_memhook_read_r15d_eax: u8;
    static cannoli_memhook_read_r15d_eax_end: u8;
    static cannoli_memhook_read_r15d_ecx: u8;
    static cannoli_memhook_read_r15d_ecx_end: u8;
    static cannoli_memhook_read_r15d_edx: u8;
    static cannoli_memhook_read_r15d_edx_end: u8;
    static cannoli_memhook_read_r15d_ebx: u8;
    static cannoli_memhook_read_r15d_ebx_end: u8;
    static cannoli_memhook_read_r15d_esp: u8;
    static cannoli_memhook_read_r15d_esp_end: u8;
    static cannoli_memhook_read_r15d_ebp: u8;
    static cannoli_memhook_read_r15d_ebp_end: u8;
    static cannoli_memhook_read_r15d_esi: u8;
    static cannoli_memhook_read_r15d_esi_end: u8;
    static cannoli_memhook_read_r15d_edi: u8;
    static cannoli_memhook_read_r15d_edi_end: u8;
    static cannoli_memhook_read_r15d_r8d: u8;
    static cannoli_memhook_read_r15d_r8d_end: u8;
    static cannoli_memhook_read_r15d_r9d: u8;
    static cannoli_memhook_read_r15d_r9d_end: u8;
    static cannoli_memhook_read_r15d_r10d: u8;
    static cannoli_memhook_read_r15d_r10d_end: u8;
    static cannoli_memhook_read_r15d_r11d: u8;
    static cannoli_memhook_read_r15d_r11d_end: u8;
    static cannoli_memhook_read_r15d_r12d: u8;
    static cannoli_memhook_read_r15d_r12d_end: u8;
    static cannoli_memhook_read_r15d_r13d: u8;
    static cannoli_memhook_read_r15d_r13d_end: u8;
    static cannoli_memhook_read_r15d_r14d: u8;
    static cannoli_memhook_read_r15d_r14d_end: u8;
    static cannoli_memhook_read_r15d_r15d: u8;
    static cannoli_memhook_read_r15d_r15d_end: u8;
    static cannoli_memhook_read_eax_rax: u8;
    static cannoli_memhook_read_eax_rax_end: u8;
    static cannoli_memhook_read_eax_rcx: u8;
    static cannoli_memhook_read_eax_rcx_end: u8;
    static cannoli_memhook_read_eax_rdx: u8;
    static cannoli_memhook_read_eax_rdx_end: u8;
    static cannoli_memhook_read_eax_rbx: u8;
    static cannoli_memhook_read_eax_rbx_end: u8;
    static cannoli_memhook_read_eax_rsp: u8;
    static cannoli_memhook_read_eax_rsp_end: u8;
    static cannoli_memhook_read_eax_rbp: u8;
    static cannoli_memhook_read_eax_rbp_end: u8;
    static cannoli_memhook_read_eax_rsi: u8;
    static cannoli_memhook_read_eax_rsi_end: u8;
    static cannoli_memhook_read_eax_rdi: u8;
    static cannoli_memhook_read_eax_rdi_end: u8;
    static cannoli_memhook_read_eax_r8: u8;
    static cannoli_memhook_read_eax_r8_end: u8;
    static cannoli_memhook_read_eax_r9: u8;
    static cannoli_memhook_read_eax_r9_end: u8;
    static cannoli_memhook_read_eax_r10: u8;
    static cannoli_memhook_read_eax_r10_end: u8;
    static cannoli_memhook_read_eax_r11: u8;
    static cannoli_memhook_read_eax_r11_end: u8;
    static cannoli_memhook_read_eax_r12: u8;
    static cannoli_memhook_read_eax_r12_end: u8;
    static cannoli_memhook_read_eax_r13: u8;
    static cannoli_memhook_read_eax_r13_end: u8;
    static cannoli_memhook_read_eax_r14: u8;
    static cannoli_memhook_read_eax_r14_end: u8;
    static cannoli_memhook_read_eax_r15: u8;
    static cannoli_memhook_read_eax_r15_end: u8;
    static cannoli_memhook_read_ecx_rax: u8;
    static cannoli_memhook_read_ecx_rax_end: u8;
    static cannoli_memhook_read_ecx_rcx: u8;
    static cannoli_memhook_read_ecx_rcx_end: u8;
    static cannoli_memhook_read_ecx_rdx: u8;
    static cannoli_memhook_read_ecx_rdx_end: u8;
    static cannoli_memhook_read_ecx_rbx: u8;
    static cannoli_memhook_read_ecx_rbx_end: u8;
    static cannoli_memhook_read_ecx_rsp: u8;
    static cannoli_memhook_read_ecx_rsp_end: u8;
    static cannoli_memhook_read_ecx_rbp: u8;
    static cannoli_memhook_read_ecx_rbp_end: u8;
    static cannoli_memhook_read_ecx_rsi: u8;
    static cannoli_memhook_read_ecx_rsi_end: u8;
    static cannoli_memhook_read_ecx_rdi: u8;
    static cannoli_memhook_read_ecx_rdi_end: u8;
    static cannoli_memhook_read_ecx_r8: u8;
    static cannoli_memhook_read_ecx_r8_end: u8;
    static cannoli_memhook_read_ecx_r9: u8;
    static cannoli_memhook_read_ecx_r9_end: u8;
    static cannoli_memhook_read_ecx_r10: u8;
    static cannoli_memhook_read_ecx_r10_end: u8;
    static cannoli_memhook_read_ecx_r11: u8;
    static cannoli_memhook_read_ecx_r11_end: u8;
    static cannoli_memhook_read_ecx_r12: u8;
    static cannoli_memhook_read_ecx_r12_end: u8;
    static cannoli_memhook_read_ecx_r13: u8;
    static cannoli_memhook_read_ecx_r13_end: u8;
    static cannoli_memhook_read_ecx_r14: u8;
    static cannoli_memhook_read_ecx_r14_end: u8;
    static cannoli_memhook_read_ecx_r15: u8;
    static cannoli_memhook_read_ecx_r15_end: u8;
    static cannoli_memhook_read_edx_rax: u8;
    static cannoli_memhook_read_edx_rax_end: u8;
    static cannoli_memhook_read_edx_rcx: u8;
    static cannoli_memhook_read_edx_rcx_end: u8;
    static cannoli_memhook_read_edx_rdx: u8;
    static cannoli_memhook_read_edx_rdx_end: u8;
    static cannoli_memhook_read_edx_rbx: u8;
    static cannoli_memhook_read_edx_rbx_end: u8;
    static cannoli_memhook_read_edx_rsp: u8;
    static cannoli_memhook_read_edx_rsp_end: u8;
    static cannoli_memhook_read_edx_rbp: u8;
    static cannoli_memhook_read_edx_rbp_end: u8;
    static cannoli_memhook_read_edx_rsi: u8;
    static cannoli_memhook_read_edx_rsi_end: u8;
    static cannoli_memhook_read_edx_rdi: u8;
    static cannoli_memhook_read_edx_rdi_end: u8;
    static cannoli_memhook_read_edx_r8: u8;
    static cannoli_memhook_read_edx_r8_end: u8;
    static cannoli_memhook_read_edx_r9: u8;
    static cannoli_memhook_read_edx_r9_end: u8;
    static cannoli_memhook_read_edx_r10: u8;
    static cannoli_memhook_read_edx_r10_end: u8;
    static cannoli_memhook_read_edx_r11: u8;
    static cannoli_memhook_read_edx_r11_end: u8;
    static cannoli_memhook_read_edx_r12: u8;
    static cannoli_memhook_read_edx_r12_end: u8;
    static cannoli_memhook_read_edx_r13: u8;
    static cannoli_memhook_read_edx_r13_end: u8;
    static cannoli_memhook_read_edx_r14: u8;
    static cannoli_memhook_read_edx_r14_end: u8;
    static cannoli_memhook_read_edx_r15: u8;
    static cannoli_memhook_read_edx_r15_end: u8;
    static cannoli_memhook_read_ebx_rax: u8;
    static cannoli_memhook_read_ebx_rax_end: u8;
    static cannoli_memhook_read_ebx_rcx: u8;
    static cannoli_memhook_read_ebx_rcx_end: u8;
    static cannoli_memhook_read_ebx_rdx: u8;
    static cannoli_memhook_read_ebx_rdx_end: u8;
    static cannoli_memhook_read_ebx_rbx: u8;
    static cannoli_memhook_read_ebx_rbx_end: u8;
    static cannoli_memhook_read_ebx_rsp: u8;
    static cannoli_memhook_read_ebx_rsp_end: u8;
    static cannoli_memhook_read_ebx_rbp: u8;
    static cannoli_memhook_read_ebx_rbp_end: u8;
    static cannoli_memhook_read_ebx_rsi: u8;
    static cannoli_memhook_read_ebx_rsi_end: u8;
    static cannoli_memhook_read_ebx_rdi: u8;
    static cannoli_memhook_read_ebx_rdi_end: u8;
    static cannoli_memhook_read_ebx_r8: u8;
    static cannoli_memhook_read_ebx_r8_end: u8;
    static cannoli_memhook_read_ebx_r9: u8;
    static cannoli_memhook_read_ebx_r9_end: u8;
    static cannoli_memhook_read_ebx_r10: u8;
    static cannoli_memhook_read_ebx_r10_end: u8;
    static cannoli_memhook_read_ebx_r11: u8;
    static cannoli_memhook_read_ebx_r11_end: u8;
    static cannoli_memhook_read_ebx_r12: u8;
    static cannoli_memhook_read_ebx_r12_end: u8;
    static cannoli_memhook_read_ebx_r13: u8;
    static cannoli_memhook_read_ebx_r13_end: u8;
    static cannoli_memhook_read_ebx_r14: u8;
    static cannoli_memhook_read_ebx_r14_end: u8;
    static cannoli_memhook_read_ebx_r15: u8;
    static cannoli_memhook_read_ebx_r15_end: u8;
    static cannoli_memhook_read_esp_rax: u8;
    static cannoli_memhook_read_esp_rax_end: u8;
    static cannoli_memhook_read_esp_rcx: u8;
    static cannoli_memhook_read_esp_rcx_end: u8;
    static cannoli_memhook_read_esp_rdx: u8;
    static cannoli_memhook_read_esp_rdx_end: u8;
    static cannoli_memhook_read_esp_rbx: u8;
    static cannoli_memhook_read_esp_rbx_end: u8;
    static cannoli_memhook_read_esp_rsp: u8;
    static cannoli_memhook_read_esp_rsp_end: u8;
    static cannoli_memhook_read_esp_rbp: u8;
    static cannoli_memhook_read_esp_rbp_end: u8;
    static cannoli_memhook_read_esp_rsi: u8;
    static cannoli_memhook_read_esp_rsi_end: u8;
    static cannoli_memhook_read_esp_rdi: u8;
    static cannoli_memhook_read_esp_rdi_end: u8;
    static cannoli_memhook_read_esp_r8: u8;
    static cannoli_memhook_read_esp_r8_end: u8;
    static cannoli_memhook_read_esp_r9: u8;
    static cannoli_memhook_read_esp_r9_end: u8;
    static cannoli_memhook_read_esp_r10: u8;
    static cannoli_memhook_read_esp_r10_end: u8;
    static cannoli_memhook_read_esp_r11: u8;
    static cannoli_memhook_read_esp_r11_end: u8;
    static cannoli_memhook_read_esp_r12: u8;
    static cannoli_memhook_read_esp_r12_end: u8;
    static cannoli_memhook_read_esp_r13: u8;
    static cannoli_memhook_read_esp_r13_end: u8;
    static cannoli_memhook_read_esp_r14: u8;
    static cannoli_memhook_read_esp_r14_end: u8;
    static cannoli_memhook_read_esp_r15: u8;
    static cannoli_memhook_read_esp_r15_end: u8;
    static cannoli_memhook_read_ebp_rax: u8;
    static cannoli_memhook_read_ebp_rax_end: u8;
    static cannoli_memhook_read_ebp_rcx: u8;
    static cannoli_memhook_read_ebp_rcx_end: u8;
    static cannoli_memhook_read_ebp_rdx: u8;
    static cannoli_memhook_read_ebp_rdx_end: u8;
    static cannoli_memhook_read_ebp_rbx: u8;
    static cannoli_memhook_read_ebp_rbx_end: u8;
    static cannoli_memhook_read_ebp_rsp: u8;
    static cannoli_memhook_read_ebp_rsp_end: u8;
    static cannoli_memhook_read_ebp_rbp: u8;
    static cannoli_memhook_read_ebp_rbp_end: u8;
    static cannoli_memhook_read_ebp_rsi: u8;
    static cannoli_memhook_read_ebp_rsi_end: u8;
    static cannoli_memhook_read_ebp_rdi: u8;
    static cannoli_memhook_read_ebp_rdi_end: u8;
    static cannoli_memhook_read_ebp_r8: u8;
    static cannoli_memhook_read_ebp_r8_end: u8;
    static cannoli_memhook_read_ebp_r9: u8;
    static cannoli_memhook_read_ebp_r9_end: u8;
    static cannoli_memhook_read_ebp_r10: u8;
    static cannoli_memhook_read_ebp_r10_end: u8;
    static cannoli_memhook_read_ebp_r11: u8;
    static cannoli_memhook_read_ebp_r11_end: u8;
    static cannoli_memhook_read_ebp_r12: u8;
    static cannoli_memhook_read_ebp_r12_end: u8;
    static cannoli_memhook_read_ebp_r13: u8;
    static cannoli_memhook_read_ebp_r13_end: u8;
    static cannoli_memhook_read_ebp_r14: u8;
    static cannoli_memhook_read_ebp_r14_end: u8;
    static cannoli_memhook_read_ebp_r15: u8;
    static cannoli_memhook_read_ebp_r15_end: u8;
    static cannoli_memhook_read_esi_rax: u8;
    static cannoli_memhook_read_esi_rax_end: u8;
    static cannoli_memhook_read_esi_rcx: u8;
    static cannoli_memhook_read_esi_rcx_end: u8;
    static cannoli_memhook_read_esi_rdx: u8;
    static cannoli_memhook_read_esi_rdx_end: u8;
    static cannoli_memhook_read_esi_rbx: u8;
    static cannoli_memhook_read_esi_rbx_end: u8;
    static cannoli_memhook_read_esi_rsp: u8;
    static cannoli_memhook_read_esi_rsp_end: u8;
    static cannoli_memhook_read_esi_rbp: u8;
    static cannoli_memhook_read_esi_rbp_end: u8;
    static cannoli_memhook_read_esi_rsi: u8;
    static cannoli_memhook_read_esi_rsi_end: u8;
    static cannoli_memhook_read_esi_rdi: u8;
    static cannoli_memhook_read_esi_rdi_end: u8;
    static cannoli_memhook_read_esi_r8: u8;
    static cannoli_memhook_read_esi_r8_end: u8;
    static cannoli_memhook_read_esi_r9: u8;
    static cannoli_memhook_read_esi_r9_end: u8;
    static cannoli_memhook_read_esi_r10: u8;
    static cannoli_memhook_read_esi_r10_end: u8;
    static cannoli_memhook_read_esi_r11: u8;
    static cannoli_memhook_read_esi_r11_end: u8;
    static cannoli_memhook_read_esi_r12: u8;
    static cannoli_memhook_read_esi_r12_end: u8;
    static cannoli_memhook_read_esi_r13: u8;
    static cannoli_memhook_read_esi_r13_end: u8;
    static cannoli_memhook_read_esi_r14: u8;
    static cannoli_memhook_read_esi_r14_end: u8;
    static cannoli_memhook_read_esi_r15: u8;
    static cannoli_memhook_read_esi_r15_end: u8;
    static cannoli_memhook_read_edi_rax: u8;
    static cannoli_memhook_read_edi_rax_end: u8;
    static cannoli_memhook_read_edi_rcx: u8;
    static cannoli_memhook_read_edi_rcx_end: u8;
    static cannoli_memhook_read_edi_rdx: u8;
    static cannoli_memhook_read_edi_rdx_end: u8;
    static cannoli_memhook_read_edi_rbx: u8;
    static cannoli_memhook_read_edi_rbx_end: u8;
    static cannoli_memhook_read_edi_rsp: u8;
    static cannoli_memhook_read_edi_rsp_end: u8;
    static cannoli_memhook_read_edi_rbp: u8;
    static cannoli_memhook_read_edi_rbp_end: u8;
    static cannoli_memhook_read_edi_rsi: u8;
    static cannoli_memhook_read_edi_rsi_end: u8;
    static cannoli_memhook_read_edi_rdi: u8;
    static cannoli_memhook_read_edi_rdi_end: u8;
    static cannoli_memhook_read_edi_r8: u8;
    static cannoli_memhook_read_edi_r8_end: u8;
    static cannoli_memhook_read_edi_r9: u8;
    static cannoli_memhook_read_edi_r9_end: u8;
    static cannoli_memhook_read_edi_r10: u8;
    static cannoli_memhook_read_edi_r10_end: u8;
    static cannoli_memhook_read_edi_r11: u8;
    static cannoli_memhook_read_edi_r11_end: u8;
    static cannoli_memhook_read_edi_r12: u8;
    static cannoli_memhook_read_edi_r12_end: u8;
    static cannoli_memhook_read_edi_r13: u8;
    static cannoli_memhook_read_edi_r13_end: u8;
    static cannoli_memhook_read_edi_r14: u8;
    static cannoli_memhook_read_edi_r14_end: u8;
    static cannoli_memhook_read_edi_r15: u8;
    static cannoli_memhook_read_edi_r15_end: u8;
    static cannoli_memhook_read_r8d_rax: u8;
    static cannoli_memhook_read_r8d_rax_end: u8;
    static cannoli_memhook_read_r8d_rcx: u8;
    static cannoli_memhook_read_r8d_rcx_end: u8;
    static cannoli_memhook_read_r8d_rdx: u8;
    static cannoli_memhook_read_r8d_rdx_end: u8;
    static cannoli_memhook_read_r8d_rbx: u8;
    static cannoli_memhook_read_r8d_rbx_end: u8;
    static cannoli_memhook_read_r8d_rsp: u8;
    static cannoli_memhook_read_r8d_rsp_end: u8;
    static cannoli_memhook_read_r8d_rbp: u8;
    static cannoli_memhook_read_r8d_rbp_end: u8;
    static cannoli_memhook_read_r8d_rsi: u8;
    static cannoli_memhook_read_r8d_rsi_end: u8;
    static cannoli_memhook_read_r8d_rdi: u8;
    static cannoli_memhook_read_r8d_rdi_end: u8;
    static cannoli_memhook_read_r8d_r8: u8;
    static cannoli_memhook_read_r8d_r8_end: u8;
    static cannoli_memhook_read_r8d_r9: u8;
    static cannoli_memhook_read_r8d_r9_end: u8;
    static cannoli_memhook_read_r8d_r10: u8;
    static cannoli_memhook_read_r8d_r10_end: u8;
    static cannoli_memhook_read_r8d_r11: u8;
    static cannoli_memhook_read_r8d_r11_end: u8;
    static cannoli_memhook_read_r8d_r12: u8;
    static cannoli_memhook_read_r8d_r12_end: u8;
    static cannoli_memhook_read_r8d_r13: u8;
    static cannoli_memhook_read_r8d_r13_end: u8;
    static cannoli_memhook_read_r8d_r14: u8;
    static cannoli_memhook_read_r8d_r14_end: u8;
    static cannoli_memhook_read_r8d_r15: u8;
    static cannoli_memhook_read_r8d_r15_end: u8;
    static cannoli_memhook_read_r9d_rax: u8;
    static cannoli_memhook_read_r9d_rax_end: u8;
    static cannoli_memhook_read_r9d_rcx: u8;
    static cannoli_memhook_read_r9d_rcx_end: u8;
    static cannoli_memhook_read_r9d_rdx: u8;
    static cannoli_memhook_read_r9d_rdx_end: u8;
    static cannoli_memhook_read_r9d_rbx: u8;
    static cannoli_memhook_read_r9d_rbx_end: u8;
    static cannoli_memhook_read_r9d_rsp: u8;
    static cannoli_memhook_read_r9d_rsp_end: u8;
    static cannoli_memhook_read_r9d_rbp: u8;
    static cannoli_memhook_read_r9d_rbp_end: u8;
    static cannoli_memhook_read_r9d_rsi: u8;
    static cannoli_memhook_read_r9d_rsi_end: u8;
    static cannoli_memhook_read_r9d_rdi: u8;
    static cannoli_memhook_read_r9d_rdi_end: u8;
    static cannoli_memhook_read_r9d_r8: u8;
    static cannoli_memhook_read_r9d_r8_end: u8;
    static cannoli_memhook_read_r9d_r9: u8;
    static cannoli_memhook_read_r9d_r9_end: u8;
    static cannoli_memhook_read_r9d_r10: u8;
    static cannoli_memhook_read_r9d_r10_end: u8;
    static cannoli_memhook_read_r9d_r11: u8;
    static cannoli_memhook_read_r9d_r11_end: u8;
    static cannoli_memhook_read_r9d_r12: u8;
    static cannoli_memhook_read_r9d_r12_end: u8;
    static cannoli_memhook_read_r9d_r13: u8;
    static cannoli_memhook_read_r9d_r13_end: u8;
    static cannoli_memhook_read_r9d_r14: u8;
    static cannoli_memhook_read_r9d_r14_end: u8;
    static cannoli_memhook_read_r9d_r15: u8;
    static cannoli_memhook_read_r9d_r15_end: u8;
    static cannoli_memhook_read_r10d_rax: u8;
    static cannoli_memhook_read_r10d_rax_end: u8;
    static cannoli_memhook_read_r10d_rcx: u8;
    static cannoli_memhook_read_r10d_rcx_end: u8;
    static cannoli_memhook_read_r10d_rdx: u8;
    static cannoli_memhook_read_r10d_rdx_end: u8;
    static cannoli_memhook_read_r10d_rbx: u8;
    static cannoli_memhook_read_r10d_rbx_end: u8;
    static cannoli_memhook_read_r10d_rsp: u8;
    static cannoli_memhook_read_r10d_rsp_end: u8;
    static cannoli_memhook_read_r10d_rbp: u8;
    static cannoli_memhook_read_r10d_rbp_end: u8;
    static cannoli_memhook_read_r10d_rsi: u8;
    static cannoli_memhook_read_r10d_rsi_end: u8;
    static cannoli_memhook_read_r10d_rdi: u8;
    static cannoli_memhook_read_r10d_rdi_end: u8;
    static cannoli_memhook_read_r10d_r8: u8;
    static cannoli_memhook_read_r10d_r8_end: u8;
    static cannoli_memhook_read_r10d_r9: u8;
    static cannoli_memhook_read_r10d_r9_end: u8;
    static cannoli_memhook_read_r10d_r10: u8;
    static cannoli_memhook_read_r10d_r10_end: u8;
    static cannoli_memhook_read_r10d_r11: u8;
    static cannoli_memhook_read_r10d_r11_end: u8;
    static cannoli_memhook_read_r10d_r12: u8;
    static cannoli_memhook_read_r10d_r12_end: u8;
    static cannoli_memhook_read_r10d_r13: u8;
    static cannoli_memhook_read_r10d_r13_end: u8;
    static cannoli_memhook_read_r10d_r14: u8;
    static cannoli_memhook_read_r10d_r14_end: u8;
    static cannoli_memhook_read_r10d_r15: u8;
    static cannoli_memhook_read_r10d_r15_end: u8;
    static cannoli_memhook_read_r11d_rax: u8;
    static cannoli_memhook_read_r11d_rax_end: u8;
    static cannoli_memhook_read_r11d_rcx: u8;
    static cannoli_memhook_read_r11d_rcx_end: u8;
    static cannoli_memhook_read_r11d_rdx: u8;
    static cannoli_memhook_read_r11d_rdx_end: u8;
    static cannoli_memhook_read_r11d_rbx: u8;
    static cannoli_memhook_read_r11d_rbx_end: u8;
    static cannoli_memhook_read_r11d_rsp: u8;
    static cannoli_memhook_read_r11d_rsp_end: u8;
    static cannoli_memhook_read_r11d_rbp: u8;
    static cannoli_memhook_read_r11d_rbp_end: u8;
    static cannoli_memhook_read_r11d_rsi: u8;
    static cannoli_memhook_read_r11d_rsi_end: u8;
    static cannoli_memhook_read_r11d_rdi: u8;
    static cannoli_memhook_read_r11d_rdi_end: u8;
    static cannoli_memhook_read_r11d_r8: u8;
    static cannoli_memhook_read_r11d_r8_end: u8;
    static cannoli_memhook_read_r11d_r9: u8;
    static cannoli_memhook_read_r11d_r9_end: u8;
    static cannoli_memhook_read_r11d_r10: u8;
    static cannoli_memhook_read_r11d_r10_end: u8;
    static cannoli_memhook_read_r11d_r11: u8;
    static cannoli_memhook_read_r11d_r11_end: u8;
    static cannoli_memhook_read_r11d_r12: u8;
    static cannoli_memhook_read_r11d_r12_end: u8;
    static cannoli_memhook_read_r11d_r13: u8;
    static cannoli_memhook_read_r11d_r13_end: u8;
    static cannoli_memhook_read_r11d_r14: u8;
    static cannoli_memhook_read_r11d_r14_end: u8;
    static cannoli_memhook_read_r11d_r15: u8;
    static cannoli_memhook_read_r11d_r15_end: u8;
    static cannoli_memhook_read_r12d_rax: u8;
    static cannoli_memhook_read_r12d_rax_end: u8;
    static cannoli_memhook_read_r12d_rcx: u8;
    static cannoli_memhook_read_r12d_rcx_end: u8;
    static cannoli_memhook_read_r12d_rdx: u8;
    static cannoli_memhook_read_r12d_rdx_end: u8;
    static cannoli_memhook_read_r12d_rbx: u8;
    static cannoli_memhook_read_r12d_rbx_end: u8;
    static cannoli_memhook_read_r12d_rsp: u8;
    static cannoli_memhook_read_r12d_rsp_end: u8;
    static cannoli_memhook_read_r12d_rbp: u8;
    static cannoli_memhook_read_r12d_rbp_end: u8;
    static cannoli_memhook_read_r12d_rsi: u8;
    static cannoli_memhook_read_r12d_rsi_end: u8;
    static cannoli_memhook_read_r12d_rdi: u8;
    static cannoli_memhook_read_r12d_rdi_end: u8;
    static cannoli_memhook_read_r12d_r8: u8;
    static cannoli_memhook_read_r12d_r8_end: u8;
    static cannoli_memhook_read_r12d_r9: u8;
    static cannoli_memhook_read_r12d_r9_end: u8;
    static cannoli_memhook_read_r12d_r10: u8;
    static cannoli_memhook_read_r12d_r10_end: u8;
    static cannoli_memhook_read_r12d_r11: u8;
    static cannoli_memhook_read_r12d_r11_end: u8;
    static cannoli_memhook_read_r12d_r12: u8;
    static cannoli_memhook_read_r12d_r12_end: u8;
    static cannoli_memhook_read_r12d_r13: u8;
    static cannoli_memhook_read_r12d_r13_end: u8;
    static cannoli_memhook_read_r12d_r14: u8;
    static cannoli_memhook_read_r12d_r14_end: u8;
    static cannoli_memhook_read_r12d_r15: u8;
    static cannoli_memhook_read_r12d_r15_end: u8;
    static cannoli_memhook_read_r13d_rax: u8;
    static cannoli_memhook_read_r13d_rax_end: u8;
    static cannoli_memhook_read_r13d_rcx: u8;
    static cannoli_memhook_read_r13d_rcx_end: u8;
    static cannoli_memhook_read_r13d_rdx: u8;
    static cannoli_memhook_read_r13d_rdx_end: u8;
    static cannoli_memhook_read_r13d_rbx: u8;
    static cannoli_memhook_read_r13d_rbx_end: u8;
    static cannoli_memhook_read_r13d_rsp: u8;
    static cannoli_memhook_read_r13d_rsp_end: u8;
    static cannoli_memhook_read_r13d_rbp: u8;
    static cannoli_memhook_read_r13d_rbp_end: u8;
    static cannoli_memhook_read_r13d_rsi: u8;
    static cannoli_memhook_read_r13d_rsi_end: u8;
    static cannoli_memhook_read_r13d_rdi: u8;
    static cannoli_memhook_read_r13d_rdi_end: u8;
    static cannoli_memhook_read_r13d_r8: u8;
    static cannoli_memhook_read_r13d_r8_end: u8;
    static cannoli_memhook_read_r13d_r9: u8;
    static cannoli_memhook_read_r13d_r9_end: u8;
    static cannoli_memhook_read_r13d_r10: u8;
    static cannoli_memhook_read_r13d_r10_end: u8;
    static cannoli_memhook_read_r13d_r11: u8;
    static cannoli_memhook_read_r13d_r11_end: u8;
    static cannoli_memhook_read_r13d_r12: u8;
    static cannoli_memhook_read_r13d_r12_end: u8;
    static cannoli_memhook_read_r13d_r13: u8;
    static cannoli_memhook_read_r13d_r13_end: u8;
    static cannoli_memhook_read_r13d_r14: u8;
    static cannoli_memhook_read_r13d_r14_end: u8;
    static cannoli_memhook_read_r13d_r15: u8;
    static cannoli_memhook_read_r13d_r15_end: u8;
    static cannoli_memhook_read_r14d_rax: u8;
    static cannoli_memhook_read_r14d_rax_end: u8;
    static cannoli_memhook_read_r14d_rcx: u8;
    static cannoli_memhook_read_r14d_rcx_end: u8;
    static cannoli_memhook_read_r14d_rdx: u8;
    static cannoli_memhook_read_r14d_rdx_end: u8;
    static cannoli_memhook_read_r14d_rbx: u8;
    static cannoli_memhook_read_r14d_rbx_end: u8;
    static cannoli_memhook_read_r14d_rsp: u8;
    static cannoli_memhook_read_r14d_rsp_end: u8;
    static cannoli_memhook_read_r14d_rbp: u8;
    static cannoli_memhook_read_r14d_rbp_end: u8;
    static cannoli_memhook_read_r14d_rsi: u8;
    static cannoli_memhook_read_r14d_rsi_end: u8;
    static cannoli_memhook_read_r14d_rdi: u8;
    static cannoli_memhook_read_r14d_rdi_end: u8;
    static cannoli_memhook_read_r14d_r8: u8;
    static cannoli_memhook_read_r14d_r8_end: u8;
    static cannoli_memhook_read_r14d_r9: u8;
    static cannoli_memhook_read_r14d_r9_end: u8;
    static cannoli_memhook_read_r14d_r10: u8;
    static cannoli_memhook_read_r14d_r10_end: u8;
    static cannoli_memhook_read_r14d_r11: u8;
    static cannoli_memhook_read_r14d_r11_end: u8;
    static cannoli_memhook_read_r14d_r12: u8;
    static cannoli_memhook_read_r14d_r12_end: u8;
    static cannoli_memhook_read_r14d_r13: u8;
    static cannoli_memhook_read_r14d_r13_end: u8;
    static cannoli_memhook_read_r14d_r14: u8;
    static cannoli_memhook_read_r14d_r14_end: u8;
    static cannoli_memhook_read_r14d_r15: u8;
    static cannoli_memhook_read_r14d_r15_end: u8;
    static cannoli_memhook_read_r15d_rax: u8;
    static cannoli_memhook_read_r15d_rax_end: u8;
    static cannoli_memhook_read_r15d_rcx: u8;
    static cannoli_memhook_read_r15d_rcx_end: u8;
    static cannoli_memhook_read_r15d_rdx: u8;
    static cannoli_memhook_read_r15d_rdx_end: u8;
    static cannoli_memhook_read_r15d_rbx: u8;
    static cannoli_memhook_read_r15d_rbx_end: u8;
    static cannoli_memhook_read_r15d_rsp: u8;
    static cannoli_memhook_read_r15d_rsp_end: u8;
    static cannoli_memhook_read_r15d_rbp: u8;
    static cannoli_memhook_read_r15d_rbp_end: u8;
    static cannoli_memhook_read_r15d_rsi: u8;
    static cannoli_memhook_read_r15d_rsi_end: u8;
    static cannoli_memhook_read_r15d_rdi: u8;
    static cannoli_memhook_read_r15d_rdi_end: u8;
    static cannoli_memhook_read_r15d_r8: u8;
    static cannoli_memhook_read_r15d_r8_end: u8;
    static cannoli_memhook_read_r15d_r9: u8;
    static cannoli_memhook_read_r15d_r9_end: u8;
    static cannoli_memhook_read_r15d_r10: u8;
    static cannoli_memhook_read_r15d_r10_end: u8;
    static cannoli_memhook_read_r15d_r11: u8;
    static cannoli_memhook_read_r15d_r11_end: u8;
    static cannoli_memhook_read_r15d_r12: u8;
    static cannoli_memhook_read_r15d_r12_end: u8;
    static cannoli_memhook_read_r15d_r13: u8;
    static cannoli_memhook_read_r15d_r13_end: u8;
    static cannoli_memhook_read_r15d_r14: u8;
    static cannoli_memhook_read_r15d_r14_end: u8;
    static cannoli_memhook_read_r15d_r15: u8;
    static cannoli_memhook_read_r15d_r15_end: u8;
    static cannoli_memhook_read_rax_eax: u8;
    static cannoli_memhook_read_rax_eax_end: u8;
    static cannoli_memhook_read_rax_ecx: u8;
    static cannoli_memhook_read_rax_ecx_end: u8;
    static cannoli_memhook_read_rax_edx: u8;
    static cannoli_memhook_read_rax_edx_end: u8;
    static cannoli_memhook_read_rax_ebx: u8;
    static cannoli_memhook_read_rax_ebx_end: u8;
    static cannoli_memhook_read_rax_esp: u8;
    static cannoli_memhook_read_rax_esp_end: u8;
    static cannoli_memhook_read_rax_ebp: u8;
    static cannoli_memhook_read_rax_ebp_end: u8;
    static cannoli_memhook_read_rax_esi: u8;
    static cannoli_memhook_read_rax_esi_end: u8;
    static cannoli_memhook_read_rax_edi: u8;
    static cannoli_memhook_read_rax_edi_end: u8;
    static cannoli_memhook_read_rax_r8d: u8;
    static cannoli_memhook_read_rax_r8d_end: u8;
    static cannoli_memhook_read_rax_r9d: u8;
    static cannoli_memhook_read_rax_r9d_end: u8;
    static cannoli_memhook_read_rax_r10d: u8;
    static cannoli_memhook_read_rax_r10d_end: u8;
    static cannoli_memhook_read_rax_r11d: u8;
    static cannoli_memhook_read_rax_r11d_end: u8;
    static cannoli_memhook_read_rax_r12d: u8;
    static cannoli_memhook_read_rax_r12d_end: u8;
    static cannoli_memhook_read_rax_r13d: u8;
    static cannoli_memhook_read_rax_r13d_end: u8;
    static cannoli_memhook_read_rax_r14d: u8;
    static cannoli_memhook_read_rax_r14d_end: u8;
    static cannoli_memhook_read_rax_r15d: u8;
    static cannoli_memhook_read_rax_r15d_end: u8;
    static cannoli_memhook_read_rcx_eax: u8;
    static cannoli_memhook_read_rcx_eax_end: u8;
    static cannoli_memhook_read_rcx_ecx: u8;
    static cannoli_memhook_read_rcx_ecx_end: u8;
    static cannoli_memhook_read_rcx_edx: u8;
    static cannoli_memhook_read_rcx_edx_end: u8;
    static cannoli_memhook_read_rcx_ebx: u8;
    static cannoli_memhook_read_rcx_ebx_end: u8;
    static cannoli_memhook_read_rcx_esp: u8;
    static cannoli_memhook_read_rcx_esp_end: u8;
    static cannoli_memhook_read_rcx_ebp: u8;
    static cannoli_memhook_read_rcx_ebp_end: u8;
    static cannoli_memhook_read_rcx_esi: u8;
    static cannoli_memhook_read_rcx_esi_end: u8;
    static cannoli_memhook_read_rcx_edi: u8;
    static cannoli_memhook_read_rcx_edi_end: u8;
    static cannoli_memhook_read_rcx_r8d: u8;
    static cannoli_memhook_read_rcx_r8d_end: u8;
    static cannoli_memhook_read_rcx_r9d: u8;
    static cannoli_memhook_read_rcx_r9d_end: u8;
    static cannoli_memhook_read_rcx_r10d: u8;
    static cannoli_memhook_read_rcx_r10d_end: u8;
    static cannoli_memhook_read_rcx_r11d: u8;
    static cannoli_memhook_read_rcx_r11d_end: u8;
    static cannoli_memhook_read_rcx_r12d: u8;
    static cannoli_memhook_read_rcx_r12d_end: u8;
    static cannoli_memhook_read_rcx_r13d: u8;
    static cannoli_memhook_read_rcx_r13d_end: u8;
    static cannoli_memhook_read_rcx_r14d: u8;
    static cannoli_memhook_read_rcx_r14d_end: u8;
    static cannoli_memhook_read_rcx_r15d: u8;
    static cannoli_memhook_read_rcx_r15d_end: u8;
    static cannoli_memhook_read_rdx_eax: u8;
    static cannoli_memhook_read_rdx_eax_end: u8;
    static cannoli_memhook_read_rdx_ecx: u8;
    static cannoli_memhook_read_rdx_ecx_end: u8;
    static cannoli_memhook_read_rdx_edx: u8;
    static cannoli_memhook_read_rdx_edx_end: u8;
    static cannoli_memhook_read_rdx_ebx: u8;
    static cannoli_memhook_read_rdx_ebx_end: u8;
    static cannoli_memhook_read_rdx_esp: u8;
    static cannoli_memhook_read_rdx_esp_end: u8;
    static cannoli_memhook_read_rdx_ebp: u8;
    static cannoli_memhook_read_rdx_ebp_end: u8;
    static cannoli_memhook_read_rdx_esi: u8;
    static cannoli_memhook_read_rdx_esi_end: u8;
    static cannoli_memhook_read_rdx_edi: u8;
    static cannoli_memhook_read_rdx_edi_end: u8;
    static cannoli_memhook_read_rdx_r8d: u8;
    static cannoli_memhook_read_rdx_r8d_end: u8;
    static cannoli_memhook_read_rdx_r9d: u8;
    static cannoli_memhook_read_rdx_r9d_end: u8;
    static cannoli_memhook_read_rdx_r10d: u8;
    static cannoli_memhook_read_rdx_r10d_end: u8;
    static cannoli_memhook_read_rdx_r11d: u8;
    static cannoli_memhook_read_rdx_r11d_end: u8;
    static cannoli_memhook_read_rdx_r12d: u8;
    static cannoli_memhook_read_rdx_r12d_end: u8;
    static cannoli_memhook_read_rdx_r13d: u8;
    static cannoli_memhook_read_rdx_r13d_end: u8;
    static cannoli_memhook_read_rdx_r14d: u8;
    static cannoli_memhook_read_rdx_r14d_end: u8;
    static cannoli_memhook_read_rdx_r15d: u8;
    static cannoli_memhook_read_rdx_r15d_end: u8;
    static cannoli_memhook_read_rbx_eax: u8;
    static cannoli_memhook_read_rbx_eax_end: u8;
    static cannoli_memhook_read_rbx_ecx: u8;
    static cannoli_memhook_read_rbx_ecx_end: u8;
    static cannoli_memhook_read_rbx_edx: u8;
    static cannoli_memhook_read_rbx_edx_end: u8;
    static cannoli_memhook_read_rbx_ebx: u8;
    static cannoli_memhook_read_rbx_ebx_end: u8;
    static cannoli_memhook_read_rbx_esp: u8;
    static cannoli_memhook_read_rbx_esp_end: u8;
    static cannoli_memhook_read_rbx_ebp: u8;
    static cannoli_memhook_read_rbx_ebp_end: u8;
    static cannoli_memhook_read_rbx_esi: u8;
    static cannoli_memhook_read_rbx_esi_end: u8;
    static cannoli_memhook_read_rbx_edi: u8;
    static cannoli_memhook_read_rbx_edi_end: u8;
    static cannoli_memhook_read_rbx_r8d: u8;
    static cannoli_memhook_read_rbx_r8d_end: u8;
    static cannoli_memhook_read_rbx_r9d: u8;
    static cannoli_memhook_read_rbx_r9d_end: u8;
    static cannoli_memhook_read_rbx_r10d: u8;
    static cannoli_memhook_read_rbx_r10d_end: u8;
    static cannoli_memhook_read_rbx_r11d: u8;
    static cannoli_memhook_read_rbx_r11d_end: u8;
    static cannoli_memhook_read_rbx_r12d: u8;
    static cannoli_memhook_read_rbx_r12d_end: u8;
    static cannoli_memhook_read_rbx_r13d: u8;
    static cannoli_memhook_read_rbx_r13d_end: u8;
    static cannoli_memhook_read_rbx_r14d: u8;
    static cannoli_memhook_read_rbx_r14d_end: u8;
    static cannoli_memhook_read_rbx_r15d: u8;
    static cannoli_memhook_read_rbx_r15d_end: u8;
    static cannoli_memhook_read_rsp_eax: u8;
    static cannoli_memhook_read_rsp_eax_end: u8;
    static cannoli_memhook_read_rsp_ecx: u8;
    static cannoli_memhook_read_rsp_ecx_end: u8;
    static cannoli_memhook_read_rsp_edx: u8;
    static cannoli_memhook_read_rsp_edx_end: u8;
    static cannoli_memhook_read_rsp_ebx: u8;
    static cannoli_memhook_read_rsp_ebx_end: u8;
    static cannoli_memhook_read_rsp_esp: u8;
    static cannoli_memhook_read_rsp_esp_end: u8;
    static cannoli_memhook_read_rsp_ebp: u8;
    static cannoli_memhook_read_rsp_ebp_end: u8;
    static cannoli_memhook_read_rsp_esi: u8;
    static cannoli_memhook_read_rsp_esi_end: u8;
    static cannoli_memhook_read_rsp_edi: u8;
    static cannoli_memhook_read_rsp_edi_end: u8;
    static cannoli_memhook_read_rsp_r8d: u8;
    static cannoli_memhook_read_rsp_r8d_end: u8;
    static cannoli_memhook_read_rsp_r9d: u8;
    static cannoli_memhook_read_rsp_r9d_end: u8;
    static cannoli_memhook_read_rsp_r10d: u8;
    static cannoli_memhook_read_rsp_r10d_end: u8;
    static cannoli_memhook_read_rsp_r11d: u8;
    static cannoli_memhook_read_rsp_r11d_end: u8;
    static cannoli_memhook_read_rsp_r12d: u8;
    static cannoli_memhook_read_rsp_r12d_end: u8;
    static cannoli_memhook_read_rsp_r13d: u8;
    static cannoli_memhook_read_rsp_r13d_end: u8;
    static cannoli_memhook_read_rsp_r14d: u8;
    static cannoli_memhook_read_rsp_r14d_end: u8;
    static cannoli_memhook_read_rsp_r15d: u8;
    static cannoli_memhook_read_rsp_r15d_end: u8;
    static cannoli_memhook_read_rbp_eax: u8;
    static cannoli_memhook_read_rbp_eax_end: u8;
    static cannoli_memhook_read_rbp_ecx: u8;
    static cannoli_memhook_read_rbp_ecx_end: u8;
    static cannoli_memhook_read_rbp_edx: u8;
    static cannoli_memhook_read_rbp_edx_end: u8;
    static cannoli_memhook_read_rbp_ebx: u8;
    static cannoli_memhook_read_rbp_ebx_end: u8;
    static cannoli_memhook_read_rbp_esp: u8;
    static cannoli_memhook_read_rbp_esp_end: u8;
    static cannoli_memhook_read_rbp_ebp: u8;
    static cannoli_memhook_read_rbp_ebp_end: u8;
    static cannoli_memhook_read_rbp_esi: u8;
    static cannoli_memhook_read_rbp_esi_end: u8;
    static cannoli_memhook_read_rbp_edi: u8;
    static cannoli_memhook_read_rbp_edi_end: u8;
    static cannoli_memhook_read_rbp_r8d: u8;
    static cannoli_memhook_read_rbp_r8d_end: u8;
    static cannoli_memhook_read_rbp_r9d: u8;
    static cannoli_memhook_read_rbp_r9d_end: u8;
    static cannoli_memhook_read_rbp_r10d: u8;
    static cannoli_memhook_read_rbp_r10d_end: u8;
    static cannoli_memhook_read_rbp_r11d: u8;
    static cannoli_memhook_read_rbp_r11d_end: u8;
    static cannoli_memhook_read_rbp_r12d: u8;
    static cannoli_memhook_read_rbp_r12d_end: u8;
    static cannoli_memhook_read_rbp_r13d: u8;
    static cannoli_memhook_read_rbp_r13d_end: u8;
    static cannoli_memhook_read_rbp_r14d: u8;
    static cannoli_memhook_read_rbp_r14d_end: u8;
    static cannoli_memhook_read_rbp_r15d: u8;
    static cannoli_memhook_read_rbp_r15d_end: u8;
    static cannoli_memhook_read_rsi_eax: u8;
    static cannoli_memhook_read_rsi_eax_end: u8;
    static cannoli_memhook_read_rsi_ecx: u8;
    static cannoli_memhook_read_rsi_ecx_end: u8;
    static cannoli_memhook_read_rsi_edx: u8;
    static cannoli_memhook_read_rsi_edx_end: u8;
    static cannoli_memhook_read_rsi_ebx: u8;
    static cannoli_memhook_read_rsi_ebx_end: u8;
    static cannoli_memhook_read_rsi_esp: u8;
    static cannoli_memhook_read_rsi_esp_end: u8;
    static cannoli_memhook_read_rsi_ebp: u8;
    static cannoli_memhook_read_rsi_ebp_end: u8;
    static cannoli_memhook_read_rsi_esi: u8;
    static cannoli_memhook_read_rsi_esi_end: u8;
    static cannoli_memhook_read_rsi_edi: u8;
    static cannoli_memhook_read_rsi_edi_end: u8;
    static cannoli_memhook_read_rsi_r8d: u8;
    static cannoli_memhook_read_rsi_r8d_end: u8;
    static cannoli_memhook_read_rsi_r9d: u8;
    static cannoli_memhook_read_rsi_r9d_end: u8;
    static cannoli_memhook_read_rsi_r10d: u8;
    static cannoli_memhook_read_rsi_r10d_end: u8;
    static cannoli_memhook_read_rsi_r11d: u8;
    static cannoli_memhook_read_rsi_r11d_end: u8;
    static cannoli_memhook_read_rsi_r12d: u8;
    static cannoli_memhook_read_rsi_r12d_end: u8;
    static cannoli_memhook_read_rsi_r13d: u8;
    static cannoli_memhook_read_rsi_r13d_end: u8;
    static cannoli_memhook_read_rsi_r14d: u8;
    static cannoli_memhook_read_rsi_r14d_end: u8;
    static cannoli_memhook_read_rsi_r15d: u8;
    static cannoli_memhook_read_rsi_r15d_end: u8;
    static cannoli_memhook_read_rdi_eax: u8;
    static cannoli_memhook_read_rdi_eax_end: u8;
    static cannoli_memhook_read_rdi_ecx: u8;
    static cannoli_memhook_read_rdi_ecx_end: u8;
    static cannoli_memhook_read_rdi_edx: u8;
    static cannoli_memhook_read_rdi_edx_end: u8;
    static cannoli_memhook_read_rdi_ebx: u8;
    static cannoli_memhook_read_rdi_ebx_end: u8;
    static cannoli_memhook_read_rdi_esp: u8;
    static cannoli_memhook_read_rdi_esp_end: u8;
    static cannoli_memhook_read_rdi_ebp: u8;
    static cannoli_memhook_read_rdi_ebp_end: u8;
    static cannoli_memhook_read_rdi_esi: u8;
    static cannoli_memhook_read_rdi_esi_end: u8;
    static cannoli_memhook_read_rdi_edi: u8;
    static cannoli_memhook_read_rdi_edi_end: u8;
    static cannoli_memhook_read_rdi_r8d: u8;
    static cannoli_memhook_read_rdi_r8d_end: u8;
    static cannoli_memhook_read_rdi_r9d: u8;
    static cannoli_memhook_read_rdi_r9d_end: u8;
    static cannoli_memhook_read_rdi_r10d: u8;
    static cannoli_memhook_read_rdi_r10d_end: u8;
    static cannoli_memhook_read_rdi_r11d: u8;
    static cannoli_memhook_read_rdi_r11d_end: u8;
    static cannoli_memhook_read_rdi_r12d: u8;
    static cannoli_memhook_read_rdi_r12d_end: u8;
    static cannoli_memhook_read_rdi_r13d: u8;
    static cannoli_memhook_read_rdi_r13d_end: u8;
    static cannoli_memhook_read_rdi_r14d: u8;
    static cannoli_memhook_read_rdi_r14d_end: u8;
    static cannoli_memhook_read_rdi_r15d: u8;
    static cannoli_memhook_read_rdi_r15d_end: u8;
    static cannoli_memhook_read_r8_eax: u8;
    static cannoli_memhook_read_r8_eax_end: u8;
    static cannoli_memhook_read_r8_ecx: u8;
    static cannoli_memhook_read_r8_ecx_end: u8;
    static cannoli_memhook_read_r8_edx: u8;
    static cannoli_memhook_read_r8_edx_end: u8;
    static cannoli_memhook_read_r8_ebx: u8;
    static cannoli_memhook_read_r8_ebx_end: u8;
    static cannoli_memhook_read_r8_esp: u8;
    static cannoli_memhook_read_r8_esp_end: u8;
    static cannoli_memhook_read_r8_ebp: u8;
    static cannoli_memhook_read_r8_ebp_end: u8;
    static cannoli_memhook_read_r8_esi: u8;
    static cannoli_memhook_read_r8_esi_end: u8;
    static cannoli_memhook_read_r8_edi: u8;
    static cannoli_memhook_read_r8_edi_end: u8;
    static cannoli_memhook_read_r8_r8d: u8;
    static cannoli_memhook_read_r8_r8d_end: u8;
    static cannoli_memhook_read_r8_r9d: u8;
    static cannoli_memhook_read_r8_r9d_end: u8;
    static cannoli_memhook_read_r8_r10d: u8;
    static cannoli_memhook_read_r8_r10d_end: u8;
    static cannoli_memhook_read_r8_r11d: u8;
    static cannoli_memhook_read_r8_r11d_end: u8;
    static cannoli_memhook_read_r8_r12d: u8;
    static cannoli_memhook_read_r8_r12d_end: u8;
    static cannoli_memhook_read_r8_r13d: u8;
    static cannoli_memhook_read_r8_r13d_end: u8;
    static cannoli_memhook_read_r8_r14d: u8;
    static cannoli_memhook_read_r8_r14d_end: u8;
    static cannoli_memhook_read_r8_r15d: u8;
    static cannoli_memhook_read_r8_r15d_end: u8;
    static cannoli_memhook_read_r9_eax: u8;
    static cannoli_memhook_read_r9_eax_end: u8;
    static cannoli_memhook_read_r9_ecx: u8;
    static cannoli_memhook_read_r9_ecx_end: u8;
    static cannoli_memhook_read_r9_edx: u8;
    static cannoli_memhook_read_r9_edx_end: u8;
    static cannoli_memhook_read_r9_ebx: u8;
    static cannoli_memhook_read_r9_ebx_end: u8;
    static cannoli_memhook_read_r9_esp: u8;
    static cannoli_memhook_read_r9_esp_end: u8;
    static cannoli_memhook_read_r9_ebp: u8;
    static cannoli_memhook_read_r9_ebp_end: u8;
    static cannoli_memhook_read_r9_esi: u8;
    static cannoli_memhook_read_r9_esi_end: u8;
    static cannoli_memhook_read_r9_edi: u8;
    static cannoli_memhook_read_r9_edi_end: u8;
    static cannoli_memhook_read_r9_r8d: u8;
    static cannoli_memhook_read_r9_r8d_end: u8;
    static cannoli_memhook_read_r9_r9d: u8;
    static cannoli_memhook_read_r9_r9d_end: u8;
    static cannoli_memhook_read_r9_r10d: u8;
    static cannoli_memhook_read_r9_r10d_end: u8;
    static cannoli_memhook_read_r9_r11d: u8;
    static cannoli_memhook_read_r9_r11d_end: u8;
    static cannoli_memhook_read_r9_r12d: u8;
    static cannoli_memhook_read_r9_r12d_end: u8;
    static cannoli_memhook_read_r9_r13d: u8;
    static cannoli_memhook_read_r9_r13d_end: u8;
    static cannoli_memhook_read_r9_r14d: u8;
    static cannoli_memhook_read_r9_r14d_end: u8;
    static cannoli_memhook_read_r9_r15d: u8;
    static cannoli_memhook_read_r9_r15d_end: u8;
    static cannoli_memhook_read_r10_eax: u8;
    static cannoli_memhook_read_r10_eax_end: u8;
    static cannoli_memhook_read_r10_ecx: u8;
    static cannoli_memhook_read_r10_ecx_end: u8;
    static cannoli_memhook_read_r10_edx: u8;
    static cannoli_memhook_read_r10_edx_end: u8;
    static cannoli_memhook_read_r10_ebx: u8;
    static cannoli_memhook_read_r10_ebx_end: u8;
    static cannoli_memhook_read_r10_esp: u8;
    static cannoli_memhook_read_r10_esp_end: u8;
    static cannoli_memhook_read_r10_ebp: u8;
    static cannoli_memhook_read_r10_ebp_end: u8;
    static cannoli_memhook_read_r10_esi: u8;
    static cannoli_memhook_read_r10_esi_end: u8;
    static cannoli_memhook_read_r10_edi: u8;
    static cannoli_memhook_read_r10_edi_end: u8;
    static cannoli_memhook_read_r10_r8d: u8;
    static cannoli_memhook_read_r10_r8d_end: u8;
    static cannoli_memhook_read_r10_r9d: u8;
    static cannoli_memhook_read_r10_r9d_end: u8;
    static cannoli_memhook_read_r10_r10d: u8;
    static cannoli_memhook_read_r10_r10d_end: u8;
    static cannoli_memhook_read_r10_r11d: u8;
    static cannoli_memhook_read_r10_r11d_end: u8;
    static cannoli_memhook_read_r10_r12d: u8;
    static cannoli_memhook_read_r10_r12d_end: u8;
    static cannoli_memhook_read_r10_r13d: u8;
    static cannoli_memhook_read_r10_r13d_end: u8;
    static cannoli_memhook_read_r10_r14d: u8;
    static cannoli_memhook_read_r10_r14d_end: u8;
    static cannoli_memhook_read_r10_r15d: u8;
    static cannoli_memhook_read_r10_r15d_end: u8;
    static cannoli_memhook_read_r11_eax: u8;
    static cannoli_memhook_read_r11_eax_end: u8;
    static cannoli_memhook_read_r11_ecx: u8;
    static cannoli_memhook_read_r11_ecx_end: u8;
    static cannoli_memhook_read_r11_edx: u8;
    static cannoli_memhook_read_r11_edx_end: u8;
    static cannoli_memhook_read_r11_ebx: u8;
    static cannoli_memhook_read_r11_ebx_end: u8;
    static cannoli_memhook_read_r11_esp: u8;
    static cannoli_memhook_read_r11_esp_end: u8;
    static cannoli_memhook_read_r11_ebp: u8;
    static cannoli_memhook_read_r11_ebp_end: u8;
    static cannoli_memhook_read_r11_esi: u8;
    static cannoli_memhook_read_r11_esi_end: u8;
    static cannoli_memhook_read_r11_edi: u8;
    static cannoli_memhook_read_r11_edi_end: u8;
    static cannoli_memhook_read_r11_r8d: u8;
    static cannoli_memhook_read_r11_r8d_end: u8;
    static cannoli_memhook_read_r11_r9d: u8;
    static cannoli_memhook_read_r11_r9d_end: u8;
    static cannoli_memhook_read_r11_r10d: u8;
    static cannoli_memhook_read_r11_r10d_end: u8;
    static cannoli_memhook_read_r11_r11d: u8;
    static cannoli_memhook_read_r11_r11d_end: u8;
    static cannoli_memhook_read_r11_r12d: u8;
    static cannoli_memhook_read_r11_r12d_end: u8;
    static cannoli_memhook_read_r11_r13d: u8;
    static cannoli_memhook_read_r11_r13d_end: u8;
    static cannoli_memhook_read_r11_r14d: u8;
    static cannoli_memhook_read_r11_r14d_end: u8;
    static cannoli_memhook_read_r11_r15d: u8;
    static cannoli_memhook_read_r11_r15d_end: u8;
    static cannoli_memhook_read_r12_eax: u8;
    static cannoli_memhook_read_r12_eax_end: u8;
    static cannoli_memhook_read_r12_ecx: u8;
    static cannoli_memhook_read_r12_ecx_end: u8;
    static cannoli_memhook_read_r12_edx: u8;
    static cannoli_memhook_read_r12_edx_end: u8;
    static cannoli_memhook_read_r12_ebx: u8;
    static cannoli_memhook_read_r12_ebx_end: u8;
    static cannoli_memhook_read_r12_esp: u8;
    static cannoli_memhook_read_r12_esp_end: u8;
    static cannoli_memhook_read_r12_ebp: u8;
    static cannoli_memhook_read_r12_ebp_end: u8;
    static cannoli_memhook_read_r12_esi: u8;
    static cannoli_memhook_read_r12_esi_end: u8;
    static cannoli_memhook_read_r12_edi: u8;
    static cannoli_memhook_read_r12_edi_end: u8;
    static cannoli_memhook_read_r12_r8d: u8;
    static cannoli_memhook_read_r12_r8d_end: u8;
    static cannoli_memhook_read_r12_r9d: u8;
    static cannoli_memhook_read_r12_r9d_end: u8;
    static cannoli_memhook_read_r12_r10d: u8;
    static cannoli_memhook_read_r12_r10d_end: u8;
    static cannoli_memhook_read_r12_r11d: u8;
    static cannoli_memhook_read_r12_r11d_end: u8;
    static cannoli_memhook_read_r12_r12d: u8;
    static cannoli_memhook_read_r12_r12d_end: u8;
    static cannoli_memhook_read_r12_r13d: u8;
    static cannoli_memhook_read_r12_r13d_end: u8;
    static cannoli_memhook_read_r12_r14d: u8;
    static cannoli_memhook_read_r12_r14d_end: u8;
    static cannoli_memhook_read_r12_r15d: u8;
    static cannoli_memhook_read_r12_r15d_end: u8;
    static cannoli_memhook_read_r13_eax: u8;
    static cannoli_memhook_read_r13_eax_end: u8;
    static cannoli_memhook_read_r13_ecx: u8;
    static cannoli_memhook_read_r13_ecx_end: u8;
    static cannoli_memhook_read_r13_edx: u8;
    static cannoli_memhook_read_r13_edx_end: u8;
    static cannoli_memhook_read_r13_ebx: u8;
    static cannoli_memhook_read_r13_ebx_end: u8;
    static cannoli_memhook_read_r13_esp: u8;
    static cannoli_memhook_read_r13_esp_end: u8;
    static cannoli_memhook_read_r13_ebp: u8;
    static cannoli_memhook_read_r13_ebp_end: u8;
    static cannoli_memhook_read_r13_esi: u8;
    static cannoli_memhook_read_r13_esi_end: u8;
    static cannoli_memhook_read_r13_edi: u8;
    static cannoli_memhook_read_r13_edi_end: u8;
    static cannoli_memhook_read_r13_r8d: u8;
    static cannoli_memhook_read_r13_r8d_end: u8;
    static cannoli_memhook_read_r13_r9d: u8;
    static cannoli_memhook_read_r13_r9d_end: u8;
    static cannoli_memhook_read_r13_r10d: u8;
    static cannoli_memhook_read_r13_r10d_end: u8;
    static cannoli_memhook_read_r13_r11d: u8;
    static cannoli_memhook_read_r13_r11d_end: u8;
    static cannoli_memhook_read_r13_r12d: u8;
    static cannoli_memhook_read_r13_r12d_end: u8;
    static cannoli_memhook_read_r13_r13d: u8;
    static cannoli_memhook_read_r13_r13d_end: u8;
    static cannoli_memhook_read_r13_r14d: u8;
    static cannoli_memhook_read_r13_r14d_end: u8;
    static cannoli_memhook_read_r13_r15d: u8;
    static cannoli_memhook_read_r13_r15d_end: u8;
    static cannoli_memhook_read_r14_eax: u8;
    static cannoli_memhook_read_r14_eax_end: u8;
    static cannoli_memhook_read_r14_ecx: u8;
    static cannoli_memhook_read_r14_ecx_end: u8;
    static cannoli_memhook_read_r14_edx: u8;
    static cannoli_memhook_read_r14_edx_end: u8;
    static cannoli_memhook_read_r14_ebx: u8;
    static cannoli_memhook_read_r14_ebx_end: u8;
    static cannoli_memhook_read_r14_esp: u8;
    static cannoli_memhook_read_r14_esp_end: u8;
    static cannoli_memhook_read_r14_ebp: u8;
    static cannoli_memhook_read_r14_ebp_end: u8;
    static cannoli_memhook_read_r14_esi: u8;
    static cannoli_memhook_read_r14_esi_end: u8;
    static cannoli_memhook_read_r14_edi: u8;
    static cannoli_memhook_read_r14_edi_end: u8;
    static cannoli_memhook_read_r14_r8d: u8;
    static cannoli_memhook_read_r14_r8d_end: u8;
    static cannoli_memhook_read_r14_r9d: u8;
    static cannoli_memhook_read_r14_r9d_end: u8;
    static cannoli_memhook_read_r14_r10d: u8;
    static cannoli_memhook_read_r14_r10d_end: u8;
    static cannoli_memhook_read_r14_r11d: u8;
    static cannoli_memhook_read_r14_r11d_end: u8;
    static cannoli_memhook_read_r14_r12d: u8;
    static cannoli_memhook_read_r14_r12d_end: u8;
    static cannoli_memhook_read_r14_r13d: u8;
    static cannoli_memhook_read_r14_r13d_end: u8;
    static cannoli_memhook_read_r14_r14d: u8;
    static cannoli_memhook_read_r14_r14d_end: u8;
    static cannoli_memhook_read_r14_r15d: u8;
    static cannoli_memhook_read_r14_r15d_end: u8;
    static cannoli_memhook_read_r15_eax: u8;
    static cannoli_memhook_read_r15_eax_end: u8;
    static cannoli_memhook_read_r15_ecx: u8;
    static cannoli_memhook_read_r15_ecx_end: u8;
    static cannoli_memhook_read_r15_edx: u8;
    static cannoli_memhook_read_r15_edx_end: u8;
    static cannoli_memhook_read_r15_ebx: u8;
    static cannoli_memhook_read_r15_ebx_end: u8;
    static cannoli_memhook_read_r15_esp: u8;
    static cannoli_memhook_read_r15_esp_end: u8;
    static cannoli_memhook_read_r15_ebp: u8;
    static cannoli_memhook_read_r15_ebp_end: u8;
    static cannoli_memhook_read_r15_esi: u8;
    static cannoli_memhook_read_r15_esi_end: u8;
    static cannoli_memhook_read_r15_edi: u8;
    static cannoli_memhook_read_r15_edi_end: u8;
    static cannoli_memhook_read_r15_r8d: u8;
    static cannoli_memhook_read_r15_r8d_end: u8;
    static cannoli_memhook_read_r15_r9d: u8;
    static cannoli_memhook_read_r15_r9d_end: u8;
    static cannoli_memhook_read_r15_r10d: u8;
    static cannoli_memhook_read_r15_r10d_end: u8;
    static cannoli_memhook_read_r15_r11d: u8;
    static cannoli_memhook_read_r15_r11d_end: u8;
    static cannoli_memhook_read_r15_r12d: u8;
    static cannoli_memhook_read_r15_r12d_end: u8;
    static cannoli_memhook_read_r15_r13d: u8;
    static cannoli_memhook_read_r15_r13d_end: u8;
    static cannoli_memhook_read_r15_r14d: u8;
    static cannoli_memhook_read_r15_r14d_end: u8;
    static cannoli_memhook_read_r15_r15d: u8;
    static cannoli_memhook_read_r15_r15d_end: u8;
    static cannoli_memhook_read_rax_rax: u8;
    static cannoli_memhook_read_rax_rax_end: u8;
    static cannoli_memhook_read_rax_rcx: u8;
    static cannoli_memhook_read_rax_rcx_end: u8;
    static cannoli_memhook_read_rax_rdx: u8;
    static cannoli_memhook_read_rax_rdx_end: u8;
    static cannoli_memhook_read_rax_rbx: u8;
    static cannoli_memhook_read_rax_rbx_end: u8;
    static cannoli_memhook_read_rax_rsp: u8;
    static cannoli_memhook_read_rax_rsp_end: u8;
    static cannoli_memhook_read_rax_rbp: u8;
    static cannoli_memhook_read_rax_rbp_end: u8;
    static cannoli_memhook_read_rax_rsi: u8;
    static cannoli_memhook_read_rax_rsi_end: u8;
    static cannoli_memhook_read_rax_rdi: u8;
    static cannoli_memhook_read_rax_rdi_end: u8;
    static cannoli_memhook_read_rax_r8: u8;
    static cannoli_memhook_read_rax_r8_end: u8;
    static cannoli_memhook_read_rax_r9: u8;
    static cannoli_memhook_read_rax_r9_end: u8;
    static cannoli_memhook_read_rax_r10: u8;
    static cannoli_memhook_read_rax_r10_end: u8;
    static cannoli_memhook_read_rax_r11: u8;
    static cannoli_memhook_read_rax_r11_end: u8;
    static cannoli_memhook_read_rax_r12: u8;
    static cannoli_memhook_read_rax_r12_end: u8;
    static cannoli_memhook_read_rax_r13: u8;
    static cannoli_memhook_read_rax_r13_end: u8;
    static cannoli_memhook_read_rax_r14: u8;
    static cannoli_memhook_read_rax_r14_end: u8;
    static cannoli_memhook_read_rax_r15: u8;
    static cannoli_memhook_read_rax_r15_end: u8;
    static cannoli_memhook_read_rcx_rax: u8;
    static cannoli_memhook_read_rcx_rax_end: u8;
    static cannoli_memhook_read_rcx_rcx: u8;
    static cannoli_memhook_read_rcx_rcx_end: u8;
    static cannoli_memhook_read_rcx_rdx: u8;
    static cannoli_memhook_read_rcx_rdx_end: u8;
    static cannoli_memhook_read_rcx_rbx: u8;
    static cannoli_memhook_read_rcx_rbx_end: u8;
    static cannoli_memhook_read_rcx_rsp: u8;
    static cannoli_memhook_read_rcx_rsp_end: u8;
    static cannoli_memhook_read_rcx_rbp: u8;
    static cannoli_memhook_read_rcx_rbp_end: u8;
    static cannoli_memhook_read_rcx_rsi: u8;
    static cannoli_memhook_read_rcx_rsi_end: u8;
    static cannoli_memhook_read_rcx_rdi: u8;
    static cannoli_memhook_read_rcx_rdi_end: u8;
    static cannoli_memhook_read_rcx_r8: u8;
    static cannoli_memhook_read_rcx_r8_end: u8;
    static cannoli_memhook_read_rcx_r9: u8;
    static cannoli_memhook_read_rcx_r9_end: u8;
    static cannoli_memhook_read_rcx_r10: u8;
    static cannoli_memhook_read_rcx_r10_end: u8;
    static cannoli_memhook_read_rcx_r11: u8;
    static cannoli_memhook_read_rcx_r11_end: u8;
    static cannoli_memhook_read_rcx_r12: u8;
    static cannoli_memhook_read_rcx_r12_end: u8;
    static cannoli_memhook_read_rcx_r13: u8;
    static cannoli_memhook_read_rcx_r13_end: u8;
    static cannoli_memhook_read_rcx_r14: u8;
    static cannoli_memhook_read_rcx_r14_end: u8;
    static cannoli_memhook_read_rcx_r15: u8;
    static cannoli_memhook_read_rcx_r15_end: u8;
    static cannoli_memhook_read_rdx_rax: u8;
    static cannoli_memhook_read_rdx_rax_end: u8;
    static cannoli_memhook_read_rdx_rcx: u8;
    static cannoli_memhook_read_rdx_rcx_end: u8;
    static cannoli_memhook_read_rdx_rdx: u8;
    static cannoli_memhook_read_rdx_rdx_end: u8;
    static cannoli_memhook_read_rdx_rbx: u8;
    static cannoli_memhook_read_rdx_rbx_end: u8;
    static cannoli_memhook_read_rdx_rsp: u8;
    static cannoli_memhook_read_rdx_rsp_end: u8;
    static cannoli_memhook_read_rdx_rbp: u8;
    static cannoli_memhook_read_rdx_rbp_end: u8;
    static cannoli_memhook_read_rdx_rsi: u8;
    static cannoli_memhook_read_rdx_rsi_end: u8;
    static cannoli_memhook_read_rdx_rdi: u8;
    static cannoli_memhook_read_rdx_rdi_end: u8;
    static cannoli_memhook_read_rdx_r8: u8;
    static cannoli_memhook_read_rdx_r8_end: u8;
    static cannoli_memhook_read_rdx_r9: u8;
    static cannoli_memhook_read_rdx_r9_end: u8;
    static cannoli_memhook_read_rdx_r10: u8;
    static cannoli_memhook_read_rdx_r10_end: u8;
    static cannoli_memhook_read_rdx_r11: u8;
    static cannoli_memhook_read_rdx_r11_end: u8;
    static cannoli_memhook_read_rdx_r12: u8;
    static cannoli_memhook_read_rdx_r12_end: u8;
    static cannoli_memhook_read_rdx_r13: u8;
    static cannoli_memhook_read_rdx_r13_end: u8;
    static cannoli_memhook_read_rdx_r14: u8;
    static cannoli_memhook_read_rdx_r14_end: u8;
    static cannoli_memhook_read_rdx_r15: u8;
    static cannoli_memhook_read_rdx_r15_end: u8;
    static cannoli_memhook_read_rbx_rax: u8;
    static cannoli_memhook_read_rbx_rax_end: u8;
    static cannoli_memhook_read_rbx_rcx: u8;
    static cannoli_memhook_read_rbx_rcx_end: u8;
    static cannoli_memhook_read_rbx_rdx: u8;
    static cannoli_memhook_read_rbx_rdx_end: u8;
    static cannoli_memhook_read_rbx_rbx: u8;
    static cannoli_memhook_read_rbx_rbx_end: u8;
    static cannoli_memhook_read_rbx_rsp: u8;
    static cannoli_memhook_read_rbx_rsp_end: u8;
    static cannoli_memhook_read_rbx_rbp: u8;
    static cannoli_memhook_read_rbx_rbp_end: u8;
    static cannoli_memhook_read_rbx_rsi: u8;
    static cannoli_memhook_read_rbx_rsi_end: u8;
    static cannoli_memhook_read_rbx_rdi: u8;
    static cannoli_memhook_read_rbx_rdi_end: u8;
    static cannoli_memhook_read_rbx_r8: u8;
    static cannoli_memhook_read_rbx_r8_end: u8;
    static cannoli_memhook_read_rbx_r9: u8;
    static cannoli_memhook_read_rbx_r9_end: u8;
    static cannoli_memhook_read_rbx_r10: u8;
    static cannoli_memhook_read_rbx_r10_end: u8;
    static cannoli_memhook_read_rbx_r11: u8;
    static cannoli_memhook_read_rbx_r11_end: u8;
    static cannoli_memhook_read_rbx_r12: u8;
    static cannoli_memhook_read_rbx_r12_end: u8;
    static cannoli_memhook_read_rbx_r13: u8;
    static cannoli_memhook_read_rbx_r13_end: u8;
    static cannoli_memhook_read_rbx_r14: u8;
    static cannoli_memhook_read_rbx_r14_end: u8;
    static cannoli_memhook_read_rbx_r15: u8;
    static cannoli_memhook_read_rbx_r15_end: u8;
    static cannoli_memhook_read_rsp_rax: u8;
    static cannoli_memhook_read_rsp_rax_end: u8;
    static cannoli_memhook_read_rsp_rcx: u8;
    static cannoli_memhook_read_rsp_rcx_end: u8;
    static cannoli_memhook_read_rsp_rdx: u8;
    static cannoli_memhook_read_rsp_rdx_end: u8;
    static cannoli_memhook_read_rsp_rbx: u8;
    static cannoli_memhook_read_rsp_rbx_end: u8;
    static cannoli_memhook_read_rsp_rsp: u8;
    static cannoli_memhook_read_rsp_rsp_end: u8;
    static cannoli_memhook_read_rsp_rbp: u8;
    static cannoli_memhook_read_rsp_rbp_end: u8;
    static cannoli_memhook_read_rsp_rsi: u8;
    static cannoli_memhook_read_rsp_rsi_end: u8;
    static cannoli_memhook_read_rsp_rdi: u8;
    static cannoli_memhook_read_rsp_rdi_end: u8;
    static cannoli_memhook_read_rsp_r8: u8;
    static cannoli_memhook_read_rsp_r8_end: u8;
    static cannoli_memhook_read_rsp_r9: u8;
    static cannoli_memhook_read_rsp_r9_end: u8;
    static cannoli_memhook_read_rsp_r10: u8;
    static cannoli_memhook_read_rsp_r10_end: u8;
    static cannoli_memhook_read_rsp_r11: u8;
    static cannoli_memhook_read_rsp_r11_end: u8;
    static cannoli_memhook_read_rsp_r12: u8;
    static cannoli_memhook_read_rsp_r12_end: u8;
    static cannoli_memhook_read_rsp_r13: u8;
    static cannoli_memhook_read_rsp_r13_end: u8;
    static cannoli_memhook_read_rsp_r14: u8;
    static cannoli_memhook_read_rsp_r14_end: u8;
    static cannoli_memhook_read_rsp_r15: u8;
    static cannoli_memhook_read_rsp_r15_end: u8;
    static cannoli_memhook_read_rbp_rax: u8;
    static cannoli_memhook_read_rbp_rax_end: u8;
    static cannoli_memhook_read_rbp_rcx: u8;
    static cannoli_memhook_read_rbp_rcx_end: u8;
    static cannoli_memhook_read_rbp_rdx: u8;
    static cannoli_memhook_read_rbp_rdx_end: u8;
    static cannoli_memhook_read_rbp_rbx: u8;
    static cannoli_memhook_read_rbp_rbx_end: u8;
    static cannoli_memhook_read_rbp_rsp: u8;
    static cannoli_memhook_read_rbp_rsp_end: u8;
    static cannoli_memhook_read_rbp_rbp: u8;
    static cannoli_memhook_read_rbp_rbp_end: u8;
    static cannoli_memhook_read_rbp_rsi: u8;
    static cannoli_memhook_read_rbp_rsi_end: u8;
    static cannoli_memhook_read_rbp_rdi: u8;
    static cannoli_memhook_read_rbp_rdi_end: u8;
    static cannoli_memhook_read_rbp_r8: u8;
    static cannoli_memhook_read_rbp_r8_end: u8;
    static cannoli_memhook_read_rbp_r9: u8;
    static cannoli_memhook_read_rbp_r9_end: u8;
    static cannoli_memhook_read_rbp_r10: u8;
    static cannoli_memhook_read_rbp_r10_end: u8;
    static cannoli_memhook_read_rbp_r11: u8;
    static cannoli_memhook_read_rbp_r11_end: u8;
    static cannoli_memhook_read_rbp_r12: u8;
    static cannoli_memhook_read_rbp_r12_end: u8;
    static cannoli_memhook_read_rbp_r13: u8;
    static cannoli_memhook_read_rbp_r13_end: u8;
    static cannoli_memhook_read_rbp_r14: u8;
    static cannoli_memhook_read_rbp_r14_end: u8;
    static cannoli_memhook_read_rbp_r15: u8;
    static cannoli_memhook_read_rbp_r15_end: u8;
    static cannoli_memhook_read_rsi_rax: u8;
    static cannoli_memhook_read_rsi_rax_end: u8;
    static cannoli_memhook_read_rsi_rcx: u8;
    static cannoli_memhook_read_rsi_rcx_end: u8;
    static cannoli_memhook_read_rsi_rdx: u8;
    static cannoli_memhook_read_rsi_rdx_end: u8;
    static cannoli_memhook_read_rsi_rbx: u8;
    static cannoli_memhook_read_rsi_rbx_end: u8;
    static cannoli_memhook_read_rsi_rsp: u8;
    static cannoli_memhook_read_rsi_rsp_end: u8;
    static cannoli_memhook_read_rsi_rbp: u8;
    static cannoli_memhook_read_rsi_rbp_end: u8;
    static cannoli_memhook_read_rsi_rsi: u8;
    static cannoli_memhook_read_rsi_rsi_end: u8;
    static cannoli_memhook_read_rsi_rdi: u8;
    static cannoli_memhook_read_rsi_rdi_end: u8;
    static cannoli_memhook_read_rsi_r8: u8;
    static cannoli_memhook_read_rsi_r8_end: u8;
    static cannoli_memhook_read_rsi_r9: u8;
    static cannoli_memhook_read_rsi_r9_end: u8;
    static cannoli_memhook_read_rsi_r10: u8;
    static cannoli_memhook_read_rsi_r10_end: u8;
    static cannoli_memhook_read_rsi_r11: u8;
    static cannoli_memhook_read_rsi_r11_end: u8;
    static cannoli_memhook_read_rsi_r12: u8;
    static cannoli_memhook_read_rsi_r12_end: u8;
    static cannoli_memhook_read_rsi_r13: u8;
    static cannoli_memhook_read_rsi_r13_end: u8;
    static cannoli_memhook_read_rsi_r14: u8;
    static cannoli_memhook_read_rsi_r14_end: u8;
    static cannoli_memhook_read_rsi_r15: u8;
    static cannoli_memhook_read_rsi_r15_end: u8;
    static cannoli_memhook_read_rdi_rax: u8;
    static cannoli_memhook_read_rdi_rax_end: u8;
    static cannoli_memhook_read_rdi_rcx: u8;
    static cannoli_memhook_read_rdi_rcx_end: u8;
    static cannoli_memhook_read_rdi_rdx: u8;
    static cannoli_memhook_read_rdi_rdx_end: u8;
    static cannoli_memhook_read_rdi_rbx: u8;
    static cannoli_memhook_read_rdi_rbx_end: u8;
    static cannoli_memhook_read_rdi_rsp: u8;
    static cannoli_memhook_read_rdi_rsp_end: u8;
    static cannoli_memhook_read_rdi_rbp: u8;
    static cannoli_memhook_read_rdi_rbp_end: u8;
    static cannoli_memhook_read_rdi_rsi: u8;
    static cannoli_memhook_read_rdi_rsi_end: u8;
    static cannoli_memhook_read_rdi_rdi: u8;
    static cannoli_memhook_read_rdi_rdi_end: u8;
    static cannoli_memhook_read_rdi_r8: u8;
    static cannoli_memhook_read_rdi_r8_end: u8;
    static cannoli_memhook_read_rdi_r9: u8;
    static cannoli_memhook_read_rdi_r9_end: u8;
    static cannoli_memhook_read_rdi_r10: u8;
    static cannoli_memhook_read_rdi_r10_end: u8;
    static cannoli_memhook_read_rdi_r11: u8;
    static cannoli_memhook_read_rdi_r11_end: u8;
    static cannoli_memhook_read_rdi_r12: u8;
    static cannoli_memhook_read_rdi_r12_end: u8;
    static cannoli_memhook_read_rdi_r13: u8;
    static cannoli_memhook_read_rdi_r13_end: u8;
    static cannoli_memhook_read_rdi_r14: u8;
    static cannoli_memhook_read_rdi_r14_end: u8;
    static cannoli_memhook_read_rdi_r15: u8;
    static cannoli_memhook_read_rdi_r15_end: u8;
    static cannoli_memhook_read_r8_rax: u8;
    static cannoli_memhook_read_r8_rax_end: u8;
    static cannoli_memhook_read_r8_rcx: u8;
    static cannoli_memhook_read_r8_rcx_end: u8;
    static cannoli_memhook_read_r8_rdx: u8;
    static cannoli_memhook_read_r8_rdx_end: u8;
    static cannoli_memhook_read_r8_rbx: u8;
    static cannoli_memhook_read_r8_rbx_end: u8;
    static cannoli_memhook_read_r8_rsp: u8;
    static cannoli_memhook_read_r8_rsp_end: u8;
    static cannoli_memhook_read_r8_rbp: u8;
    static cannoli_memhook_read_r8_rbp_end: u8;
    static cannoli_memhook_read_r8_rsi: u8;
    static cannoli_memhook_read_r8_rsi_end: u8;
    static cannoli_memhook_read_r8_rdi: u8;
    static cannoli_memhook_read_r8_rdi_end: u8;
    static cannoli_memhook_read_r8_r8: u8;
    static cannoli_memhook_read_r8_r8_end: u8;
    static cannoli_memhook_read_r8_r9: u8;
    static cannoli_memhook_read_r8_r9_end: u8;
    static cannoli_memhook_read_r8_r10: u8;
    static cannoli_memhook_read_r8_r10_end: u8;
    static cannoli_memhook_read_r8_r11: u8;
    static cannoli_memhook_read_r8_r11_end: u8;
    static cannoli_memhook_read_r8_r12: u8;
    static cannoli_memhook_read_r8_r12_end: u8;
    static cannoli_memhook_read_r8_r13: u8;
    static cannoli_memhook_read_r8_r13_end: u8;
    static cannoli_memhook_read_r8_r14: u8;
    static cannoli_memhook_read_r8_r14_end: u8;
    static cannoli_memhook_read_r8_r15: u8;
    static cannoli_memhook_read_r8_r15_end: u8;
    static cannoli_memhook_read_r9_rax: u8;
    static cannoli_memhook_read_r9_rax_end: u8;
    static cannoli_memhook_read_r9_rcx: u8;
    static cannoli_memhook_read_r9_rcx_end: u8;
    static cannoli_memhook_read_r9_rdx: u8;
    static cannoli_memhook_read_r9_rdx_end: u8;
    static cannoli_memhook_read_r9_rbx: u8;
    static cannoli_memhook_read_r9_rbx_end: u8;
    static cannoli_memhook_read_r9_rsp: u8;
    static cannoli_memhook_read_r9_rsp_end: u8;
    static cannoli_memhook_read_r9_rbp: u8;
    static cannoli_memhook_read_r9_rbp_end: u8;
    static cannoli_memhook_read_r9_rsi: u8;
    static cannoli_memhook_read_r9_rsi_end: u8;
    static cannoli_memhook_read_r9_rdi: u8;
    static cannoli_memhook_read_r9_rdi_end: u8;
    static cannoli_memhook_read_r9_r8: u8;
    static cannoli_memhook_read_r9_r8_end: u8;
    static cannoli_memhook_read_r9_r9: u8;
    static cannoli_memhook_read_r9_r9_end: u8;
    static cannoli_memhook_read_r9_r10: u8;
    static cannoli_memhook_read_r9_r10_end: u8;
    static cannoli_memhook_read_r9_r11: u8;
    static cannoli_memhook_read_r9_r11_end: u8;
    static cannoli_memhook_read_r9_r12: u8;
    static cannoli_memhook_read_r9_r12_end: u8;
    static cannoli_memhook_read_r9_r13: u8;
    static cannoli_memhook_read_r9_r13_end: u8;
    static cannoli_memhook_read_r9_r14: u8;
    static cannoli_memhook_read_r9_r14_end: u8;
    static cannoli_memhook_read_r9_r15: u8;
    static cannoli_memhook_read_r9_r15_end: u8;
    static cannoli_memhook_read_r10_rax: u8;
    static cannoli_memhook_read_r10_rax_end: u8;
    static cannoli_memhook_read_r10_rcx: u8;
    static cannoli_memhook_read_r10_rcx_end: u8;
    static cannoli_memhook_read_r10_rdx: u8;
    static cannoli_memhook_read_r10_rdx_end: u8;
    static cannoli_memhook_read_r10_rbx: u8;
    static cannoli_memhook_read_r10_rbx_end: u8;
    static cannoli_memhook_read_r10_rsp: u8;
    static cannoli_memhook_read_r10_rsp_end: u8;
    static cannoli_memhook_read_r10_rbp: u8;
    static cannoli_memhook_read_r10_rbp_end: u8;
    static cannoli_memhook_read_r10_rsi: u8;
    static cannoli_memhook_read_r10_rsi_end: u8;
    static cannoli_memhook_read_r10_rdi: u8;
    static cannoli_memhook_read_r10_rdi_end: u8;
    static cannoli_memhook_read_r10_r8: u8;
    static cannoli_memhook_read_r10_r8_end: u8;
    static cannoli_memhook_read_r10_r9: u8;
    static cannoli_memhook_read_r10_r9_end: u8;
    static cannoli_memhook_read_r10_r10: u8;
    static cannoli_memhook_read_r10_r10_end: u8;
    static cannoli_memhook_read_r10_r11: u8;
    static cannoli_memhook_read_r10_r11_end: u8;
    static cannoli_memhook_read_r10_r12: u8;
    static cannoli_memhook_read_r10_r12_end: u8;
    static cannoli_memhook_read_r10_r13: u8;
    static cannoli_memhook_read_r10_r13_end: u8;
    static cannoli_memhook_read_r10_r14: u8;
    static cannoli_memhook_read_r10_r14_end: u8;
    static cannoli_memhook_read_r10_r15: u8;
    static cannoli_memhook_read_r10_r15_end: u8;
    static cannoli_memhook_read_r11_rax: u8;
    static cannoli_memhook_read_r11_rax_end: u8;
    static cannoli_memhook_read_r11_rcx: u8;
    static cannoli_memhook_read_r11_rcx_end: u8;
    static cannoli_memhook_read_r11_rdx: u8;
    static cannoli_memhook_read_r11_rdx_end: u8;
    static cannoli_memhook_read_r11_rbx: u8;
    static cannoli_memhook_read_r11_rbx_end: u8;
    static cannoli_memhook_read_r11_rsp: u8;
    static cannoli_memhook_read_r11_rsp_end: u8;
    static cannoli_memhook_read_r11_rbp: u8;
    static cannoli_memhook_read_r11_rbp_end: u8;
    static cannoli_memhook_read_r11_rsi: u8;
    static cannoli_memhook_read_r11_rsi_end: u8;
    static cannoli_memhook_read_r11_rdi: u8;
    static cannoli_memhook_read_r11_rdi_end: u8;
    static cannoli_memhook_read_r11_r8: u8;
    static cannoli_memhook_read_r11_r8_end: u8;
    static cannoli_memhook_read_r11_r9: u8;
    static cannoli_memhook_read_r11_r9_end: u8;
    static cannoli_memhook_read_r11_r10: u8;
    static cannoli_memhook_read_r11_r10_end: u8;
    static cannoli_memhook_read_r11_r11: u8;
    static cannoli_memhook_read_r11_r11_end: u8;
    static cannoli_memhook_read_r11_r12: u8;
    static cannoli_memhook_read_r11_r12_end: u8;
    static cannoli_memhook_read_r11_r13: u8;
    static cannoli_memhook_read_r11_r13_end: u8;
    static cannoli_memhook_read_r11_r14: u8;
    static cannoli_memhook_read_r11_r14_end: u8;
    static cannoli_memhook_read_r11_r15: u8;
    static cannoli_memhook_read_r11_r15_end: u8;
    static cannoli_memhook_read_r12_rax: u8;
    static cannoli_memhook_read_r12_rax_end: u8;
    static cannoli_memhook_read_r12_rcx: u8;
    static cannoli_memhook_read_r12_rcx_end: u8;
    static cannoli_memhook_read_r12_rdx: u8;
    static cannoli_memhook_read_r12_rdx_end: u8;
    static cannoli_memhook_read_r12_rbx: u8;
    static cannoli_memhook_read_r12_rbx_end: u8;
    static cannoli_memhook_read_r12_rsp: u8;
    static cannoli_memhook_read_r12_rsp_end: u8;
    static cannoli_memhook_read_r12_rbp: u8;
    static cannoli_memhook_read_r12_rbp_end: u8;
    static cannoli_memhook_read_r12_rsi: u8;
    static cannoli_memhook_read_r12_rsi_end: u8;
    static cannoli_memhook_read_r12_rdi: u8;
    static cannoli_memhook_read_r12_rdi_end: u8;
    static cannoli_memhook_read_r12_r8: u8;
    static cannoli_memhook_read_r12_r8_end: u8;
    static cannoli_memhook_read_r12_r9: u8;
    static cannoli_memhook_read_r12_r9_end: u8;
    static cannoli_memhook_read_r12_r10: u8;
    static cannoli_memhook_read_r12_r10_end: u8;
    static cannoli_memhook_read_r12_r11: u8;
    static cannoli_memhook_read_r12_r11_end: u8;
    static cannoli_memhook_read_r12_r12: u8;
    static cannoli_memhook_read_r12_r12_end: u8;
    static cannoli_memhook_read_r12_r13: u8;
    static cannoli_memhook_read_r12_r13_end: u8;
    static cannoli_memhook_read_r12_r14: u8;
    static cannoli_memhook_read_r12_r14_end: u8;
    static cannoli_memhook_read_r12_r15: u8;
    static cannoli_memhook_read_r12_r15_end: u8;
    static cannoli_memhook_read_r13_rax: u8;
    static cannoli_memhook_read_r13_rax_end: u8;
    static cannoli_memhook_read_r13_rcx: u8;
    static cannoli_memhook_read_r13_rcx_end: u8;
    static cannoli_memhook_read_r13_rdx: u8;
    static cannoli_memhook_read_r13_rdx_end: u8;
    static cannoli_memhook_read_r13_rbx: u8;
    static cannoli_memhook_read_r13_rbx_end: u8;
    static cannoli_memhook_read_r13_rsp: u8;
    static cannoli_memhook_read_r13_rsp_end: u8;
    static cannoli_memhook_read_r13_rbp: u8;
    static cannoli_memhook_read_r13_rbp_end: u8;
    static cannoli_memhook_read_r13_rsi: u8;
    static cannoli_memhook_read_r13_rsi_end: u8;
    static cannoli_memhook_read_r13_rdi: u8;
    static cannoli_memhook_read_r13_rdi_end: u8;
    static cannoli_memhook_read_r13_r8: u8;
    static cannoli_memhook_read_r13_r8_end: u8;
    static cannoli_memhook_read_r13_r9: u8;
    static cannoli_memhook_read_r13_r9_end: u8;
    static cannoli_memhook_read_r13_r10: u8;
    static cannoli_memhook_read_r13_r10_end: u8;
    static cannoli_memhook_read_r13_r11: u8;
    static cannoli_memhook_read_r13_r11_end: u8;
    static cannoli_memhook_read_r13_r12: u8;
    static cannoli_memhook_read_r13_r12_end: u8;
    static cannoli_memhook_read_r13_r13: u8;
    static cannoli_memhook_read_r13_r13_end: u8;
    static cannoli_memhook_read_r13_r14: u8;
    static cannoli_memhook_read_r13_r14_end: u8;
    static cannoli_memhook_read_r13_r15: u8;
    static cannoli_memhook_read_r13_r15_end: u8;
    static cannoli_memhook_read_r14_rax: u8;
    static cannoli_memhook_read_r14_rax_end: u8;
    static cannoli_memhook_read_r14_rcx: u8;
    static cannoli_memhook_read_r14_rcx_end: u8;
    static cannoli_memhook_read_r14_rdx: u8;
    static cannoli_memhook_read_r14_rdx_end: u8;
    static cannoli_memhook_read_r14_rbx: u8;
    static cannoli_memhook_read_r14_rbx_end: u8;
    static cannoli_memhook_read_r14_rsp: u8;
    static cannoli_memhook_read_r14_rsp_end: u8;
    static cannoli_memhook_read_r14_rbp: u8;
    static cannoli_memhook_read_r14_rbp_end: u8;
    static cannoli_memhook_read_r14_rsi: u8;
    static cannoli_memhook_read_r14_rsi_end: u8;
    static cannoli_memhook_read_r14_rdi: u8;
    static cannoli_memhook_read_r14_rdi_end: u8;
    static cannoli_memhook_read_r14_r8: u8;
    static cannoli_memhook_read_r14_r8_end: u8;
    static cannoli_memhook_read_r14_r9: u8;
    static cannoli_memhook_read_r14_r9_end: u8;
    static cannoli_memhook_read_r14_r10: u8;
    static cannoli_memhook_read_r14_r10_end: u8;
    static cannoli_memhook_read_r14_r11: u8;
    static cannoli_memhook_read_r14_r11_end: u8;
    static cannoli_memhook_read_r14_r12: u8;
    static cannoli_memhook_read_r14_r12_end: u8;
    static cannoli_memhook_read_r14_r13: u8;
    static cannoli_memhook_read_r14_r13_end: u8;
    static cannoli_memhook_read_r14_r14: u8;
    static cannoli_memhook_read_r14_r14_end: u8;
    static cannoli_memhook_read_r14_r15: u8;
    static cannoli_memhook_read_r14_r15_end: u8;
    static cannoli_memhook_read_r15_rax: u8;
    static cannoli_memhook_read_r15_rax_end: u8;
    static cannoli_memhook_read_r15_rcx: u8;
    static cannoli_memhook_read_r15_rcx_end: u8;
    static cannoli_memhook_read_r15_rdx: u8;
    static cannoli_memhook_read_r15_rdx_end: u8;
    static cannoli_memhook_read_r15_rbx: u8;
    static cannoli_memhook_read_r15_rbx_end: u8;
    static cannoli_memhook_read_r15_rsp: u8;
    static cannoli_memhook_read_r15_rsp_end: u8;
    static cannoli_memhook_read_r15_rbp: u8;
    static cannoli_memhook_read_r15_rbp_end: u8;
    static cannoli_memhook_read_r15_rsi: u8;
    static cannoli_memhook_read_r15_rsi_end: u8;
    static cannoli_memhook_read_r15_rdi: u8;
    static cannoli_memhook_read_r15_rdi_end: u8;
    static cannoli_memhook_read_r15_r8: u8;
    static cannoli_memhook_read_r15_r8_end: u8;
    static cannoli_memhook_read_r15_r9: u8;
    static cannoli_memhook_read_r15_r9_end: u8;
    static cannoli_memhook_read_r15_r10: u8;
    static cannoli_memhook_read_r15_r10_end: u8;
    static cannoli_memhook_read_r15_r11: u8;
    static cannoli_memhook_read_r15_r11_end: u8;
    static cannoli_memhook_read_r15_r12: u8;
    static cannoli_memhook_read_r15_r12_end: u8;
    static cannoli_memhook_read_r15_r13: u8;
    static cannoli_memhook_read_r15_r13_end: u8;
    static cannoli_memhook_read_r15_r14: u8;
    static cannoli_memhook_read_r15_r14_end: u8;
    static cannoli_memhook_read_r15_r15: u8;
    static cannoli_memhook_read_r15_r15_end: u8;
    static cannoli_memhook_write_al_eax: u8;
    static cannoli_memhook_write_al_eax_end: u8;
    static cannoli_memhook_write_al_ecx: u8;
    static cannoli_memhook_write_al_ecx_end: u8;
    static cannoli_memhook_write_al_edx: u8;
    static cannoli_memhook_write_al_edx_end: u8;
    static cannoli_memhook_write_al_ebx: u8;
    static cannoli_memhook_write_al_ebx_end: u8;
    static cannoli_memhook_write_al_esp: u8;
    static cannoli_memhook_write_al_esp_end: u8;
    static cannoli_memhook_write_al_ebp: u8;
    static cannoli_memhook_write_al_ebp_end: u8;
    static cannoli_memhook_write_al_esi: u8;
    static cannoli_memhook_write_al_esi_end: u8;
    static cannoli_memhook_write_al_edi: u8;
    static cannoli_memhook_write_al_edi_end: u8;
    static cannoli_memhook_write_al_r8d: u8;
    static cannoli_memhook_write_al_r8d_end: u8;
    static cannoli_memhook_write_al_r9d: u8;
    static cannoli_memhook_write_al_r9d_end: u8;
    static cannoli_memhook_write_al_r10d: u8;
    static cannoli_memhook_write_al_r10d_end: u8;
    static cannoli_memhook_write_al_r11d: u8;
    static cannoli_memhook_write_al_r11d_end: u8;
    static cannoli_memhook_write_al_r12d: u8;
    static cannoli_memhook_write_al_r12d_end: u8;
    static cannoli_memhook_write_al_r13d: u8;
    static cannoli_memhook_write_al_r13d_end: u8;
    static cannoli_memhook_write_al_r14d: u8;
    static cannoli_memhook_write_al_r14d_end: u8;
    static cannoli_memhook_write_al_r15d: u8;
    static cannoli_memhook_write_al_r15d_end: u8;
    static cannoli_memhook_write_cl_eax: u8;
    static cannoli_memhook_write_cl_eax_end: u8;
    static cannoli_memhook_write_cl_ecx: u8;
    static cannoli_memhook_write_cl_ecx_end: u8;
    static cannoli_memhook_write_cl_edx: u8;
    static cannoli_memhook_write_cl_edx_end: u8;
    static cannoli_memhook_write_cl_ebx: u8;
    static cannoli_memhook_write_cl_ebx_end: u8;
    static cannoli_memhook_write_cl_esp: u8;
    static cannoli_memhook_write_cl_esp_end: u8;
    static cannoli_memhook_write_cl_ebp: u8;
    static cannoli_memhook_write_cl_ebp_end: u8;
    static cannoli_memhook_write_cl_esi: u8;
    static cannoli_memhook_write_cl_esi_end: u8;
    static cannoli_memhook_write_cl_edi: u8;
    static cannoli_memhook_write_cl_edi_end: u8;
    static cannoli_memhook_write_cl_r8d: u8;
    static cannoli_memhook_write_cl_r8d_end: u8;
    static cannoli_memhook_write_cl_r9d: u8;
    static cannoli_memhook_write_cl_r9d_end: u8;
    static cannoli_memhook_write_cl_r10d: u8;
    static cannoli_memhook_write_cl_r10d_end: u8;
    static cannoli_memhook_write_cl_r11d: u8;
    static cannoli_memhook_write_cl_r11d_end: u8;
    static cannoli_memhook_write_cl_r12d: u8;
    static cannoli_memhook_write_cl_r12d_end: u8;
    static cannoli_memhook_write_cl_r13d: u8;
    static cannoli_memhook_write_cl_r13d_end: u8;
    static cannoli_memhook_write_cl_r14d: u8;
    static cannoli_memhook_write_cl_r14d_end: u8;
    static cannoli_memhook_write_cl_r15d: u8;
    static cannoli_memhook_write_cl_r15d_end: u8;
    static cannoli_memhook_write_dl_eax: u8;
    static cannoli_memhook_write_dl_eax_end: u8;
    static cannoli_memhook_write_dl_ecx: u8;
    static cannoli_memhook_write_dl_ecx_end: u8;
    static cannoli_memhook_write_dl_edx: u8;
    static cannoli_memhook_write_dl_edx_end: u8;
    static cannoli_memhook_write_dl_ebx: u8;
    static cannoli_memhook_write_dl_ebx_end: u8;
    static cannoli_memhook_write_dl_esp: u8;
    static cannoli_memhook_write_dl_esp_end: u8;
    static cannoli_memhook_write_dl_ebp: u8;
    static cannoli_memhook_write_dl_ebp_end: u8;
    static cannoli_memhook_write_dl_esi: u8;
    static cannoli_memhook_write_dl_esi_end: u8;
    static cannoli_memhook_write_dl_edi: u8;
    static cannoli_memhook_write_dl_edi_end: u8;
    static cannoli_memhook_write_dl_r8d: u8;
    static cannoli_memhook_write_dl_r8d_end: u8;
    static cannoli_memhook_write_dl_r9d: u8;
    static cannoli_memhook_write_dl_r9d_end: u8;
    static cannoli_memhook_write_dl_r10d: u8;
    static cannoli_memhook_write_dl_r10d_end: u8;
    static cannoli_memhook_write_dl_r11d: u8;
    static cannoli_memhook_write_dl_r11d_end: u8;
    static cannoli_memhook_write_dl_r12d: u8;
    static cannoli_memhook_write_dl_r12d_end: u8;
    static cannoli_memhook_write_dl_r13d: u8;
    static cannoli_memhook_write_dl_r13d_end: u8;
    static cannoli_memhook_write_dl_r14d: u8;
    static cannoli_memhook_write_dl_r14d_end: u8;
    static cannoli_memhook_write_dl_r15d: u8;
    static cannoli_memhook_write_dl_r15d_end: u8;
    static cannoli_memhook_write_bl_eax: u8;
    static cannoli_memhook_write_bl_eax_end: u8;
    static cannoli_memhook_write_bl_ecx: u8;
    static cannoli_memhook_write_bl_ecx_end: u8;
    static cannoli_memhook_write_bl_edx: u8;
    static cannoli_memhook_write_bl_edx_end: u8;
    static cannoli_memhook_write_bl_ebx: u8;
    static cannoli_memhook_write_bl_ebx_end: u8;
    static cannoli_memhook_write_bl_esp: u8;
    static cannoli_memhook_write_bl_esp_end: u8;
    static cannoli_memhook_write_bl_ebp: u8;
    static cannoli_memhook_write_bl_ebp_end: u8;
    static cannoli_memhook_write_bl_esi: u8;
    static cannoli_memhook_write_bl_esi_end: u8;
    static cannoli_memhook_write_bl_edi: u8;
    static cannoli_memhook_write_bl_edi_end: u8;
    static cannoli_memhook_write_bl_r8d: u8;
    static cannoli_memhook_write_bl_r8d_end: u8;
    static cannoli_memhook_write_bl_r9d: u8;
    static cannoli_memhook_write_bl_r9d_end: u8;
    static cannoli_memhook_write_bl_r10d: u8;
    static cannoli_memhook_write_bl_r10d_end: u8;
    static cannoli_memhook_write_bl_r11d: u8;
    static cannoli_memhook_write_bl_r11d_end: u8;
    static cannoli_memhook_write_bl_r12d: u8;
    static cannoli_memhook_write_bl_r12d_end: u8;
    static cannoli_memhook_write_bl_r13d: u8;
    static cannoli_memhook_write_bl_r13d_end: u8;
    static cannoli_memhook_write_bl_r14d: u8;
    static cannoli_memhook_write_bl_r14d_end: u8;
    static cannoli_memhook_write_bl_r15d: u8;
    static cannoli_memhook_write_bl_r15d_end: u8;
    static cannoli_memhook_write_spl_eax: u8;
    static cannoli_memhook_write_spl_eax_end: u8;
    static cannoli_memhook_write_spl_ecx: u8;
    static cannoli_memhook_write_spl_ecx_end: u8;
    static cannoli_memhook_write_spl_edx: u8;
    static cannoli_memhook_write_spl_edx_end: u8;
    static cannoli_memhook_write_spl_ebx: u8;
    static cannoli_memhook_write_spl_ebx_end: u8;
    static cannoli_memhook_write_spl_esp: u8;
    static cannoli_memhook_write_spl_esp_end: u8;
    static cannoli_memhook_write_spl_ebp: u8;
    static cannoli_memhook_write_spl_ebp_end: u8;
    static cannoli_memhook_write_spl_esi: u8;
    static cannoli_memhook_write_spl_esi_end: u8;
    static cannoli_memhook_write_spl_edi: u8;
    static cannoli_memhook_write_spl_edi_end: u8;
    static cannoli_memhook_write_spl_r8d: u8;
    static cannoli_memhook_write_spl_r8d_end: u8;
    static cannoli_memhook_write_spl_r9d: u8;
    static cannoli_memhook_write_spl_r9d_end: u8;
    static cannoli_memhook_write_spl_r10d: u8;
    static cannoli_memhook_write_spl_r10d_end: u8;
    static cannoli_memhook_write_spl_r11d: u8;
    static cannoli_memhook_write_spl_r11d_end: u8;
    static cannoli_memhook_write_spl_r12d: u8;
    static cannoli_memhook_write_spl_r12d_end: u8;
    static cannoli_memhook_write_spl_r13d: u8;
    static cannoli_memhook_write_spl_r13d_end: u8;
    static cannoli_memhook_write_spl_r14d: u8;
    static cannoli_memhook_write_spl_r14d_end: u8;
    static cannoli_memhook_write_spl_r15d: u8;
    static cannoli_memhook_write_spl_r15d_end: u8;
    static cannoli_memhook_write_bpl_eax: u8;
    static cannoli_memhook_write_bpl_eax_end: u8;
    static cannoli_memhook_write_bpl_ecx: u8;
    static cannoli_memhook_write_bpl_ecx_end: u8;
    static cannoli_memhook_write_bpl_edx: u8;
    static cannoli_memhook_write_bpl_edx_end: u8;
    static cannoli_memhook_write_bpl_ebx: u8;
    static cannoli_memhook_write_bpl_ebx_end: u8;
    static cannoli_memhook_write_bpl_esp: u8;
    static cannoli_memhook_write_bpl_esp_end: u8;
    static cannoli_memhook_write_bpl_ebp: u8;
    static cannoli_memhook_write_bpl_ebp_end: u8;
    static cannoli_memhook_write_bpl_esi: u8;
    static cannoli_memhook_write_bpl_esi_end: u8;
    static cannoli_memhook_write_bpl_edi: u8;
    static cannoli_memhook_write_bpl_edi_end: u8;
    static cannoli_memhook_write_bpl_r8d: u8;
    static cannoli_memhook_write_bpl_r8d_end: u8;
    static cannoli_memhook_write_bpl_r9d: u8;
    static cannoli_memhook_write_bpl_r9d_end: u8;
    static cannoli_memhook_write_bpl_r10d: u8;
    static cannoli_memhook_write_bpl_r10d_end: u8;
    static cannoli_memhook_write_bpl_r11d: u8;
    static cannoli_memhook_write_bpl_r11d_end: u8;
    static cannoli_memhook_write_bpl_r12d: u8;
    static cannoli_memhook_write_bpl_r12d_end: u8;
    static cannoli_memhook_write_bpl_r13d: u8;
    static cannoli_memhook_write_bpl_r13d_end: u8;
    static cannoli_memhook_write_bpl_r14d: u8;
    static cannoli_memhook_write_bpl_r14d_end: u8;
    static cannoli_memhook_write_bpl_r15d: u8;
    static cannoli_memhook_write_bpl_r15d_end: u8;
    static cannoli_memhook_write_sil_eax: u8;
    static cannoli_memhook_write_sil_eax_end: u8;
    static cannoli_memhook_write_sil_ecx: u8;
    static cannoli_memhook_write_sil_ecx_end: u8;
    static cannoli_memhook_write_sil_edx: u8;
    static cannoli_memhook_write_sil_edx_end: u8;
    static cannoli_memhook_write_sil_ebx: u8;
    static cannoli_memhook_write_sil_ebx_end: u8;
    static cannoli_memhook_write_sil_esp: u8;
    static cannoli_memhook_write_sil_esp_end: u8;
    static cannoli_memhook_write_sil_ebp: u8;
    static cannoli_memhook_write_sil_ebp_end: u8;
    static cannoli_memhook_write_sil_esi: u8;
    static cannoli_memhook_write_sil_esi_end: u8;
    static cannoli_memhook_write_sil_edi: u8;
    static cannoli_memhook_write_sil_edi_end: u8;
    static cannoli_memhook_write_sil_r8d: u8;
    static cannoli_memhook_write_sil_r8d_end: u8;
    static cannoli_memhook_write_sil_r9d: u8;
    static cannoli_memhook_write_sil_r9d_end: u8;
    static cannoli_memhook_write_sil_r10d: u8;
    static cannoli_memhook_write_sil_r10d_end: u8;
    static cannoli_memhook_write_sil_r11d: u8;
    static cannoli_memhook_write_sil_r11d_end: u8;
    static cannoli_memhook_write_sil_r12d: u8;
    static cannoli_memhook_write_sil_r12d_end: u8;
    static cannoli_memhook_write_sil_r13d: u8;
    static cannoli_memhook_write_sil_r13d_end: u8;
    static cannoli_memhook_write_sil_r14d: u8;
    static cannoli_memhook_write_sil_r14d_end: u8;
    static cannoli_memhook_write_sil_r15d: u8;
    static cannoli_memhook_write_sil_r15d_end: u8;
    static cannoli_memhook_write_dil_eax: u8;
    static cannoli_memhook_write_dil_eax_end: u8;
    static cannoli_memhook_write_dil_ecx: u8;
    static cannoli_memhook_write_dil_ecx_end: u8;
    static cannoli_memhook_write_dil_edx: u8;
    static cannoli_memhook_write_dil_edx_end: u8;
    static cannoli_memhook_write_dil_ebx: u8;
    static cannoli_memhook_write_dil_ebx_end: u8;
    static cannoli_memhook_write_dil_esp: u8;
    static cannoli_memhook_write_dil_esp_end: u8;
    static cannoli_memhook_write_dil_ebp: u8;
    static cannoli_memhook_write_dil_ebp_end: u8;
    static cannoli_memhook_write_dil_esi: u8;
    static cannoli_memhook_write_dil_esi_end: u8;
    static cannoli_memhook_write_dil_edi: u8;
    static cannoli_memhook_write_dil_edi_end: u8;
    static cannoli_memhook_write_dil_r8d: u8;
    static cannoli_memhook_write_dil_r8d_end: u8;
    static cannoli_memhook_write_dil_r9d: u8;
    static cannoli_memhook_write_dil_r9d_end: u8;
    static cannoli_memhook_write_dil_r10d: u8;
    static cannoli_memhook_write_dil_r10d_end: u8;
    static cannoli_memhook_write_dil_r11d: u8;
    static cannoli_memhook_write_dil_r11d_end: u8;
    static cannoli_memhook_write_dil_r12d: u8;
    static cannoli_memhook_write_dil_r12d_end: u8;
    static cannoli_memhook_write_dil_r13d: u8;
    static cannoli_memhook_write_dil_r13d_end: u8;
    static cannoli_memhook_write_dil_r14d: u8;
    static cannoli_memhook_write_dil_r14d_end: u8;
    static cannoli_memhook_write_dil_r15d: u8;
    static cannoli_memhook_write_dil_r15d_end: u8;
    static cannoli_memhook_write_r8b_eax: u8;
    static cannoli_memhook_write_r8b_eax_end: u8;
    static cannoli_memhook_write_r8b_ecx: u8;
    static cannoli_memhook_write_r8b_ecx_end: u8;
    static cannoli_memhook_write_r8b_edx: u8;
    static cannoli_memhook_write_r8b_edx_end: u8;
    static cannoli_memhook_write_r8b_ebx: u8;
    static cannoli_memhook_write_r8b_ebx_end: u8;
    static cannoli_memhook_write_r8b_esp: u8;
    static cannoli_memhook_write_r8b_esp_end: u8;
    static cannoli_memhook_write_r8b_ebp: u8;
    static cannoli_memhook_write_r8b_ebp_end: u8;
    static cannoli_memhook_write_r8b_esi: u8;
    static cannoli_memhook_write_r8b_esi_end: u8;
    static cannoli_memhook_write_r8b_edi: u8;
    static cannoli_memhook_write_r8b_edi_end: u8;
    static cannoli_memhook_write_r8b_r8d: u8;
    static cannoli_memhook_write_r8b_r8d_end: u8;
    static cannoli_memhook_write_r8b_r9d: u8;
    static cannoli_memhook_write_r8b_r9d_end: u8;
    static cannoli_memhook_write_r8b_r10d: u8;
    static cannoli_memhook_write_r8b_r10d_end: u8;
    static cannoli_memhook_write_r8b_r11d: u8;
    static cannoli_memhook_write_r8b_r11d_end: u8;
    static cannoli_memhook_write_r8b_r12d: u8;
    static cannoli_memhook_write_r8b_r12d_end: u8;
    static cannoli_memhook_write_r8b_r13d: u8;
    static cannoli_memhook_write_r8b_r13d_end: u8;
    static cannoli_memhook_write_r8b_r14d: u8;
    static cannoli_memhook_write_r8b_r14d_end: u8;
    static cannoli_memhook_write_r8b_r15d: u8;
    static cannoli_memhook_write_r8b_r15d_end: u8;
    static cannoli_memhook_write_r9b_eax: u8;
    static cannoli_memhook_write_r9b_eax_end: u8;
    static cannoli_memhook_write_r9b_ecx: u8;
    static cannoli_memhook_write_r9b_ecx_end: u8;
    static cannoli_memhook_write_r9b_edx: u8;
    static cannoli_memhook_write_r9b_edx_end: u8;
    static cannoli_memhook_write_r9b_ebx: u8;
    static cannoli_memhook_write_r9b_ebx_end: u8;
    static cannoli_memhook_write_r9b_esp: u8;
    static cannoli_memhook_write_r9b_esp_end: u8;
    static cannoli_memhook_write_r9b_ebp: u8;
    static cannoli_memhook_write_r9b_ebp_end: u8;
    static cannoli_memhook_write_r9b_esi: u8;
    static cannoli_memhook_write_r9b_esi_end: u8;
    static cannoli_memhook_write_r9b_edi: u8;
    static cannoli_memhook_write_r9b_edi_end: u8;
    static cannoli_memhook_write_r9b_r8d: u8;
    static cannoli_memhook_write_r9b_r8d_end: u8;
    static cannoli_memhook_write_r9b_r9d: u8;
    static cannoli_memhook_write_r9b_r9d_end: u8;
    static cannoli_memhook_write_r9b_r10d: u8;
    static cannoli_memhook_write_r9b_r10d_end: u8;
    static cannoli_memhook_write_r9b_r11d: u8;
    static cannoli_memhook_write_r9b_r11d_end: u8;
    static cannoli_memhook_write_r9b_r12d: u8;
    static cannoli_memhook_write_r9b_r12d_end: u8;
    static cannoli_memhook_write_r9b_r13d: u8;
    static cannoli_memhook_write_r9b_r13d_end: u8;
    static cannoli_memhook_write_r9b_r14d: u8;
    static cannoli_memhook_write_r9b_r14d_end: u8;
    static cannoli_memhook_write_r9b_r15d: u8;
    static cannoli_memhook_write_r9b_r15d_end: u8;
    static cannoli_memhook_write_r10b_eax: u8;
    static cannoli_memhook_write_r10b_eax_end: u8;
    static cannoli_memhook_write_r10b_ecx: u8;
    static cannoli_memhook_write_r10b_ecx_end: u8;
    static cannoli_memhook_write_r10b_edx: u8;
    static cannoli_memhook_write_r10b_edx_end: u8;
    static cannoli_memhook_write_r10b_ebx: u8;
    static cannoli_memhook_write_r10b_ebx_end: u8;
    static cannoli_memhook_write_r10b_esp: u8;
    static cannoli_memhook_write_r10b_esp_end: u8;
    static cannoli_memhook_write_r10b_ebp: u8;
    static cannoli_memhook_write_r10b_ebp_end: u8;
    static cannoli_memhook_write_r10b_esi: u8;
    static cannoli_memhook_write_r10b_esi_end: u8;
    static cannoli_memhook_write_r10b_edi: u8;
    static cannoli_memhook_write_r10b_edi_end: u8;
    static cannoli_memhook_write_r10b_r8d: u8;
    static cannoli_memhook_write_r10b_r8d_end: u8;
    static cannoli_memhook_write_r10b_r9d: u8;
    static cannoli_memhook_write_r10b_r9d_end: u8;
    static cannoli_memhook_write_r10b_r10d: u8;
    static cannoli_memhook_write_r10b_r10d_end: u8;
    static cannoli_memhook_write_r10b_r11d: u8;
    static cannoli_memhook_write_r10b_r11d_end: u8;
    static cannoli_memhook_write_r10b_r12d: u8;
    static cannoli_memhook_write_r10b_r12d_end: u8;
    static cannoli_memhook_write_r10b_r13d: u8;
    static cannoli_memhook_write_r10b_r13d_end: u8;
    static cannoli_memhook_write_r10b_r14d: u8;
    static cannoli_memhook_write_r10b_r14d_end: u8;
    static cannoli_memhook_write_r10b_r15d: u8;
    static cannoli_memhook_write_r10b_r15d_end: u8;
    static cannoli_memhook_write_r11b_eax: u8;
    static cannoli_memhook_write_r11b_eax_end: u8;
    static cannoli_memhook_write_r11b_ecx: u8;
    static cannoli_memhook_write_r11b_ecx_end: u8;
    static cannoli_memhook_write_r11b_edx: u8;
    static cannoli_memhook_write_r11b_edx_end: u8;
    static cannoli_memhook_write_r11b_ebx: u8;
    static cannoli_memhook_write_r11b_ebx_end: u8;
    static cannoli_memhook_write_r11b_esp: u8;
    static cannoli_memhook_write_r11b_esp_end: u8;
    static cannoli_memhook_write_r11b_ebp: u8;
    static cannoli_memhook_write_r11b_ebp_end: u8;
    static cannoli_memhook_write_r11b_esi: u8;
    static cannoli_memhook_write_r11b_esi_end: u8;
    static cannoli_memhook_write_r11b_edi: u8;
    static cannoli_memhook_write_r11b_edi_end: u8;
    static cannoli_memhook_write_r11b_r8d: u8;
    static cannoli_memhook_write_r11b_r8d_end: u8;
    static cannoli_memhook_write_r11b_r9d: u8;
    static cannoli_memhook_write_r11b_r9d_end: u8;
    static cannoli_memhook_write_r11b_r10d: u8;
    static cannoli_memhook_write_r11b_r10d_end: u8;
    static cannoli_memhook_write_r11b_r11d: u8;
    static cannoli_memhook_write_r11b_r11d_end: u8;
    static cannoli_memhook_write_r11b_r12d: u8;
    static cannoli_memhook_write_r11b_r12d_end: u8;
    static cannoli_memhook_write_r11b_r13d: u8;
    static cannoli_memhook_write_r11b_r13d_end: u8;
    static cannoli_memhook_write_r11b_r14d: u8;
    static cannoli_memhook_write_r11b_r14d_end: u8;
    static cannoli_memhook_write_r11b_r15d: u8;
    static cannoli_memhook_write_r11b_r15d_end: u8;
    static cannoli_memhook_write_r12b_eax: u8;
    static cannoli_memhook_write_r12b_eax_end: u8;
    static cannoli_memhook_write_r12b_ecx: u8;
    static cannoli_memhook_write_r12b_ecx_end: u8;
    static cannoli_memhook_write_r12b_edx: u8;
    static cannoli_memhook_write_r12b_edx_end: u8;
    static cannoli_memhook_write_r12b_ebx: u8;
    static cannoli_memhook_write_r12b_ebx_end: u8;
    static cannoli_memhook_write_r12b_esp: u8;
    static cannoli_memhook_write_r12b_esp_end: u8;
    static cannoli_memhook_write_r12b_ebp: u8;
    static cannoli_memhook_write_r12b_ebp_end: u8;
    static cannoli_memhook_write_r12b_esi: u8;
    static cannoli_memhook_write_r12b_esi_end: u8;
    static cannoli_memhook_write_r12b_edi: u8;
    static cannoli_memhook_write_r12b_edi_end: u8;
    static cannoli_memhook_write_r12b_r8d: u8;
    static cannoli_memhook_write_r12b_r8d_end: u8;
    static cannoli_memhook_write_r12b_r9d: u8;
    static cannoli_memhook_write_r12b_r9d_end: u8;
    static cannoli_memhook_write_r12b_r10d: u8;
    static cannoli_memhook_write_r12b_r10d_end: u8;
    static cannoli_memhook_write_r12b_r11d: u8;
    static cannoli_memhook_write_r12b_r11d_end: u8;
    static cannoli_memhook_write_r12b_r12d: u8;
    static cannoli_memhook_write_r12b_r12d_end: u8;
    static cannoli_memhook_write_r12b_r13d: u8;
    static cannoli_memhook_write_r12b_r13d_end: u8;
    static cannoli_memhook_write_r12b_r14d: u8;
    static cannoli_memhook_write_r12b_r14d_end: u8;
    static cannoli_memhook_write_r12b_r15d: u8;
    static cannoli_memhook_write_r12b_r15d_end: u8;
    static cannoli_memhook_write_r13b_eax: u8;
    static cannoli_memhook_write_r13b_eax_end: u8;
    static cannoli_memhook_write_r13b_ecx: u8;
    static cannoli_memhook_write_r13b_ecx_end: u8;
    static cannoli_memhook_write_r13b_edx: u8;
    static cannoli_memhook_write_r13b_edx_end: u8;
    static cannoli_memhook_write_r13b_ebx: u8;
    static cannoli_memhook_write_r13b_ebx_end: u8;
    static cannoli_memhook_write_r13b_esp: u8;
    static cannoli_memhook_write_r13b_esp_end: u8;
    static cannoli_memhook_write_r13b_ebp: u8;
    static cannoli_memhook_write_r13b_ebp_end: u8;
    static cannoli_memhook_write_r13b_esi: u8;
    static cannoli_memhook_write_r13b_esi_end: u8;
    static cannoli_memhook_write_r13b_edi: u8;
    static cannoli_memhook_write_r13b_edi_end: u8;
    static cannoli_memhook_write_r13b_r8d: u8;
    static cannoli_memhook_write_r13b_r8d_end: u8;
    static cannoli_memhook_write_r13b_r9d: u8;
    static cannoli_memhook_write_r13b_r9d_end: u8;
    static cannoli_memhook_write_r13b_r10d: u8;
    static cannoli_memhook_write_r13b_r10d_end: u8;
    static cannoli_memhook_write_r13b_r11d: u8;
    static cannoli_memhook_write_r13b_r11d_end: u8;
    static cannoli_memhook_write_r13b_r12d: u8;
    static cannoli_memhook_write_r13b_r12d_end: u8;
    static cannoli_memhook_write_r13b_r13d: u8;
    static cannoli_memhook_write_r13b_r13d_end: u8;
    static cannoli_memhook_write_r13b_r14d: u8;
    static cannoli_memhook_write_r13b_r14d_end: u8;
    static cannoli_memhook_write_r13b_r15d: u8;
    static cannoli_memhook_write_r13b_r15d_end: u8;
    static cannoli_memhook_write_r14b_eax: u8;
    static cannoli_memhook_write_r14b_eax_end: u8;
    static cannoli_memhook_write_r14b_ecx: u8;
    static cannoli_memhook_write_r14b_ecx_end: u8;
    static cannoli_memhook_write_r14b_edx: u8;
    static cannoli_memhook_write_r14b_edx_end: u8;
    static cannoli_memhook_write_r14b_ebx: u8;
    static cannoli_memhook_write_r14b_ebx_end: u8;
    static cannoli_memhook_write_r14b_esp: u8;
    static cannoli_memhook_write_r14b_esp_end: u8;
    static cannoli_memhook_write_r14b_ebp: u8;
    static cannoli_memhook_write_r14b_ebp_end: u8;
    static cannoli_memhook_write_r14b_esi: u8;
    static cannoli_memhook_write_r14b_esi_end: u8;
    static cannoli_memhook_write_r14b_edi: u8;
    static cannoli_memhook_write_r14b_edi_end: u8;
    static cannoli_memhook_write_r14b_r8d: u8;
    static cannoli_memhook_write_r14b_r8d_end: u8;
    static cannoli_memhook_write_r14b_r9d: u8;
    static cannoli_memhook_write_r14b_r9d_end: u8;
    static cannoli_memhook_write_r14b_r10d: u8;
    static cannoli_memhook_write_r14b_r10d_end: u8;
    static cannoli_memhook_write_r14b_r11d: u8;
    static cannoli_memhook_write_r14b_r11d_end: u8;
    static cannoli_memhook_write_r14b_r12d: u8;
    static cannoli_memhook_write_r14b_r12d_end: u8;
    static cannoli_memhook_write_r14b_r13d: u8;
    static cannoli_memhook_write_r14b_r13d_end: u8;
    static cannoli_memhook_write_r14b_r14d: u8;
    static cannoli_memhook_write_r14b_r14d_end: u8;
    static cannoli_memhook_write_r14b_r15d: u8;
    static cannoli_memhook_write_r14b_r15d_end: u8;
    static cannoli_memhook_write_r15b_eax: u8;
    static cannoli_memhook_write_r15b_eax_end: u8;
    static cannoli_memhook_write_r15b_ecx: u8;
    static cannoli_memhook_write_r15b_ecx_end: u8;
    static cannoli_memhook_write_r15b_edx: u8;
    static cannoli_memhook_write_r15b_edx_end: u8;
    static cannoli_memhook_write_r15b_ebx: u8;
    static cannoli_memhook_write_r15b_ebx_end: u8;
    static cannoli_memhook_write_r15b_esp: u8;
    static cannoli_memhook_write_r15b_esp_end: u8;
    static cannoli_memhook_write_r15b_ebp: u8;
    static cannoli_memhook_write_r15b_ebp_end: u8;
    static cannoli_memhook_write_r15b_esi: u8;
    static cannoli_memhook_write_r15b_esi_end: u8;
    static cannoli_memhook_write_r15b_edi: u8;
    static cannoli_memhook_write_r15b_edi_end: u8;
    static cannoli_memhook_write_r15b_r8d: u8;
    static cannoli_memhook_write_r15b_r8d_end: u8;
    static cannoli_memhook_write_r15b_r9d: u8;
    static cannoli_memhook_write_r15b_r9d_end: u8;
    static cannoli_memhook_write_r15b_r10d: u8;
    static cannoli_memhook_write_r15b_r10d_end: u8;
    static cannoli_memhook_write_r15b_r11d: u8;
    static cannoli_memhook_write_r15b_r11d_end: u8;
    static cannoli_memhook_write_r15b_r12d: u8;
    static cannoli_memhook_write_r15b_r12d_end: u8;
    static cannoli_memhook_write_r15b_r13d: u8;
    static cannoli_memhook_write_r15b_r13d_end: u8;
    static cannoli_memhook_write_r15b_r14d: u8;
    static cannoli_memhook_write_r15b_r14d_end: u8;
    static cannoli_memhook_write_r15b_r15d: u8;
    static cannoli_memhook_write_r15b_r15d_end: u8;
    static cannoli_memhook_write_al_rax: u8;
    static cannoli_memhook_write_al_rax_end: u8;
    static cannoli_memhook_write_al_rcx: u8;
    static cannoli_memhook_write_al_rcx_end: u8;
    static cannoli_memhook_write_al_rdx: u8;
    static cannoli_memhook_write_al_rdx_end: u8;
    static cannoli_memhook_write_al_rbx: u8;
    static cannoli_memhook_write_al_rbx_end: u8;
    static cannoli_memhook_write_al_rsp: u8;
    static cannoli_memhook_write_al_rsp_end: u8;
    static cannoli_memhook_write_al_rbp: u8;
    static cannoli_memhook_write_al_rbp_end: u8;
    static cannoli_memhook_write_al_rsi: u8;
    static cannoli_memhook_write_al_rsi_end: u8;
    static cannoli_memhook_write_al_rdi: u8;
    static cannoli_memhook_write_al_rdi_end: u8;
    static cannoli_memhook_write_al_r8: u8;
    static cannoli_memhook_write_al_r8_end: u8;
    static cannoli_memhook_write_al_r9: u8;
    static cannoli_memhook_write_al_r9_end: u8;
    static cannoli_memhook_write_al_r10: u8;
    static cannoli_memhook_write_al_r10_end: u8;
    static cannoli_memhook_write_al_r11: u8;
    static cannoli_memhook_write_al_r11_end: u8;
    static cannoli_memhook_write_al_r12: u8;
    static cannoli_memhook_write_al_r12_end: u8;
    static cannoli_memhook_write_al_r13: u8;
    static cannoli_memhook_write_al_r13_end: u8;
    static cannoli_memhook_write_al_r14: u8;
    static cannoli_memhook_write_al_r14_end: u8;
    static cannoli_memhook_write_al_r15: u8;
    static cannoli_memhook_write_al_r15_end: u8;
    static cannoli_memhook_write_cl_rax: u8;
    static cannoli_memhook_write_cl_rax_end: u8;
    static cannoli_memhook_write_cl_rcx: u8;
    static cannoli_memhook_write_cl_rcx_end: u8;
    static cannoli_memhook_write_cl_rdx: u8;
    static cannoli_memhook_write_cl_rdx_end: u8;
    static cannoli_memhook_write_cl_rbx: u8;
    static cannoli_memhook_write_cl_rbx_end: u8;
    static cannoli_memhook_write_cl_rsp: u8;
    static cannoli_memhook_write_cl_rsp_end: u8;
    static cannoli_memhook_write_cl_rbp: u8;
    static cannoli_memhook_write_cl_rbp_end: u8;
    static cannoli_memhook_write_cl_rsi: u8;
    static cannoli_memhook_write_cl_rsi_end: u8;
    static cannoli_memhook_write_cl_rdi: u8;
    static cannoli_memhook_write_cl_rdi_end: u8;
    static cannoli_memhook_write_cl_r8: u8;
    static cannoli_memhook_write_cl_r8_end: u8;
    static cannoli_memhook_write_cl_r9: u8;
    static cannoli_memhook_write_cl_r9_end: u8;
    static cannoli_memhook_write_cl_r10: u8;
    static cannoli_memhook_write_cl_r10_end: u8;
    static cannoli_memhook_write_cl_r11: u8;
    static cannoli_memhook_write_cl_r11_end: u8;
    static cannoli_memhook_write_cl_r12: u8;
    static cannoli_memhook_write_cl_r12_end: u8;
    static cannoli_memhook_write_cl_r13: u8;
    static cannoli_memhook_write_cl_r13_end: u8;
    static cannoli_memhook_write_cl_r14: u8;
    static cannoli_memhook_write_cl_r14_end: u8;
    static cannoli_memhook_write_cl_r15: u8;
    static cannoli_memhook_write_cl_r15_end: u8;
    static cannoli_memhook_write_dl_rax: u8;
    static cannoli_memhook_write_dl_rax_end: u8;
    static cannoli_memhook_write_dl_rcx: u8;
    static cannoli_memhook_write_dl_rcx_end: u8;
    static cannoli_memhook_write_dl_rdx: u8;
    static cannoli_memhook_write_dl_rdx_end: u8;
    static cannoli_memhook_write_dl_rbx: u8;
    static cannoli_memhook_write_dl_rbx_end: u8;
    static cannoli_memhook_write_dl_rsp: u8;
    static cannoli_memhook_write_dl_rsp_end: u8;
    static cannoli_memhook_write_dl_rbp: u8;
    static cannoli_memhook_write_dl_rbp_end: u8;
    static cannoli_memhook_write_dl_rsi: u8;
    static cannoli_memhook_write_dl_rsi_end: u8;
    static cannoli_memhook_write_dl_rdi: u8;
    static cannoli_memhook_write_dl_rdi_end: u8;
    static cannoli_memhook_write_dl_r8: u8;
    static cannoli_memhook_write_dl_r8_end: u8;
    static cannoli_memhook_write_dl_r9: u8;
    static cannoli_memhook_write_dl_r9_end: u8;
    static cannoli_memhook_write_dl_r10: u8;
    static cannoli_memhook_write_dl_r10_end: u8;
    static cannoli_memhook_write_dl_r11: u8;
    static cannoli_memhook_write_dl_r11_end: u8;
    static cannoli_memhook_write_dl_r12: u8;
    static cannoli_memhook_write_dl_r12_end: u8;
    static cannoli_memhook_write_dl_r13: u8;
    static cannoli_memhook_write_dl_r13_end: u8;
    static cannoli_memhook_write_dl_r14: u8;
    static cannoli_memhook_write_dl_r14_end: u8;
    static cannoli_memhook_write_dl_r15: u8;
    static cannoli_memhook_write_dl_r15_end: u8;
    static cannoli_memhook_write_bl_rax: u8;
    static cannoli_memhook_write_bl_rax_end: u8;
    static cannoli_memhook_write_bl_rcx: u8;
    static cannoli_memhook_write_bl_rcx_end: u8;
    static cannoli_memhook_write_bl_rdx: u8;
    static cannoli_memhook_write_bl_rdx_end: u8;
    static cannoli_memhook_write_bl_rbx: u8;
    static cannoli_memhook_write_bl_rbx_end: u8;
    static cannoli_memhook_write_bl_rsp: u8;
    static cannoli_memhook_write_bl_rsp_end: u8;
    static cannoli_memhook_write_bl_rbp: u8;
    static cannoli_memhook_write_bl_rbp_end: u8;
    static cannoli_memhook_write_bl_rsi: u8;
    static cannoli_memhook_write_bl_rsi_end: u8;
    static cannoli_memhook_write_bl_rdi: u8;
    static cannoli_memhook_write_bl_rdi_end: u8;
    static cannoli_memhook_write_bl_r8: u8;
    static cannoli_memhook_write_bl_r8_end: u8;
    static cannoli_memhook_write_bl_r9: u8;
    static cannoli_memhook_write_bl_r9_end: u8;
    static cannoli_memhook_write_bl_r10: u8;
    static cannoli_memhook_write_bl_r10_end: u8;
    static cannoli_memhook_write_bl_r11: u8;
    static cannoli_memhook_write_bl_r11_end: u8;
    static cannoli_memhook_write_bl_r12: u8;
    static cannoli_memhook_write_bl_r12_end: u8;
    static cannoli_memhook_write_bl_r13: u8;
    static cannoli_memhook_write_bl_r13_end: u8;
    static cannoli_memhook_write_bl_r14: u8;
    static cannoli_memhook_write_bl_r14_end: u8;
    static cannoli_memhook_write_bl_r15: u8;
    static cannoli_memhook_write_bl_r15_end: u8;
    static cannoli_memhook_write_spl_rax: u8;
    static cannoli_memhook_write_spl_rax_end: u8;
    static cannoli_memhook_write_spl_rcx: u8;
    static cannoli_memhook_write_spl_rcx_end: u8;
    static cannoli_memhook_write_spl_rdx: u8;
    static cannoli_memhook_write_spl_rdx_end: u8;
    static cannoli_memhook_write_spl_rbx: u8;
    static cannoli_memhook_write_spl_rbx_end: u8;
    static cannoli_memhook_write_spl_rsp: u8;
    static cannoli_memhook_write_spl_rsp_end: u8;
    static cannoli_memhook_write_spl_rbp: u8;
    static cannoli_memhook_write_spl_rbp_end: u8;
    static cannoli_memhook_write_spl_rsi: u8;
    static cannoli_memhook_write_spl_rsi_end: u8;
    static cannoli_memhook_write_spl_rdi: u8;
    static cannoli_memhook_write_spl_rdi_end: u8;
    static cannoli_memhook_write_spl_r8: u8;
    static cannoli_memhook_write_spl_r8_end: u8;
    static cannoli_memhook_write_spl_r9: u8;
    static cannoli_memhook_write_spl_r9_end: u8;
    static cannoli_memhook_write_spl_r10: u8;
    static cannoli_memhook_write_spl_r10_end: u8;
    static cannoli_memhook_write_spl_r11: u8;
    static cannoli_memhook_write_spl_r11_end: u8;
    static cannoli_memhook_write_spl_r12: u8;
    static cannoli_memhook_write_spl_r12_end: u8;
    static cannoli_memhook_write_spl_r13: u8;
    static cannoli_memhook_write_spl_r13_end: u8;
    static cannoli_memhook_write_spl_r14: u8;
    static cannoli_memhook_write_spl_r14_end: u8;
    static cannoli_memhook_write_spl_r15: u8;
    static cannoli_memhook_write_spl_r15_end: u8;
    static cannoli_memhook_write_bpl_rax: u8;
    static cannoli_memhook_write_bpl_rax_end: u8;
    static cannoli_memhook_write_bpl_rcx: u8;
    static cannoli_memhook_write_bpl_rcx_end: u8;
    static cannoli_memhook_write_bpl_rdx: u8;
    static cannoli_memhook_write_bpl_rdx_end: u8;
    static cannoli_memhook_write_bpl_rbx: u8;
    static cannoli_memhook_write_bpl_rbx_end: u8;
    static cannoli_memhook_write_bpl_rsp: u8;
    static cannoli_memhook_write_bpl_rsp_end: u8;
    static cannoli_memhook_write_bpl_rbp: u8;
    static cannoli_memhook_write_bpl_rbp_end: u8;
    static cannoli_memhook_write_bpl_rsi: u8;
    static cannoli_memhook_write_bpl_rsi_end: u8;
    static cannoli_memhook_write_bpl_rdi: u8;
    static cannoli_memhook_write_bpl_rdi_end: u8;
    static cannoli_memhook_write_bpl_r8: u8;
    static cannoli_memhook_write_bpl_r8_end: u8;
    static cannoli_memhook_write_bpl_r9: u8;
    static cannoli_memhook_write_bpl_r9_end: u8;
    static cannoli_memhook_write_bpl_r10: u8;
    static cannoli_memhook_write_bpl_r10_end: u8;
    static cannoli_memhook_write_bpl_r11: u8;
    static cannoli_memhook_write_bpl_r11_end: u8;
    static cannoli_memhook_write_bpl_r12: u8;
    static cannoli_memhook_write_bpl_r12_end: u8;
    static cannoli_memhook_write_bpl_r13: u8;
    static cannoli_memhook_write_bpl_r13_end: u8;
    static cannoli_memhook_write_bpl_r14: u8;
    static cannoli_memhook_write_bpl_r14_end: u8;
    static cannoli_memhook_write_bpl_r15: u8;
    static cannoli_memhook_write_bpl_r15_end: u8;
    static cannoli_memhook_write_sil_rax: u8;
    static cannoli_memhook_write_sil_rax_end: u8;
    static cannoli_memhook_write_sil_rcx: u8;
    static cannoli_memhook_write_sil_rcx_end: u8;
    static cannoli_memhook_write_sil_rdx: u8;
    static cannoli_memhook_write_sil_rdx_end: u8;
    static cannoli_memhook_write_sil_rbx: u8;
    static cannoli_memhook_write_sil_rbx_end: u8;
    static cannoli_memhook_write_sil_rsp: u8;
    static cannoli_memhook_write_sil_rsp_end: u8;
    static cannoli_memhook_write_sil_rbp: u8;
    static cannoli_memhook_write_sil_rbp_end: u8;
    static cannoli_memhook_write_sil_rsi: u8;
    static cannoli_memhook_write_sil_rsi_end: u8;
    static cannoli_memhook_write_sil_rdi: u8;
    static cannoli_memhook_write_sil_rdi_end: u8;
    static cannoli_memhook_write_sil_r8: u8;
    static cannoli_memhook_write_sil_r8_end: u8;
    static cannoli_memhook_write_sil_r9: u8;
    static cannoli_memhook_write_sil_r9_end: u8;
    static cannoli_memhook_write_sil_r10: u8;
    static cannoli_memhook_write_sil_r10_end: u8;
    static cannoli_memhook_write_sil_r11: u8;
    static cannoli_memhook_write_sil_r11_end: u8;
    static cannoli_memhook_write_sil_r12: u8;
    static cannoli_memhook_write_sil_r12_end: u8;
    static cannoli_memhook_write_sil_r13: u8;
    static cannoli_memhook_write_sil_r13_end: u8;
    static cannoli_memhook_write_sil_r14: u8;
    static cannoli_memhook_write_sil_r14_end: u8;
    static cannoli_memhook_write_sil_r15: u8;
    static cannoli_memhook_write_sil_r15_end: u8;
    static cannoli_memhook_write_dil_rax: u8;
    static cannoli_memhook_write_dil_rax_end: u8;
    static cannoli_memhook_write_dil_rcx: u8;
    static cannoli_memhook_write_dil_rcx_end: u8;
    static cannoli_memhook_write_dil_rdx: u8;
    static cannoli_memhook_write_dil_rdx_end: u8;
    static cannoli_memhook_write_dil_rbx: u8;
    static cannoli_memhook_write_dil_rbx_end: u8;
    static cannoli_memhook_write_dil_rsp: u8;
    static cannoli_memhook_write_dil_rsp_end: u8;
    static cannoli_memhook_write_dil_rbp: u8;
    static cannoli_memhook_write_dil_rbp_end: u8;
    static cannoli_memhook_write_dil_rsi: u8;
    static cannoli_memhook_write_dil_rsi_end: u8;
    static cannoli_memhook_write_dil_rdi: u8;
    static cannoli_memhook_write_dil_rdi_end: u8;
    static cannoli_memhook_write_dil_r8: u8;
    static cannoli_memhook_write_dil_r8_end: u8;
    static cannoli_memhook_write_dil_r9: u8;
    static cannoli_memhook_write_dil_r9_end: u8;
    static cannoli_memhook_write_dil_r10: u8;
    static cannoli_memhook_write_dil_r10_end: u8;
    static cannoli_memhook_write_dil_r11: u8;
    static cannoli_memhook_write_dil_r11_end: u8;
    static cannoli_memhook_write_dil_r12: u8;
    static cannoli_memhook_write_dil_r12_end: u8;
    static cannoli_memhook_write_dil_r13: u8;
    static cannoli_memhook_write_dil_r13_end: u8;
    static cannoli_memhook_write_dil_r14: u8;
    static cannoli_memhook_write_dil_r14_end: u8;
    static cannoli_memhook_write_dil_r15: u8;
    static cannoli_memhook_write_dil_r15_end: u8;
    static cannoli_memhook_write_r8b_rax: u8;
    static cannoli_memhook_write_r8b_rax_end: u8;
    static cannoli_memhook_write_r8b_rcx: u8;
    static cannoli_memhook_write_r8b_rcx_end: u8;
    static cannoli_memhook_write_r8b_rdx: u8;
    static cannoli_memhook_write_r8b_rdx_end: u8;
    static cannoli_memhook_write_r8b_rbx: u8;
    static cannoli_memhook_write_r8b_rbx_end: u8;
    static cannoli_memhook_write_r8b_rsp: u8;
    static cannoli_memhook_write_r8b_rsp_end: u8;
    static cannoli_memhook_write_r8b_rbp: u8;
    static cannoli_memhook_write_r8b_rbp_end: u8;
    static cannoli_memhook_write_r8b_rsi: u8;
    static cannoli_memhook_write_r8b_rsi_end: u8;
    static cannoli_memhook_write_r8b_rdi: u8;
    static cannoli_memhook_write_r8b_rdi_end: u8;
    static cannoli_memhook_write_r8b_r8: u8;
    static cannoli_memhook_write_r8b_r8_end: u8;
    static cannoli_memhook_write_r8b_r9: u8;
    static cannoli_memhook_write_r8b_r9_end: u8;
    static cannoli_memhook_write_r8b_r10: u8;
    static cannoli_memhook_write_r8b_r10_end: u8;
    static cannoli_memhook_write_r8b_r11: u8;
    static cannoli_memhook_write_r8b_r11_end: u8;
    static cannoli_memhook_write_r8b_r12: u8;
    static cannoli_memhook_write_r8b_r12_end: u8;
    static cannoli_memhook_write_r8b_r13: u8;
    static cannoli_memhook_write_r8b_r13_end: u8;
    static cannoli_memhook_write_r8b_r14: u8;
    static cannoli_memhook_write_r8b_r14_end: u8;
    static cannoli_memhook_write_r8b_r15: u8;
    static cannoli_memhook_write_r8b_r15_end: u8;
    static cannoli_memhook_write_r9b_rax: u8;
    static cannoli_memhook_write_r9b_rax_end: u8;
    static cannoli_memhook_write_r9b_rcx: u8;
    static cannoli_memhook_write_r9b_rcx_end: u8;
    static cannoli_memhook_write_r9b_rdx: u8;
    static cannoli_memhook_write_r9b_rdx_end: u8;
    static cannoli_memhook_write_r9b_rbx: u8;
    static cannoli_memhook_write_r9b_rbx_end: u8;
    static cannoli_memhook_write_r9b_rsp: u8;
    static cannoli_memhook_write_r9b_rsp_end: u8;
    static cannoli_memhook_write_r9b_rbp: u8;
    static cannoli_memhook_write_r9b_rbp_end: u8;
    static cannoli_memhook_write_r9b_rsi: u8;
    static cannoli_memhook_write_r9b_rsi_end: u8;
    static cannoli_memhook_write_r9b_rdi: u8;
    static cannoli_memhook_write_r9b_rdi_end: u8;
    static cannoli_memhook_write_r9b_r8: u8;
    static cannoli_memhook_write_r9b_r8_end: u8;
    static cannoli_memhook_write_r9b_r9: u8;
    static cannoli_memhook_write_r9b_r9_end: u8;
    static cannoli_memhook_write_r9b_r10: u8;
    static cannoli_memhook_write_r9b_r10_end: u8;
    static cannoli_memhook_write_r9b_r11: u8;
    static cannoli_memhook_write_r9b_r11_end: u8;
    static cannoli_memhook_write_r9b_r12: u8;
    static cannoli_memhook_write_r9b_r12_end: u8;
    static cannoli_memhook_write_r9b_r13: u8;
    static cannoli_memhook_write_r9b_r13_end: u8;
    static cannoli_memhook_write_r9b_r14: u8;
    static cannoli_memhook_write_r9b_r14_end: u8;
    static cannoli_memhook_write_r9b_r15: u8;
    static cannoli_memhook_write_r9b_r15_end: u8;
    static cannoli_memhook_write_r10b_rax: u8;
    static cannoli_memhook_write_r10b_rax_end: u8;
    static cannoli_memhook_write_r10b_rcx: u8;
    static cannoli_memhook_write_r10b_rcx_end: u8;
    static cannoli_memhook_write_r10b_rdx: u8;
    static cannoli_memhook_write_r10b_rdx_end: u8;
    static cannoli_memhook_write_r10b_rbx: u8;
    static cannoli_memhook_write_r10b_rbx_end: u8;
    static cannoli_memhook_write_r10b_rsp: u8;
    static cannoli_memhook_write_r10b_rsp_end: u8;
    static cannoli_memhook_write_r10b_rbp: u8;
    static cannoli_memhook_write_r10b_rbp_end: u8;
    static cannoli_memhook_write_r10b_rsi: u8;
    static cannoli_memhook_write_r10b_rsi_end: u8;
    static cannoli_memhook_write_r10b_rdi: u8;
    static cannoli_memhook_write_r10b_rdi_end: u8;
    static cannoli_memhook_write_r10b_r8: u8;
    static cannoli_memhook_write_r10b_r8_end: u8;
    static cannoli_memhook_write_r10b_r9: u8;
    static cannoli_memhook_write_r10b_r9_end: u8;
    static cannoli_memhook_write_r10b_r10: u8;
    static cannoli_memhook_write_r10b_r10_end: u8;
    static cannoli_memhook_write_r10b_r11: u8;
    static cannoli_memhook_write_r10b_r11_end: u8;
    static cannoli_memhook_write_r10b_r12: u8;
    static cannoli_memhook_write_r10b_r12_end: u8;
    static cannoli_memhook_write_r10b_r13: u8;
    static cannoli_memhook_write_r10b_r13_end: u8;
    static cannoli_memhook_write_r10b_r14: u8;
    static cannoli_memhook_write_r10b_r14_end: u8;
    static cannoli_memhook_write_r10b_r15: u8;
    static cannoli_memhook_write_r10b_r15_end: u8;
    static cannoli_memhook_write_r11b_rax: u8;
    static cannoli_memhook_write_r11b_rax_end: u8;
    static cannoli_memhook_write_r11b_rcx: u8;
    static cannoli_memhook_write_r11b_rcx_end: u8;
    static cannoli_memhook_write_r11b_rdx: u8;
    static cannoli_memhook_write_r11b_rdx_end: u8;
    static cannoli_memhook_write_r11b_rbx: u8;
    static cannoli_memhook_write_r11b_rbx_end: u8;
    static cannoli_memhook_write_r11b_rsp: u8;
    static cannoli_memhook_write_r11b_rsp_end: u8;
    static cannoli_memhook_write_r11b_rbp: u8;
    static cannoli_memhook_write_r11b_rbp_end: u8;
    static cannoli_memhook_write_r11b_rsi: u8;
    static cannoli_memhook_write_r11b_rsi_end: u8;
    static cannoli_memhook_write_r11b_rdi: u8;
    static cannoli_memhook_write_r11b_rdi_end: u8;
    static cannoli_memhook_write_r11b_r8: u8;
    static cannoli_memhook_write_r11b_r8_end: u8;
    static cannoli_memhook_write_r11b_r9: u8;
    static cannoli_memhook_write_r11b_r9_end: u8;
    static cannoli_memhook_write_r11b_r10: u8;
    static cannoli_memhook_write_r11b_r10_end: u8;
    static cannoli_memhook_write_r11b_r11: u8;
    static cannoli_memhook_write_r11b_r11_end: u8;
    static cannoli_memhook_write_r11b_r12: u8;
    static cannoli_memhook_write_r11b_r12_end: u8;
    static cannoli_memhook_write_r11b_r13: u8;
    static cannoli_memhook_write_r11b_r13_end: u8;
    static cannoli_memhook_write_r11b_r14: u8;
    static cannoli_memhook_write_r11b_r14_end: u8;
    static cannoli_memhook_write_r11b_r15: u8;
    static cannoli_memhook_write_r11b_r15_end: u8;
    static cannoli_memhook_write_r12b_rax: u8;
    static cannoli_memhook_write_r12b_rax_end: u8;
    static cannoli_memhook_write_r12b_rcx: u8;
    static cannoli_memhook_write_r12b_rcx_end: u8;
    static cannoli_memhook_write_r12b_rdx: u8;
    static cannoli_memhook_write_r12b_rdx_end: u8;
    static cannoli_memhook_write_r12b_rbx: u8;
    static cannoli_memhook_write_r12b_rbx_end: u8;
    static cannoli_memhook_write_r12b_rsp: u8;
    static cannoli_memhook_write_r12b_rsp_end: u8;
    static cannoli_memhook_write_r12b_rbp: u8;
    static cannoli_memhook_write_r12b_rbp_end: u8;
    static cannoli_memhook_write_r12b_rsi: u8;
    static cannoli_memhook_write_r12b_rsi_end: u8;
    static cannoli_memhook_write_r12b_rdi: u8;
    static cannoli_memhook_write_r12b_rdi_end: u8;
    static cannoli_memhook_write_r12b_r8: u8;
    static cannoli_memhook_write_r12b_r8_end: u8;
    static cannoli_memhook_write_r12b_r9: u8;
    static cannoli_memhook_write_r12b_r9_end: u8;
    static cannoli_memhook_write_r12b_r10: u8;
    static cannoli_memhook_write_r12b_r10_end: u8;
    static cannoli_memhook_write_r12b_r11: u8;
    static cannoli_memhook_write_r12b_r11_end: u8;
    static cannoli_memhook_write_r12b_r12: u8;
    static cannoli_memhook_write_r12b_r12_end: u8;
    static cannoli_memhook_write_r12b_r13: u8;
    static cannoli_memhook_write_r12b_r13_end: u8;
    static cannoli_memhook_write_r12b_r14: u8;
    static cannoli_memhook_write_r12b_r14_end: u8;
    static cannoli_memhook_write_r12b_r15: u8;
    static cannoli_memhook_write_r12b_r15_end: u8;
    static cannoli_memhook_write_r13b_rax: u8;
    static cannoli_memhook_write_r13b_rax_end: u8;
    static cannoli_memhook_write_r13b_rcx: u8;
    static cannoli_memhook_write_r13b_rcx_end: u8;
    static cannoli_memhook_write_r13b_rdx: u8;
    static cannoli_memhook_write_r13b_rdx_end: u8;
    static cannoli_memhook_write_r13b_rbx: u8;
    static cannoli_memhook_write_r13b_rbx_end: u8;
    static cannoli_memhook_write_r13b_rsp: u8;
    static cannoli_memhook_write_r13b_rsp_end: u8;
    static cannoli_memhook_write_r13b_rbp: u8;
    static cannoli_memhook_write_r13b_rbp_end: u8;
    static cannoli_memhook_write_r13b_rsi: u8;
    static cannoli_memhook_write_r13b_rsi_end: u8;
    static cannoli_memhook_write_r13b_rdi: u8;
    static cannoli_memhook_write_r13b_rdi_end: u8;
    static cannoli_memhook_write_r13b_r8: u8;
    static cannoli_memhook_write_r13b_r8_end: u8;
    static cannoli_memhook_write_r13b_r9: u8;
    static cannoli_memhook_write_r13b_r9_end: u8;
    static cannoli_memhook_write_r13b_r10: u8;
    static cannoli_memhook_write_r13b_r10_end: u8;
    static cannoli_memhook_write_r13b_r11: u8;
    static cannoli_memhook_write_r13b_r11_end: u8;
    static cannoli_memhook_write_r13b_r12: u8;
    static cannoli_memhook_write_r13b_r12_end: u8;
    static cannoli_memhook_write_r13b_r13: u8;
    static cannoli_memhook_write_r13b_r13_end: u8;
    static cannoli_memhook_write_r13b_r14: u8;
    static cannoli_memhook_write_r13b_r14_end: u8;
    static cannoli_memhook_write_r13b_r15: u8;
    static cannoli_memhook_write_r13b_r15_end: u8;
    static cannoli_memhook_write_r14b_rax: u8;
    static cannoli_memhook_write_r14b_rax_end: u8;
    static cannoli_memhook_write_r14b_rcx: u8;
    static cannoli_memhook_write_r14b_rcx_end: u8;
    static cannoli_memhook_write_r14b_rdx: u8;
    static cannoli_memhook_write_r14b_rdx_end: u8;
    static cannoli_memhook_write_r14b_rbx: u8;
    static cannoli_memhook_write_r14b_rbx_end: u8;
    static cannoli_memhook_write_r14b_rsp: u8;
    static cannoli_memhook_write_r14b_rsp_end: u8;
    static cannoli_memhook_write_r14b_rbp: u8;
    static cannoli_memhook_write_r14b_rbp_end: u8;
    static cannoli_memhook_write_r14b_rsi: u8;
    static cannoli_memhook_write_r14b_rsi_end: u8;
    static cannoli_memhook_write_r14b_rdi: u8;
    static cannoli_memhook_write_r14b_rdi_end: u8;
    static cannoli_memhook_write_r14b_r8: u8;
    static cannoli_memhook_write_r14b_r8_end: u8;
    static cannoli_memhook_write_r14b_r9: u8;
    static cannoli_memhook_write_r14b_r9_end: u8;
    static cannoli_memhook_write_r14b_r10: u8;
    static cannoli_memhook_write_r14b_r10_end: u8;
    static cannoli_memhook_write_r14b_r11: u8;
    static cannoli_memhook_write_r14b_r11_end: u8;
    static cannoli_memhook_write_r14b_r12: u8;
    static cannoli_memhook_write_r14b_r12_end: u8;
    static cannoli_memhook_write_r14b_r13: u8;
    static cannoli_memhook_write_r14b_r13_end: u8;
    static cannoli_memhook_write_r14b_r14: u8;
    static cannoli_memhook_write_r14b_r14_end: u8;
    static cannoli_memhook_write_r14b_r15: u8;
    static cannoli_memhook_write_r14b_r15_end: u8;
    static cannoli_memhook_write_r15b_rax: u8;
    static cannoli_memhook_write_r15b_rax_end: u8;
    static cannoli_memhook_write_r15b_rcx: u8;
    static cannoli_memhook_write_r15b_rcx_end: u8;
    static cannoli_memhook_write_r15b_rdx: u8;
    static cannoli_memhook_write_r15b_rdx_end: u8;
    static cannoli_memhook_write_r15b_rbx: u8;
    static cannoli_memhook_write_r15b_rbx_end: u8;
    static cannoli_memhook_write_r15b_rsp: u8;
    static cannoli_memhook_write_r15b_rsp_end: u8;
    static cannoli_memhook_write_r15b_rbp: u8;
    static cannoli_memhook_write_r15b_rbp_end: u8;
    static cannoli_memhook_write_r15b_rsi: u8;
    static cannoli_memhook_write_r15b_rsi_end: u8;
    static cannoli_memhook_write_r15b_rdi: u8;
    static cannoli_memhook_write_r15b_rdi_end: u8;
    static cannoli_memhook_write_r15b_r8: u8;
    static cannoli_memhook_write_r15b_r8_end: u8;
    static cannoli_memhook_write_r15b_r9: u8;
    static cannoli_memhook_write_r15b_r9_end: u8;
    static cannoli_memhook_write_r15b_r10: u8;
    static cannoli_memhook_write_r15b_r10_end: u8;
    static cannoli_memhook_write_r15b_r11: u8;
    static cannoli_memhook_write_r15b_r11_end: u8;
    static cannoli_memhook_write_r15b_r12: u8;
    static cannoli_memhook_write_r15b_r12_end: u8;
    static cannoli_memhook_write_r15b_r13: u8;
    static cannoli_memhook_write_r15b_r13_end: u8;
    static cannoli_memhook_write_r15b_r14: u8;
    static cannoli_memhook_write_r15b_r14_end: u8;
    static cannoli_memhook_write_r15b_r15: u8;
    static cannoli_memhook_write_r15b_r15_end: u8;
    static cannoli_memhook_write_ax_eax: u8;
    static cannoli_memhook_write_ax_eax_end: u8;
    static cannoli_memhook_write_ax_ecx: u8;
    static cannoli_memhook_write_ax_ecx_end: u8;
    static cannoli_memhook_write_ax_edx: u8;
    static cannoli_memhook_write_ax_edx_end: u8;
    static cannoli_memhook_write_ax_ebx: u8;
    static cannoli_memhook_write_ax_ebx_end: u8;
    static cannoli_memhook_write_ax_esp: u8;
    static cannoli_memhook_write_ax_esp_end: u8;
    static cannoli_memhook_write_ax_ebp: u8;
    static cannoli_memhook_write_ax_ebp_end: u8;
    static cannoli_memhook_write_ax_esi: u8;
    static cannoli_memhook_write_ax_esi_end: u8;
    static cannoli_memhook_write_ax_edi: u8;
    static cannoli_memhook_write_ax_edi_end: u8;
    static cannoli_memhook_write_ax_r8d: u8;
    static cannoli_memhook_write_ax_r8d_end: u8;
    static cannoli_memhook_write_ax_r9d: u8;
    static cannoli_memhook_write_ax_r9d_end: u8;
    static cannoli_memhook_write_ax_r10d: u8;
    static cannoli_memhook_write_ax_r10d_end: u8;
    static cannoli_memhook_write_ax_r11d: u8;
    static cannoli_memhook_write_ax_r11d_end: u8;
    static cannoli_memhook_write_ax_r12d: u8;
    static cannoli_memhook_write_ax_r12d_end: u8;
    static cannoli_memhook_write_ax_r13d: u8;
    static cannoli_memhook_write_ax_r13d_end: u8;
    static cannoli_memhook_write_ax_r14d: u8;
    static cannoli_memhook_write_ax_r14d_end: u8;
    static cannoli_memhook_write_ax_r15d: u8;
    static cannoli_memhook_write_ax_r15d_end: u8;
    static cannoli_memhook_write_cx_eax: u8;
    static cannoli_memhook_write_cx_eax_end: u8;
    static cannoli_memhook_write_cx_ecx: u8;
    static cannoli_memhook_write_cx_ecx_end: u8;
    static cannoli_memhook_write_cx_edx: u8;
    static cannoli_memhook_write_cx_edx_end: u8;
    static cannoli_memhook_write_cx_ebx: u8;
    static cannoli_memhook_write_cx_ebx_end: u8;
    static cannoli_memhook_write_cx_esp: u8;
    static cannoli_memhook_write_cx_esp_end: u8;
    static cannoli_memhook_write_cx_ebp: u8;
    static cannoli_memhook_write_cx_ebp_end: u8;
    static cannoli_memhook_write_cx_esi: u8;
    static cannoli_memhook_write_cx_esi_end: u8;
    static cannoli_memhook_write_cx_edi: u8;
    static cannoli_memhook_write_cx_edi_end: u8;
    static cannoli_memhook_write_cx_r8d: u8;
    static cannoli_memhook_write_cx_r8d_end: u8;
    static cannoli_memhook_write_cx_r9d: u8;
    static cannoli_memhook_write_cx_r9d_end: u8;
    static cannoli_memhook_write_cx_r10d: u8;
    static cannoli_memhook_write_cx_r10d_end: u8;
    static cannoli_memhook_write_cx_r11d: u8;
    static cannoli_memhook_write_cx_r11d_end: u8;
    static cannoli_memhook_write_cx_r12d: u8;
    static cannoli_memhook_write_cx_r12d_end: u8;
    static cannoli_memhook_write_cx_r13d: u8;
    static cannoli_memhook_write_cx_r13d_end: u8;
    static cannoli_memhook_write_cx_r14d: u8;
    static cannoli_memhook_write_cx_r14d_end: u8;
    static cannoli_memhook_write_cx_r15d: u8;
    static cannoli_memhook_write_cx_r15d_end: u8;
    static cannoli_memhook_write_dx_eax: u8;
    static cannoli_memhook_write_dx_eax_end: u8;
    static cannoli_memhook_write_dx_ecx: u8;
    static cannoli_memhook_write_dx_ecx_end: u8;
    static cannoli_memhook_write_dx_edx: u8;
    static cannoli_memhook_write_dx_edx_end: u8;
    static cannoli_memhook_write_dx_ebx: u8;
    static cannoli_memhook_write_dx_ebx_end: u8;
    static cannoli_memhook_write_dx_esp: u8;
    static cannoli_memhook_write_dx_esp_end: u8;
    static cannoli_memhook_write_dx_ebp: u8;
    static cannoli_memhook_write_dx_ebp_end: u8;
    static cannoli_memhook_write_dx_esi: u8;
    static cannoli_memhook_write_dx_esi_end: u8;
    static cannoli_memhook_write_dx_edi: u8;
    static cannoli_memhook_write_dx_edi_end: u8;
    static cannoli_memhook_write_dx_r8d: u8;
    static cannoli_memhook_write_dx_r8d_end: u8;
    static cannoli_memhook_write_dx_r9d: u8;
    static cannoli_memhook_write_dx_r9d_end: u8;
    static cannoli_memhook_write_dx_r10d: u8;
    static cannoli_memhook_write_dx_r10d_end: u8;
    static cannoli_memhook_write_dx_r11d: u8;
    static cannoli_memhook_write_dx_r11d_end: u8;
    static cannoli_memhook_write_dx_r12d: u8;
    static cannoli_memhook_write_dx_r12d_end: u8;
    static cannoli_memhook_write_dx_r13d: u8;
    static cannoli_memhook_write_dx_r13d_end: u8;
    static cannoli_memhook_write_dx_r14d: u8;
    static cannoli_memhook_write_dx_r14d_end: u8;
    static cannoli_memhook_write_dx_r15d: u8;
    static cannoli_memhook_write_dx_r15d_end: u8;
    static cannoli_memhook_write_bx_eax: u8;
    static cannoli_memhook_write_bx_eax_end: u8;
    static cannoli_memhook_write_bx_ecx: u8;
    static cannoli_memhook_write_bx_ecx_end: u8;
    static cannoli_memhook_write_bx_edx: u8;
    static cannoli_memhook_write_bx_edx_end: u8;
    static cannoli_memhook_write_bx_ebx: u8;
    static cannoli_memhook_write_bx_ebx_end: u8;
    static cannoli_memhook_write_bx_esp: u8;
    static cannoli_memhook_write_bx_esp_end: u8;
    static cannoli_memhook_write_bx_ebp: u8;
    static cannoli_memhook_write_bx_ebp_end: u8;
    static cannoli_memhook_write_bx_esi: u8;
    static cannoli_memhook_write_bx_esi_end: u8;
    static cannoli_memhook_write_bx_edi: u8;
    static cannoli_memhook_write_bx_edi_end: u8;
    static cannoli_memhook_write_bx_r8d: u8;
    static cannoli_memhook_write_bx_r8d_end: u8;
    static cannoli_memhook_write_bx_r9d: u8;
    static cannoli_memhook_write_bx_r9d_end: u8;
    static cannoli_memhook_write_bx_r10d: u8;
    static cannoli_memhook_write_bx_r10d_end: u8;
    static cannoli_memhook_write_bx_r11d: u8;
    static cannoli_memhook_write_bx_r11d_end: u8;
    static cannoli_memhook_write_bx_r12d: u8;
    static cannoli_memhook_write_bx_r12d_end: u8;
    static cannoli_memhook_write_bx_r13d: u8;
    static cannoli_memhook_write_bx_r13d_end: u8;
    static cannoli_memhook_write_bx_r14d: u8;
    static cannoli_memhook_write_bx_r14d_end: u8;
    static cannoli_memhook_write_bx_r15d: u8;
    static cannoli_memhook_write_bx_r15d_end: u8;
    static cannoli_memhook_write_sp_eax: u8;
    static cannoli_memhook_write_sp_eax_end: u8;
    static cannoli_memhook_write_sp_ecx: u8;
    static cannoli_memhook_write_sp_ecx_end: u8;
    static cannoli_memhook_write_sp_edx: u8;
    static cannoli_memhook_write_sp_edx_end: u8;
    static cannoli_memhook_write_sp_ebx: u8;
    static cannoli_memhook_write_sp_ebx_end: u8;
    static cannoli_memhook_write_sp_esp: u8;
    static cannoli_memhook_write_sp_esp_end: u8;
    static cannoli_memhook_write_sp_ebp: u8;
    static cannoli_memhook_write_sp_ebp_end: u8;
    static cannoli_memhook_write_sp_esi: u8;
    static cannoli_memhook_write_sp_esi_end: u8;
    static cannoli_memhook_write_sp_edi: u8;
    static cannoli_memhook_write_sp_edi_end: u8;
    static cannoli_memhook_write_sp_r8d: u8;
    static cannoli_memhook_write_sp_r8d_end: u8;
    static cannoli_memhook_write_sp_r9d: u8;
    static cannoli_memhook_write_sp_r9d_end: u8;
    static cannoli_memhook_write_sp_r10d: u8;
    static cannoli_memhook_write_sp_r10d_end: u8;
    static cannoli_memhook_write_sp_r11d: u8;
    static cannoli_memhook_write_sp_r11d_end: u8;
    static cannoli_memhook_write_sp_r12d: u8;
    static cannoli_memhook_write_sp_r12d_end: u8;
    static cannoli_memhook_write_sp_r13d: u8;
    static cannoli_memhook_write_sp_r13d_end: u8;
    static cannoli_memhook_write_sp_r14d: u8;
    static cannoli_memhook_write_sp_r14d_end: u8;
    static cannoli_memhook_write_sp_r15d: u8;
    static cannoli_memhook_write_sp_r15d_end: u8;
    static cannoli_memhook_write_bp_eax: u8;
    static cannoli_memhook_write_bp_eax_end: u8;
    static cannoli_memhook_write_bp_ecx: u8;
    static cannoli_memhook_write_bp_ecx_end: u8;
    static cannoli_memhook_write_bp_edx: u8;
    static cannoli_memhook_write_bp_edx_end: u8;
    static cannoli_memhook_write_bp_ebx: u8;
    static cannoli_memhook_write_bp_ebx_end: u8;
    static cannoli_memhook_write_bp_esp: u8;
    static cannoli_memhook_write_bp_esp_end: u8;
    static cannoli_memhook_write_bp_ebp: u8;
    static cannoli_memhook_write_bp_ebp_end: u8;
    static cannoli_memhook_write_bp_esi: u8;
    static cannoli_memhook_write_bp_esi_end: u8;
    static cannoli_memhook_write_bp_edi: u8;
    static cannoli_memhook_write_bp_edi_end: u8;
    static cannoli_memhook_write_bp_r8d: u8;
    static cannoli_memhook_write_bp_r8d_end: u8;
    static cannoli_memhook_write_bp_r9d: u8;
    static cannoli_memhook_write_bp_r9d_end: u8;
    static cannoli_memhook_write_bp_r10d: u8;
    static cannoli_memhook_write_bp_r10d_end: u8;
    static cannoli_memhook_write_bp_r11d: u8;
    static cannoli_memhook_write_bp_r11d_end: u8;
    static cannoli_memhook_write_bp_r12d: u8;
    static cannoli_memhook_write_bp_r12d_end: u8;
    static cannoli_memhook_write_bp_r13d: u8;
    static cannoli_memhook_write_bp_r13d_end: u8;
    static cannoli_memhook_write_bp_r14d: u8;
    static cannoli_memhook_write_bp_r14d_end: u8;
    static cannoli_memhook_write_bp_r15d: u8;
    static cannoli_memhook_write_bp_r15d_end: u8;
    static cannoli_memhook_write_si_eax: u8;
    static cannoli_memhook_write_si_eax_end: u8;
    static cannoli_memhook_write_si_ecx: u8;
    static cannoli_memhook_write_si_ecx_end: u8;
    static cannoli_memhook_write_si_edx: u8;
    static cannoli_memhook_write_si_edx_end: u8;
    static cannoli_memhook_write_si_ebx: u8;
    static cannoli_memhook_write_si_ebx_end: u8;
    static cannoli_memhook_write_si_esp: u8;
    static cannoli_memhook_write_si_esp_end: u8;
    static cannoli_memhook_write_si_ebp: u8;
    static cannoli_memhook_write_si_ebp_end: u8;
    static cannoli_memhook_write_si_esi: u8;
    static cannoli_memhook_write_si_esi_end: u8;
    static cannoli_memhook_write_si_edi: u8;
    static cannoli_memhook_write_si_edi_end: u8;
    static cannoli_memhook_write_si_r8d: u8;
    static cannoli_memhook_write_si_r8d_end: u8;
    static cannoli_memhook_write_si_r9d: u8;
    static cannoli_memhook_write_si_r9d_end: u8;
    static cannoli_memhook_write_si_r10d: u8;
    static cannoli_memhook_write_si_r10d_end: u8;
    static cannoli_memhook_write_si_r11d: u8;
    static cannoli_memhook_write_si_r11d_end: u8;
    static cannoli_memhook_write_si_r12d: u8;
    static cannoli_memhook_write_si_r12d_end: u8;
    static cannoli_memhook_write_si_r13d: u8;
    static cannoli_memhook_write_si_r13d_end: u8;
    static cannoli_memhook_write_si_r14d: u8;
    static cannoli_memhook_write_si_r14d_end: u8;
    static cannoli_memhook_write_si_r15d: u8;
    static cannoli_memhook_write_si_r15d_end: u8;
    static cannoli_memhook_write_di_eax: u8;
    static cannoli_memhook_write_di_eax_end: u8;
    static cannoli_memhook_write_di_ecx: u8;
    static cannoli_memhook_write_di_ecx_end: u8;
    static cannoli_memhook_write_di_edx: u8;
    static cannoli_memhook_write_di_edx_end: u8;
    static cannoli_memhook_write_di_ebx: u8;
    static cannoli_memhook_write_di_ebx_end: u8;
    static cannoli_memhook_write_di_esp: u8;
    static cannoli_memhook_write_di_esp_end: u8;
    static cannoli_memhook_write_di_ebp: u8;
    static cannoli_memhook_write_di_ebp_end: u8;
    static cannoli_memhook_write_di_esi: u8;
    static cannoli_memhook_write_di_esi_end: u8;
    static cannoli_memhook_write_di_edi: u8;
    static cannoli_memhook_write_di_edi_end: u8;
    static cannoli_memhook_write_di_r8d: u8;
    static cannoli_memhook_write_di_r8d_end: u8;
    static cannoli_memhook_write_di_r9d: u8;
    static cannoli_memhook_write_di_r9d_end: u8;
    static cannoli_memhook_write_di_r10d: u8;
    static cannoli_memhook_write_di_r10d_end: u8;
    static cannoli_memhook_write_di_r11d: u8;
    static cannoli_memhook_write_di_r11d_end: u8;
    static cannoli_memhook_write_di_r12d: u8;
    static cannoli_memhook_write_di_r12d_end: u8;
    static cannoli_memhook_write_di_r13d: u8;
    static cannoli_memhook_write_di_r13d_end: u8;
    static cannoli_memhook_write_di_r14d: u8;
    static cannoli_memhook_write_di_r14d_end: u8;
    static cannoli_memhook_write_di_r15d: u8;
    static cannoli_memhook_write_di_r15d_end: u8;
    static cannoli_memhook_write_r8w_eax: u8;
    static cannoli_memhook_write_r8w_eax_end: u8;
    static cannoli_memhook_write_r8w_ecx: u8;
    static cannoli_memhook_write_r8w_ecx_end: u8;
    static cannoli_memhook_write_r8w_edx: u8;
    static cannoli_memhook_write_r8w_edx_end: u8;
    static cannoli_memhook_write_r8w_ebx: u8;
    static cannoli_memhook_write_r8w_ebx_end: u8;
    static cannoli_memhook_write_r8w_esp: u8;
    static cannoli_memhook_write_r8w_esp_end: u8;
    static cannoli_memhook_write_r8w_ebp: u8;
    static cannoli_memhook_write_r8w_ebp_end: u8;
    static cannoli_memhook_write_r8w_esi: u8;
    static cannoli_memhook_write_r8w_esi_end: u8;
    static cannoli_memhook_write_r8w_edi: u8;
    static cannoli_memhook_write_r8w_edi_end: u8;
    static cannoli_memhook_write_r8w_r8d: u8;
    static cannoli_memhook_write_r8w_r8d_end: u8;
    static cannoli_memhook_write_r8w_r9d: u8;
    static cannoli_memhook_write_r8w_r9d_end: u8;
    static cannoli_memhook_write_r8w_r10d: u8;
    static cannoli_memhook_write_r8w_r10d_end: u8;
    static cannoli_memhook_write_r8w_r11d: u8;
    static cannoli_memhook_write_r8w_r11d_end: u8;
    static cannoli_memhook_write_r8w_r12d: u8;
    static cannoli_memhook_write_r8w_r12d_end: u8;
    static cannoli_memhook_write_r8w_r13d: u8;
    static cannoli_memhook_write_r8w_r13d_end: u8;
    static cannoli_memhook_write_r8w_r14d: u8;
    static cannoli_memhook_write_r8w_r14d_end: u8;
    static cannoli_memhook_write_r8w_r15d: u8;
    static cannoli_memhook_write_r8w_r15d_end: u8;
    static cannoli_memhook_write_r9w_eax: u8;
    static cannoli_memhook_write_r9w_eax_end: u8;
    static cannoli_memhook_write_r9w_ecx: u8;
    static cannoli_memhook_write_r9w_ecx_end: u8;
    static cannoli_memhook_write_r9w_edx: u8;
    static cannoli_memhook_write_r9w_edx_end: u8;
    static cannoli_memhook_write_r9w_ebx: u8;
    static cannoli_memhook_write_r9w_ebx_end: u8;
    static cannoli_memhook_write_r9w_esp: u8;
    static cannoli_memhook_write_r9w_esp_end: u8;
    static cannoli_memhook_write_r9w_ebp: u8;
    static cannoli_memhook_write_r9w_ebp_end: u8;
    static cannoli_memhook_write_r9w_esi: u8;
    static cannoli_memhook_write_r9w_esi_end: u8;
    static cannoli_memhook_write_r9w_edi: u8;
    static cannoli_memhook_write_r9w_edi_end: u8;
    static cannoli_memhook_write_r9w_r8d: u8;
    static cannoli_memhook_write_r9w_r8d_end: u8;
    static cannoli_memhook_write_r9w_r9d: u8;
    static cannoli_memhook_write_r9w_r9d_end: u8;
    static cannoli_memhook_write_r9w_r10d: u8;
    static cannoli_memhook_write_r9w_r10d_end: u8;
    static cannoli_memhook_write_r9w_r11d: u8;
    static cannoli_memhook_write_r9w_r11d_end: u8;
    static cannoli_memhook_write_r9w_r12d: u8;
    static cannoli_memhook_write_r9w_r12d_end: u8;
    static cannoli_memhook_write_r9w_r13d: u8;
    static cannoli_memhook_write_r9w_r13d_end: u8;
    static cannoli_memhook_write_r9w_r14d: u8;
    static cannoli_memhook_write_r9w_r14d_end: u8;
    static cannoli_memhook_write_r9w_r15d: u8;
    static cannoli_memhook_write_r9w_r15d_end: u8;
    static cannoli_memhook_write_r10w_eax: u8;
    static cannoli_memhook_write_r10w_eax_end: u8;
    static cannoli_memhook_write_r10w_ecx: u8;
    static cannoli_memhook_write_r10w_ecx_end: u8;
    static cannoli_memhook_write_r10w_edx: u8;
    static cannoli_memhook_write_r10w_edx_end: u8;
    static cannoli_memhook_write_r10w_ebx: u8;
    static cannoli_memhook_write_r10w_ebx_end: u8;
    static cannoli_memhook_write_r10w_esp: u8;
    static cannoli_memhook_write_r10w_esp_end: u8;
    static cannoli_memhook_write_r10w_ebp: u8;
    static cannoli_memhook_write_r10w_ebp_end: u8;
    static cannoli_memhook_write_r10w_esi: u8;
    static cannoli_memhook_write_r10w_esi_end: u8;
    static cannoli_memhook_write_r10w_edi: u8;
    static cannoli_memhook_write_r10w_edi_end: u8;
    static cannoli_memhook_write_r10w_r8d: u8;
    static cannoli_memhook_write_r10w_r8d_end: u8;
    static cannoli_memhook_write_r10w_r9d: u8;
    static cannoli_memhook_write_r10w_r9d_end: u8;
    static cannoli_memhook_write_r10w_r10d: u8;
    static cannoli_memhook_write_r10w_r10d_end: u8;
    static cannoli_memhook_write_r10w_r11d: u8;
    static cannoli_memhook_write_r10w_r11d_end: u8;
    static cannoli_memhook_write_r10w_r12d: u8;
    static cannoli_memhook_write_r10w_r12d_end: u8;
    static cannoli_memhook_write_r10w_r13d: u8;
    static cannoli_memhook_write_r10w_r13d_end: u8;
    static cannoli_memhook_write_r10w_r14d: u8;
    static cannoli_memhook_write_r10w_r14d_end: u8;
    static cannoli_memhook_write_r10w_r15d: u8;
    static cannoli_memhook_write_r10w_r15d_end: u8;
    static cannoli_memhook_write_r11w_eax: u8;
    static cannoli_memhook_write_r11w_eax_end: u8;
    static cannoli_memhook_write_r11w_ecx: u8;
    static cannoli_memhook_write_r11w_ecx_end: u8;
    static cannoli_memhook_write_r11w_edx: u8;
    static cannoli_memhook_write_r11w_edx_end: u8;
    static cannoli_memhook_write_r11w_ebx: u8;
    static cannoli_memhook_write_r11w_ebx_end: u8;
    static cannoli_memhook_write_r11w_esp: u8;
    static cannoli_memhook_write_r11w_esp_end: u8;
    static cannoli_memhook_write_r11w_ebp: u8;
    static cannoli_memhook_write_r11w_ebp_end: u8;
    static cannoli_memhook_write_r11w_esi: u8;
    static cannoli_memhook_write_r11w_esi_end: u8;
    static cannoli_memhook_write_r11w_edi: u8;
    static cannoli_memhook_write_r11w_edi_end: u8;
    static cannoli_memhook_write_r11w_r8d: u8;
    static cannoli_memhook_write_r11w_r8d_end: u8;
    static cannoli_memhook_write_r11w_r9d: u8;
    static cannoli_memhook_write_r11w_r9d_end: u8;
    static cannoli_memhook_write_r11w_r10d: u8;
    static cannoli_memhook_write_r11w_r10d_end: u8;
    static cannoli_memhook_write_r11w_r11d: u8;
    static cannoli_memhook_write_r11w_r11d_end: u8;
    static cannoli_memhook_write_r11w_r12d: u8;
    static cannoli_memhook_write_r11w_r12d_end: u8;
    static cannoli_memhook_write_r11w_r13d: u8;
    static cannoli_memhook_write_r11w_r13d_end: u8;
    static cannoli_memhook_write_r11w_r14d: u8;
    static cannoli_memhook_write_r11w_r14d_end: u8;
    static cannoli_memhook_write_r11w_r15d: u8;
    static cannoli_memhook_write_r11w_r15d_end: u8;
    static cannoli_memhook_write_r12w_eax: u8;
    static cannoli_memhook_write_r12w_eax_end: u8;
    static cannoli_memhook_write_r12w_ecx: u8;
    static cannoli_memhook_write_r12w_ecx_end: u8;
    static cannoli_memhook_write_r12w_edx: u8;
    static cannoli_memhook_write_r12w_edx_end: u8;
    static cannoli_memhook_write_r12w_ebx: u8;
    static cannoli_memhook_write_r12w_ebx_end: u8;
    static cannoli_memhook_write_r12w_esp: u8;
    static cannoli_memhook_write_r12w_esp_end: u8;
    static cannoli_memhook_write_r12w_ebp: u8;
    static cannoli_memhook_write_r12w_ebp_end: u8;
    static cannoli_memhook_write_r12w_esi: u8;
    static cannoli_memhook_write_r12w_esi_end: u8;
    static cannoli_memhook_write_r12w_edi: u8;
    static cannoli_memhook_write_r12w_edi_end: u8;
    static cannoli_memhook_write_r12w_r8d: u8;
    static cannoli_memhook_write_r12w_r8d_end: u8;
    static cannoli_memhook_write_r12w_r9d: u8;
    static cannoli_memhook_write_r12w_r9d_end: u8;
    static cannoli_memhook_write_r12w_r10d: u8;
    static cannoli_memhook_write_r12w_r10d_end: u8;
    static cannoli_memhook_write_r12w_r11d: u8;
    static cannoli_memhook_write_r12w_r11d_end: u8;
    static cannoli_memhook_write_r12w_r12d: u8;
    static cannoli_memhook_write_r12w_r12d_end: u8;
    static cannoli_memhook_write_r12w_r13d: u8;
    static cannoli_memhook_write_r12w_r13d_end: u8;
    static cannoli_memhook_write_r12w_r14d: u8;
    static cannoli_memhook_write_r12w_r14d_end: u8;
    static cannoli_memhook_write_r12w_r15d: u8;
    static cannoli_memhook_write_r12w_r15d_end: u8;
    static cannoli_memhook_write_r13w_eax: u8;
    static cannoli_memhook_write_r13w_eax_end: u8;
    static cannoli_memhook_write_r13w_ecx: u8;
    static cannoli_memhook_write_r13w_ecx_end: u8;
    static cannoli_memhook_write_r13w_edx: u8;
    static cannoli_memhook_write_r13w_edx_end: u8;
    static cannoli_memhook_write_r13w_ebx: u8;
    static cannoli_memhook_write_r13w_ebx_end: u8;
    static cannoli_memhook_write_r13w_esp: u8;
    static cannoli_memhook_write_r13w_esp_end: u8;
    static cannoli_memhook_write_r13w_ebp: u8;
    static cannoli_memhook_write_r13w_ebp_end: u8;
    static cannoli_memhook_write_r13w_esi: u8;
    static cannoli_memhook_write_r13w_esi_end: u8;
    static cannoli_memhook_write_r13w_edi: u8;
    static cannoli_memhook_write_r13w_edi_end: u8;
    static cannoli_memhook_write_r13w_r8d: u8;
    static cannoli_memhook_write_r13w_r8d_end: u8;
    static cannoli_memhook_write_r13w_r9d: u8;
    static cannoli_memhook_write_r13w_r9d_end: u8;
    static cannoli_memhook_write_r13w_r10d: u8;
    static cannoli_memhook_write_r13w_r10d_end: u8;
    static cannoli_memhook_write_r13w_r11d: u8;
    static cannoli_memhook_write_r13w_r11d_end: u8;
    static cannoli_memhook_write_r13w_r12d: u8;
    static cannoli_memhook_write_r13w_r12d_end: u8;
    static cannoli_memhook_write_r13w_r13d: u8;
    static cannoli_memhook_write_r13w_r13d_end: u8;
    static cannoli_memhook_write_r13w_r14d: u8;
    static cannoli_memhook_write_r13w_r14d_end: u8;
    static cannoli_memhook_write_r13w_r15d: u8;
    static cannoli_memhook_write_r13w_r15d_end: u8;
    static cannoli_memhook_write_r14w_eax: u8;
    static cannoli_memhook_write_r14w_eax_end: u8;
    static cannoli_memhook_write_r14w_ecx: u8;
    static cannoli_memhook_write_r14w_ecx_end: u8;
    static cannoli_memhook_write_r14w_edx: u8;
    static cannoli_memhook_write_r14w_edx_end: u8;
    static cannoli_memhook_write_r14w_ebx: u8;
    static cannoli_memhook_write_r14w_ebx_end: u8;
    static cannoli_memhook_write_r14w_esp: u8;
    static cannoli_memhook_write_r14w_esp_end: u8;
    static cannoli_memhook_write_r14w_ebp: u8;
    static cannoli_memhook_write_r14w_ebp_end: u8;
    static cannoli_memhook_write_r14w_esi: u8;
    static cannoli_memhook_write_r14w_esi_end: u8;
    static cannoli_memhook_write_r14w_edi: u8;
    static cannoli_memhook_write_r14w_edi_end: u8;
    static cannoli_memhook_write_r14w_r8d: u8;
    static cannoli_memhook_write_r14w_r8d_end: u8;
    static cannoli_memhook_write_r14w_r9d: u8;
    static cannoli_memhook_write_r14w_r9d_end: u8;
    static cannoli_memhook_write_r14w_r10d: u8;
    static cannoli_memhook_write_r14w_r10d_end: u8;
    static cannoli_memhook_write_r14w_r11d: u8;
    static cannoli_memhook_write_r14w_r11d_end: u8;
    static cannoli_memhook_write_r14w_r12d: u8;
    static cannoli_memhook_write_r14w_r12d_end: u8;
    static cannoli_memhook_write_r14w_r13d: u8;
    static cannoli_memhook_write_r14w_r13d_end: u8;
    static cannoli_memhook_write_r14w_r14d: u8;
    static cannoli_memhook_write_r14w_r14d_end: u8;
    static cannoli_memhook_write_r14w_r15d: u8;
    static cannoli_memhook_write_r14w_r15d_end: u8;
    static cannoli_memhook_write_r15w_eax: u8;
    static cannoli_memhook_write_r15w_eax_end: u8;
    static cannoli_memhook_write_r15w_ecx: u8;
    static cannoli_memhook_write_r15w_ecx_end: u8;
    static cannoli_memhook_write_r15w_edx: u8;
    static cannoli_memhook_write_r15w_edx_end: u8;
    static cannoli_memhook_write_r15w_ebx: u8;
    static cannoli_memhook_write_r15w_ebx_end: u8;
    static cannoli_memhook_write_r15w_esp: u8;
    static cannoli_memhook_write_r15w_esp_end: u8;
    static cannoli_memhook_write_r15w_ebp: u8;
    static cannoli_memhook_write_r15w_ebp_end: u8;
    static cannoli_memhook_write_r15w_esi: u8;
    static cannoli_memhook_write_r15w_esi_end: u8;
    static cannoli_memhook_write_r15w_edi: u8;
    static cannoli_memhook_write_r15w_edi_end: u8;
    static cannoli_memhook_write_r15w_r8d: u8;
    static cannoli_memhook_write_r15w_r8d_end: u8;
    static cannoli_memhook_write_r15w_r9d: u8;
    static cannoli_memhook_write_r15w_r9d_end: u8;
    static cannoli_memhook_write_r15w_r10d: u8;
    static cannoli_memhook_write_r15w_r10d_end: u8;
    static cannoli_memhook_write_r15w_r11d: u8;
    static cannoli_memhook_write_r15w_r11d_end: u8;
    static cannoli_memhook_write_r15w_r12d: u8;
    static cannoli_memhook_write_r15w_r12d_end: u8;
    static cannoli_memhook_write_r15w_r13d: u8;
    static cannoli_memhook_write_r15w_r13d_end: u8;
    static cannoli_memhook_write_r15w_r14d: u8;
    static cannoli_memhook_write_r15w_r14d_end: u8;
    static cannoli_memhook_write_r15w_r15d: u8;
    static cannoli_memhook_write_r15w_r15d_end: u8;
    static cannoli_memhook_write_ax_rax: u8;
    static cannoli_memhook_write_ax_rax_end: u8;
    static cannoli_memhook_write_ax_rcx: u8;
    static cannoli_memhook_write_ax_rcx_end: u8;
    static cannoli_memhook_write_ax_rdx: u8;
    static cannoli_memhook_write_ax_rdx_end: u8;
    static cannoli_memhook_write_ax_rbx: u8;
    static cannoli_memhook_write_ax_rbx_end: u8;
    static cannoli_memhook_write_ax_rsp: u8;
    static cannoli_memhook_write_ax_rsp_end: u8;
    static cannoli_memhook_write_ax_rbp: u8;
    static cannoli_memhook_write_ax_rbp_end: u8;
    static cannoli_memhook_write_ax_rsi: u8;
    static cannoli_memhook_write_ax_rsi_end: u8;
    static cannoli_memhook_write_ax_rdi: u8;
    static cannoli_memhook_write_ax_rdi_end: u8;
    static cannoli_memhook_write_ax_r8: u8;
    static cannoli_memhook_write_ax_r8_end: u8;
    static cannoli_memhook_write_ax_r9: u8;
    static cannoli_memhook_write_ax_r9_end: u8;
    static cannoli_memhook_write_ax_r10: u8;
    static cannoli_memhook_write_ax_r10_end: u8;
    static cannoli_memhook_write_ax_r11: u8;
    static cannoli_memhook_write_ax_r11_end: u8;
    static cannoli_memhook_write_ax_r12: u8;
    static cannoli_memhook_write_ax_r12_end: u8;
    static cannoli_memhook_write_ax_r13: u8;
    static cannoli_memhook_write_ax_r13_end: u8;
    static cannoli_memhook_write_ax_r14: u8;
    static cannoli_memhook_write_ax_r14_end: u8;
    static cannoli_memhook_write_ax_r15: u8;
    static cannoli_memhook_write_ax_r15_end: u8;
    static cannoli_memhook_write_cx_rax: u8;
    static cannoli_memhook_write_cx_rax_end: u8;
    static cannoli_memhook_write_cx_rcx: u8;
    static cannoli_memhook_write_cx_rcx_end: u8;
    static cannoli_memhook_write_cx_rdx: u8;
    static cannoli_memhook_write_cx_rdx_end: u8;
    static cannoli_memhook_write_cx_rbx: u8;
    static cannoli_memhook_write_cx_rbx_end: u8;
    static cannoli_memhook_write_cx_rsp: u8;
    static cannoli_memhook_write_cx_rsp_end: u8;
    static cannoli_memhook_write_cx_rbp: u8;
    static cannoli_memhook_write_cx_rbp_end: u8;
    static cannoli_memhook_write_cx_rsi: u8;
    static cannoli_memhook_write_cx_rsi_end: u8;
    static cannoli_memhook_write_cx_rdi: u8;
    static cannoli_memhook_write_cx_rdi_end: u8;
    static cannoli_memhook_write_cx_r8: u8;
    static cannoli_memhook_write_cx_r8_end: u8;
    static cannoli_memhook_write_cx_r9: u8;
    static cannoli_memhook_write_cx_r9_end: u8;
    static cannoli_memhook_write_cx_r10: u8;
    static cannoli_memhook_write_cx_r10_end: u8;
    static cannoli_memhook_write_cx_r11: u8;
    static cannoli_memhook_write_cx_r11_end: u8;
    static cannoli_memhook_write_cx_r12: u8;
    static cannoli_memhook_write_cx_r12_end: u8;
    static cannoli_memhook_write_cx_r13: u8;
    static cannoli_memhook_write_cx_r13_end: u8;
    static cannoli_memhook_write_cx_r14: u8;
    static cannoli_memhook_write_cx_r14_end: u8;
    static cannoli_memhook_write_cx_r15: u8;
    static cannoli_memhook_write_cx_r15_end: u8;
    static cannoli_memhook_write_dx_rax: u8;
    static cannoli_memhook_write_dx_rax_end: u8;
    static cannoli_memhook_write_dx_rcx: u8;
    static cannoli_memhook_write_dx_rcx_end: u8;
    static cannoli_memhook_write_dx_rdx: u8;
    static cannoli_memhook_write_dx_rdx_end: u8;
    static cannoli_memhook_write_dx_rbx: u8;
    static cannoli_memhook_write_dx_rbx_end: u8;
    static cannoli_memhook_write_dx_rsp: u8;
    static cannoli_memhook_write_dx_rsp_end: u8;
    static cannoli_memhook_write_dx_rbp: u8;
    static cannoli_memhook_write_dx_rbp_end: u8;
    static cannoli_memhook_write_dx_rsi: u8;
    static cannoli_memhook_write_dx_rsi_end: u8;
    static cannoli_memhook_write_dx_rdi: u8;
    static cannoli_memhook_write_dx_rdi_end: u8;
    static cannoli_memhook_write_dx_r8: u8;
    static cannoli_memhook_write_dx_r8_end: u8;
    static cannoli_memhook_write_dx_r9: u8;
    static cannoli_memhook_write_dx_r9_end: u8;
    static cannoli_memhook_write_dx_r10: u8;
    static cannoli_memhook_write_dx_r10_end: u8;
    static cannoli_memhook_write_dx_r11: u8;
    static cannoli_memhook_write_dx_r11_end: u8;
    static cannoli_memhook_write_dx_r12: u8;
    static cannoli_memhook_write_dx_r12_end: u8;
    static cannoli_memhook_write_dx_r13: u8;
    static cannoli_memhook_write_dx_r13_end: u8;
    static cannoli_memhook_write_dx_r14: u8;
    static cannoli_memhook_write_dx_r14_end: u8;
    static cannoli_memhook_write_dx_r15: u8;
    static cannoli_memhook_write_dx_r15_end: u8;
    static cannoli_memhook_write_bx_rax: u8;
    static cannoli_memhook_write_bx_rax_end: u8;
    static cannoli_memhook_write_bx_rcx: u8;
    static cannoli_memhook_write_bx_rcx_end: u8;
    static cannoli_memhook_write_bx_rdx: u8;
    static cannoli_memhook_write_bx_rdx_end: u8;
    static cannoli_memhook_write_bx_rbx: u8;
    static cannoli_memhook_write_bx_rbx_end: u8;
    static cannoli_memhook_write_bx_rsp: u8;
    static cannoli_memhook_write_bx_rsp_end: u8;
    static cannoli_memhook_write_bx_rbp: u8;
    static cannoli_memhook_write_bx_rbp_end: u8;
    static cannoli_memhook_write_bx_rsi: u8;
    static cannoli_memhook_write_bx_rsi_end: u8;
    static cannoli_memhook_write_bx_rdi: u8;
    static cannoli_memhook_write_bx_rdi_end: u8;
    static cannoli_memhook_write_bx_r8: u8;
    static cannoli_memhook_write_bx_r8_end: u8;
    static cannoli_memhook_write_bx_r9: u8;
    static cannoli_memhook_write_bx_r9_end: u8;
    static cannoli_memhook_write_bx_r10: u8;
    static cannoli_memhook_write_bx_r10_end: u8;
    static cannoli_memhook_write_bx_r11: u8;
    static cannoli_memhook_write_bx_r11_end: u8;
    static cannoli_memhook_write_bx_r12: u8;
    static cannoli_memhook_write_bx_r12_end: u8;
    static cannoli_memhook_write_bx_r13: u8;
    static cannoli_memhook_write_bx_r13_end: u8;
    static cannoli_memhook_write_bx_r14: u8;
    static cannoli_memhook_write_bx_r14_end: u8;
    static cannoli_memhook_write_bx_r15: u8;
    static cannoli_memhook_write_bx_r15_end: u8;
    static cannoli_memhook_write_sp_rax: u8;
    static cannoli_memhook_write_sp_rax_end: u8;
    static cannoli_memhook_write_sp_rcx: u8;
    static cannoli_memhook_write_sp_rcx_end: u8;
    static cannoli_memhook_write_sp_rdx: u8;
    static cannoli_memhook_write_sp_rdx_end: u8;
    static cannoli_memhook_write_sp_rbx: u8;
    static cannoli_memhook_write_sp_rbx_end: u8;
    static cannoli_memhook_write_sp_rsp: u8;
    static cannoli_memhook_write_sp_rsp_end: u8;
    static cannoli_memhook_write_sp_rbp: u8;
    static cannoli_memhook_write_sp_rbp_end: u8;
    static cannoli_memhook_write_sp_rsi: u8;
    static cannoli_memhook_write_sp_rsi_end: u8;
    static cannoli_memhook_write_sp_rdi: u8;
    static cannoli_memhook_write_sp_rdi_end: u8;
    static cannoli_memhook_write_sp_r8: u8;
    static cannoli_memhook_write_sp_r8_end: u8;
    static cannoli_memhook_write_sp_r9: u8;
    static cannoli_memhook_write_sp_r9_end: u8;
    static cannoli_memhook_write_sp_r10: u8;
    static cannoli_memhook_write_sp_r10_end: u8;
    static cannoli_memhook_write_sp_r11: u8;
    static cannoli_memhook_write_sp_r11_end: u8;
    static cannoli_memhook_write_sp_r12: u8;
    static cannoli_memhook_write_sp_r12_end: u8;
    static cannoli_memhook_write_sp_r13: u8;
    static cannoli_memhook_write_sp_r13_end: u8;
    static cannoli_memhook_write_sp_r14: u8;
    static cannoli_memhook_write_sp_r14_end: u8;
    static cannoli_memhook_write_sp_r15: u8;
    static cannoli_memhook_write_sp_r15_end: u8;
    static cannoli_memhook_write_bp_rax: u8;
    static cannoli_memhook_write_bp_rax_end: u8;
    static cannoli_memhook_write_bp_rcx: u8;
    static cannoli_memhook_write_bp_rcx_end: u8;
    static cannoli_memhook_write_bp_rdx: u8;
    static cannoli_memhook_write_bp_rdx_end: u8;
    static cannoli_memhook_write_bp_rbx: u8;
    static cannoli_memhook_write_bp_rbx_end: u8;
    static cannoli_memhook_write_bp_rsp: u8;
    static cannoli_memhook_write_bp_rsp_end: u8;
    static cannoli_memhook_write_bp_rbp: u8;
    static cannoli_memhook_write_bp_rbp_end: u8;
    static cannoli_memhook_write_bp_rsi: u8;
    static cannoli_memhook_write_bp_rsi_end: u8;
    static cannoli_memhook_write_bp_rdi: u8;
    static cannoli_memhook_write_bp_rdi_end: u8;
    static cannoli_memhook_write_bp_r8: u8;
    static cannoli_memhook_write_bp_r8_end: u8;
    static cannoli_memhook_write_bp_r9: u8;
    static cannoli_memhook_write_bp_r9_end: u8;
    static cannoli_memhook_write_bp_r10: u8;
    static cannoli_memhook_write_bp_r10_end: u8;
    static cannoli_memhook_write_bp_r11: u8;
    static cannoli_memhook_write_bp_r11_end: u8;
    static cannoli_memhook_write_bp_r12: u8;
    static cannoli_memhook_write_bp_r12_end: u8;
    static cannoli_memhook_write_bp_r13: u8;
    static cannoli_memhook_write_bp_r13_end: u8;
    static cannoli_memhook_write_bp_r14: u8;
    static cannoli_memhook_write_bp_r14_end: u8;
    static cannoli_memhook_write_bp_r15: u8;
    static cannoli_memhook_write_bp_r15_end: u8;
    static cannoli_memhook_write_si_rax: u8;
    static cannoli_memhook_write_si_rax_end: u8;
    static cannoli_memhook_write_si_rcx: u8;
    static cannoli_memhook_write_si_rcx_end: u8;
    static cannoli_memhook_write_si_rdx: u8;
    static cannoli_memhook_write_si_rdx_end: u8;
    static cannoli_memhook_write_si_rbx: u8;
    static cannoli_memhook_write_si_rbx_end: u8;
    static cannoli_memhook_write_si_rsp: u8;
    static cannoli_memhook_write_si_rsp_end: u8;
    static cannoli_memhook_write_si_rbp: u8;
    static cannoli_memhook_write_si_rbp_end: u8;
    static cannoli_memhook_write_si_rsi: u8;
    static cannoli_memhook_write_si_rsi_end: u8;
    static cannoli_memhook_write_si_rdi: u8;
    static cannoli_memhook_write_si_rdi_end: u8;
    static cannoli_memhook_write_si_r8: u8;
    static cannoli_memhook_write_si_r8_end: u8;
    static cannoli_memhook_write_si_r9: u8;
    static cannoli_memhook_write_si_r9_end: u8;
    static cannoli_memhook_write_si_r10: u8;
    static cannoli_memhook_write_si_r10_end: u8;
    static cannoli_memhook_write_si_r11: u8;
    static cannoli_memhook_write_si_r11_end: u8;
    static cannoli_memhook_write_si_r12: u8;
    static cannoli_memhook_write_si_r12_end: u8;
    static cannoli_memhook_write_si_r13: u8;
    static cannoli_memhook_write_si_r13_end: u8;
    static cannoli_memhook_write_si_r14: u8;
    static cannoli_memhook_write_si_r14_end: u8;
    static cannoli_memhook_write_si_r15: u8;
    static cannoli_memhook_write_si_r15_end: u8;
    static cannoli_memhook_write_di_rax: u8;
    static cannoli_memhook_write_di_rax_end: u8;
    static cannoli_memhook_write_di_rcx: u8;
    static cannoli_memhook_write_di_rcx_end: u8;
    static cannoli_memhook_write_di_rdx: u8;
    static cannoli_memhook_write_di_rdx_end: u8;
    static cannoli_memhook_write_di_rbx: u8;
    static cannoli_memhook_write_di_rbx_end: u8;
    static cannoli_memhook_write_di_rsp: u8;
    static cannoli_memhook_write_di_rsp_end: u8;
    static cannoli_memhook_write_di_rbp: u8;
    static cannoli_memhook_write_di_rbp_end: u8;
    static cannoli_memhook_write_di_rsi: u8;
    static cannoli_memhook_write_di_rsi_end: u8;
    static cannoli_memhook_write_di_rdi: u8;
    static cannoli_memhook_write_di_rdi_end: u8;
    static cannoli_memhook_write_di_r8: u8;
    static cannoli_memhook_write_di_r8_end: u8;
    static cannoli_memhook_write_di_r9: u8;
    static cannoli_memhook_write_di_r9_end: u8;
    static cannoli_memhook_write_di_r10: u8;
    static cannoli_memhook_write_di_r10_end: u8;
    static cannoli_memhook_write_di_r11: u8;
    static cannoli_memhook_write_di_r11_end: u8;
    static cannoli_memhook_write_di_r12: u8;
    static cannoli_memhook_write_di_r12_end: u8;
    static cannoli_memhook_write_di_r13: u8;
    static cannoli_memhook_write_di_r13_end: u8;
    static cannoli_memhook_write_di_r14: u8;
    static cannoli_memhook_write_di_r14_end: u8;
    static cannoli_memhook_write_di_r15: u8;
    static cannoli_memhook_write_di_r15_end: u8;
    static cannoli_memhook_write_r8w_rax: u8;
    static cannoli_memhook_write_r8w_rax_end: u8;
    static cannoli_memhook_write_r8w_rcx: u8;
    static cannoli_memhook_write_r8w_rcx_end: u8;
    static cannoli_memhook_write_r8w_rdx: u8;
    static cannoli_memhook_write_r8w_rdx_end: u8;
    static cannoli_memhook_write_r8w_rbx: u8;
    static cannoli_memhook_write_r8w_rbx_end: u8;
    static cannoli_memhook_write_r8w_rsp: u8;
    static cannoli_memhook_write_r8w_rsp_end: u8;
    static cannoli_memhook_write_r8w_rbp: u8;
    static cannoli_memhook_write_r8w_rbp_end: u8;
    static cannoli_memhook_write_r8w_rsi: u8;
    static cannoli_memhook_write_r8w_rsi_end: u8;
    static cannoli_memhook_write_r8w_rdi: u8;
    static cannoli_memhook_write_r8w_rdi_end: u8;
    static cannoli_memhook_write_r8w_r8: u8;
    static cannoli_memhook_write_r8w_r8_end: u8;
    static cannoli_memhook_write_r8w_r9: u8;
    static cannoli_memhook_write_r8w_r9_end: u8;
    static cannoli_memhook_write_r8w_r10: u8;
    static cannoli_memhook_write_r8w_r10_end: u8;
    static cannoli_memhook_write_r8w_r11: u8;
    static cannoli_memhook_write_r8w_r11_end: u8;
    static cannoli_memhook_write_r8w_r12: u8;
    static cannoli_memhook_write_r8w_r12_end: u8;
    static cannoli_memhook_write_r8w_r13: u8;
    static cannoli_memhook_write_r8w_r13_end: u8;
    static cannoli_memhook_write_r8w_r14: u8;
    static cannoli_memhook_write_r8w_r14_end: u8;
    static cannoli_memhook_write_r8w_r15: u8;
    static cannoli_memhook_write_r8w_r15_end: u8;
    static cannoli_memhook_write_r9w_rax: u8;
    static cannoli_memhook_write_r9w_rax_end: u8;
    static cannoli_memhook_write_r9w_rcx: u8;
    static cannoli_memhook_write_r9w_rcx_end: u8;
    static cannoli_memhook_write_r9w_rdx: u8;
    static cannoli_memhook_write_r9w_rdx_end: u8;
    static cannoli_memhook_write_r9w_rbx: u8;
    static cannoli_memhook_write_r9w_rbx_end: u8;
    static cannoli_memhook_write_r9w_rsp: u8;
    static cannoli_memhook_write_r9w_rsp_end: u8;
    static cannoli_memhook_write_r9w_rbp: u8;
    static cannoli_memhook_write_r9w_rbp_end: u8;
    static cannoli_memhook_write_r9w_rsi: u8;
    static cannoli_memhook_write_r9w_rsi_end: u8;
    static cannoli_memhook_write_r9w_rdi: u8;
    static cannoli_memhook_write_r9w_rdi_end: u8;
    static cannoli_memhook_write_r9w_r8: u8;
    static cannoli_memhook_write_r9w_r8_end: u8;
    static cannoli_memhook_write_r9w_r9: u8;
    static cannoli_memhook_write_r9w_r9_end: u8;
    static cannoli_memhook_write_r9w_r10: u8;
    static cannoli_memhook_write_r9w_r10_end: u8;
    static cannoli_memhook_write_r9w_r11: u8;
    static cannoli_memhook_write_r9w_r11_end: u8;
    static cannoli_memhook_write_r9w_r12: u8;
    static cannoli_memhook_write_r9w_r12_end: u8;
    static cannoli_memhook_write_r9w_r13: u8;
    static cannoli_memhook_write_r9w_r13_end: u8;
    static cannoli_memhook_write_r9w_r14: u8;
    static cannoli_memhook_write_r9w_r14_end: u8;
    static cannoli_memhook_write_r9w_r15: u8;
    static cannoli_memhook_write_r9w_r15_end: u8;
    static cannoli_memhook_write_r10w_rax: u8;
    static cannoli_memhook_write_r10w_rax_end: u8;
    static cannoli_memhook_write_r10w_rcx: u8;
    static cannoli_memhook_write_r10w_rcx_end: u8;
    static cannoli_memhook_write_r10w_rdx: u8;
    static cannoli_memhook_write_r10w_rdx_end: u8;
    static cannoli_memhook_write_r10w_rbx: u8;
    static cannoli_memhook_write_r10w_rbx_end: u8;
    static cannoli_memhook_write_r10w_rsp: u8;
    static cannoli_memhook_write_r10w_rsp_end: u8;
    static cannoli_memhook_write_r10w_rbp: u8;
    static cannoli_memhook_write_r10w_rbp_end: u8;
    static cannoli_memhook_write_r10w_rsi: u8;
    static cannoli_memhook_write_r10w_rsi_end: u8;
    static cannoli_memhook_write_r10w_rdi: u8;
    static cannoli_memhook_write_r10w_rdi_end: u8;
    static cannoli_memhook_write_r10w_r8: u8;
    static cannoli_memhook_write_r10w_r8_end: u8;
    static cannoli_memhook_write_r10w_r9: u8;
    static cannoli_memhook_write_r10w_r9_end: u8;
    static cannoli_memhook_write_r10w_r10: u8;
    static cannoli_memhook_write_r10w_r10_end: u8;
    static cannoli_memhook_write_r10w_r11: u8;
    static cannoli_memhook_write_r10w_r11_end: u8;
    static cannoli_memhook_write_r10w_r12: u8;
    static cannoli_memhook_write_r10w_r12_end: u8;
    static cannoli_memhook_write_r10w_r13: u8;
    static cannoli_memhook_write_r10w_r13_end: u8;
    static cannoli_memhook_write_r10w_r14: u8;
    static cannoli_memhook_write_r10w_r14_end: u8;
    static cannoli_memhook_write_r10w_r15: u8;
    static cannoli_memhook_write_r10w_r15_end: u8;
    static cannoli_memhook_write_r11w_rax: u8;
    static cannoli_memhook_write_r11w_rax_end: u8;
    static cannoli_memhook_write_r11w_rcx: u8;
    static cannoli_memhook_write_r11w_rcx_end: u8;
    static cannoli_memhook_write_r11w_rdx: u8;
    static cannoli_memhook_write_r11w_rdx_end: u8;
    static cannoli_memhook_write_r11w_rbx: u8;
    static cannoli_memhook_write_r11w_rbx_end: u8;
    static cannoli_memhook_write_r11w_rsp: u8;
    static cannoli_memhook_write_r11w_rsp_end: u8;
    static cannoli_memhook_write_r11w_rbp: u8;
    static cannoli_memhook_write_r11w_rbp_end: u8;
    static cannoli_memhook_write_r11w_rsi: u8;
    static cannoli_memhook_write_r11w_rsi_end: u8;
    static cannoli_memhook_write_r11w_rdi: u8;
    static cannoli_memhook_write_r11w_rdi_end: u8;
    static cannoli_memhook_write_r11w_r8: u8;
    static cannoli_memhook_write_r11w_r8_end: u8;
    static cannoli_memhook_write_r11w_r9: u8;
    static cannoli_memhook_write_r11w_r9_end: u8;
    static cannoli_memhook_write_r11w_r10: u8;
    static cannoli_memhook_write_r11w_r10_end: u8;
    static cannoli_memhook_write_r11w_r11: u8;
    static cannoli_memhook_write_r11w_r11_end: u8;
    static cannoli_memhook_write_r11w_r12: u8;
    static cannoli_memhook_write_r11w_r12_end: u8;
    static cannoli_memhook_write_r11w_r13: u8;
    static cannoli_memhook_write_r11w_r13_end: u8;
    static cannoli_memhook_write_r11w_r14: u8;
    static cannoli_memhook_write_r11w_r14_end: u8;
    static cannoli_memhook_write_r11w_r15: u8;
    static cannoli_memhook_write_r11w_r15_end: u8;
    static cannoli_memhook_write_r12w_rax: u8;
    static cannoli_memhook_write_r12w_rax_end: u8;
    static cannoli_memhook_write_r12w_rcx: u8;
    static cannoli_memhook_write_r12w_rcx_end: u8;
    static cannoli_memhook_write_r12w_rdx: u8;
    static cannoli_memhook_write_r12w_rdx_end: u8;
    static cannoli_memhook_write_r12w_rbx: u8;
    static cannoli_memhook_write_r12w_rbx_end: u8;
    static cannoli_memhook_write_r12w_rsp: u8;
    static cannoli_memhook_write_r12w_rsp_end: u8;
    static cannoli_memhook_write_r12w_rbp: u8;
    static cannoli_memhook_write_r12w_rbp_end: u8;
    static cannoli_memhook_write_r12w_rsi: u8;
    static cannoli_memhook_write_r12w_rsi_end: u8;
    static cannoli_memhook_write_r12w_rdi: u8;
    static cannoli_memhook_write_r12w_rdi_end: u8;
    static cannoli_memhook_write_r12w_r8: u8;
    static cannoli_memhook_write_r12w_r8_end: u8;
    static cannoli_memhook_write_r12w_r9: u8;
    static cannoli_memhook_write_r12w_r9_end: u8;
    static cannoli_memhook_write_r12w_r10: u8;
    static cannoli_memhook_write_r12w_r10_end: u8;
    static cannoli_memhook_write_r12w_r11: u8;
    static cannoli_memhook_write_r12w_r11_end: u8;
    static cannoli_memhook_write_r12w_r12: u8;
    static cannoli_memhook_write_r12w_r12_end: u8;
    static cannoli_memhook_write_r12w_r13: u8;
    static cannoli_memhook_write_r12w_r13_end: u8;
    static cannoli_memhook_write_r12w_r14: u8;
    static cannoli_memhook_write_r12w_r14_end: u8;
    static cannoli_memhook_write_r12w_r15: u8;
    static cannoli_memhook_write_r12w_r15_end: u8;
    static cannoli_memhook_write_r13w_rax: u8;
    static cannoli_memhook_write_r13w_rax_end: u8;
    static cannoli_memhook_write_r13w_rcx: u8;
    static cannoli_memhook_write_r13w_rcx_end: u8;
    static cannoli_memhook_write_r13w_rdx: u8;
    static cannoli_memhook_write_r13w_rdx_end: u8;
    static cannoli_memhook_write_r13w_rbx: u8;
    static cannoli_memhook_write_r13w_rbx_end: u8;
    static cannoli_memhook_write_r13w_rsp: u8;
    static cannoli_memhook_write_r13w_rsp_end: u8;
    static cannoli_memhook_write_r13w_rbp: u8;
    static cannoli_memhook_write_r13w_rbp_end: u8;
    static cannoli_memhook_write_r13w_rsi: u8;
    static cannoli_memhook_write_r13w_rsi_end: u8;
    static cannoli_memhook_write_r13w_rdi: u8;
    static cannoli_memhook_write_r13w_rdi_end: u8;
    static cannoli_memhook_write_r13w_r8: u8;
    static cannoli_memhook_write_r13w_r8_end: u8;
    static cannoli_memhook_write_r13w_r9: u8;
    static cannoli_memhook_write_r13w_r9_end: u8;
    static cannoli_memhook_write_r13w_r10: u8;
    static cannoli_memhook_write_r13w_r10_end: u8;
    static cannoli_memhook_write_r13w_r11: u8;
    static cannoli_memhook_write_r13w_r11_end: u8;
    static cannoli_memhook_write_r13w_r12: u8;
    static cannoli_memhook_write_r13w_r12_end: u8;
    static cannoli_memhook_write_r13w_r13: u8;
    static cannoli_memhook_write_r13w_r13_end: u8;
    static cannoli_memhook_write_r13w_r14: u8;
    static cannoli_memhook_write_r13w_r14_end: u8;
    static cannoli_memhook_write_r13w_r15: u8;
    static cannoli_memhook_write_r13w_r15_end: u8;
    static cannoli_memhook_write_r14w_rax: u8;
    static cannoli_memhook_write_r14w_rax_end: u8;
    static cannoli_memhook_write_r14w_rcx: u8;
    static cannoli_memhook_write_r14w_rcx_end: u8;
    static cannoli_memhook_write_r14w_rdx: u8;
    static cannoli_memhook_write_r14w_rdx_end: u8;
    static cannoli_memhook_write_r14w_rbx: u8;
    static cannoli_memhook_write_r14w_rbx_end: u8;
    static cannoli_memhook_write_r14w_rsp: u8;
    static cannoli_memhook_write_r14w_rsp_end: u8;
    static cannoli_memhook_write_r14w_rbp: u8;
    static cannoli_memhook_write_r14w_rbp_end: u8;
    static cannoli_memhook_write_r14w_rsi: u8;
    static cannoli_memhook_write_r14w_rsi_end: u8;
    static cannoli_memhook_write_r14w_rdi: u8;
    static cannoli_memhook_write_r14w_rdi_end: u8;
    static cannoli_memhook_write_r14w_r8: u8;
    static cannoli_memhook_write_r14w_r8_end: u8;
    static cannoli_memhook_write_r14w_r9: u8;
    static cannoli_memhook_write_r14w_r9_end: u8;
    static cannoli_memhook_write_r14w_r10: u8;
    static cannoli_memhook_write_r14w_r10_end: u8;
    static cannoli_memhook_write_r14w_r11: u8;
    static cannoli_memhook_write_r14w_r11_end: u8;
    static cannoli_memhook_write_r14w_r12: u8;
    static cannoli_memhook_write_r14w_r12_end: u8;
    static cannoli_memhook_write_r14w_r13: u8;
    static cannoli_memhook_write_r14w_r13_end: u8;
    static cannoli_memhook_write_r14w_r14: u8;
    static cannoli_memhook_write_r14w_r14_end: u8;
    static cannoli_memhook_write_r14w_r15: u8;
    static cannoli_memhook_write_r14w_r15_end: u8;
    static cannoli_memhook_write_r15w_rax: u8;
    static cannoli_memhook_write_r15w_rax_end: u8;
    static cannoli_memhook_write_r15w_rcx: u8;
    static cannoli_memhook_write_r15w_rcx_end: u8;
    static cannoli_memhook_write_r15w_rdx: u8;
    static cannoli_memhook_write_r15w_rdx_end: u8;
    static cannoli_memhook_write_r15w_rbx: u8;
    static cannoli_memhook_write_r15w_rbx_end: u8;
    static cannoli_memhook_write_r15w_rsp: u8;
    static cannoli_memhook_write_r15w_rsp_end: u8;
    static cannoli_memhook_write_r15w_rbp: u8;
    static cannoli_memhook_write_r15w_rbp_end: u8;
    static cannoli_memhook_write_r15w_rsi: u8;
    static cannoli_memhook_write_r15w_rsi_end: u8;
    static cannoli_memhook_write_r15w_rdi: u8;
    static cannoli_memhook_write_r15w_rdi_end: u8;
    static cannoli_memhook_write_r15w_r8: u8;
    static cannoli_memhook_write_r15w_r8_end: u8;
    static cannoli_memhook_write_r15w_r9: u8;
    static cannoli_memhook_write_r15w_r9_end: u8;
    static cannoli_memhook_write_r15w_r10: u8;
    static cannoli_memhook_write_r15w_r10_end: u8;
    static cannoli_memhook_write_r15w_r11: u8;
    static cannoli_memhook_write_r15w_r11_end: u8;
    static cannoli_memhook_write_r15w_r12: u8;
    static cannoli_memhook_write_r15w_r12_end: u8;
    static cannoli_memhook_write_r15w_r13: u8;
    static cannoli_memhook_write_r15w_r13_end: u8;
    static cannoli_memhook_write_r15w_r14: u8;
    static cannoli_memhook_write_r15w_r14_end: u8;
    static cannoli_memhook_write_r15w_r15: u8;
    static cannoli_memhook_write_r15w_r15_end: u8;
    static cannoli_memhook_write_eax_eax: u8;
    static cannoli_memhook_write_eax_eax_end: u8;
    static cannoli_memhook_write_eax_ecx: u8;
    static cannoli_memhook_write_eax_ecx_end: u8;
    static cannoli_memhook_write_eax_edx: u8;
    static cannoli_memhook_write_eax_edx_end: u8;
    static cannoli_memhook_write_eax_ebx: u8;
    static cannoli_memhook_write_eax_ebx_end: u8;
    static cannoli_memhook_write_eax_esp: u8;
    static cannoli_memhook_write_eax_esp_end: u8;
    static cannoli_memhook_write_eax_ebp: u8;
    static cannoli_memhook_write_eax_ebp_end: u8;
    static cannoli_memhook_write_eax_esi: u8;
    static cannoli_memhook_write_eax_esi_end: u8;
    static cannoli_memhook_write_eax_edi: u8;
    static cannoli_memhook_write_eax_edi_end: u8;
    static cannoli_memhook_write_eax_r8d: u8;
    static cannoli_memhook_write_eax_r8d_end: u8;
    static cannoli_memhook_write_eax_r9d: u8;
    static cannoli_memhook_write_eax_r9d_end: u8;
    static cannoli_memhook_write_eax_r10d: u8;
    static cannoli_memhook_write_eax_r10d_end: u8;
    static cannoli_memhook_write_eax_r11d: u8;
    static cannoli_memhook_write_eax_r11d_end: u8;
    static cannoli_memhook_write_eax_r12d: u8;
    static cannoli_memhook_write_eax_r12d_end: u8;
    static cannoli_memhook_write_eax_r13d: u8;
    static cannoli_memhook_write_eax_r13d_end: u8;
    static cannoli_memhook_write_eax_r14d: u8;
    static cannoli_memhook_write_eax_r14d_end: u8;
    static cannoli_memhook_write_eax_r15d: u8;
    static cannoli_memhook_write_eax_r15d_end: u8;
    static cannoli_memhook_write_ecx_eax: u8;
    static cannoli_memhook_write_ecx_eax_end: u8;
    static cannoli_memhook_write_ecx_ecx: u8;
    static cannoli_memhook_write_ecx_ecx_end: u8;
    static cannoli_memhook_write_ecx_edx: u8;
    static cannoli_memhook_write_ecx_edx_end: u8;
    static cannoli_memhook_write_ecx_ebx: u8;
    static cannoli_memhook_write_ecx_ebx_end: u8;
    static cannoli_memhook_write_ecx_esp: u8;
    static cannoli_memhook_write_ecx_esp_end: u8;
    static cannoli_memhook_write_ecx_ebp: u8;
    static cannoli_memhook_write_ecx_ebp_end: u8;
    static cannoli_memhook_write_ecx_esi: u8;
    static cannoli_memhook_write_ecx_esi_end: u8;
    static cannoli_memhook_write_ecx_edi: u8;
    static cannoli_memhook_write_ecx_edi_end: u8;
    static cannoli_memhook_write_ecx_r8d: u8;
    static cannoli_memhook_write_ecx_r8d_end: u8;
    static cannoli_memhook_write_ecx_r9d: u8;
    static cannoli_memhook_write_ecx_r9d_end: u8;
    static cannoli_memhook_write_ecx_r10d: u8;
    static cannoli_memhook_write_ecx_r10d_end: u8;
    static cannoli_memhook_write_ecx_r11d: u8;
    static cannoli_memhook_write_ecx_r11d_end: u8;
    static cannoli_memhook_write_ecx_r12d: u8;
    static cannoli_memhook_write_ecx_r12d_end: u8;
    static cannoli_memhook_write_ecx_r13d: u8;
    static cannoli_memhook_write_ecx_r13d_end: u8;
    static cannoli_memhook_write_ecx_r14d: u8;
    static cannoli_memhook_write_ecx_r14d_end: u8;
    static cannoli_memhook_write_ecx_r15d: u8;
    static cannoli_memhook_write_ecx_r15d_end: u8;
    static cannoli_memhook_write_edx_eax: u8;
    static cannoli_memhook_write_edx_eax_end: u8;
    static cannoli_memhook_write_edx_ecx: u8;
    static cannoli_memhook_write_edx_ecx_end: u8;
    static cannoli_memhook_write_edx_edx: u8;
    static cannoli_memhook_write_edx_edx_end: u8;
    static cannoli_memhook_write_edx_ebx: u8;
    static cannoli_memhook_write_edx_ebx_end: u8;
    static cannoli_memhook_write_edx_esp: u8;
    static cannoli_memhook_write_edx_esp_end: u8;
    static cannoli_memhook_write_edx_ebp: u8;
    static cannoli_memhook_write_edx_ebp_end: u8;
    static cannoli_memhook_write_edx_esi: u8;
    static cannoli_memhook_write_edx_esi_end: u8;
    static cannoli_memhook_write_edx_edi: u8;
    static cannoli_memhook_write_edx_edi_end: u8;
    static cannoli_memhook_write_edx_r8d: u8;
    static cannoli_memhook_write_edx_r8d_end: u8;
    static cannoli_memhook_write_edx_r9d: u8;
    static cannoli_memhook_write_edx_r9d_end: u8;
    static cannoli_memhook_write_edx_r10d: u8;
    static cannoli_memhook_write_edx_r10d_end: u8;
    static cannoli_memhook_write_edx_r11d: u8;
    static cannoli_memhook_write_edx_r11d_end: u8;
    static cannoli_memhook_write_edx_r12d: u8;
    static cannoli_memhook_write_edx_r12d_end: u8;
    static cannoli_memhook_write_edx_r13d: u8;
    static cannoli_memhook_write_edx_r13d_end: u8;
    static cannoli_memhook_write_edx_r14d: u8;
    static cannoli_memhook_write_edx_r14d_end: u8;
    static cannoli_memhook_write_edx_r15d: u8;
    static cannoli_memhook_write_edx_r15d_end: u8;
    static cannoli_memhook_write_ebx_eax: u8;
    static cannoli_memhook_write_ebx_eax_end: u8;
    static cannoli_memhook_write_ebx_ecx: u8;
    static cannoli_memhook_write_ebx_ecx_end: u8;
    static cannoli_memhook_write_ebx_edx: u8;
    static cannoli_memhook_write_ebx_edx_end: u8;
    static cannoli_memhook_write_ebx_ebx: u8;
    static cannoli_memhook_write_ebx_ebx_end: u8;
    static cannoli_memhook_write_ebx_esp: u8;
    static cannoli_memhook_write_ebx_esp_end: u8;
    static cannoli_memhook_write_ebx_ebp: u8;
    static cannoli_memhook_write_ebx_ebp_end: u8;
    static cannoli_memhook_write_ebx_esi: u8;
    static cannoli_memhook_write_ebx_esi_end: u8;
    static cannoli_memhook_write_ebx_edi: u8;
    static cannoli_memhook_write_ebx_edi_end: u8;
    static cannoli_memhook_write_ebx_r8d: u8;
    static cannoli_memhook_write_ebx_r8d_end: u8;
    static cannoli_memhook_write_ebx_r9d: u8;
    static cannoli_memhook_write_ebx_r9d_end: u8;
    static cannoli_memhook_write_ebx_r10d: u8;
    static cannoli_memhook_write_ebx_r10d_end: u8;
    static cannoli_memhook_write_ebx_r11d: u8;
    static cannoli_memhook_write_ebx_r11d_end: u8;
    static cannoli_memhook_write_ebx_r12d: u8;
    static cannoli_memhook_write_ebx_r12d_end: u8;
    static cannoli_memhook_write_ebx_r13d: u8;
    static cannoli_memhook_write_ebx_r13d_end: u8;
    static cannoli_memhook_write_ebx_r14d: u8;
    static cannoli_memhook_write_ebx_r14d_end: u8;
    static cannoli_memhook_write_ebx_r15d: u8;
    static cannoli_memhook_write_ebx_r15d_end: u8;
    static cannoli_memhook_write_esp_eax: u8;
    static cannoli_memhook_write_esp_eax_end: u8;
    static cannoli_memhook_write_esp_ecx: u8;
    static cannoli_memhook_write_esp_ecx_end: u8;
    static cannoli_memhook_write_esp_edx: u8;
    static cannoli_memhook_write_esp_edx_end: u8;
    static cannoli_memhook_write_esp_ebx: u8;
    static cannoli_memhook_write_esp_ebx_end: u8;
    static cannoli_memhook_write_esp_esp: u8;
    static cannoli_memhook_write_esp_esp_end: u8;
    static cannoli_memhook_write_esp_ebp: u8;
    static cannoli_memhook_write_esp_ebp_end: u8;
    static cannoli_memhook_write_esp_esi: u8;
    static cannoli_memhook_write_esp_esi_end: u8;
    static cannoli_memhook_write_esp_edi: u8;
    static cannoli_memhook_write_esp_edi_end: u8;
    static cannoli_memhook_write_esp_r8d: u8;
    static cannoli_memhook_write_esp_r8d_end: u8;
    static cannoli_memhook_write_esp_r9d: u8;
    static cannoli_memhook_write_esp_r9d_end: u8;
    static cannoli_memhook_write_esp_r10d: u8;
    static cannoli_memhook_write_esp_r10d_end: u8;
    static cannoli_memhook_write_esp_r11d: u8;
    static cannoli_memhook_write_esp_r11d_end: u8;
    static cannoli_memhook_write_esp_r12d: u8;
    static cannoli_memhook_write_esp_r12d_end: u8;
    static cannoli_memhook_write_esp_r13d: u8;
    static cannoli_memhook_write_esp_r13d_end: u8;
    static cannoli_memhook_write_esp_r14d: u8;
    static cannoli_memhook_write_esp_r14d_end: u8;
    static cannoli_memhook_write_esp_r15d: u8;
    static cannoli_memhook_write_esp_r15d_end: u8;
    static cannoli_memhook_write_ebp_eax: u8;
    static cannoli_memhook_write_ebp_eax_end: u8;
    static cannoli_memhook_write_ebp_ecx: u8;
    static cannoli_memhook_write_ebp_ecx_end: u8;
    static cannoli_memhook_write_ebp_edx: u8;
    static cannoli_memhook_write_ebp_edx_end: u8;
    static cannoli_memhook_write_ebp_ebx: u8;
    static cannoli_memhook_write_ebp_ebx_end: u8;
    static cannoli_memhook_write_ebp_esp: u8;
    static cannoli_memhook_write_ebp_esp_end: u8;
    static cannoli_memhook_write_ebp_ebp: u8;
    static cannoli_memhook_write_ebp_ebp_end: u8;
    static cannoli_memhook_write_ebp_esi: u8;
    static cannoli_memhook_write_ebp_esi_end: u8;
    static cannoli_memhook_write_ebp_edi: u8;
    static cannoli_memhook_write_ebp_edi_end: u8;
    static cannoli_memhook_write_ebp_r8d: u8;
    static cannoli_memhook_write_ebp_r8d_end: u8;
    static cannoli_memhook_write_ebp_r9d: u8;
    static cannoli_memhook_write_ebp_r9d_end: u8;
    static cannoli_memhook_write_ebp_r10d: u8;
    static cannoli_memhook_write_ebp_r10d_end: u8;
    static cannoli_memhook_write_ebp_r11d: u8;
    static cannoli_memhook_write_ebp_r11d_end: u8;
    static cannoli_memhook_write_ebp_r12d: u8;
    static cannoli_memhook_write_ebp_r12d_end: u8;
    static cannoli_memhook_write_ebp_r13d: u8;
    static cannoli_memhook_write_ebp_r13d_end: u8;
    static cannoli_memhook_write_ebp_r14d: u8;
    static cannoli_memhook_write_ebp_r14d_end: u8;
    static cannoli_memhook_write_ebp_r15d: u8;
    static cannoli_memhook_write_ebp_r15d_end: u8;
    static cannoli_memhook_write_esi_eax: u8;
    static cannoli_memhook_write_esi_eax_end: u8;
    static cannoli_memhook_write_esi_ecx: u8;
    static cannoli_memhook_write_esi_ecx_end: u8;
    static cannoli_memhook_write_esi_edx: u8;
    static cannoli_memhook_write_esi_edx_end: u8;
    static cannoli_memhook_write_esi_ebx: u8;
    static cannoli_memhook_write_esi_ebx_end: u8;
    static cannoli_memhook_write_esi_esp: u8;
    static cannoli_memhook_write_esi_esp_end: u8;
    static cannoli_memhook_write_esi_ebp: u8;
    static cannoli_memhook_write_esi_ebp_end: u8;
    static cannoli_memhook_write_esi_esi: u8;
    static cannoli_memhook_write_esi_esi_end: u8;
    static cannoli_memhook_write_esi_edi: u8;
    static cannoli_memhook_write_esi_edi_end: u8;
    static cannoli_memhook_write_esi_r8d: u8;
    static cannoli_memhook_write_esi_r8d_end: u8;
    static cannoli_memhook_write_esi_r9d: u8;
    static cannoli_memhook_write_esi_r9d_end: u8;
    static cannoli_memhook_write_esi_r10d: u8;
    static cannoli_memhook_write_esi_r10d_end: u8;
    static cannoli_memhook_write_esi_r11d: u8;
    static cannoli_memhook_write_esi_r11d_end: u8;
    static cannoli_memhook_write_esi_r12d: u8;
    static cannoli_memhook_write_esi_r12d_end: u8;
    static cannoli_memhook_write_esi_r13d: u8;
    static cannoli_memhook_write_esi_r13d_end: u8;
    static cannoli_memhook_write_esi_r14d: u8;
    static cannoli_memhook_write_esi_r14d_end: u8;
    static cannoli_memhook_write_esi_r15d: u8;
    static cannoli_memhook_write_esi_r15d_end: u8;
    static cannoli_memhook_write_edi_eax: u8;
    static cannoli_memhook_write_edi_eax_end: u8;
    static cannoli_memhook_write_edi_ecx: u8;
    static cannoli_memhook_write_edi_ecx_end: u8;
    static cannoli_memhook_write_edi_edx: u8;
    static cannoli_memhook_write_edi_edx_end: u8;
    static cannoli_memhook_write_edi_ebx: u8;
    static cannoli_memhook_write_edi_ebx_end: u8;
    static cannoli_memhook_write_edi_esp: u8;
    static cannoli_memhook_write_edi_esp_end: u8;
    static cannoli_memhook_write_edi_ebp: u8;
    static cannoli_memhook_write_edi_ebp_end: u8;
    static cannoli_memhook_write_edi_esi: u8;
    static cannoli_memhook_write_edi_esi_end: u8;
    static cannoli_memhook_write_edi_edi: u8;
    static cannoli_memhook_write_edi_edi_end: u8;
    static cannoli_memhook_write_edi_r8d: u8;
    static cannoli_memhook_write_edi_r8d_end: u8;
    static cannoli_memhook_write_edi_r9d: u8;
    static cannoli_memhook_write_edi_r9d_end: u8;
    static cannoli_memhook_write_edi_r10d: u8;
    static cannoli_memhook_write_edi_r10d_end: u8;
    static cannoli_memhook_write_edi_r11d: u8;
    static cannoli_memhook_write_edi_r11d_end: u8;
    static cannoli_memhook_write_edi_r12d: u8;
    static cannoli_memhook_write_edi_r12d_end: u8;
    static cannoli_memhook_write_edi_r13d: u8;
    static cannoli_memhook_write_edi_r13d_end: u8;
    static cannoli_memhook_write_edi_r14d: u8;
    static cannoli_memhook_write_edi_r14d_end: u8;
    static cannoli_memhook_write_edi_r15d: u8;
    static cannoli_memhook_write_edi_r15d_end: u8;
    static cannoli_memhook_write_r8d_eax: u8;
    static cannoli_memhook_write_r8d_eax_end: u8;
    static cannoli_memhook_write_r8d_ecx: u8;
    static cannoli_memhook_write_r8d_ecx_end: u8;
    static cannoli_memhook_write_r8d_edx: u8;
    static cannoli_memhook_write_r8d_edx_end: u8;
    static cannoli_memhook_write_r8d_ebx: u8;
    static cannoli_memhook_write_r8d_ebx_end: u8;
    static cannoli_memhook_write_r8d_esp: u8;
    static cannoli_memhook_write_r8d_esp_end: u8;
    static cannoli_memhook_write_r8d_ebp: u8;
    static cannoli_memhook_write_r8d_ebp_end: u8;
    static cannoli_memhook_write_r8d_esi: u8;
    static cannoli_memhook_write_r8d_esi_end: u8;
    static cannoli_memhook_write_r8d_edi: u8;
    static cannoli_memhook_write_r8d_edi_end: u8;
    static cannoli_memhook_write_r8d_r8d: u8;
    static cannoli_memhook_write_r8d_r8d_end: u8;
    static cannoli_memhook_write_r8d_r9d: u8;
    static cannoli_memhook_write_r8d_r9d_end: u8;
    static cannoli_memhook_write_r8d_r10d: u8;
    static cannoli_memhook_write_r8d_r10d_end: u8;
    static cannoli_memhook_write_r8d_r11d: u8;
    static cannoli_memhook_write_r8d_r11d_end: u8;
    static cannoli_memhook_write_r8d_r12d: u8;
    static cannoli_memhook_write_r8d_r12d_end: u8;
    static cannoli_memhook_write_r8d_r13d: u8;
    static cannoli_memhook_write_r8d_r13d_end: u8;
    static cannoli_memhook_write_r8d_r14d: u8;
    static cannoli_memhook_write_r8d_r14d_end: u8;
    static cannoli_memhook_write_r8d_r15d: u8;
    static cannoli_memhook_write_r8d_r15d_end: u8;
    static cannoli_memhook_write_r9d_eax: u8;
    static cannoli_memhook_write_r9d_eax_end: u8;
    static cannoli_memhook_write_r9d_ecx: u8;
    static cannoli_memhook_write_r9d_ecx_end: u8;
    static cannoli_memhook_write_r9d_edx: u8;
    static cannoli_memhook_write_r9d_edx_end: u8;
    static cannoli_memhook_write_r9d_ebx: u8;
    static cannoli_memhook_write_r9d_ebx_end: u8;
    static cannoli_memhook_write_r9d_esp: u8;
    static cannoli_memhook_write_r9d_esp_end: u8;
    static cannoli_memhook_write_r9d_ebp: u8;
    static cannoli_memhook_write_r9d_ebp_end: u8;
    static cannoli_memhook_write_r9d_esi: u8;
    static cannoli_memhook_write_r9d_esi_end: u8;
    static cannoli_memhook_write_r9d_edi: u8;
    static cannoli_memhook_write_r9d_edi_end: u8;
    static cannoli_memhook_write_r9d_r8d: u8;
    static cannoli_memhook_write_r9d_r8d_end: u8;
    static cannoli_memhook_write_r9d_r9d: u8;
    static cannoli_memhook_write_r9d_r9d_end: u8;
    static cannoli_memhook_write_r9d_r10d: u8;
    static cannoli_memhook_write_r9d_r10d_end: u8;
    static cannoli_memhook_write_r9d_r11d: u8;
    static cannoli_memhook_write_r9d_r11d_end: u8;
    static cannoli_memhook_write_r9d_r12d: u8;
    static cannoli_memhook_write_r9d_r12d_end: u8;
    static cannoli_memhook_write_r9d_r13d: u8;
    static cannoli_memhook_write_r9d_r13d_end: u8;
    static cannoli_memhook_write_r9d_r14d: u8;
    static cannoli_memhook_write_r9d_r14d_end: u8;
    static cannoli_memhook_write_r9d_r15d: u8;
    static cannoli_memhook_write_r9d_r15d_end: u8;
    static cannoli_memhook_write_r10d_eax: u8;
    static cannoli_memhook_write_r10d_eax_end: u8;
    static cannoli_memhook_write_r10d_ecx: u8;
    static cannoli_memhook_write_r10d_ecx_end: u8;
    static cannoli_memhook_write_r10d_edx: u8;
    static cannoli_memhook_write_r10d_edx_end: u8;
    static cannoli_memhook_write_r10d_ebx: u8;
    static cannoli_memhook_write_r10d_ebx_end: u8;
    static cannoli_memhook_write_r10d_esp: u8;
    static cannoli_memhook_write_r10d_esp_end: u8;
    static cannoli_memhook_write_r10d_ebp: u8;
    static cannoli_memhook_write_r10d_ebp_end: u8;
    static cannoli_memhook_write_r10d_esi: u8;
    static cannoli_memhook_write_r10d_esi_end: u8;
    static cannoli_memhook_write_r10d_edi: u8;
    static cannoli_memhook_write_r10d_edi_end: u8;
    static cannoli_memhook_write_r10d_r8d: u8;
    static cannoli_memhook_write_r10d_r8d_end: u8;
    static cannoli_memhook_write_r10d_r9d: u8;
    static cannoli_memhook_write_r10d_r9d_end: u8;
    static cannoli_memhook_write_r10d_r10d: u8;
    static cannoli_memhook_write_r10d_r10d_end: u8;
    static cannoli_memhook_write_r10d_r11d: u8;
    static cannoli_memhook_write_r10d_r11d_end: u8;
    static cannoli_memhook_write_r10d_r12d: u8;
    static cannoli_memhook_write_r10d_r12d_end: u8;
    static cannoli_memhook_write_r10d_r13d: u8;
    static cannoli_memhook_write_r10d_r13d_end: u8;
    static cannoli_memhook_write_r10d_r14d: u8;
    static cannoli_memhook_write_r10d_r14d_end: u8;
    static cannoli_memhook_write_r10d_r15d: u8;
    static cannoli_memhook_write_r10d_r15d_end: u8;
    static cannoli_memhook_write_r11d_eax: u8;
    static cannoli_memhook_write_r11d_eax_end: u8;
    static cannoli_memhook_write_r11d_ecx: u8;
    static cannoli_memhook_write_r11d_ecx_end: u8;
    static cannoli_memhook_write_r11d_edx: u8;
    static cannoli_memhook_write_r11d_edx_end: u8;
    static cannoli_memhook_write_r11d_ebx: u8;
    static cannoli_memhook_write_r11d_ebx_end: u8;
    static cannoli_memhook_write_r11d_esp: u8;
    static cannoli_memhook_write_r11d_esp_end: u8;
    static cannoli_memhook_write_r11d_ebp: u8;
    static cannoli_memhook_write_r11d_ebp_end: u8;
    static cannoli_memhook_write_r11d_esi: u8;
    static cannoli_memhook_write_r11d_esi_end: u8;
    static cannoli_memhook_write_r11d_edi: u8;
    static cannoli_memhook_write_r11d_edi_end: u8;
    static cannoli_memhook_write_r11d_r8d: u8;
    static cannoli_memhook_write_r11d_r8d_end: u8;
    static cannoli_memhook_write_r11d_r9d: u8;
    static cannoli_memhook_write_r11d_r9d_end: u8;
    static cannoli_memhook_write_r11d_r10d: u8;
    static cannoli_memhook_write_r11d_r10d_end: u8;
    static cannoli_memhook_write_r11d_r11d: u8;
    static cannoli_memhook_write_r11d_r11d_end: u8;
    static cannoli_memhook_write_r11d_r12d: u8;
    static cannoli_memhook_write_r11d_r12d_end: u8;
    static cannoli_memhook_write_r11d_r13d: u8;
    static cannoli_memhook_write_r11d_r13d_end: u8;
    static cannoli_memhook_write_r11d_r14d: u8;
    static cannoli_memhook_write_r11d_r14d_end: u8;
    static cannoli_memhook_write_r11d_r15d: u8;
    static cannoli_memhook_write_r11d_r15d_end: u8;
    static cannoli_memhook_write_r12d_eax: u8;
    static cannoli_memhook_write_r12d_eax_end: u8;
    static cannoli_memhook_write_r12d_ecx: u8;
    static cannoli_memhook_write_r12d_ecx_end: u8;
    static cannoli_memhook_write_r12d_edx: u8;
    static cannoli_memhook_write_r12d_edx_end: u8;
    static cannoli_memhook_write_r12d_ebx: u8;
    static cannoli_memhook_write_r12d_ebx_end: u8;
    static cannoli_memhook_write_r12d_esp: u8;
    static cannoli_memhook_write_r12d_esp_end: u8;
    static cannoli_memhook_write_r12d_ebp: u8;
    static cannoli_memhook_write_r12d_ebp_end: u8;
    static cannoli_memhook_write_r12d_esi: u8;
    static cannoli_memhook_write_r12d_esi_end: u8;
    static cannoli_memhook_write_r12d_edi: u8;
    static cannoli_memhook_write_r12d_edi_end: u8;
    static cannoli_memhook_write_r12d_r8d: u8;
    static cannoli_memhook_write_r12d_r8d_end: u8;
    static cannoli_memhook_write_r12d_r9d: u8;
    static cannoli_memhook_write_r12d_r9d_end: u8;
    static cannoli_memhook_write_r12d_r10d: u8;
    static cannoli_memhook_write_r12d_r10d_end: u8;
    static cannoli_memhook_write_r12d_r11d: u8;
    static cannoli_memhook_write_r12d_r11d_end: u8;
    static cannoli_memhook_write_r12d_r12d: u8;
    static cannoli_memhook_write_r12d_r12d_end: u8;
    static cannoli_memhook_write_r12d_r13d: u8;
    static cannoli_memhook_write_r12d_r13d_end: u8;
    static cannoli_memhook_write_r12d_r14d: u8;
    static cannoli_memhook_write_r12d_r14d_end: u8;
    static cannoli_memhook_write_r12d_r15d: u8;
    static cannoli_memhook_write_r12d_r15d_end: u8;
    static cannoli_memhook_write_r13d_eax: u8;
    static cannoli_memhook_write_r13d_eax_end: u8;
    static cannoli_memhook_write_r13d_ecx: u8;
    static cannoli_memhook_write_r13d_ecx_end: u8;
    static cannoli_memhook_write_r13d_edx: u8;
    static cannoli_memhook_write_r13d_edx_end: u8;
    static cannoli_memhook_write_r13d_ebx: u8;
    static cannoli_memhook_write_r13d_ebx_end: u8;
    static cannoli_memhook_write_r13d_esp: u8;
    static cannoli_memhook_write_r13d_esp_end: u8;
    static cannoli_memhook_write_r13d_ebp: u8;
    static cannoli_memhook_write_r13d_ebp_end: u8;
    static cannoli_memhook_write_r13d_esi: u8;
    static cannoli_memhook_write_r13d_esi_end: u8;
    static cannoli_memhook_write_r13d_edi: u8;
    static cannoli_memhook_write_r13d_edi_end: u8;
    static cannoli_memhook_write_r13d_r8d: u8;
    static cannoli_memhook_write_r13d_r8d_end: u8;
    static cannoli_memhook_write_r13d_r9d: u8;
    static cannoli_memhook_write_r13d_r9d_end: u8;
    static cannoli_memhook_write_r13d_r10d: u8;
    static cannoli_memhook_write_r13d_r10d_end: u8;
    static cannoli_memhook_write_r13d_r11d: u8;
    static cannoli_memhook_write_r13d_r11d_end: u8;
    static cannoli_memhook_write_r13d_r12d: u8;
    static cannoli_memhook_write_r13d_r12d_end: u8;
    static cannoli_memhook_write_r13d_r13d: u8;
    static cannoli_memhook_write_r13d_r13d_end: u8;
    static cannoli_memhook_write_r13d_r14d: u8;
    static cannoli_memhook_write_r13d_r14d_end: u8;
    static cannoli_memhook_write_r13d_r15d: u8;
    static cannoli_memhook_write_r13d_r15d_end: u8;
    static cannoli_memhook_write_r14d_eax: u8;
    static cannoli_memhook_write_r14d_eax_end: u8;
    static cannoli_memhook_write_r14d_ecx: u8;
    static cannoli_memhook_write_r14d_ecx_end: u8;
    static cannoli_memhook_write_r14d_edx: u8;
    static cannoli_memhook_write_r14d_edx_end: u8;
    static cannoli_memhook_write_r14d_ebx: u8;
    static cannoli_memhook_write_r14d_ebx_end: u8;
    static cannoli_memhook_write_r14d_esp: u8;
    static cannoli_memhook_write_r14d_esp_end: u8;
    static cannoli_memhook_write_r14d_ebp: u8;
    static cannoli_memhook_write_r14d_ebp_end: u8;
    static cannoli_memhook_write_r14d_esi: u8;
    static cannoli_memhook_write_r14d_esi_end: u8;
    static cannoli_memhook_write_r14d_edi: u8;
    static cannoli_memhook_write_r14d_edi_end: u8;
    static cannoli_memhook_write_r14d_r8d: u8;
    static cannoli_memhook_write_r14d_r8d_end: u8;
    static cannoli_memhook_write_r14d_r9d: u8;
    static cannoli_memhook_write_r14d_r9d_end: u8;
    static cannoli_memhook_write_r14d_r10d: u8;
    static cannoli_memhook_write_r14d_r10d_end: u8;
    static cannoli_memhook_write_r14d_r11d: u8;
    static cannoli_memhook_write_r14d_r11d_end: u8;
    static cannoli_memhook_write_r14d_r12d: u8;
    static cannoli_memhook_write_r14d_r12d_end: u8;
    static cannoli_memhook_write_r14d_r13d: u8;
    static cannoli_memhook_write_r14d_r13d_end: u8;
    static cannoli_memhook_write_r14d_r14d: u8;
    static cannoli_memhook_write_r14d_r14d_end: u8;
    static cannoli_memhook_write_r14d_r15d: u8;
    static cannoli_memhook_write_r14d_r15d_end: u8;
    static cannoli_memhook_write_r15d_eax: u8;
    static cannoli_memhook_write_r15d_eax_end: u8;
    static cannoli_memhook_write_r15d_ecx: u8;
    static cannoli_memhook_write_r15d_ecx_end: u8;
    static cannoli_memhook_write_r15d_edx: u8;
    static cannoli_memhook_write_r15d_edx_end: u8;
    static cannoli_memhook_write_r15d_ebx: u8;
    static cannoli_memhook_write_r15d_ebx_end: u8;
    static cannoli_memhook_write_r15d_esp: u8;
    static cannoli_memhook_write_r15d_esp_end: u8;
    static cannoli_memhook_write_r15d_ebp: u8;
    static cannoli_memhook_write_r15d_ebp_end: u8;
    static cannoli_memhook_write_r15d_esi: u8;
    static cannoli_memhook_write_r15d_esi_end: u8;
    static cannoli_memhook_write_r15d_edi: u8;
    static cannoli_memhook_write_r15d_edi_end: u8;
    static cannoli_memhook_write_r15d_r8d: u8;
    static cannoli_memhook_write_r15d_r8d_end: u8;
    static cannoli_memhook_write_r15d_r9d: u8;
    static cannoli_memhook_write_r15d_r9d_end: u8;
    static cannoli_memhook_write_r15d_r10d: u8;
    static cannoli_memhook_write_r15d_r10d_end: u8;
    static cannoli_memhook_write_r15d_r11d: u8;
    static cannoli_memhook_write_r15d_r11d_end: u8;
    static cannoli_memhook_write_r15d_r12d: u8;
    static cannoli_memhook_write_r15d_r12d_end: u8;
    static cannoli_memhook_write_r15d_r13d: u8;
    static cannoli_memhook_write_r15d_r13d_end: u8;
    static cannoli_memhook_write_r15d_r14d: u8;
    static cannoli_memhook_write_r15d_r14d_end: u8;
    static cannoli_memhook_write_r15d_r15d: u8;
    static cannoli_memhook_write_r15d_r15d_end: u8;
    static cannoli_memhook_write_eax_rax: u8;
    static cannoli_memhook_write_eax_rax_end: u8;
    static cannoli_memhook_write_eax_rcx: u8;
    static cannoli_memhook_write_eax_rcx_end: u8;
    static cannoli_memhook_write_eax_rdx: u8;
    static cannoli_memhook_write_eax_rdx_end: u8;
    static cannoli_memhook_write_eax_rbx: u8;
    static cannoli_memhook_write_eax_rbx_end: u8;
    static cannoli_memhook_write_eax_rsp: u8;
    static cannoli_memhook_write_eax_rsp_end: u8;
    static cannoli_memhook_write_eax_rbp: u8;
    static cannoli_memhook_write_eax_rbp_end: u8;
    static cannoli_memhook_write_eax_rsi: u8;
    static cannoli_memhook_write_eax_rsi_end: u8;
    static cannoli_memhook_write_eax_rdi: u8;
    static cannoli_memhook_write_eax_rdi_end: u8;
    static cannoli_memhook_write_eax_r8: u8;
    static cannoli_memhook_write_eax_r8_end: u8;
    static cannoli_memhook_write_eax_r9: u8;
    static cannoli_memhook_write_eax_r9_end: u8;
    static cannoli_memhook_write_eax_r10: u8;
    static cannoli_memhook_write_eax_r10_end: u8;
    static cannoli_memhook_write_eax_r11: u8;
    static cannoli_memhook_write_eax_r11_end: u8;
    static cannoli_memhook_write_eax_r12: u8;
    static cannoli_memhook_write_eax_r12_end: u8;
    static cannoli_memhook_write_eax_r13: u8;
    static cannoli_memhook_write_eax_r13_end: u8;
    static cannoli_memhook_write_eax_r14: u8;
    static cannoli_memhook_write_eax_r14_end: u8;
    static cannoli_memhook_write_eax_r15: u8;
    static cannoli_memhook_write_eax_r15_end: u8;
    static cannoli_memhook_write_ecx_rax: u8;
    static cannoli_memhook_write_ecx_rax_end: u8;
    static cannoli_memhook_write_ecx_rcx: u8;
    static cannoli_memhook_write_ecx_rcx_end: u8;
    static cannoli_memhook_write_ecx_rdx: u8;
    static cannoli_memhook_write_ecx_rdx_end: u8;
    static cannoli_memhook_write_ecx_rbx: u8;
    static cannoli_memhook_write_ecx_rbx_end: u8;
    static cannoli_memhook_write_ecx_rsp: u8;
    static cannoli_memhook_write_ecx_rsp_end: u8;
    static cannoli_memhook_write_ecx_rbp: u8;
    static cannoli_memhook_write_ecx_rbp_end: u8;
    static cannoli_memhook_write_ecx_rsi: u8;
    static cannoli_memhook_write_ecx_rsi_end: u8;
    static cannoli_memhook_write_ecx_rdi: u8;
    static cannoli_memhook_write_ecx_rdi_end: u8;
    static cannoli_memhook_write_ecx_r8: u8;
    static cannoli_memhook_write_ecx_r8_end: u8;
    static cannoli_memhook_write_ecx_r9: u8;
    static cannoli_memhook_write_ecx_r9_end: u8;
    static cannoli_memhook_write_ecx_r10: u8;
    static cannoli_memhook_write_ecx_r10_end: u8;
    static cannoli_memhook_write_ecx_r11: u8;
    static cannoli_memhook_write_ecx_r11_end: u8;
    static cannoli_memhook_write_ecx_r12: u8;
    static cannoli_memhook_write_ecx_r12_end: u8;
    static cannoli_memhook_write_ecx_r13: u8;
    static cannoli_memhook_write_ecx_r13_end: u8;
    static cannoli_memhook_write_ecx_r14: u8;
    static cannoli_memhook_write_ecx_r14_end: u8;
    static cannoli_memhook_write_ecx_r15: u8;
    static cannoli_memhook_write_ecx_r15_end: u8;
    static cannoli_memhook_write_edx_rax: u8;
    static cannoli_memhook_write_edx_rax_end: u8;
    static cannoli_memhook_write_edx_rcx: u8;
    static cannoli_memhook_write_edx_rcx_end: u8;
    static cannoli_memhook_write_edx_rdx: u8;
    static cannoli_memhook_write_edx_rdx_end: u8;
    static cannoli_memhook_write_edx_rbx: u8;
    static cannoli_memhook_write_edx_rbx_end: u8;
    static cannoli_memhook_write_edx_rsp: u8;
    static cannoli_memhook_write_edx_rsp_end: u8;
    static cannoli_memhook_write_edx_rbp: u8;
    static cannoli_memhook_write_edx_rbp_end: u8;
    static cannoli_memhook_write_edx_rsi: u8;
    static cannoli_memhook_write_edx_rsi_end: u8;
    static cannoli_memhook_write_edx_rdi: u8;
    static cannoli_memhook_write_edx_rdi_end: u8;
    static cannoli_memhook_write_edx_r8: u8;
    static cannoli_memhook_write_edx_r8_end: u8;
    static cannoli_memhook_write_edx_r9: u8;
    static cannoli_memhook_write_edx_r9_end: u8;
    static cannoli_memhook_write_edx_r10: u8;
    static cannoli_memhook_write_edx_r10_end: u8;
    static cannoli_memhook_write_edx_r11: u8;
    static cannoli_memhook_write_edx_r11_end: u8;
    static cannoli_memhook_write_edx_r12: u8;
    static cannoli_memhook_write_edx_r12_end: u8;
    static cannoli_memhook_write_edx_r13: u8;
    static cannoli_memhook_write_edx_r13_end: u8;
    static cannoli_memhook_write_edx_r14: u8;
    static cannoli_memhook_write_edx_r14_end: u8;
    static cannoli_memhook_write_edx_r15: u8;
    static cannoli_memhook_write_edx_r15_end: u8;
    static cannoli_memhook_write_ebx_rax: u8;
    static cannoli_memhook_write_ebx_rax_end: u8;
    static cannoli_memhook_write_ebx_rcx: u8;
    static cannoli_memhook_write_ebx_rcx_end: u8;
    static cannoli_memhook_write_ebx_rdx: u8;
    static cannoli_memhook_write_ebx_rdx_end: u8;
    static cannoli_memhook_write_ebx_rbx: u8;
    static cannoli_memhook_write_ebx_rbx_end: u8;
    static cannoli_memhook_write_ebx_rsp: u8;
    static cannoli_memhook_write_ebx_rsp_end: u8;
    static cannoli_memhook_write_ebx_rbp: u8;
    static cannoli_memhook_write_ebx_rbp_end: u8;
    static cannoli_memhook_write_ebx_rsi: u8;
    static cannoli_memhook_write_ebx_rsi_end: u8;
    static cannoli_memhook_write_ebx_rdi: u8;
    static cannoli_memhook_write_ebx_rdi_end: u8;
    static cannoli_memhook_write_ebx_r8: u8;
    static cannoli_memhook_write_ebx_r8_end: u8;
    static cannoli_memhook_write_ebx_r9: u8;
    static cannoli_memhook_write_ebx_r9_end: u8;
    static cannoli_memhook_write_ebx_r10: u8;
    static cannoli_memhook_write_ebx_r10_end: u8;
    static cannoli_memhook_write_ebx_r11: u8;
    static cannoli_memhook_write_ebx_r11_end: u8;
    static cannoli_memhook_write_ebx_r12: u8;
    static cannoli_memhook_write_ebx_r12_end: u8;
    static cannoli_memhook_write_ebx_r13: u8;
    static cannoli_memhook_write_ebx_r13_end: u8;
    static cannoli_memhook_write_ebx_r14: u8;
    static cannoli_memhook_write_ebx_r14_end: u8;
    static cannoli_memhook_write_ebx_r15: u8;
    static cannoli_memhook_write_ebx_r15_end: u8;
    static cannoli_memhook_write_esp_rax: u8;
    static cannoli_memhook_write_esp_rax_end: u8;
    static cannoli_memhook_write_esp_rcx: u8;
    static cannoli_memhook_write_esp_rcx_end: u8;
    static cannoli_memhook_write_esp_rdx: u8;
    static cannoli_memhook_write_esp_rdx_end: u8;
    static cannoli_memhook_write_esp_rbx: u8;
    static cannoli_memhook_write_esp_rbx_end: u8;
    static cannoli_memhook_write_esp_rsp: u8;
    static cannoli_memhook_write_esp_rsp_end: u8;
    static cannoli_memhook_write_esp_rbp: u8;
    static cannoli_memhook_write_esp_rbp_end: u8;
    static cannoli_memhook_write_esp_rsi: u8;
    static cannoli_memhook_write_esp_rsi_end: u8;
    static cannoli_memhook_write_esp_rdi: u8;
    static cannoli_memhook_write_esp_rdi_end: u8;
    static cannoli_memhook_write_esp_r8: u8;
    static cannoli_memhook_write_esp_r8_end: u8;
    static cannoli_memhook_write_esp_r9: u8;
    static cannoli_memhook_write_esp_r9_end: u8;
    static cannoli_memhook_write_esp_r10: u8;
    static cannoli_memhook_write_esp_r10_end: u8;
    static cannoli_memhook_write_esp_r11: u8;
    static cannoli_memhook_write_esp_r11_end: u8;
    static cannoli_memhook_write_esp_r12: u8;
    static cannoli_memhook_write_esp_r12_end: u8;
    static cannoli_memhook_write_esp_r13: u8;
    static cannoli_memhook_write_esp_r13_end: u8;
    static cannoli_memhook_write_esp_r14: u8;
    static cannoli_memhook_write_esp_r14_end: u8;
    static cannoli_memhook_write_esp_r15: u8;
    static cannoli_memhook_write_esp_r15_end: u8;
    static cannoli_memhook_write_ebp_rax: u8;
    static cannoli_memhook_write_ebp_rax_end: u8;
    static cannoli_memhook_write_ebp_rcx: u8;
    static cannoli_memhook_write_ebp_rcx_end: u8;
    static cannoli_memhook_write_ebp_rdx: u8;
    static cannoli_memhook_write_ebp_rdx_end: u8;
    static cannoli_memhook_write_ebp_rbx: u8;
    static cannoli_memhook_write_ebp_rbx_end: u8;
    static cannoli_memhook_write_ebp_rsp: u8;
    static cannoli_memhook_write_ebp_rsp_end: u8;
    static cannoli_memhook_write_ebp_rbp: u8;
    static cannoli_memhook_write_ebp_rbp_end: u8;
    static cannoli_memhook_write_ebp_rsi: u8;
    static cannoli_memhook_write_ebp_rsi_end: u8;
    static cannoli_memhook_write_ebp_rdi: u8;
    static cannoli_memhook_write_ebp_rdi_end: u8;
    static cannoli_memhook_write_ebp_r8: u8;
    static cannoli_memhook_write_ebp_r8_end: u8;
    static cannoli_memhook_write_ebp_r9: u8;
    static cannoli_memhook_write_ebp_r9_end: u8;
    static cannoli_memhook_write_ebp_r10: u8;
    static cannoli_memhook_write_ebp_r10_end: u8;
    static cannoli_memhook_write_ebp_r11: u8;
    static cannoli_memhook_write_ebp_r11_end: u8;
    static cannoli_memhook_write_ebp_r12: u8;
    static cannoli_memhook_write_ebp_r12_end: u8;
    static cannoli_memhook_write_ebp_r13: u8;
    static cannoli_memhook_write_ebp_r13_end: u8;
    static cannoli_memhook_write_ebp_r14: u8;
    static cannoli_memhook_write_ebp_r14_end: u8;
    static cannoli_memhook_write_ebp_r15: u8;
    static cannoli_memhook_write_ebp_r15_end: u8;
    static cannoli_memhook_write_esi_rax: u8;
    static cannoli_memhook_write_esi_rax_end: u8;
    static cannoli_memhook_write_esi_rcx: u8;
    static cannoli_memhook_write_esi_rcx_end: u8;
    static cannoli_memhook_write_esi_rdx: u8;
    static cannoli_memhook_write_esi_rdx_end: u8;
    static cannoli_memhook_write_esi_rbx: u8;
    static cannoli_memhook_write_esi_rbx_end: u8;
    static cannoli_memhook_write_esi_rsp: u8;
    static cannoli_memhook_write_esi_rsp_end: u8;
    static cannoli_memhook_write_esi_rbp: u8;
    static cannoli_memhook_write_esi_rbp_end: u8;
    static cannoli_memhook_write_esi_rsi: u8;
    static cannoli_memhook_write_esi_rsi_end: u8;
    static cannoli_memhook_write_esi_rdi: u8;
    static cannoli_memhook_write_esi_rdi_end: u8;
    static cannoli_memhook_write_esi_r8: u8;
    static cannoli_memhook_write_esi_r8_end: u8;
    static cannoli_memhook_write_esi_r9: u8;
    static cannoli_memhook_write_esi_r9_end: u8;
    static cannoli_memhook_write_esi_r10: u8;
    static cannoli_memhook_write_esi_r10_end: u8;
    static cannoli_memhook_write_esi_r11: u8;
    static cannoli_memhook_write_esi_r11_end: u8;
    static cannoli_memhook_write_esi_r12: u8;
    static cannoli_memhook_write_esi_r12_end: u8;
    static cannoli_memhook_write_esi_r13: u8;
    static cannoli_memhook_write_esi_r13_end: u8;
    static cannoli_memhook_write_esi_r14: u8;
    static cannoli_memhook_write_esi_r14_end: u8;
    static cannoli_memhook_write_esi_r15: u8;
    static cannoli_memhook_write_esi_r15_end: u8;
    static cannoli_memhook_write_edi_rax: u8;
    static cannoli_memhook_write_edi_rax_end: u8;
    static cannoli_memhook_write_edi_rcx: u8;
    static cannoli_memhook_write_edi_rcx_end: u8;
    static cannoli_memhook_write_edi_rdx: u8;
    static cannoli_memhook_write_edi_rdx_end: u8;
    static cannoli_memhook_write_edi_rbx: u8;
    static cannoli_memhook_write_edi_rbx_end: u8;
    static cannoli_memhook_write_edi_rsp: u8;
    static cannoli_memhook_write_edi_rsp_end: u8;
    static cannoli_memhook_write_edi_rbp: u8;
    static cannoli_memhook_write_edi_rbp_end: u8;
    static cannoli_memhook_write_edi_rsi: u8;
    static cannoli_memhook_write_edi_rsi_end: u8;
    static cannoli_memhook_write_edi_rdi: u8;
    static cannoli_memhook_write_edi_rdi_end: u8;
    static cannoli_memhook_write_edi_r8: u8;
    static cannoli_memhook_write_edi_r8_end: u8;
    static cannoli_memhook_write_edi_r9: u8;
    static cannoli_memhook_write_edi_r9_end: u8;
    static cannoli_memhook_write_edi_r10: u8;
    static cannoli_memhook_write_edi_r10_end: u8;
    static cannoli_memhook_write_edi_r11: u8;
    static cannoli_memhook_write_edi_r11_end: u8;
    static cannoli_memhook_write_edi_r12: u8;
    static cannoli_memhook_write_edi_r12_end: u8;
    static cannoli_memhook_write_edi_r13: u8;
    static cannoli_memhook_write_edi_r13_end: u8;
    static cannoli_memhook_write_edi_r14: u8;
    static cannoli_memhook_write_edi_r14_end: u8;
    static cannoli_memhook_write_edi_r15: u8;
    static cannoli_memhook_write_edi_r15_end: u8;
    static cannoli_memhook_write_r8d_rax: u8;
    static cannoli_memhook_write_r8d_rax_end: u8;
    static cannoli_memhook_write_r8d_rcx: u8;
    static cannoli_memhook_write_r8d_rcx_end: u8;
    static cannoli_memhook_write_r8d_rdx: u8;
    static cannoli_memhook_write_r8d_rdx_end: u8;
    static cannoli_memhook_write_r8d_rbx: u8;
    static cannoli_memhook_write_r8d_rbx_end: u8;
    static cannoli_memhook_write_r8d_rsp: u8;
    static cannoli_memhook_write_r8d_rsp_end: u8;
    static cannoli_memhook_write_r8d_rbp: u8;
    static cannoli_memhook_write_r8d_rbp_end: u8;
    static cannoli_memhook_write_r8d_rsi: u8;
    static cannoli_memhook_write_r8d_rsi_end: u8;
    static cannoli_memhook_write_r8d_rdi: u8;
    static cannoli_memhook_write_r8d_rdi_end: u8;
    static cannoli_memhook_write_r8d_r8: u8;
    static cannoli_memhook_write_r8d_r8_end: u8;
    static cannoli_memhook_write_r8d_r9: u8;
    static cannoli_memhook_write_r8d_r9_end: u8;
    static cannoli_memhook_write_r8d_r10: u8;
    static cannoli_memhook_write_r8d_r10_end: u8;
    static cannoli_memhook_write_r8d_r11: u8;
    static cannoli_memhook_write_r8d_r11_end: u8;
    static cannoli_memhook_write_r8d_r12: u8;
    static cannoli_memhook_write_r8d_r12_end: u8;
    static cannoli_memhook_write_r8d_r13: u8;
    static cannoli_memhook_write_r8d_r13_end: u8;
    static cannoli_memhook_write_r8d_r14: u8;
    static cannoli_memhook_write_r8d_r14_end: u8;
    static cannoli_memhook_write_r8d_r15: u8;
    static cannoli_memhook_write_r8d_r15_end: u8;
    static cannoli_memhook_write_r9d_rax: u8;
    static cannoli_memhook_write_r9d_rax_end: u8;
    static cannoli_memhook_write_r9d_rcx: u8;
    static cannoli_memhook_write_r9d_rcx_end: u8;
    static cannoli_memhook_write_r9d_rdx: u8;
    static cannoli_memhook_write_r9d_rdx_end: u8;
    static cannoli_memhook_write_r9d_rbx: u8;
    static cannoli_memhook_write_r9d_rbx_end: u8;
    static cannoli_memhook_write_r9d_rsp: u8;
    static cannoli_memhook_write_r9d_rsp_end: u8;
    static cannoli_memhook_write_r9d_rbp: u8;
    static cannoli_memhook_write_r9d_rbp_end: u8;
    static cannoli_memhook_write_r9d_rsi: u8;
    static cannoli_memhook_write_r9d_rsi_end: u8;
    static cannoli_memhook_write_r9d_rdi: u8;
    static cannoli_memhook_write_r9d_rdi_end: u8;
    static cannoli_memhook_write_r9d_r8: u8;
    static cannoli_memhook_write_r9d_r8_end: u8;
    static cannoli_memhook_write_r9d_r9: u8;
    static cannoli_memhook_write_r9d_r9_end: u8;
    static cannoli_memhook_write_r9d_r10: u8;
    static cannoli_memhook_write_r9d_r10_end: u8;
    static cannoli_memhook_write_r9d_r11: u8;
    static cannoli_memhook_write_r9d_r11_end: u8;
    static cannoli_memhook_write_r9d_r12: u8;
    static cannoli_memhook_write_r9d_r12_end: u8;
    static cannoli_memhook_write_r9d_r13: u8;
    static cannoli_memhook_write_r9d_r13_end: u8;
    static cannoli_memhook_write_r9d_r14: u8;
    static cannoli_memhook_write_r9d_r14_end: u8;
    static cannoli_memhook_write_r9d_r15: u8;
    static cannoli_memhook_write_r9d_r15_end: u8;
    static cannoli_memhook_write_r10d_rax: u8;
    static cannoli_memhook_write_r10d_rax_end: u8;
    static cannoli_memhook_write_r10d_rcx: u8;
    static cannoli_memhook_write_r10d_rcx_end: u8;
    static cannoli_memhook_write_r10d_rdx: u8;
    static cannoli_memhook_write_r10d_rdx_end: u8;
    static cannoli_memhook_write_r10d_rbx: u8;
    static cannoli_memhook_write_r10d_rbx_end: u8;
    static cannoli_memhook_write_r10d_rsp: u8;
    static cannoli_memhook_write_r10d_rsp_end: u8;
    static cannoli_memhook_write_r10d_rbp: u8;
    static cannoli_memhook_write_r10d_rbp_end: u8;
    static cannoli_memhook_write_r10d_rsi: u8;
    static cannoli_memhook_write_r10d_rsi_end: u8;
    static cannoli_memhook_write_r10d_rdi: u8;
    static cannoli_memhook_write_r10d_rdi_end: u8;
    static cannoli_memhook_write_r10d_r8: u8;
    static cannoli_memhook_write_r10d_r8_end: u8;
    static cannoli_memhook_write_r10d_r9: u8;
    static cannoli_memhook_write_r10d_r9_end: u8;
    static cannoli_memhook_write_r10d_r10: u8;
    static cannoli_memhook_write_r10d_r10_end: u8;
    static cannoli_memhook_write_r10d_r11: u8;
    static cannoli_memhook_write_r10d_r11_end: u8;
    static cannoli_memhook_write_r10d_r12: u8;
    static cannoli_memhook_write_r10d_r12_end: u8;
    static cannoli_memhook_write_r10d_r13: u8;
    static cannoli_memhook_write_r10d_r13_end: u8;
    static cannoli_memhook_write_r10d_r14: u8;
    static cannoli_memhook_write_r10d_r14_end: u8;
    static cannoli_memhook_write_r10d_r15: u8;
    static cannoli_memhook_write_r10d_r15_end: u8;
    static cannoli_memhook_write_r11d_rax: u8;
    static cannoli_memhook_write_r11d_rax_end: u8;
    static cannoli_memhook_write_r11d_rcx: u8;
    static cannoli_memhook_write_r11d_rcx_end: u8;
    static cannoli_memhook_write_r11d_rdx: u8;
    static cannoli_memhook_write_r11d_rdx_end: u8;
    static cannoli_memhook_write_r11d_rbx: u8;
    static cannoli_memhook_write_r11d_rbx_end: u8;
    static cannoli_memhook_write_r11d_rsp: u8;
    static cannoli_memhook_write_r11d_rsp_end: u8;
    static cannoli_memhook_write_r11d_rbp: u8;
    static cannoli_memhook_write_r11d_rbp_end: u8;
    static cannoli_memhook_write_r11d_rsi: u8;
    static cannoli_memhook_write_r11d_rsi_end: u8;
    static cannoli_memhook_write_r11d_rdi: u8;
    static cannoli_memhook_write_r11d_rdi_end: u8;
    static cannoli_memhook_write_r11d_r8: u8;
    static cannoli_memhook_write_r11d_r8_end: u8;
    static cannoli_memhook_write_r11d_r9: u8;
    static cannoli_memhook_write_r11d_r9_end: u8;
    static cannoli_memhook_write_r11d_r10: u8;
    static cannoli_memhook_write_r11d_r10_end: u8;
    static cannoli_memhook_write_r11d_r11: u8;
    static cannoli_memhook_write_r11d_r11_end: u8;
    static cannoli_memhook_write_r11d_r12: u8;
    static cannoli_memhook_write_r11d_r12_end: u8;
    static cannoli_memhook_write_r11d_r13: u8;
    static cannoli_memhook_write_r11d_r13_end: u8;
    static cannoli_memhook_write_r11d_r14: u8;
    static cannoli_memhook_write_r11d_r14_end: u8;
    static cannoli_memhook_write_r11d_r15: u8;
    static cannoli_memhook_write_r11d_r15_end: u8;
    static cannoli_memhook_write_r12d_rax: u8;
    static cannoli_memhook_write_r12d_rax_end: u8;
    static cannoli_memhook_write_r12d_rcx: u8;
    static cannoli_memhook_write_r12d_rcx_end: u8;
    static cannoli_memhook_write_r12d_rdx: u8;
    static cannoli_memhook_write_r12d_rdx_end: u8;
    static cannoli_memhook_write_r12d_rbx: u8;
    static cannoli_memhook_write_r12d_rbx_end: u8;
    static cannoli_memhook_write_r12d_rsp: u8;
    static cannoli_memhook_write_r12d_rsp_end: u8;
    static cannoli_memhook_write_r12d_rbp: u8;
    static cannoli_memhook_write_r12d_rbp_end: u8;
    static cannoli_memhook_write_r12d_rsi: u8;
    static cannoli_memhook_write_r12d_rsi_end: u8;
    static cannoli_memhook_write_r12d_rdi: u8;
    static cannoli_memhook_write_r12d_rdi_end: u8;
    static cannoli_memhook_write_r12d_r8: u8;
    static cannoli_memhook_write_r12d_r8_end: u8;
    static cannoli_memhook_write_r12d_r9: u8;
    static cannoli_memhook_write_r12d_r9_end: u8;
    static cannoli_memhook_write_r12d_r10: u8;
    static cannoli_memhook_write_r12d_r10_end: u8;
    static cannoli_memhook_write_r12d_r11: u8;
    static cannoli_memhook_write_r12d_r11_end: u8;
    static cannoli_memhook_write_r12d_r12: u8;
    static cannoli_memhook_write_r12d_r12_end: u8;
    static cannoli_memhook_write_r12d_r13: u8;
    static cannoli_memhook_write_r12d_r13_end: u8;
    static cannoli_memhook_write_r12d_r14: u8;
    static cannoli_memhook_write_r12d_r14_end: u8;
    static cannoli_memhook_write_r12d_r15: u8;
    static cannoli_memhook_write_r12d_r15_end: u8;
    static cannoli_memhook_write_r13d_rax: u8;
    static cannoli_memhook_write_r13d_rax_end: u8;
    static cannoli_memhook_write_r13d_rcx: u8;
    static cannoli_memhook_write_r13d_rcx_end: u8;
    static cannoli_memhook_write_r13d_rdx: u8;
    static cannoli_memhook_write_r13d_rdx_end: u8;
    static cannoli_memhook_write_r13d_rbx: u8;
    static cannoli_memhook_write_r13d_rbx_end: u8;
    static cannoli_memhook_write_r13d_rsp: u8;
    static cannoli_memhook_write_r13d_rsp_end: u8;
    static cannoli_memhook_write_r13d_rbp: u8;
    static cannoli_memhook_write_r13d_rbp_end: u8;
    static cannoli_memhook_write_r13d_rsi: u8;
    static cannoli_memhook_write_r13d_rsi_end: u8;
    static cannoli_memhook_write_r13d_rdi: u8;
    static cannoli_memhook_write_r13d_rdi_end: u8;
    static cannoli_memhook_write_r13d_r8: u8;
    static cannoli_memhook_write_r13d_r8_end: u8;
    static cannoli_memhook_write_r13d_r9: u8;
    static cannoli_memhook_write_r13d_r9_end: u8;
    static cannoli_memhook_write_r13d_r10: u8;
    static cannoli_memhook_write_r13d_r10_end: u8;
    static cannoli_memhook_write_r13d_r11: u8;
    static cannoli_memhook_write_r13d_r11_end: u8;
    static cannoli_memhook_write_r13d_r12: u8;
    static cannoli_memhook_write_r13d_r12_end: u8;
    static cannoli_memhook_write_r13d_r13: u8;
    static cannoli_memhook_write_r13d_r13_end: u8;
    static cannoli_memhook_write_r13d_r14: u8;
    static cannoli_memhook_write_r13d_r14_end: u8;
    static cannoli_memhook_write_r13d_r15: u8;
    static cannoli_memhook_write_r13d_r15_end: u8;
    static cannoli_memhook_write_r14d_rax: u8;
    static cannoli_memhook_write_r14d_rax_end: u8;
    static cannoli_memhook_write_r14d_rcx: u8;
    static cannoli_memhook_write_r14d_rcx_end: u8;
    static cannoli_memhook_write_r14d_rdx: u8;
    static cannoli_memhook_write_r14d_rdx_end: u8;
    static cannoli_memhook_write_r14d_rbx: u8;
    static cannoli_memhook_write_r14d_rbx_end: u8;
    static cannoli_memhook_write_r14d_rsp: u8;
    static cannoli_memhook_write_r14d_rsp_end: u8;
    static cannoli_memhook_write_r14d_rbp: u8;
    static cannoli_memhook_write_r14d_rbp_end: u8;
    static cannoli_memhook_write_r14d_rsi: u8;
    static cannoli_memhook_write_r14d_rsi_end: u8;
    static cannoli_memhook_write_r14d_rdi: u8;
    static cannoli_memhook_write_r14d_rdi_end: u8;
    static cannoli_memhook_write_r14d_r8: u8;
    static cannoli_memhook_write_r14d_r8_end: u8;
    static cannoli_memhook_write_r14d_r9: u8;
    static cannoli_memhook_write_r14d_r9_end: u8;
    static cannoli_memhook_write_r14d_r10: u8;
    static cannoli_memhook_write_r14d_r10_end: u8;
    static cannoli_memhook_write_r14d_r11: u8;
    static cannoli_memhook_write_r14d_r11_end: u8;
    static cannoli_memhook_write_r14d_r12: u8;
    static cannoli_memhook_write_r14d_r12_end: u8;
    static cannoli_memhook_write_r14d_r13: u8;
    static cannoli_memhook_write_r14d_r13_end: u8;
    static cannoli_memhook_write_r14d_r14: u8;
    static cannoli_memhook_write_r14d_r14_end: u8;
    static cannoli_memhook_write_r14d_r15: u8;
    static cannoli_memhook_write_r14d_r15_end: u8;
    static cannoli_memhook_write_r15d_rax: u8;
    static cannoli_memhook_write_r15d_rax_end: u8;
    static cannoli_memhook_write_r15d_rcx: u8;
    static cannoli_memhook_write_r15d_rcx_end: u8;
    static cannoli_memhook_write_r15d_rdx: u8;
    static cannoli_memhook_write_r15d_rdx_end: u8;
    static cannoli_memhook_write_r15d_rbx: u8;
    static cannoli_memhook_write_r15d_rbx_end: u8;
    static cannoli_memhook_write_r15d_rsp: u8;
    static cannoli_memhook_write_r15d_rsp_end: u8;
    static cannoli_memhook_write_r15d_rbp: u8;
    static cannoli_memhook_write_r15d_rbp_end: u8;
    static cannoli_memhook_write_r15d_rsi: u8;
    static cannoli_memhook_write_r15d_rsi_end: u8;
    static cannoli_memhook_write_r15d_rdi: u8;
    static cannoli_memhook_write_r15d_rdi_end: u8;
    static cannoli_memhook_write_r15d_r8: u8;
    static cannoli_memhook_write_r15d_r8_end: u8;
    static cannoli_memhook_write_r15d_r9: u8;
    static cannoli_memhook_write_r15d_r9_end: u8;
    static cannoli_memhook_write_r15d_r10: u8;
    static cannoli_memhook_write_r15d_r10_end: u8;
    static cannoli_memhook_write_r15d_r11: u8;
    static cannoli_memhook_write_r15d_r11_end: u8;
    static cannoli_memhook_write_r15d_r12: u8;
    static cannoli_memhook_write_r15d_r12_end: u8;
    static cannoli_memhook_write_r15d_r13: u8;
    static cannoli_memhook_write_r15d_r13_end: u8;
    static cannoli_memhook_write_r15d_r14: u8;
    static cannoli_memhook_write_r15d_r14_end: u8;
    static cannoli_memhook_write_r15d_r15: u8;
    static cannoli_memhook_write_r15d_r15_end: u8;
    static cannoli_memhook_write_rax_eax: u8;
    static cannoli_memhook_write_rax_eax_end: u8;
    static cannoli_memhook_write_rax_ecx: u8;
    static cannoli_memhook_write_rax_ecx_end: u8;
    static cannoli_memhook_write_rax_edx: u8;
    static cannoli_memhook_write_rax_edx_end: u8;
    static cannoli_memhook_write_rax_ebx: u8;
    static cannoli_memhook_write_rax_ebx_end: u8;
    static cannoli_memhook_write_rax_esp: u8;
    static cannoli_memhook_write_rax_esp_end: u8;
    static cannoli_memhook_write_rax_ebp: u8;
    static cannoli_memhook_write_rax_ebp_end: u8;
    static cannoli_memhook_write_rax_esi: u8;
    static cannoli_memhook_write_rax_esi_end: u8;
    static cannoli_memhook_write_rax_edi: u8;
    static cannoli_memhook_write_rax_edi_end: u8;
    static cannoli_memhook_write_rax_r8d: u8;
    static cannoli_memhook_write_rax_r8d_end: u8;
    static cannoli_memhook_write_rax_r9d: u8;
    static cannoli_memhook_write_rax_r9d_end: u8;
    static cannoli_memhook_write_rax_r10d: u8;
    static cannoli_memhook_write_rax_r10d_end: u8;
    static cannoli_memhook_write_rax_r11d: u8;
    static cannoli_memhook_write_rax_r11d_end: u8;
    static cannoli_memhook_write_rax_r12d: u8;
    static cannoli_memhook_write_rax_r12d_end: u8;
    static cannoli_memhook_write_rax_r13d: u8;
    static cannoli_memhook_write_rax_r13d_end: u8;
    static cannoli_memhook_write_rax_r14d: u8;
    static cannoli_memhook_write_rax_r14d_end: u8;
    static cannoli_memhook_write_rax_r15d: u8;
    static cannoli_memhook_write_rax_r15d_end: u8;
    static cannoli_memhook_write_rcx_eax: u8;
    static cannoli_memhook_write_rcx_eax_end: u8;
    static cannoli_memhook_write_rcx_ecx: u8;
    static cannoli_memhook_write_rcx_ecx_end: u8;
    static cannoli_memhook_write_rcx_edx: u8;
    static cannoli_memhook_write_rcx_edx_end: u8;
    static cannoli_memhook_write_rcx_ebx: u8;
    static cannoli_memhook_write_rcx_ebx_end: u8;
    static cannoli_memhook_write_rcx_esp: u8;
    static cannoli_memhook_write_rcx_esp_end: u8;
    static cannoli_memhook_write_rcx_ebp: u8;
    static cannoli_memhook_write_rcx_ebp_end: u8;
    static cannoli_memhook_write_rcx_esi: u8;
    static cannoli_memhook_write_rcx_esi_end: u8;
    static cannoli_memhook_write_rcx_edi: u8;
    static cannoli_memhook_write_rcx_edi_end: u8;
    static cannoli_memhook_write_rcx_r8d: u8;
    static cannoli_memhook_write_rcx_r8d_end: u8;
    static cannoli_memhook_write_rcx_r9d: u8;
    static cannoli_memhook_write_rcx_r9d_end: u8;
    static cannoli_memhook_write_rcx_r10d: u8;
    static cannoli_memhook_write_rcx_r10d_end: u8;
    static cannoli_memhook_write_rcx_r11d: u8;
    static cannoli_memhook_write_rcx_r11d_end: u8;
    static cannoli_memhook_write_rcx_r12d: u8;
    static cannoli_memhook_write_rcx_r12d_end: u8;
    static cannoli_memhook_write_rcx_r13d: u8;
    static cannoli_memhook_write_rcx_r13d_end: u8;
    static cannoli_memhook_write_rcx_r14d: u8;
    static cannoli_memhook_write_rcx_r14d_end: u8;
    static cannoli_memhook_write_rcx_r15d: u8;
    static cannoli_memhook_write_rcx_r15d_end: u8;
    static cannoli_memhook_write_rdx_eax: u8;
    static cannoli_memhook_write_rdx_eax_end: u8;
    static cannoli_memhook_write_rdx_ecx: u8;
    static cannoli_memhook_write_rdx_ecx_end: u8;
    static cannoli_memhook_write_rdx_edx: u8;
    static cannoli_memhook_write_rdx_edx_end: u8;
    static cannoli_memhook_write_rdx_ebx: u8;
    static cannoli_memhook_write_rdx_ebx_end: u8;
    static cannoli_memhook_write_rdx_esp: u8;
    static cannoli_memhook_write_rdx_esp_end: u8;
    static cannoli_memhook_write_rdx_ebp: u8;
    static cannoli_memhook_write_rdx_ebp_end: u8;
    static cannoli_memhook_write_rdx_esi: u8;
    static cannoli_memhook_write_rdx_esi_end: u8;
    static cannoli_memhook_write_rdx_edi: u8;
    static cannoli_memhook_write_rdx_edi_end: u8;
    static cannoli_memhook_write_rdx_r8d: u8;
    static cannoli_memhook_write_rdx_r8d_end: u8;
    static cannoli_memhook_write_rdx_r9d: u8;
    static cannoli_memhook_write_rdx_r9d_end: u8;
    static cannoli_memhook_write_rdx_r10d: u8;
    static cannoli_memhook_write_rdx_r10d_end: u8;
    static cannoli_memhook_write_rdx_r11d: u8;
    static cannoli_memhook_write_rdx_r11d_end: u8;
    static cannoli_memhook_write_rdx_r12d: u8;
    static cannoli_memhook_write_rdx_r12d_end: u8;
    static cannoli_memhook_write_rdx_r13d: u8;
    static cannoli_memhook_write_rdx_r13d_end: u8;
    static cannoli_memhook_write_rdx_r14d: u8;
    static cannoli_memhook_write_rdx_r14d_end: u8;
    static cannoli_memhook_write_rdx_r15d: u8;
    static cannoli_memhook_write_rdx_r15d_end: u8;
    static cannoli_memhook_write_rbx_eax: u8;
    static cannoli_memhook_write_rbx_eax_end: u8;
    static cannoli_memhook_write_rbx_ecx: u8;
    static cannoli_memhook_write_rbx_ecx_end: u8;
    static cannoli_memhook_write_rbx_edx: u8;
    static cannoli_memhook_write_rbx_edx_end: u8;
    static cannoli_memhook_write_rbx_ebx: u8;
    static cannoli_memhook_write_rbx_ebx_end: u8;
    static cannoli_memhook_write_rbx_esp: u8;
    static cannoli_memhook_write_rbx_esp_end: u8;
    static cannoli_memhook_write_rbx_ebp: u8;
    static cannoli_memhook_write_rbx_ebp_end: u8;
    static cannoli_memhook_write_rbx_esi: u8;
    static cannoli_memhook_write_rbx_esi_end: u8;
    static cannoli_memhook_write_rbx_edi: u8;
    static cannoli_memhook_write_rbx_edi_end: u8;
    static cannoli_memhook_write_rbx_r8d: u8;
    static cannoli_memhook_write_rbx_r8d_end: u8;
    static cannoli_memhook_write_rbx_r9d: u8;
    static cannoli_memhook_write_rbx_r9d_end: u8;
    static cannoli_memhook_write_rbx_r10d: u8;
    static cannoli_memhook_write_rbx_r10d_end: u8;
    static cannoli_memhook_write_rbx_r11d: u8;
    static cannoli_memhook_write_rbx_r11d_end: u8;
    static cannoli_memhook_write_rbx_r12d: u8;
    static cannoli_memhook_write_rbx_r12d_end: u8;
    static cannoli_memhook_write_rbx_r13d: u8;
    static cannoli_memhook_write_rbx_r13d_end: u8;
    static cannoli_memhook_write_rbx_r14d: u8;
    static cannoli_memhook_write_rbx_r14d_end: u8;
    static cannoli_memhook_write_rbx_r15d: u8;
    static cannoli_memhook_write_rbx_r15d_end: u8;
    static cannoli_memhook_write_rsp_eax: u8;
    static cannoli_memhook_write_rsp_eax_end: u8;
    static cannoli_memhook_write_rsp_ecx: u8;
    static cannoli_memhook_write_rsp_ecx_end: u8;
    static cannoli_memhook_write_rsp_edx: u8;
    static cannoli_memhook_write_rsp_edx_end: u8;
    static cannoli_memhook_write_rsp_ebx: u8;
    static cannoli_memhook_write_rsp_ebx_end: u8;
    static cannoli_memhook_write_rsp_esp: u8;
    static cannoli_memhook_write_rsp_esp_end: u8;
    static cannoli_memhook_write_rsp_ebp: u8;
    static cannoli_memhook_write_rsp_ebp_end: u8;
    static cannoli_memhook_write_rsp_esi: u8;
    static cannoli_memhook_write_rsp_esi_end: u8;
    static cannoli_memhook_write_rsp_edi: u8;
    static cannoli_memhook_write_rsp_edi_end: u8;
    static cannoli_memhook_write_rsp_r8d: u8;
    static cannoli_memhook_write_rsp_r8d_end: u8;
    static cannoli_memhook_write_rsp_r9d: u8;
    static cannoli_memhook_write_rsp_r9d_end: u8;
    static cannoli_memhook_write_rsp_r10d: u8;
    static cannoli_memhook_write_rsp_r10d_end: u8;
    static cannoli_memhook_write_rsp_r11d: u8;
    static cannoli_memhook_write_rsp_r11d_end: u8;
    static cannoli_memhook_write_rsp_r12d: u8;
    static cannoli_memhook_write_rsp_r12d_end: u8;
    static cannoli_memhook_write_rsp_r13d: u8;
    static cannoli_memhook_write_rsp_r13d_end: u8;
    static cannoli_memhook_write_rsp_r14d: u8;
    static cannoli_memhook_write_rsp_r14d_end: u8;
    static cannoli_memhook_write_rsp_r15d: u8;
    static cannoli_memhook_write_rsp_r15d_end: u8;
    static cannoli_memhook_write_rbp_eax: u8;
    static cannoli_memhook_write_rbp_eax_end: u8;
    static cannoli_memhook_write_rbp_ecx: u8;
    static cannoli_memhook_write_rbp_ecx_end: u8;
    static cannoli_memhook_write_rbp_edx: u8;
    static cannoli_memhook_write_rbp_edx_end: u8;
    static cannoli_memhook_write_rbp_ebx: u8;
    static cannoli_memhook_write_rbp_ebx_end: u8;
    static cannoli_memhook_write_rbp_esp: u8;
    static cannoli_memhook_write_rbp_esp_end: u8;
    static cannoli_memhook_write_rbp_ebp: u8;
    static cannoli_memhook_write_rbp_ebp_end: u8;
    static cannoli_memhook_write_rbp_esi: u8;
    static cannoli_memhook_write_rbp_esi_end: u8;
    static cannoli_memhook_write_rbp_edi: u8;
    static cannoli_memhook_write_rbp_edi_end: u8;
    static cannoli_memhook_write_rbp_r8d: u8;
    static cannoli_memhook_write_rbp_r8d_end: u8;
    static cannoli_memhook_write_rbp_r9d: u8;
    static cannoli_memhook_write_rbp_r9d_end: u8;
    static cannoli_memhook_write_rbp_r10d: u8;
    static cannoli_memhook_write_rbp_r10d_end: u8;
    static cannoli_memhook_write_rbp_r11d: u8;
    static cannoli_memhook_write_rbp_r11d_end: u8;
    static cannoli_memhook_write_rbp_r12d: u8;
    static cannoli_memhook_write_rbp_r12d_end: u8;
    static cannoli_memhook_write_rbp_r13d: u8;
    static cannoli_memhook_write_rbp_r13d_end: u8;
    static cannoli_memhook_write_rbp_r14d: u8;
    static cannoli_memhook_write_rbp_r14d_end: u8;
    static cannoli_memhook_write_rbp_r15d: u8;
    static cannoli_memhook_write_rbp_r15d_end: u8;
    static cannoli_memhook_write_rsi_eax: u8;
    static cannoli_memhook_write_rsi_eax_end: u8;
    static cannoli_memhook_write_rsi_ecx: u8;
    static cannoli_memhook_write_rsi_ecx_end: u8;
    static cannoli_memhook_write_rsi_edx: u8;
    static cannoli_memhook_write_rsi_edx_end: u8;
    static cannoli_memhook_write_rsi_ebx: u8;
    static cannoli_memhook_write_rsi_ebx_end: u8;
    static cannoli_memhook_write_rsi_esp: u8;
    static cannoli_memhook_write_rsi_esp_end: u8;
    static cannoli_memhook_write_rsi_ebp: u8;
    static cannoli_memhook_write_rsi_ebp_end: u8;
    static cannoli_memhook_write_rsi_esi: u8;
    static cannoli_memhook_write_rsi_esi_end: u8;
    static cannoli_memhook_write_rsi_edi: u8;
    static cannoli_memhook_write_rsi_edi_end: u8;
    static cannoli_memhook_write_rsi_r8d: u8;
    static cannoli_memhook_write_rsi_r8d_end: u8;
    static cannoli_memhook_write_rsi_r9d: u8;
    static cannoli_memhook_write_rsi_r9d_end: u8;
    static cannoli_memhook_write_rsi_r10d: u8;
    static cannoli_memhook_write_rsi_r10d_end: u8;
    static cannoli_memhook_write_rsi_r11d: u8;
    static cannoli_memhook_write_rsi_r11d_end: u8;
    static cannoli_memhook_write_rsi_r12d: u8;
    static cannoli_memhook_write_rsi_r12d_end: u8;
    static cannoli_memhook_write_rsi_r13d: u8;
    static cannoli_memhook_write_rsi_r13d_end: u8;
    static cannoli_memhook_write_rsi_r14d: u8;
    static cannoli_memhook_write_rsi_r14d_end: u8;
    static cannoli_memhook_write_rsi_r15d: u8;
    static cannoli_memhook_write_rsi_r15d_end: u8;
    static cannoli_memhook_write_rdi_eax: u8;
    static cannoli_memhook_write_rdi_eax_end: u8;
    static cannoli_memhook_write_rdi_ecx: u8;
    static cannoli_memhook_write_rdi_ecx_end: u8;
    static cannoli_memhook_write_rdi_edx: u8;
    static cannoli_memhook_write_rdi_edx_end: u8;
    static cannoli_memhook_write_rdi_ebx: u8;
    static cannoli_memhook_write_rdi_ebx_end: u8;
    static cannoli_memhook_write_rdi_esp: u8;
    static cannoli_memhook_write_rdi_esp_end: u8;
    static cannoli_memhook_write_rdi_ebp: u8;
    static cannoli_memhook_write_rdi_ebp_end: u8;
    static cannoli_memhook_write_rdi_esi: u8;
    static cannoli_memhook_write_rdi_esi_end: u8;
    static cannoli_memhook_write_rdi_edi: u8;
    static cannoli_memhook_write_rdi_edi_end: u8;
    static cannoli_memhook_write_rdi_r8d: u8;
    static cannoli_memhook_write_rdi_r8d_end: u8;
    static cannoli_memhook_write_rdi_r9d: u8;
    static cannoli_memhook_write_rdi_r9d_end: u8;
    static cannoli_memhook_write_rdi_r10d: u8;
    static cannoli_memhook_write_rdi_r10d_end: u8;
    static cannoli_memhook_write_rdi_r11d: u8;
    static cannoli_memhook_write_rdi_r11d_end: u8;
    static cannoli_memhook_write_rdi_r12d: u8;
    static cannoli_memhook_write_rdi_r12d_end: u8;
    static cannoli_memhook_write_rdi_r13d: u8;
    static cannoli_memhook_write_rdi_r13d_end: u8;
    static cannoli_memhook_write_rdi_r14d: u8;
    static cannoli_memhook_write_rdi_r14d_end: u8;
    static cannoli_memhook_write_rdi_r15d: u8;
    static cannoli_memhook_write_rdi_r15d_end: u8;
    static cannoli_memhook_write_r8_eax: u8;
    static cannoli_memhook_write_r8_eax_end: u8;
    static cannoli_memhook_write_r8_ecx: u8;
    static cannoli_memhook_write_r8_ecx_end: u8;
    static cannoli_memhook_write_r8_edx: u8;
    static cannoli_memhook_write_r8_edx_end: u8;
    static cannoli_memhook_write_r8_ebx: u8;
    static cannoli_memhook_write_r8_ebx_end: u8;
    static cannoli_memhook_write_r8_esp: u8;
    static cannoli_memhook_write_r8_esp_end: u8;
    static cannoli_memhook_write_r8_ebp: u8;
    static cannoli_memhook_write_r8_ebp_end: u8;
    static cannoli_memhook_write_r8_esi: u8;
    static cannoli_memhook_write_r8_esi_end: u8;
    static cannoli_memhook_write_r8_edi: u8;
    static cannoli_memhook_write_r8_edi_end: u8;
    static cannoli_memhook_write_r8_r8d: u8;
    static cannoli_memhook_write_r8_r8d_end: u8;
    static cannoli_memhook_write_r8_r9d: u8;
    static cannoli_memhook_write_r8_r9d_end: u8;
    static cannoli_memhook_write_r8_r10d: u8;
    static cannoli_memhook_write_r8_r10d_end: u8;
    static cannoli_memhook_write_r8_r11d: u8;
    static cannoli_memhook_write_r8_r11d_end: u8;
    static cannoli_memhook_write_r8_r12d: u8;
    static cannoli_memhook_write_r8_r12d_end: u8;
    static cannoli_memhook_write_r8_r13d: u8;
    static cannoli_memhook_write_r8_r13d_end: u8;
    static cannoli_memhook_write_r8_r14d: u8;
    static cannoli_memhook_write_r8_r14d_end: u8;
    static cannoli_memhook_write_r8_r15d: u8;
    static cannoli_memhook_write_r8_r15d_end: u8;
    static cannoli_memhook_write_r9_eax: u8;
    static cannoli_memhook_write_r9_eax_end: u8;
    static cannoli_memhook_write_r9_ecx: u8;
    static cannoli_memhook_write_r9_ecx_end: u8;
    static cannoli_memhook_write_r9_edx: u8;
    static cannoli_memhook_write_r9_edx_end: u8;
    static cannoli_memhook_write_r9_ebx: u8;
    static cannoli_memhook_write_r9_ebx_end: u8;
    static cannoli_memhook_write_r9_esp: u8;
    static cannoli_memhook_write_r9_esp_end: u8;
    static cannoli_memhook_write_r9_ebp: u8;
    static cannoli_memhook_write_r9_ebp_end: u8;
    static cannoli_memhook_write_r9_esi: u8;
    static cannoli_memhook_write_r9_esi_end: u8;
    static cannoli_memhook_write_r9_edi: u8;
    static cannoli_memhook_write_r9_edi_end: u8;
    static cannoli_memhook_write_r9_r8d: u8;
    static cannoli_memhook_write_r9_r8d_end: u8;
    static cannoli_memhook_write_r9_r9d: u8;
    static cannoli_memhook_write_r9_r9d_end: u8;
    static cannoli_memhook_write_r9_r10d: u8;
    static cannoli_memhook_write_r9_r10d_end: u8;
    static cannoli_memhook_write_r9_r11d: u8;
    static cannoli_memhook_write_r9_r11d_end: u8;
    static cannoli_memhook_write_r9_r12d: u8;
    static cannoli_memhook_write_r9_r12d_end: u8;
    static cannoli_memhook_write_r9_r13d: u8;
    static cannoli_memhook_write_r9_r13d_end: u8;
    static cannoli_memhook_write_r9_r14d: u8;
    static cannoli_memhook_write_r9_r14d_end: u8;
    static cannoli_memhook_write_r9_r15d: u8;
    static cannoli_memhook_write_r9_r15d_end: u8;
    static cannoli_memhook_write_r10_eax: u8;
    static cannoli_memhook_write_r10_eax_end: u8;
    static cannoli_memhook_write_r10_ecx: u8;
    static cannoli_memhook_write_r10_ecx_end: u8;
    static cannoli_memhook_write_r10_edx: u8;
    static cannoli_memhook_write_r10_edx_end: u8;
    static cannoli_memhook_write_r10_ebx: u8;
    static cannoli_memhook_write_r10_ebx_end: u8;
    static cannoli_memhook_write_r10_esp: u8;
    static cannoli_memhook_write_r10_esp_end: u8;
    static cannoli_memhook_write_r10_ebp: u8;
    static cannoli_memhook_write_r10_ebp_end: u8;
    static cannoli_memhook_write_r10_esi: u8;
    static cannoli_memhook_write_r10_esi_end: u8;
    static cannoli_memhook_write_r10_edi: u8;
    static cannoli_memhook_write_r10_edi_end: u8;
    static cannoli_memhook_write_r10_r8d: u8;
    static cannoli_memhook_write_r10_r8d_end: u8;
    static cannoli_memhook_write_r10_r9d: u8;
    static cannoli_memhook_write_r10_r9d_end: u8;
    static cannoli_memhook_write_r10_r10d: u8;
    static cannoli_memhook_write_r10_r10d_end: u8;
    static cannoli_memhook_write_r10_r11d: u8;
    static cannoli_memhook_write_r10_r11d_end: u8;
    static cannoli_memhook_write_r10_r12d: u8;
    static cannoli_memhook_write_r10_r12d_end: u8;
    static cannoli_memhook_write_r10_r13d: u8;
    static cannoli_memhook_write_r10_r13d_end: u8;
    static cannoli_memhook_write_r10_r14d: u8;
    static cannoli_memhook_write_r10_r14d_end: u8;
    static cannoli_memhook_write_r10_r15d: u8;
    static cannoli_memhook_write_r10_r15d_end: u8;
    static cannoli_memhook_write_r11_eax: u8;
    static cannoli_memhook_write_r11_eax_end: u8;
    static cannoli_memhook_write_r11_ecx: u8;
    static cannoli_memhook_write_r11_ecx_end: u8;
    static cannoli_memhook_write_r11_edx: u8;
    static cannoli_memhook_write_r11_edx_end: u8;
    static cannoli_memhook_write_r11_ebx: u8;
    static cannoli_memhook_write_r11_ebx_end: u8;
    static cannoli_memhook_write_r11_esp: u8;
    static cannoli_memhook_write_r11_esp_end: u8;
    static cannoli_memhook_write_r11_ebp: u8;
    static cannoli_memhook_write_r11_ebp_end: u8;
    static cannoli_memhook_write_r11_esi: u8;
    static cannoli_memhook_write_r11_esi_end: u8;
    static cannoli_memhook_write_r11_edi: u8;
    static cannoli_memhook_write_r11_edi_end: u8;
    static cannoli_memhook_write_r11_r8d: u8;
    static cannoli_memhook_write_r11_r8d_end: u8;
    static cannoli_memhook_write_r11_r9d: u8;
    static cannoli_memhook_write_r11_r9d_end: u8;
    static cannoli_memhook_write_r11_r10d: u8;
    static cannoli_memhook_write_r11_r10d_end: u8;
    static cannoli_memhook_write_r11_r11d: u8;
    static cannoli_memhook_write_r11_r11d_end: u8;
    static cannoli_memhook_write_r11_r12d: u8;
    static cannoli_memhook_write_r11_r12d_end: u8;
    static cannoli_memhook_write_r11_r13d: u8;
    static cannoli_memhook_write_r11_r13d_end: u8;
    static cannoli_memhook_write_r11_r14d: u8;
    static cannoli_memhook_write_r11_r14d_end: u8;
    static cannoli_memhook_write_r11_r15d: u8;
    static cannoli_memhook_write_r11_r15d_end: u8;
    static cannoli_memhook_write_r12_eax: u8;
    static cannoli_memhook_write_r12_eax_end: u8;
    static cannoli_memhook_write_r12_ecx: u8;
    static cannoli_memhook_write_r12_ecx_end: u8;
    static cannoli_memhook_write_r12_edx: u8;
    static cannoli_memhook_write_r12_edx_end: u8;
    static cannoli_memhook_write_r12_ebx: u8;
    static cannoli_memhook_write_r12_ebx_end: u8;
    static cannoli_memhook_write_r12_esp: u8;
    static cannoli_memhook_write_r12_esp_end: u8;
    static cannoli_memhook_write_r12_ebp: u8;
    static cannoli_memhook_write_r12_ebp_end: u8;
    static cannoli_memhook_write_r12_esi: u8;
    static cannoli_memhook_write_r12_esi_end: u8;
    static cannoli_memhook_write_r12_edi: u8;
    static cannoli_memhook_write_r12_edi_end: u8;
    static cannoli_memhook_write_r12_r8d: u8;
    static cannoli_memhook_write_r12_r8d_end: u8;
    static cannoli_memhook_write_r12_r9d: u8;
    static cannoli_memhook_write_r12_r9d_end: u8;
    static cannoli_memhook_write_r12_r10d: u8;
    static cannoli_memhook_write_r12_r10d_end: u8;
    static cannoli_memhook_write_r12_r11d: u8;
    static cannoli_memhook_write_r12_r11d_end: u8;
    static cannoli_memhook_write_r12_r12d: u8;
    static cannoli_memhook_write_r12_r12d_end: u8;
    static cannoli_memhook_write_r12_r13d: u8;
    static cannoli_memhook_write_r12_r13d_end: u8;
    static cannoli_memhook_write_r12_r14d: u8;
    static cannoli_memhook_write_r12_r14d_end: u8;
    static cannoli_memhook_write_r12_r15d: u8;
    static cannoli_memhook_write_r12_r15d_end: u8;
    static cannoli_memhook_write_r13_eax: u8;
    static cannoli_memhook_write_r13_eax_end: u8;
    static cannoli_memhook_write_r13_ecx: u8;
    static cannoli_memhook_write_r13_ecx_end: u8;
    static cannoli_memhook_write_r13_edx: u8;
    static cannoli_memhook_write_r13_edx_end: u8;
    static cannoli_memhook_write_r13_ebx: u8;
    static cannoli_memhook_write_r13_ebx_end: u8;
    static cannoli_memhook_write_r13_esp: u8;
    static cannoli_memhook_write_r13_esp_end: u8;
    static cannoli_memhook_write_r13_ebp: u8;
    static cannoli_memhook_write_r13_ebp_end: u8;
    static cannoli_memhook_write_r13_esi: u8;
    static cannoli_memhook_write_r13_esi_end: u8;
    static cannoli_memhook_write_r13_edi: u8;
    static cannoli_memhook_write_r13_edi_end: u8;
    static cannoli_memhook_write_r13_r8d: u8;
    static cannoli_memhook_write_r13_r8d_end: u8;
    static cannoli_memhook_write_r13_r9d: u8;
    static cannoli_memhook_write_r13_r9d_end: u8;
    static cannoli_memhook_write_r13_r10d: u8;
    static cannoli_memhook_write_r13_r10d_end: u8;
    static cannoli_memhook_write_r13_r11d: u8;
    static cannoli_memhook_write_r13_r11d_end: u8;
    static cannoli_memhook_write_r13_r12d: u8;
    static cannoli_memhook_write_r13_r12d_end: u8;
    static cannoli_memhook_write_r13_r13d: u8;
    static cannoli_memhook_write_r13_r13d_end: u8;
    static cannoli_memhook_write_r13_r14d: u8;
    static cannoli_memhook_write_r13_r14d_end: u8;
    static cannoli_memhook_write_r13_r15d: u8;
    static cannoli_memhook_write_r13_r15d_end: u8;
    static cannoli_memhook_write_r14_eax: u8;
    static cannoli_memhook_write_r14_eax_end: u8;
    static cannoli_memhook_write_r14_ecx: u8;
    static cannoli_memhook_write_r14_ecx_end: u8;
    static cannoli_memhook_write_r14_edx: u8;
    static cannoli_memhook_write_r14_edx_end: u8;
    static cannoli_memhook_write_r14_ebx: u8;
    static cannoli_memhook_write_r14_ebx_end: u8;
    static cannoli_memhook_write_r14_esp: u8;
    static cannoli_memhook_write_r14_esp_end: u8;
    static cannoli_memhook_write_r14_ebp: u8;
    static cannoli_memhook_write_r14_ebp_end: u8;
    static cannoli_memhook_write_r14_esi: u8;
    static cannoli_memhook_write_r14_esi_end: u8;
    static cannoli_memhook_write_r14_edi: u8;
    static cannoli_memhook_write_r14_edi_end: u8;
    static cannoli_memhook_write_r14_r8d: u8;
    static cannoli_memhook_write_r14_r8d_end: u8;
    static cannoli_memhook_write_r14_r9d: u8;
    static cannoli_memhook_write_r14_r9d_end: u8;
    static cannoli_memhook_write_r14_r10d: u8;
    static cannoli_memhook_write_r14_r10d_end: u8;
    static cannoli_memhook_write_r14_r11d: u8;
    static cannoli_memhook_write_r14_r11d_end: u8;
    static cannoli_memhook_write_r14_r12d: u8;
    static cannoli_memhook_write_r14_r12d_end: u8;
    static cannoli_memhook_write_r14_r13d: u8;
    static cannoli_memhook_write_r14_r13d_end: u8;
    static cannoli_memhook_write_r14_r14d: u8;
    static cannoli_memhook_write_r14_r14d_end: u8;
    static cannoli_memhook_write_r14_r15d: u8;
    static cannoli_memhook_write_r14_r15d_end: u8;
    static cannoli_memhook_write_r15_eax: u8;
    static cannoli_memhook_write_r15_eax_end: u8;
    static cannoli_memhook_write_r15_ecx: u8;
    static cannoli_memhook_write_r15_ecx_end: u8;
    static cannoli_memhook_write_r15_edx: u8;
    static cannoli_memhook_write_r15_edx_end: u8;
    static cannoli_memhook_write_r15_ebx: u8;
    static cannoli_memhook_write_r15_ebx_end: u8;
    static cannoli_memhook_write_r15_esp: u8;
    static cannoli_memhook_write_r15_esp_end: u8;
    static cannoli_memhook_write_r15_ebp: u8;
    static cannoli_memhook_write_r15_ebp_end: u8;
    static cannoli_memhook_write_r15_esi: u8;
    static cannoli_memhook_write_r15_esi_end: u8;
    static cannoli_memhook_write_r15_edi: u8;
    static cannoli_memhook_write_r15_edi_end: u8;
    static cannoli_memhook_write_r15_r8d: u8;
    static cannoli_memhook_write_r15_r8d_end: u8;
    static cannoli_memhook_write_r15_r9d: u8;
    static cannoli_memhook_write_r15_r9d_end: u8;
    static cannoli_memhook_write_r15_r10d: u8;
    static cannoli_memhook_write_r15_r10d_end: u8;
    static cannoli_memhook_write_r15_r11d: u8;
    static cannoli_memhook_write_r15_r11d_end: u8;
    static cannoli_memhook_write_r15_r12d: u8;
    static cannoli_memhook_write_r15_r12d_end: u8;
    static cannoli_memhook_write_r15_r13d: u8;
    static cannoli_memhook_write_r15_r13d_end: u8;
    static cannoli_memhook_write_r15_r14d: u8;
    static cannoli_memhook_write_r15_r14d_end: u8;
    static cannoli_memhook_write_r15_r15d: u8;
    static cannoli_memhook_write_r15_r15d_end: u8;
    static cannoli_memhook_write_rax_rax: u8;
    static cannoli_memhook_write_rax_rax_end: u8;
    static cannoli_memhook_write_rax_rcx: u8;
    static cannoli_memhook_write_rax_rcx_end: u8;
    static cannoli_memhook_write_rax_rdx: u8;
    static cannoli_memhook_write_rax_rdx_end: u8;
    static cannoli_memhook_write_rax_rbx: u8;
    static cannoli_memhook_write_rax_rbx_end: u8;
    static cannoli_memhook_write_rax_rsp: u8;
    static cannoli_memhook_write_rax_rsp_end: u8;
    static cannoli_memhook_write_rax_rbp: u8;
    static cannoli_memhook_write_rax_rbp_end: u8;
    static cannoli_memhook_write_rax_rsi: u8;
    static cannoli_memhook_write_rax_rsi_end: u8;
    static cannoli_memhook_write_rax_rdi: u8;
    static cannoli_memhook_write_rax_rdi_end: u8;
    static cannoli_memhook_write_rax_r8: u8;
    static cannoli_memhook_write_rax_r8_end: u8;
    static cannoli_memhook_write_rax_r9: u8;
    static cannoli_memhook_write_rax_r9_end: u8;
    static cannoli_memhook_write_rax_r10: u8;
    static cannoli_memhook_write_rax_r10_end: u8;
    static cannoli_memhook_write_rax_r11: u8;
    static cannoli_memhook_write_rax_r11_end: u8;
    static cannoli_memhook_write_rax_r12: u8;
    static cannoli_memhook_write_rax_r12_end: u8;
    static cannoli_memhook_write_rax_r13: u8;
    static cannoli_memhook_write_rax_r13_end: u8;
    static cannoli_memhook_write_rax_r14: u8;
    static cannoli_memhook_write_rax_r14_end: u8;
    static cannoli_memhook_write_rax_r15: u8;
    static cannoli_memhook_write_rax_r15_end: u8;
    static cannoli_memhook_write_rcx_rax: u8;
    static cannoli_memhook_write_rcx_rax_end: u8;
    static cannoli_memhook_write_rcx_rcx: u8;
    static cannoli_memhook_write_rcx_rcx_end: u8;
    static cannoli_memhook_write_rcx_rdx: u8;
    static cannoli_memhook_write_rcx_rdx_end: u8;
    static cannoli_memhook_write_rcx_rbx: u8;
    static cannoli_memhook_write_rcx_rbx_end: u8;
    static cannoli_memhook_write_rcx_rsp: u8;
    static cannoli_memhook_write_rcx_rsp_end: u8;
    static cannoli_memhook_write_rcx_rbp: u8;
    static cannoli_memhook_write_rcx_rbp_end: u8;
    static cannoli_memhook_write_rcx_rsi: u8;
    static cannoli_memhook_write_rcx_rsi_end: u8;
    static cannoli_memhook_write_rcx_rdi: u8;
    static cannoli_memhook_write_rcx_rdi_end: u8;
    static cannoli_memhook_write_rcx_r8: u8;
    static cannoli_memhook_write_rcx_r8_end: u8;
    static cannoli_memhook_write_rcx_r9: u8;
    static cannoli_memhook_write_rcx_r9_end: u8;
    static cannoli_memhook_write_rcx_r10: u8;
    static cannoli_memhook_write_rcx_r10_end: u8;
    static cannoli_memhook_write_rcx_r11: u8;
    static cannoli_memhook_write_rcx_r11_end: u8;
    static cannoli_memhook_write_rcx_r12: u8;
    static cannoli_memhook_write_rcx_r12_end: u8;
    static cannoli_memhook_write_rcx_r13: u8;
    static cannoli_memhook_write_rcx_r13_end: u8;
    static cannoli_memhook_write_rcx_r14: u8;
    static cannoli_memhook_write_rcx_r14_end: u8;
    static cannoli_memhook_write_rcx_r15: u8;
    static cannoli_memhook_write_rcx_r15_end: u8;
    static cannoli_memhook_write_rdx_rax: u8;
    static cannoli_memhook_write_rdx_rax_end: u8;
    static cannoli_memhook_write_rdx_rcx: u8;
    static cannoli_memhook_write_rdx_rcx_end: u8;
    static cannoli_memhook_write_rdx_rdx: u8;
    static cannoli_memhook_write_rdx_rdx_end: u8;
    static cannoli_memhook_write_rdx_rbx: u8;
    static cannoli_memhook_write_rdx_rbx_end: u8;
    static cannoli_memhook_write_rdx_rsp: u8;
    static cannoli_memhook_write_rdx_rsp_end: u8;
    static cannoli_memhook_write_rdx_rbp: u8;
    static cannoli_memhook_write_rdx_rbp_end: u8;
    static cannoli_memhook_write_rdx_rsi: u8;
    static cannoli_memhook_write_rdx_rsi_end: u8;
    static cannoli_memhook_write_rdx_rdi: u8;
    static cannoli_memhook_write_rdx_rdi_end: u8;
    static cannoli_memhook_write_rdx_r8: u8;
    static cannoli_memhook_write_rdx_r8_end: u8;
    static cannoli_memhook_write_rdx_r9: u8;
    static cannoli_memhook_write_rdx_r9_end: u8;
    static cannoli_memhook_write_rdx_r10: u8;
    static cannoli_memhook_write_rdx_r10_end: u8;
    static cannoli_memhook_write_rdx_r11: u8;
    static cannoli_memhook_write_rdx_r11_end: u8;
    static cannoli_memhook_write_rdx_r12: u8;
    static cannoli_memhook_write_rdx_r12_end: u8;
    static cannoli_memhook_write_rdx_r13: u8;
    static cannoli_memhook_write_rdx_r13_end: u8;
    static cannoli_memhook_write_rdx_r14: u8;
    static cannoli_memhook_write_rdx_r14_end: u8;
    static cannoli_memhook_write_rdx_r15: u8;
    static cannoli_memhook_write_rdx_r15_end: u8;
    static cannoli_memhook_write_rbx_rax: u8;
    static cannoli_memhook_write_rbx_rax_end: u8;
    static cannoli_memhook_write_rbx_rcx: u8;
    static cannoli_memhook_write_rbx_rcx_end: u8;
    static cannoli_memhook_write_rbx_rdx: u8;
    static cannoli_memhook_write_rbx_rdx_end: u8;
    static cannoli_memhook_write_rbx_rbx: u8;
    static cannoli_memhook_write_rbx_rbx_end: u8;
    static cannoli_memhook_write_rbx_rsp: u8;
    static cannoli_memhook_write_rbx_rsp_end: u8;
    static cannoli_memhook_write_rbx_rbp: u8;
    static cannoli_memhook_write_rbx_rbp_end: u8;
    static cannoli_memhook_write_rbx_rsi: u8;
    static cannoli_memhook_write_rbx_rsi_end: u8;
    static cannoli_memhook_write_rbx_rdi: u8;
    static cannoli_memhook_write_rbx_rdi_end: u8;
    static cannoli_memhook_write_rbx_r8: u8;
    static cannoli_memhook_write_rbx_r8_end: u8;
    static cannoli_memhook_write_rbx_r9: u8;
    static cannoli_memhook_write_rbx_r9_end: u8;
    static cannoli_memhook_write_rbx_r10: u8;
    static cannoli_memhook_write_rbx_r10_end: u8;
    static cannoli_memhook_write_rbx_r11: u8;
    static cannoli_memhook_write_rbx_r11_end: u8;
    static cannoli_memhook_write_rbx_r12: u8;
    static cannoli_memhook_write_rbx_r12_end: u8;
    static cannoli_memhook_write_rbx_r13: u8;
    static cannoli_memhook_write_rbx_r13_end: u8;
    static cannoli_memhook_write_rbx_r14: u8;
    static cannoli_memhook_write_rbx_r14_end: u8;
    static cannoli_memhook_write_rbx_r15: u8;
    static cannoli_memhook_write_rbx_r15_end: u8;
    static cannoli_memhook_write_rsp_rax: u8;
    static cannoli_memhook_write_rsp_rax_end: u8;
    static cannoli_memhook_write_rsp_rcx: u8;
    static cannoli_memhook_write_rsp_rcx_end: u8;
    static cannoli_memhook_write_rsp_rdx: u8;
    static cannoli_memhook_write_rsp_rdx_end: u8;
    static cannoli_memhook_write_rsp_rbx: u8;
    static cannoli_memhook_write_rsp_rbx_end: u8;
    static cannoli_memhook_write_rsp_rsp: u8;
    static cannoli_memhook_write_rsp_rsp_end: u8;
    static cannoli_memhook_write_rsp_rbp: u8;
    static cannoli_memhook_write_rsp_rbp_end: u8;
    static cannoli_memhook_write_rsp_rsi: u8;
    static cannoli_memhook_write_rsp_rsi_end: u8;
    static cannoli_memhook_write_rsp_rdi: u8;
    static cannoli_memhook_write_rsp_rdi_end: u8;
    static cannoli_memhook_write_rsp_r8: u8;
    static cannoli_memhook_write_rsp_r8_end: u8;
    static cannoli_memhook_write_rsp_r9: u8;
    static cannoli_memhook_write_rsp_r9_end: u8;
    static cannoli_memhook_write_rsp_r10: u8;
    static cannoli_memhook_write_rsp_r10_end: u8;
    static cannoli_memhook_write_rsp_r11: u8;
    static cannoli_memhook_write_rsp_r11_end: u8;
    static cannoli_memhook_write_rsp_r12: u8;
    static cannoli_memhook_write_rsp_r12_end: u8;
    static cannoli_memhook_write_rsp_r13: u8;
    static cannoli_memhook_write_rsp_r13_end: u8;
    static cannoli_memhook_write_rsp_r14: u8;
    static cannoli_memhook_write_rsp_r14_end: u8;
    static cannoli_memhook_write_rsp_r15: u8;
    static cannoli_memhook_write_rsp_r15_end: u8;
    static cannoli_memhook_write_rbp_rax: u8;
    static cannoli_memhook_write_rbp_rax_end: u8;
    static cannoli_memhook_write_rbp_rcx: u8;
    static cannoli_memhook_write_rbp_rcx_end: u8;
    static cannoli_memhook_write_rbp_rdx: u8;
    static cannoli_memhook_write_rbp_rdx_end: u8;
    static cannoli_memhook_write_rbp_rbx: u8;
    static cannoli_memhook_write_rbp_rbx_end: u8;
    static cannoli_memhook_write_rbp_rsp: u8;
    static cannoli_memhook_write_rbp_rsp_end: u8;
    static cannoli_memhook_write_rbp_rbp: u8;
    static cannoli_memhook_write_rbp_rbp_end: u8;
    static cannoli_memhook_write_rbp_rsi: u8;
    static cannoli_memhook_write_rbp_rsi_end: u8;
    static cannoli_memhook_write_rbp_rdi: u8;
    static cannoli_memhook_write_rbp_rdi_end: u8;
    static cannoli_memhook_write_rbp_r8: u8;
    static cannoli_memhook_write_rbp_r8_end: u8;
    static cannoli_memhook_write_rbp_r9: u8;
    static cannoli_memhook_write_rbp_r9_end: u8;
    static cannoli_memhook_write_rbp_r10: u8;
    static cannoli_memhook_write_rbp_r10_end: u8;
    static cannoli_memhook_write_rbp_r11: u8;
    static cannoli_memhook_write_rbp_r11_end: u8;
    static cannoli_memhook_write_rbp_r12: u8;
    static cannoli_memhook_write_rbp_r12_end: u8;
    static cannoli_memhook_write_rbp_r13: u8;
    static cannoli_memhook_write_rbp_r13_end: u8;
    static cannoli_memhook_write_rbp_r14: u8;
    static cannoli_memhook_write_rbp_r14_end: u8;
    static cannoli_memhook_write_rbp_r15: u8;
    static cannoli_memhook_write_rbp_r15_end: u8;
    static cannoli_memhook_write_rsi_rax: u8;
    static cannoli_memhook_write_rsi_rax_end: u8;
    static cannoli_memhook_write_rsi_rcx: u8;
    static cannoli_memhook_write_rsi_rcx_end: u8;
    static cannoli_memhook_write_rsi_rdx: u8;
    static cannoli_memhook_write_rsi_rdx_end: u8;
    static cannoli_memhook_write_rsi_rbx: u8;
    static cannoli_memhook_write_rsi_rbx_end: u8;
    static cannoli_memhook_write_rsi_rsp: u8;
    static cannoli_memhook_write_rsi_rsp_end: u8;
    static cannoli_memhook_write_rsi_rbp: u8;
    static cannoli_memhook_write_rsi_rbp_end: u8;
    static cannoli_memhook_write_rsi_rsi: u8;
    static cannoli_memhook_write_rsi_rsi_end: u8;
    static cannoli_memhook_write_rsi_rdi: u8;
    static cannoli_memhook_write_rsi_rdi_end: u8;
    static cannoli_memhook_write_rsi_r8: u8;
    static cannoli_memhook_write_rsi_r8_end: u8;
    static cannoli_memhook_write_rsi_r9: u8;
    static cannoli_memhook_write_rsi_r9_end: u8;
    static cannoli_memhook_write_rsi_r10: u8;
    static cannoli_memhook_write_rsi_r10_end: u8;
    static cannoli_memhook_write_rsi_r11: u8;
    static cannoli_memhook_write_rsi_r11_end: u8;
    static cannoli_memhook_write_rsi_r12: u8;
    static cannoli_memhook_write_rsi_r12_end: u8;
    static cannoli_memhook_write_rsi_r13: u8;
    static cannoli_memhook_write_rsi_r13_end: u8;
    static cannoli_memhook_write_rsi_r14: u8;
    static cannoli_memhook_write_rsi_r14_end: u8;
    static cannoli_memhook_write_rsi_r15: u8;
    static cannoli_memhook_write_rsi_r15_end: u8;
    static cannoli_memhook_write_rdi_rax: u8;
    static cannoli_memhook_write_rdi_rax_end: u8;
    static cannoli_memhook_write_rdi_rcx: u8;
    static cannoli_memhook_write_rdi_rcx_end: u8;
    static cannoli_memhook_write_rdi_rdx: u8;
    static cannoli_memhook_write_rdi_rdx_end: u8;
    static cannoli_memhook_write_rdi_rbx: u8;
    static cannoli_memhook_write_rdi_rbx_end: u8;
    static cannoli_memhook_write_rdi_rsp: u8;
    static cannoli_memhook_write_rdi_rsp_end: u8;
    static cannoli_memhook_write_rdi_rbp: u8;
    static cannoli_memhook_write_rdi_rbp_end: u8;
    static cannoli_memhook_write_rdi_rsi: u8;
    static cannoli_memhook_write_rdi_rsi_end: u8;
    static cannoli_memhook_write_rdi_rdi: u8;
    static cannoli_memhook_write_rdi_rdi_end: u8;
    static cannoli_memhook_write_rdi_r8: u8;
    static cannoli_memhook_write_rdi_r8_end: u8;
    static cannoli_memhook_write_rdi_r9: u8;
    static cannoli_memhook_write_rdi_r9_end: u8;
    static cannoli_memhook_write_rdi_r10: u8;
    static cannoli_memhook_write_rdi_r10_end: u8;
    static cannoli_memhook_write_rdi_r11: u8;
    static cannoli_memhook_write_rdi_r11_end: u8;
    static cannoli_memhook_write_rdi_r12: u8;
    static cannoli_memhook_write_rdi_r12_end: u8;
    static cannoli_memhook_write_rdi_r13: u8;
    static cannoli_memhook_write_rdi_r13_end: u8;
    static cannoli_memhook_write_rdi_r14: u8;
    static cannoli_memhook_write_rdi_r14_end: u8;
    static cannoli_memhook_write_rdi_r15: u8;
    static cannoli_memhook_write_rdi_r15_end: u8;
    static cannoli_memhook_write_r8_rax: u8;
    static cannoli_memhook_write_r8_rax_end: u8;
    static cannoli_memhook_write_r8_rcx: u8;
    static cannoli_memhook_write_r8_rcx_end: u8;
    static cannoli_memhook_write_r8_rdx: u8;
    static cannoli_memhook_write_r8_rdx_end: u8;
    static cannoli_memhook_write_r8_rbx: u8;
    static cannoli_memhook_write_r8_rbx_end: u8;
    static cannoli_memhook_write_r8_rsp: u8;
    static cannoli_memhook_write_r8_rsp_end: u8;
    static cannoli_memhook_write_r8_rbp: u8;
    static cannoli_memhook_write_r8_rbp_end: u8;
    static cannoli_memhook_write_r8_rsi: u8;
    static cannoli_memhook_write_r8_rsi_end: u8;
    static cannoli_memhook_write_r8_rdi: u8;
    static cannoli_memhook_write_r8_rdi_end: u8;
    static cannoli_memhook_write_r8_r8: u8;
    static cannoli_memhook_write_r8_r8_end: u8;
    static cannoli_memhook_write_r8_r9: u8;
    static cannoli_memhook_write_r8_r9_end: u8;
    static cannoli_memhook_write_r8_r10: u8;
    static cannoli_memhook_write_r8_r10_end: u8;
    static cannoli_memhook_write_r8_r11: u8;
    static cannoli_memhook_write_r8_r11_end: u8;
    static cannoli_memhook_write_r8_r12: u8;
    static cannoli_memhook_write_r8_r12_end: u8;
    static cannoli_memhook_write_r8_r13: u8;
    static cannoli_memhook_write_r8_r13_end: u8;
    static cannoli_memhook_write_r8_r14: u8;
    static cannoli_memhook_write_r8_r14_end: u8;
    static cannoli_memhook_write_r8_r15: u8;
    static cannoli_memhook_write_r8_r15_end: u8;
    static cannoli_memhook_write_r9_rax: u8;
    static cannoli_memhook_write_r9_rax_end: u8;
    static cannoli_memhook_write_r9_rcx: u8;
    static cannoli_memhook_write_r9_rcx_end: u8;
    static cannoli_memhook_write_r9_rdx: u8;
    static cannoli_memhook_write_r9_rdx_end: u8;
    static cannoli_memhook_write_r9_rbx: u8;
    static cannoli_memhook_write_r9_rbx_end: u8;
    static cannoli_memhook_write_r9_rsp: u8;
    static cannoli_memhook_write_r9_rsp_end: u8;
    static cannoli_memhook_write_r9_rbp: u8;
    static cannoli_memhook_write_r9_rbp_end: u8;
    static cannoli_memhook_write_r9_rsi: u8;
    static cannoli_memhook_write_r9_rsi_end: u8;
    static cannoli_memhook_write_r9_rdi: u8;
    static cannoli_memhook_write_r9_rdi_end: u8;
    static cannoli_memhook_write_r9_r8: u8;
    static cannoli_memhook_write_r9_r8_end: u8;
    static cannoli_memhook_write_r9_r9: u8;
    static cannoli_memhook_write_r9_r9_end: u8;
    static cannoli_memhook_write_r9_r10: u8;
    static cannoli_memhook_write_r9_r10_end: u8;
    static cannoli_memhook_write_r9_r11: u8;
    static cannoli_memhook_write_r9_r11_end: u8;
    static cannoli_memhook_write_r9_r12: u8;
    static cannoli_memhook_write_r9_r12_end: u8;
    static cannoli_memhook_write_r9_r13: u8;
    static cannoli_memhook_write_r9_r13_end: u8;
    static cannoli_memhook_write_r9_r14: u8;
    static cannoli_memhook_write_r9_r14_end: u8;
    static cannoli_memhook_write_r9_r15: u8;
    static cannoli_memhook_write_r9_r15_end: u8;
    static cannoli_memhook_write_r10_rax: u8;
    static cannoli_memhook_write_r10_rax_end: u8;
    static cannoli_memhook_write_r10_rcx: u8;
    static cannoli_memhook_write_r10_rcx_end: u8;
    static cannoli_memhook_write_r10_rdx: u8;
    static cannoli_memhook_write_r10_rdx_end: u8;
    static cannoli_memhook_write_r10_rbx: u8;
    static cannoli_memhook_write_r10_rbx_end: u8;
    static cannoli_memhook_write_r10_rsp: u8;
    static cannoli_memhook_write_r10_rsp_end: u8;
    static cannoli_memhook_write_r10_rbp: u8;
    static cannoli_memhook_write_r10_rbp_end: u8;
    static cannoli_memhook_write_r10_rsi: u8;
    static cannoli_memhook_write_r10_rsi_end: u8;
    static cannoli_memhook_write_r10_rdi: u8;
    static cannoli_memhook_write_r10_rdi_end: u8;
    static cannoli_memhook_write_r10_r8: u8;
    static cannoli_memhook_write_r10_r8_end: u8;
    static cannoli_memhook_write_r10_r9: u8;
    static cannoli_memhook_write_r10_r9_end: u8;
    static cannoli_memhook_write_r10_r10: u8;
    static cannoli_memhook_write_r10_r10_end: u8;
    static cannoli_memhook_write_r10_r11: u8;
    static cannoli_memhook_write_r10_r11_end: u8;
    static cannoli_memhook_write_r10_r12: u8;
    static cannoli_memhook_write_r10_r12_end: u8;
    static cannoli_memhook_write_r10_r13: u8;
    static cannoli_memhook_write_r10_r13_end: u8;
    static cannoli_memhook_write_r10_r14: u8;
    static cannoli_memhook_write_r10_r14_end: u8;
    static cannoli_memhook_write_r10_r15: u8;
    static cannoli_memhook_write_r10_r15_end: u8;
    static cannoli_memhook_write_r11_rax: u8;
    static cannoli_memhook_write_r11_rax_end: u8;
    static cannoli_memhook_write_r11_rcx: u8;
    static cannoli_memhook_write_r11_rcx_end: u8;
    static cannoli_memhook_write_r11_rdx: u8;
    static cannoli_memhook_write_r11_rdx_end: u8;
    static cannoli_memhook_write_r11_rbx: u8;
    static cannoli_memhook_write_r11_rbx_end: u8;
    static cannoli_memhook_write_r11_rsp: u8;
    static cannoli_memhook_write_r11_rsp_end: u8;
    static cannoli_memhook_write_r11_rbp: u8;
    static cannoli_memhook_write_r11_rbp_end: u8;
    static cannoli_memhook_write_r11_rsi: u8;
    static cannoli_memhook_write_r11_rsi_end: u8;
    static cannoli_memhook_write_r11_rdi: u8;
    static cannoli_memhook_write_r11_rdi_end: u8;
    static cannoli_memhook_write_r11_r8: u8;
    static cannoli_memhook_write_r11_r8_end: u8;
    static cannoli_memhook_write_r11_r9: u8;
    static cannoli_memhook_write_r11_r9_end: u8;
    static cannoli_memhook_write_r11_r10: u8;
    static cannoli_memhook_write_r11_r10_end: u8;
    static cannoli_memhook_write_r11_r11: u8;
    static cannoli_memhook_write_r11_r11_end: u8;
    static cannoli_memhook_write_r11_r12: u8;
    static cannoli_memhook_write_r11_r12_end: u8;
    static cannoli_memhook_write_r11_r13: u8;
    static cannoli_memhook_write_r11_r13_end: u8;
    static cannoli_memhook_write_r11_r14: u8;
    static cannoli_memhook_write_r11_r14_end: u8;
    static cannoli_memhook_write_r11_r15: u8;
    static cannoli_memhook_write_r11_r15_end: u8;
    static cannoli_memhook_write_r12_rax: u8;
    static cannoli_memhook_write_r12_rax_end: u8;
    static cannoli_memhook_write_r12_rcx: u8;
    static cannoli_memhook_write_r12_rcx_end: u8;
    static cannoli_memhook_write_r12_rdx: u8;
    static cannoli_memhook_write_r12_rdx_end: u8;
    static cannoli_memhook_write_r12_rbx: u8;
    static cannoli_memhook_write_r12_rbx_end: u8;
    static cannoli_memhook_write_r12_rsp: u8;
    static cannoli_memhook_write_r12_rsp_end: u8;
    static cannoli_memhook_write_r12_rbp: u8;
    static cannoli_memhook_write_r12_rbp_end: u8;
    static cannoli_memhook_write_r12_rsi: u8;
    static cannoli_memhook_write_r12_rsi_end: u8;
    static cannoli_memhook_write_r12_rdi: u8;
    static cannoli_memhook_write_r12_rdi_end: u8;
    static cannoli_memhook_write_r12_r8: u8;
    static cannoli_memhook_write_r12_r8_end: u8;
    static cannoli_memhook_write_r12_r9: u8;
    static cannoli_memhook_write_r12_r9_end: u8;
    static cannoli_memhook_write_r12_r10: u8;
    static cannoli_memhook_write_r12_r10_end: u8;
    static cannoli_memhook_write_r12_r11: u8;
    static cannoli_memhook_write_r12_r11_end: u8;
    static cannoli_memhook_write_r12_r12: u8;
    static cannoli_memhook_write_r12_r12_end: u8;
    static cannoli_memhook_write_r12_r13: u8;
    static cannoli_memhook_write_r12_r13_end: u8;
    static cannoli_memhook_write_r12_r14: u8;
    static cannoli_memhook_write_r12_r14_end: u8;
    static cannoli_memhook_write_r12_r15: u8;
    static cannoli_memhook_write_r12_r15_end: u8;
    static cannoli_memhook_write_r13_rax: u8;
    static cannoli_memhook_write_r13_rax_end: u8;
    static cannoli_memhook_write_r13_rcx: u8;
    static cannoli_memhook_write_r13_rcx_end: u8;
    static cannoli_memhook_write_r13_rdx: u8;
    static cannoli_memhook_write_r13_rdx_end: u8;
    static cannoli_memhook_write_r13_rbx: u8;
    static cannoli_memhook_write_r13_rbx_end: u8;
    static cannoli_memhook_write_r13_rsp: u8;
    static cannoli_memhook_write_r13_rsp_end: u8;
    static cannoli_memhook_write_r13_rbp: u8;
    static cannoli_memhook_write_r13_rbp_end: u8;
    static cannoli_memhook_write_r13_rsi: u8;
    static cannoli_memhook_write_r13_rsi_end: u8;
    static cannoli_memhook_write_r13_rdi: u8;
    static cannoli_memhook_write_r13_rdi_end: u8;
    static cannoli_memhook_write_r13_r8: u8;
    static cannoli_memhook_write_r13_r8_end: u8;
    static cannoli_memhook_write_r13_r9: u8;
    static cannoli_memhook_write_r13_r9_end: u8;
    static cannoli_memhook_write_r13_r10: u8;
    static cannoli_memhook_write_r13_r10_end: u8;
    static cannoli_memhook_write_r13_r11: u8;
    static cannoli_memhook_write_r13_r11_end: u8;
    static cannoli_memhook_write_r13_r12: u8;
    static cannoli_memhook_write_r13_r12_end: u8;
    static cannoli_memhook_write_r13_r13: u8;
    static cannoli_memhook_write_r13_r13_end: u8;
    static cannoli_memhook_write_r13_r14: u8;
    static cannoli_memhook_write_r13_r14_end: u8;
    static cannoli_memhook_write_r13_r15: u8;
    static cannoli_memhook_write_r13_r15_end: u8;
    static cannoli_memhook_write_r14_rax: u8;
    static cannoli_memhook_write_r14_rax_end: u8;
    static cannoli_memhook_write_r14_rcx: u8;
    static cannoli_memhook_write_r14_rcx_end: u8;
    static cannoli_memhook_write_r14_rdx: u8;
    static cannoli_memhook_write_r14_rdx_end: u8;
    static cannoli_memhook_write_r14_rbx: u8;
    static cannoli_memhook_write_r14_rbx_end: u8;
    static cannoli_memhook_write_r14_rsp: u8;
    static cannoli_memhook_write_r14_rsp_end: u8;
    static cannoli_memhook_write_r14_rbp: u8;
    static cannoli_memhook_write_r14_rbp_end: u8;
    static cannoli_memhook_write_r14_rsi: u8;
    static cannoli_memhook_write_r14_rsi_end: u8;
    static cannoli_memhook_write_r14_rdi: u8;
    static cannoli_memhook_write_r14_rdi_end: u8;
    static cannoli_memhook_write_r14_r8: u8;
    static cannoli_memhook_write_r14_r8_end: u8;
    static cannoli_memhook_write_r14_r9: u8;
    static cannoli_memhook_write_r14_r9_end: u8;
    static cannoli_memhook_write_r14_r10: u8;
    static cannoli_memhook_write_r14_r10_end: u8;
    static cannoli_memhook_write_r14_r11: u8;
    static cannoli_memhook_write_r14_r11_end: u8;
    static cannoli_memhook_write_r14_r12: u8;
    static cannoli_memhook_write_r14_r12_end: u8;
    static cannoli_memhook_write_r14_r13: u8;
    static cannoli_memhook_write_r14_r13_end: u8;
    static cannoli_memhook_write_r14_r14: u8;
    static cannoli_memhook_write_r14_r14_end: u8;
    static cannoli_memhook_write_r14_r15: u8;
    static cannoli_memhook_write_r14_r15_end: u8;
    static cannoli_memhook_write_r15_rax: u8;
    static cannoli_memhook_write_r15_rax_end: u8;
    static cannoli_memhook_write_r15_rcx: u8;
    static cannoli_memhook_write_r15_rcx_end: u8;
    static cannoli_memhook_write_r15_rdx: u8;
    static cannoli_memhook_write_r15_rdx_end: u8;
    static cannoli_memhook_write_r15_rbx: u8;
    static cannoli_memhook_write_r15_rbx_end: u8;
    static cannoli_memhook_write_r15_rsp: u8;
    static cannoli_memhook_write_r15_rsp_end: u8;
    static cannoli_memhook_write_r15_rbp: u8;
    static cannoli_memhook_write_r15_rbp_end: u8;
    static cannoli_memhook_write_r15_rsi: u8;
    static cannoli_memhook_write_r15_rsi_end: u8;
    static cannoli_memhook_write_r15_rdi: u8;
    static cannoli_memhook_write_r15_rdi_end: u8;
    static cannoli_memhook_write_r15_r8: u8;
    static cannoli_memhook_write_r15_r8_end: u8;
    static cannoli_memhook_write_r15_r9: u8;
    static cannoli_memhook_write_r15_r9_end: u8;
    static cannoli_memhook_write_r15_r10: u8;
    static cannoli_memhook_write_r15_r10_end: u8;
    static cannoli_memhook_write_r15_r11: u8;
    static cannoli_memhook_write_r15_r11_end: u8;
    static cannoli_memhook_write_r15_r12: u8;
    static cannoli_memhook_write_r15_r12_end: u8;
    static cannoli_memhook_write_r15_r13: u8;
    static cannoli_memhook_write_r15_r13_end: u8;
    static cannoli_memhook_write_r15_r14: u8;
    static cannoli_memhook_write_r15_r14_end: u8;
    static cannoli_memhook_write_r15_r15: u8;
    static cannoli_memhook_write_r15_r15_end: u8;
}

/// Memory hook table, indexed by
///     `MEMHOOK_TABLE[bitness][access_width][access_type][data][addr]`
/// where `bitness` is 0 for 32-bit and 1 for 64-bit, and `data` and `addr`
/// being the register indicies that currently holds those values.
///
/// `access_width` values:
///   0 - 8-bit
///   1 - 16-bit
///   2 - 32-bit
///   3 - 64-bit
///
/// `access_type` values:
///   0 - read
///   1 - write
///
/// Yes clippy, I am aware this looks like shit. Fuck you.
#[allow(clippy::type_complexity)]
pub static MEMHOOK_TABLE: [[[[[(&u8, &u8); 16]; 16]; 2]; 4]; 2] = [
    [
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_al_eax, &cannoli_memhook_read_al_eax_end) },
                    unsafe { (&cannoli_memhook_read_al_ecx, &cannoli_memhook_read_al_ecx_end) },
                    unsafe { (&cannoli_memhook_read_al_edx, &cannoli_memhook_read_al_edx_end) },
                    unsafe { (&cannoli_memhook_read_al_ebx, &cannoli_memhook_read_al_ebx_end) },
                    unsafe { (&cannoli_memhook_read_al_esp, &cannoli_memhook_read_al_esp_end) },
                    unsafe { (&cannoli_memhook_read_al_ebp, &cannoli_memhook_read_al_ebp_end) },
                    unsafe { (&cannoli_memhook_read_al_esi, &cannoli_memhook_read_al_esi_end) },
                    unsafe { (&cannoli_memhook_read_al_edi, &cannoli_memhook_read_al_edi_end) },
                    unsafe { (&cannoli_memhook_read_al_r8d, &cannoli_memhook_read_al_r8d_end) },
                    unsafe { (&cannoli_memhook_read_al_r9d, &cannoli_memhook_read_al_r9d_end) },
                    unsafe { (&cannoli_memhook_read_al_r10d, &cannoli_memhook_read_al_r10d_end) },
                    unsafe { (&cannoli_memhook_read_al_r11d, &cannoli_memhook_read_al_r11d_end) },
                    unsafe { (&cannoli_memhook_read_al_r12d, &cannoli_memhook_read_al_r12d_end) },
                    unsafe { (&cannoli_memhook_read_al_r13d, &cannoli_memhook_read_al_r13d_end) },
                    unsafe { (&cannoli_memhook_read_al_r14d, &cannoli_memhook_read_al_r14d_end) },
                    unsafe { (&cannoli_memhook_read_al_r15d, &cannoli_memhook_read_al_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_cl_eax, &cannoli_memhook_read_cl_eax_end) },
                    unsafe { (&cannoli_memhook_read_cl_ecx, &cannoli_memhook_read_cl_ecx_end) },
                    unsafe { (&cannoli_memhook_read_cl_edx, &cannoli_memhook_read_cl_edx_end) },
                    unsafe { (&cannoli_memhook_read_cl_ebx, &cannoli_memhook_read_cl_ebx_end) },
                    unsafe { (&cannoli_memhook_read_cl_esp, &cannoli_memhook_read_cl_esp_end) },
                    unsafe { (&cannoli_memhook_read_cl_ebp, &cannoli_memhook_read_cl_ebp_end) },
                    unsafe { (&cannoli_memhook_read_cl_esi, &cannoli_memhook_read_cl_esi_end) },
                    unsafe { (&cannoli_memhook_read_cl_edi, &cannoli_memhook_read_cl_edi_end) },
                    unsafe { (&cannoli_memhook_read_cl_r8d, &cannoli_memhook_read_cl_r8d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r9d, &cannoli_memhook_read_cl_r9d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r10d, &cannoli_memhook_read_cl_r10d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r11d, &cannoli_memhook_read_cl_r11d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r12d, &cannoli_memhook_read_cl_r12d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r13d, &cannoli_memhook_read_cl_r13d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r14d, &cannoli_memhook_read_cl_r14d_end) },
                    unsafe { (&cannoli_memhook_read_cl_r15d, &cannoli_memhook_read_cl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dl_eax, &cannoli_memhook_read_dl_eax_end) },
                    unsafe { (&cannoli_memhook_read_dl_ecx, &cannoli_memhook_read_dl_ecx_end) },
                    unsafe { (&cannoli_memhook_read_dl_edx, &cannoli_memhook_read_dl_edx_end) },
                    unsafe { (&cannoli_memhook_read_dl_ebx, &cannoli_memhook_read_dl_ebx_end) },
                    unsafe { (&cannoli_memhook_read_dl_esp, &cannoli_memhook_read_dl_esp_end) },
                    unsafe { (&cannoli_memhook_read_dl_ebp, &cannoli_memhook_read_dl_ebp_end) },
                    unsafe { (&cannoli_memhook_read_dl_esi, &cannoli_memhook_read_dl_esi_end) },
                    unsafe { (&cannoli_memhook_read_dl_edi, &cannoli_memhook_read_dl_edi_end) },
                    unsafe { (&cannoli_memhook_read_dl_r8d, &cannoli_memhook_read_dl_r8d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r9d, &cannoli_memhook_read_dl_r9d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r10d, &cannoli_memhook_read_dl_r10d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r11d, &cannoli_memhook_read_dl_r11d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r12d, &cannoli_memhook_read_dl_r12d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r13d, &cannoli_memhook_read_dl_r13d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r14d, &cannoli_memhook_read_dl_r14d_end) },
                    unsafe { (&cannoli_memhook_read_dl_r15d, &cannoli_memhook_read_dl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bl_eax, &cannoli_memhook_read_bl_eax_end) },
                    unsafe { (&cannoli_memhook_read_bl_ecx, &cannoli_memhook_read_bl_ecx_end) },
                    unsafe { (&cannoli_memhook_read_bl_edx, &cannoli_memhook_read_bl_edx_end) },
                    unsafe { (&cannoli_memhook_read_bl_ebx, &cannoli_memhook_read_bl_ebx_end) },
                    unsafe { (&cannoli_memhook_read_bl_esp, &cannoli_memhook_read_bl_esp_end) },
                    unsafe { (&cannoli_memhook_read_bl_ebp, &cannoli_memhook_read_bl_ebp_end) },
                    unsafe { (&cannoli_memhook_read_bl_esi, &cannoli_memhook_read_bl_esi_end) },
                    unsafe { (&cannoli_memhook_read_bl_edi, &cannoli_memhook_read_bl_edi_end) },
                    unsafe { (&cannoli_memhook_read_bl_r8d, &cannoli_memhook_read_bl_r8d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r9d, &cannoli_memhook_read_bl_r9d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r10d, &cannoli_memhook_read_bl_r10d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r11d, &cannoli_memhook_read_bl_r11d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r12d, &cannoli_memhook_read_bl_r12d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r13d, &cannoli_memhook_read_bl_r13d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r14d, &cannoli_memhook_read_bl_r14d_end) },
                    unsafe { (&cannoli_memhook_read_bl_r15d, &cannoli_memhook_read_bl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_spl_eax, &cannoli_memhook_read_spl_eax_end) },
                    unsafe { (&cannoli_memhook_read_spl_ecx, &cannoli_memhook_read_spl_ecx_end) },
                    unsafe { (&cannoli_memhook_read_spl_edx, &cannoli_memhook_read_spl_edx_end) },
                    unsafe { (&cannoli_memhook_read_spl_ebx, &cannoli_memhook_read_spl_ebx_end) },
                    unsafe { (&cannoli_memhook_read_spl_esp, &cannoli_memhook_read_spl_esp_end) },
                    unsafe { (&cannoli_memhook_read_spl_ebp, &cannoli_memhook_read_spl_ebp_end) },
                    unsafe { (&cannoli_memhook_read_spl_esi, &cannoli_memhook_read_spl_esi_end) },
                    unsafe { (&cannoli_memhook_read_spl_edi, &cannoli_memhook_read_spl_edi_end) },
                    unsafe { (&cannoli_memhook_read_spl_r8d, &cannoli_memhook_read_spl_r8d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r9d, &cannoli_memhook_read_spl_r9d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r10d, &cannoli_memhook_read_spl_r10d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r11d, &cannoli_memhook_read_spl_r11d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r12d, &cannoli_memhook_read_spl_r12d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r13d, &cannoli_memhook_read_spl_r13d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r14d, &cannoli_memhook_read_spl_r14d_end) },
                    unsafe { (&cannoli_memhook_read_spl_r15d, &cannoli_memhook_read_spl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bpl_eax, &cannoli_memhook_read_bpl_eax_end) },
                    unsafe { (&cannoli_memhook_read_bpl_ecx, &cannoli_memhook_read_bpl_ecx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_edx, &cannoli_memhook_read_bpl_edx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_ebx, &cannoli_memhook_read_bpl_ebx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_esp, &cannoli_memhook_read_bpl_esp_end) },
                    unsafe { (&cannoli_memhook_read_bpl_ebp, &cannoli_memhook_read_bpl_ebp_end) },
                    unsafe { (&cannoli_memhook_read_bpl_esi, &cannoli_memhook_read_bpl_esi_end) },
                    unsafe { (&cannoli_memhook_read_bpl_edi, &cannoli_memhook_read_bpl_edi_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r8d, &cannoli_memhook_read_bpl_r8d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r9d, &cannoli_memhook_read_bpl_r9d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r10d, &cannoli_memhook_read_bpl_r10d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r11d, &cannoli_memhook_read_bpl_r11d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r12d, &cannoli_memhook_read_bpl_r12d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r13d, &cannoli_memhook_read_bpl_r13d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r14d, &cannoli_memhook_read_bpl_r14d_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r15d, &cannoli_memhook_read_bpl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_sil_eax, &cannoli_memhook_read_sil_eax_end) },
                    unsafe { (&cannoli_memhook_read_sil_ecx, &cannoli_memhook_read_sil_ecx_end) },
                    unsafe { (&cannoli_memhook_read_sil_edx, &cannoli_memhook_read_sil_edx_end) },
                    unsafe { (&cannoli_memhook_read_sil_ebx, &cannoli_memhook_read_sil_ebx_end) },
                    unsafe { (&cannoli_memhook_read_sil_esp, &cannoli_memhook_read_sil_esp_end) },
                    unsafe { (&cannoli_memhook_read_sil_ebp, &cannoli_memhook_read_sil_ebp_end) },
                    unsafe { (&cannoli_memhook_read_sil_esi, &cannoli_memhook_read_sil_esi_end) },
                    unsafe { (&cannoli_memhook_read_sil_edi, &cannoli_memhook_read_sil_edi_end) },
                    unsafe { (&cannoli_memhook_read_sil_r8d, &cannoli_memhook_read_sil_r8d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r9d, &cannoli_memhook_read_sil_r9d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r10d, &cannoli_memhook_read_sil_r10d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r11d, &cannoli_memhook_read_sil_r11d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r12d, &cannoli_memhook_read_sil_r12d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r13d, &cannoli_memhook_read_sil_r13d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r14d, &cannoli_memhook_read_sil_r14d_end) },
                    unsafe { (&cannoli_memhook_read_sil_r15d, &cannoli_memhook_read_sil_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dil_eax, &cannoli_memhook_read_dil_eax_end) },
                    unsafe { (&cannoli_memhook_read_dil_ecx, &cannoli_memhook_read_dil_ecx_end) },
                    unsafe { (&cannoli_memhook_read_dil_edx, &cannoli_memhook_read_dil_edx_end) },
                    unsafe { (&cannoli_memhook_read_dil_ebx, &cannoli_memhook_read_dil_ebx_end) },
                    unsafe { (&cannoli_memhook_read_dil_esp, &cannoli_memhook_read_dil_esp_end) },
                    unsafe { (&cannoli_memhook_read_dil_ebp, &cannoli_memhook_read_dil_ebp_end) },
                    unsafe { (&cannoli_memhook_read_dil_esi, &cannoli_memhook_read_dil_esi_end) },
                    unsafe { (&cannoli_memhook_read_dil_edi, &cannoli_memhook_read_dil_edi_end) },
                    unsafe { (&cannoli_memhook_read_dil_r8d, &cannoli_memhook_read_dil_r8d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r9d, &cannoli_memhook_read_dil_r9d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r10d, &cannoli_memhook_read_dil_r10d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r11d, &cannoli_memhook_read_dil_r11d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r12d, &cannoli_memhook_read_dil_r12d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r13d, &cannoli_memhook_read_dil_r13d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r14d, &cannoli_memhook_read_dil_r14d_end) },
                    unsafe { (&cannoli_memhook_read_dil_r15d, &cannoli_memhook_read_dil_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8b_eax, &cannoli_memhook_read_r8b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r8b_ecx, &cannoli_memhook_read_r8b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_edx, &cannoli_memhook_read_r8b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_ebx, &cannoli_memhook_read_r8b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_esp, &cannoli_memhook_read_r8b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r8b_ebp, &cannoli_memhook_read_r8b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r8b_esi, &cannoli_memhook_read_r8b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r8b_edi, &cannoli_memhook_read_r8b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r8d, &cannoli_memhook_read_r8b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r9d, &cannoli_memhook_read_r8b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r10d, &cannoli_memhook_read_r8b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r11d, &cannoli_memhook_read_r8b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r12d, &cannoli_memhook_read_r8b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r13d, &cannoli_memhook_read_r8b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r14d, &cannoli_memhook_read_r8b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r15d, &cannoli_memhook_read_r8b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9b_eax, &cannoli_memhook_read_r9b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r9b_ecx, &cannoli_memhook_read_r9b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_edx, &cannoli_memhook_read_r9b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_ebx, &cannoli_memhook_read_r9b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_esp, &cannoli_memhook_read_r9b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r9b_ebp, &cannoli_memhook_read_r9b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r9b_esi, &cannoli_memhook_read_r9b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r9b_edi, &cannoli_memhook_read_r9b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r8d, &cannoli_memhook_read_r9b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r9d, &cannoli_memhook_read_r9b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r10d, &cannoli_memhook_read_r9b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r11d, &cannoli_memhook_read_r9b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r12d, &cannoli_memhook_read_r9b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r13d, &cannoli_memhook_read_r9b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r14d, &cannoli_memhook_read_r9b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r15d, &cannoli_memhook_read_r9b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10b_eax, &cannoli_memhook_read_r10b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r10b_ecx, &cannoli_memhook_read_r10b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_edx, &cannoli_memhook_read_r10b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_ebx, &cannoli_memhook_read_r10b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_esp, &cannoli_memhook_read_r10b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r10b_ebp, &cannoli_memhook_read_r10b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r10b_esi, &cannoli_memhook_read_r10b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r10b_edi, &cannoli_memhook_read_r10b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r8d, &cannoli_memhook_read_r10b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r9d, &cannoli_memhook_read_r10b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r10d, &cannoli_memhook_read_r10b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r11d, &cannoli_memhook_read_r10b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r12d, &cannoli_memhook_read_r10b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r13d, &cannoli_memhook_read_r10b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r14d, &cannoli_memhook_read_r10b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r15d, &cannoli_memhook_read_r10b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11b_eax, &cannoli_memhook_read_r11b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r11b_ecx, &cannoli_memhook_read_r11b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_edx, &cannoli_memhook_read_r11b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_ebx, &cannoli_memhook_read_r11b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_esp, &cannoli_memhook_read_r11b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r11b_ebp, &cannoli_memhook_read_r11b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r11b_esi, &cannoli_memhook_read_r11b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r11b_edi, &cannoli_memhook_read_r11b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r8d, &cannoli_memhook_read_r11b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r9d, &cannoli_memhook_read_r11b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r10d, &cannoli_memhook_read_r11b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r11d, &cannoli_memhook_read_r11b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r12d, &cannoli_memhook_read_r11b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r13d, &cannoli_memhook_read_r11b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r14d, &cannoli_memhook_read_r11b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r15d, &cannoli_memhook_read_r11b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12b_eax, &cannoli_memhook_read_r12b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r12b_ecx, &cannoli_memhook_read_r12b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_edx, &cannoli_memhook_read_r12b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_ebx, &cannoli_memhook_read_r12b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_esp, &cannoli_memhook_read_r12b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r12b_ebp, &cannoli_memhook_read_r12b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r12b_esi, &cannoli_memhook_read_r12b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r12b_edi, &cannoli_memhook_read_r12b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r8d, &cannoli_memhook_read_r12b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r9d, &cannoli_memhook_read_r12b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r10d, &cannoli_memhook_read_r12b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r11d, &cannoli_memhook_read_r12b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r12d, &cannoli_memhook_read_r12b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r13d, &cannoli_memhook_read_r12b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r14d, &cannoli_memhook_read_r12b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r15d, &cannoli_memhook_read_r12b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13b_eax, &cannoli_memhook_read_r13b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r13b_ecx, &cannoli_memhook_read_r13b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_edx, &cannoli_memhook_read_r13b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_ebx, &cannoli_memhook_read_r13b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_esp, &cannoli_memhook_read_r13b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r13b_ebp, &cannoli_memhook_read_r13b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r13b_esi, &cannoli_memhook_read_r13b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r13b_edi, &cannoli_memhook_read_r13b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r8d, &cannoli_memhook_read_r13b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r9d, &cannoli_memhook_read_r13b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r10d, &cannoli_memhook_read_r13b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r11d, &cannoli_memhook_read_r13b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r12d, &cannoli_memhook_read_r13b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r13d, &cannoli_memhook_read_r13b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r14d, &cannoli_memhook_read_r13b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r15d, &cannoli_memhook_read_r13b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14b_eax, &cannoli_memhook_read_r14b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r14b_ecx, &cannoli_memhook_read_r14b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_edx, &cannoli_memhook_read_r14b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_ebx, &cannoli_memhook_read_r14b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_esp, &cannoli_memhook_read_r14b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r14b_ebp, &cannoli_memhook_read_r14b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r14b_esi, &cannoli_memhook_read_r14b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r14b_edi, &cannoli_memhook_read_r14b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r8d, &cannoli_memhook_read_r14b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r9d, &cannoli_memhook_read_r14b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r10d, &cannoli_memhook_read_r14b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r11d, &cannoli_memhook_read_r14b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r12d, &cannoli_memhook_read_r14b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r13d, &cannoli_memhook_read_r14b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r14d, &cannoli_memhook_read_r14b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r15d, &cannoli_memhook_read_r14b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15b_eax, &cannoli_memhook_read_r15b_eax_end) },
                    unsafe { (&cannoli_memhook_read_r15b_ecx, &cannoli_memhook_read_r15b_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_edx, &cannoli_memhook_read_r15b_edx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_ebx, &cannoli_memhook_read_r15b_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_esp, &cannoli_memhook_read_r15b_esp_end) },
                    unsafe { (&cannoli_memhook_read_r15b_ebp, &cannoli_memhook_read_r15b_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r15b_esi, &cannoli_memhook_read_r15b_esi_end) },
                    unsafe { (&cannoli_memhook_read_r15b_edi, &cannoli_memhook_read_r15b_edi_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r8d, &cannoli_memhook_read_r15b_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r9d, &cannoli_memhook_read_r15b_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r10d, &cannoli_memhook_read_r15b_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r11d, &cannoli_memhook_read_r15b_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r12d, &cannoli_memhook_read_r15b_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r13d, &cannoli_memhook_read_r15b_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r14d, &cannoli_memhook_read_r15b_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r15d, &cannoli_memhook_read_r15b_r15d_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_al_eax, &cannoli_memhook_write_al_eax_end) },
                    unsafe { (&cannoli_memhook_write_al_ecx, &cannoli_memhook_write_al_ecx_end) },
                    unsafe { (&cannoli_memhook_write_al_edx, &cannoli_memhook_write_al_edx_end) },
                    unsafe { (&cannoli_memhook_write_al_ebx, &cannoli_memhook_write_al_ebx_end) },
                    unsafe { (&cannoli_memhook_write_al_esp, &cannoli_memhook_write_al_esp_end) },
                    unsafe { (&cannoli_memhook_write_al_ebp, &cannoli_memhook_write_al_ebp_end) },
                    unsafe { (&cannoli_memhook_write_al_esi, &cannoli_memhook_write_al_esi_end) },
                    unsafe { (&cannoli_memhook_write_al_edi, &cannoli_memhook_write_al_edi_end) },
                    unsafe { (&cannoli_memhook_write_al_r8d, &cannoli_memhook_write_al_r8d_end) },
                    unsafe { (&cannoli_memhook_write_al_r9d, &cannoli_memhook_write_al_r9d_end) },
                    unsafe { (&cannoli_memhook_write_al_r10d, &cannoli_memhook_write_al_r10d_end) },
                    unsafe { (&cannoli_memhook_write_al_r11d, &cannoli_memhook_write_al_r11d_end) },
                    unsafe { (&cannoli_memhook_write_al_r12d, &cannoli_memhook_write_al_r12d_end) },
                    unsafe { (&cannoli_memhook_write_al_r13d, &cannoli_memhook_write_al_r13d_end) },
                    unsafe { (&cannoli_memhook_write_al_r14d, &cannoli_memhook_write_al_r14d_end) },
                    unsafe { (&cannoli_memhook_write_al_r15d, &cannoli_memhook_write_al_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_cl_eax, &cannoli_memhook_write_cl_eax_end) },
                    unsafe { (&cannoli_memhook_write_cl_ecx, &cannoli_memhook_write_cl_ecx_end) },
                    unsafe { (&cannoli_memhook_write_cl_edx, &cannoli_memhook_write_cl_edx_end) },
                    unsafe { (&cannoli_memhook_write_cl_ebx, &cannoli_memhook_write_cl_ebx_end) },
                    unsafe { (&cannoli_memhook_write_cl_esp, &cannoli_memhook_write_cl_esp_end) },
                    unsafe { (&cannoli_memhook_write_cl_ebp, &cannoli_memhook_write_cl_ebp_end) },
                    unsafe { (&cannoli_memhook_write_cl_esi, &cannoli_memhook_write_cl_esi_end) },
                    unsafe { (&cannoli_memhook_write_cl_edi, &cannoli_memhook_write_cl_edi_end) },
                    unsafe { (&cannoli_memhook_write_cl_r8d, &cannoli_memhook_write_cl_r8d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r9d, &cannoli_memhook_write_cl_r9d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r10d, &cannoli_memhook_write_cl_r10d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r11d, &cannoli_memhook_write_cl_r11d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r12d, &cannoli_memhook_write_cl_r12d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r13d, &cannoli_memhook_write_cl_r13d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r14d, &cannoli_memhook_write_cl_r14d_end) },
                    unsafe { (&cannoli_memhook_write_cl_r15d, &cannoli_memhook_write_cl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dl_eax, &cannoli_memhook_write_dl_eax_end) },
                    unsafe { (&cannoli_memhook_write_dl_ecx, &cannoli_memhook_write_dl_ecx_end) },
                    unsafe { (&cannoli_memhook_write_dl_edx, &cannoli_memhook_write_dl_edx_end) },
                    unsafe { (&cannoli_memhook_write_dl_ebx, &cannoli_memhook_write_dl_ebx_end) },
                    unsafe { (&cannoli_memhook_write_dl_esp, &cannoli_memhook_write_dl_esp_end) },
                    unsafe { (&cannoli_memhook_write_dl_ebp, &cannoli_memhook_write_dl_ebp_end) },
                    unsafe { (&cannoli_memhook_write_dl_esi, &cannoli_memhook_write_dl_esi_end) },
                    unsafe { (&cannoli_memhook_write_dl_edi, &cannoli_memhook_write_dl_edi_end) },
                    unsafe { (&cannoli_memhook_write_dl_r8d, &cannoli_memhook_write_dl_r8d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r9d, &cannoli_memhook_write_dl_r9d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r10d, &cannoli_memhook_write_dl_r10d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r11d, &cannoli_memhook_write_dl_r11d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r12d, &cannoli_memhook_write_dl_r12d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r13d, &cannoli_memhook_write_dl_r13d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r14d, &cannoli_memhook_write_dl_r14d_end) },
                    unsafe { (&cannoli_memhook_write_dl_r15d, &cannoli_memhook_write_dl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bl_eax, &cannoli_memhook_write_bl_eax_end) },
                    unsafe { (&cannoli_memhook_write_bl_ecx, &cannoli_memhook_write_bl_ecx_end) },
                    unsafe { (&cannoli_memhook_write_bl_edx, &cannoli_memhook_write_bl_edx_end) },
                    unsafe { (&cannoli_memhook_write_bl_ebx, &cannoli_memhook_write_bl_ebx_end) },
                    unsafe { (&cannoli_memhook_write_bl_esp, &cannoli_memhook_write_bl_esp_end) },
                    unsafe { (&cannoli_memhook_write_bl_ebp, &cannoli_memhook_write_bl_ebp_end) },
                    unsafe { (&cannoli_memhook_write_bl_esi, &cannoli_memhook_write_bl_esi_end) },
                    unsafe { (&cannoli_memhook_write_bl_edi, &cannoli_memhook_write_bl_edi_end) },
                    unsafe { (&cannoli_memhook_write_bl_r8d, &cannoli_memhook_write_bl_r8d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r9d, &cannoli_memhook_write_bl_r9d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r10d, &cannoli_memhook_write_bl_r10d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r11d, &cannoli_memhook_write_bl_r11d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r12d, &cannoli_memhook_write_bl_r12d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r13d, &cannoli_memhook_write_bl_r13d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r14d, &cannoli_memhook_write_bl_r14d_end) },
                    unsafe { (&cannoli_memhook_write_bl_r15d, &cannoli_memhook_write_bl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_spl_eax, &cannoli_memhook_write_spl_eax_end) },
                    unsafe { (&cannoli_memhook_write_spl_ecx, &cannoli_memhook_write_spl_ecx_end) },
                    unsafe { (&cannoli_memhook_write_spl_edx, &cannoli_memhook_write_spl_edx_end) },
                    unsafe { (&cannoli_memhook_write_spl_ebx, &cannoli_memhook_write_spl_ebx_end) },
                    unsafe { (&cannoli_memhook_write_spl_esp, &cannoli_memhook_write_spl_esp_end) },
                    unsafe { (&cannoli_memhook_write_spl_ebp, &cannoli_memhook_write_spl_ebp_end) },
                    unsafe { (&cannoli_memhook_write_spl_esi, &cannoli_memhook_write_spl_esi_end) },
                    unsafe { (&cannoli_memhook_write_spl_edi, &cannoli_memhook_write_spl_edi_end) },
                    unsafe { (&cannoli_memhook_write_spl_r8d, &cannoli_memhook_write_spl_r8d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r9d, &cannoli_memhook_write_spl_r9d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r10d, &cannoli_memhook_write_spl_r10d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r11d, &cannoli_memhook_write_spl_r11d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r12d, &cannoli_memhook_write_spl_r12d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r13d, &cannoli_memhook_write_spl_r13d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r14d, &cannoli_memhook_write_spl_r14d_end) },
                    unsafe { (&cannoli_memhook_write_spl_r15d, &cannoli_memhook_write_spl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bpl_eax, &cannoli_memhook_write_bpl_eax_end) },
                    unsafe { (&cannoli_memhook_write_bpl_ecx, &cannoli_memhook_write_bpl_ecx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_edx, &cannoli_memhook_write_bpl_edx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_ebx, &cannoli_memhook_write_bpl_ebx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_esp, &cannoli_memhook_write_bpl_esp_end) },
                    unsafe { (&cannoli_memhook_write_bpl_ebp, &cannoli_memhook_write_bpl_ebp_end) },
                    unsafe { (&cannoli_memhook_write_bpl_esi, &cannoli_memhook_write_bpl_esi_end) },
                    unsafe { (&cannoli_memhook_write_bpl_edi, &cannoli_memhook_write_bpl_edi_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r8d, &cannoli_memhook_write_bpl_r8d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r9d, &cannoli_memhook_write_bpl_r9d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r10d, &cannoli_memhook_write_bpl_r10d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r11d, &cannoli_memhook_write_bpl_r11d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r12d, &cannoli_memhook_write_bpl_r12d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r13d, &cannoli_memhook_write_bpl_r13d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r14d, &cannoli_memhook_write_bpl_r14d_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r15d, &cannoli_memhook_write_bpl_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_sil_eax, &cannoli_memhook_write_sil_eax_end) },
                    unsafe { (&cannoli_memhook_write_sil_ecx, &cannoli_memhook_write_sil_ecx_end) },
                    unsafe { (&cannoli_memhook_write_sil_edx, &cannoli_memhook_write_sil_edx_end) },
                    unsafe { (&cannoli_memhook_write_sil_ebx, &cannoli_memhook_write_sil_ebx_end) },
                    unsafe { (&cannoli_memhook_write_sil_esp, &cannoli_memhook_write_sil_esp_end) },
                    unsafe { (&cannoli_memhook_write_sil_ebp, &cannoli_memhook_write_sil_ebp_end) },
                    unsafe { (&cannoli_memhook_write_sil_esi, &cannoli_memhook_write_sil_esi_end) },
                    unsafe { (&cannoli_memhook_write_sil_edi, &cannoli_memhook_write_sil_edi_end) },
                    unsafe { (&cannoli_memhook_write_sil_r8d, &cannoli_memhook_write_sil_r8d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r9d, &cannoli_memhook_write_sil_r9d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r10d, &cannoli_memhook_write_sil_r10d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r11d, &cannoli_memhook_write_sil_r11d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r12d, &cannoli_memhook_write_sil_r12d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r13d, &cannoli_memhook_write_sil_r13d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r14d, &cannoli_memhook_write_sil_r14d_end) },
                    unsafe { (&cannoli_memhook_write_sil_r15d, &cannoli_memhook_write_sil_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dil_eax, &cannoli_memhook_write_dil_eax_end) },
                    unsafe { (&cannoli_memhook_write_dil_ecx, &cannoli_memhook_write_dil_ecx_end) },
                    unsafe { (&cannoli_memhook_write_dil_edx, &cannoli_memhook_write_dil_edx_end) },
                    unsafe { (&cannoli_memhook_write_dil_ebx, &cannoli_memhook_write_dil_ebx_end) },
                    unsafe { (&cannoli_memhook_write_dil_esp, &cannoli_memhook_write_dil_esp_end) },
                    unsafe { (&cannoli_memhook_write_dil_ebp, &cannoli_memhook_write_dil_ebp_end) },
                    unsafe { (&cannoli_memhook_write_dil_esi, &cannoli_memhook_write_dil_esi_end) },
                    unsafe { (&cannoli_memhook_write_dil_edi, &cannoli_memhook_write_dil_edi_end) },
                    unsafe { (&cannoli_memhook_write_dil_r8d, &cannoli_memhook_write_dil_r8d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r9d, &cannoli_memhook_write_dil_r9d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r10d, &cannoli_memhook_write_dil_r10d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r11d, &cannoli_memhook_write_dil_r11d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r12d, &cannoli_memhook_write_dil_r12d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r13d, &cannoli_memhook_write_dil_r13d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r14d, &cannoli_memhook_write_dil_r14d_end) },
                    unsafe { (&cannoli_memhook_write_dil_r15d, &cannoli_memhook_write_dil_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8b_eax, &cannoli_memhook_write_r8b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r8b_ecx, &cannoli_memhook_write_r8b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_edx, &cannoli_memhook_write_r8b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_ebx, &cannoli_memhook_write_r8b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_esp, &cannoli_memhook_write_r8b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r8b_ebp, &cannoli_memhook_write_r8b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r8b_esi, &cannoli_memhook_write_r8b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r8b_edi, &cannoli_memhook_write_r8b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r8d, &cannoli_memhook_write_r8b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r9d, &cannoli_memhook_write_r8b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r10d, &cannoli_memhook_write_r8b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r11d, &cannoli_memhook_write_r8b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r12d, &cannoli_memhook_write_r8b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r13d, &cannoli_memhook_write_r8b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r14d, &cannoli_memhook_write_r8b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r15d, &cannoli_memhook_write_r8b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9b_eax, &cannoli_memhook_write_r9b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r9b_ecx, &cannoli_memhook_write_r9b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_edx, &cannoli_memhook_write_r9b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_ebx, &cannoli_memhook_write_r9b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_esp, &cannoli_memhook_write_r9b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r9b_ebp, &cannoli_memhook_write_r9b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r9b_esi, &cannoli_memhook_write_r9b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r9b_edi, &cannoli_memhook_write_r9b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r8d, &cannoli_memhook_write_r9b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r9d, &cannoli_memhook_write_r9b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r10d, &cannoli_memhook_write_r9b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r11d, &cannoli_memhook_write_r9b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r12d, &cannoli_memhook_write_r9b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r13d, &cannoli_memhook_write_r9b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r14d, &cannoli_memhook_write_r9b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r15d, &cannoli_memhook_write_r9b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10b_eax, &cannoli_memhook_write_r10b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r10b_ecx, &cannoli_memhook_write_r10b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_edx, &cannoli_memhook_write_r10b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_ebx, &cannoli_memhook_write_r10b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_esp, &cannoli_memhook_write_r10b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r10b_ebp, &cannoli_memhook_write_r10b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r10b_esi, &cannoli_memhook_write_r10b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r10b_edi, &cannoli_memhook_write_r10b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r8d, &cannoli_memhook_write_r10b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r9d, &cannoli_memhook_write_r10b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r10d, &cannoli_memhook_write_r10b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r11d, &cannoli_memhook_write_r10b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r12d, &cannoli_memhook_write_r10b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r13d, &cannoli_memhook_write_r10b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r14d, &cannoli_memhook_write_r10b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r15d, &cannoli_memhook_write_r10b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11b_eax, &cannoli_memhook_write_r11b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r11b_ecx, &cannoli_memhook_write_r11b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_edx, &cannoli_memhook_write_r11b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_ebx, &cannoli_memhook_write_r11b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_esp, &cannoli_memhook_write_r11b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r11b_ebp, &cannoli_memhook_write_r11b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r11b_esi, &cannoli_memhook_write_r11b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r11b_edi, &cannoli_memhook_write_r11b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r8d, &cannoli_memhook_write_r11b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r9d, &cannoli_memhook_write_r11b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r10d, &cannoli_memhook_write_r11b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r11d, &cannoli_memhook_write_r11b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r12d, &cannoli_memhook_write_r11b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r13d, &cannoli_memhook_write_r11b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r14d, &cannoli_memhook_write_r11b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r15d, &cannoli_memhook_write_r11b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12b_eax, &cannoli_memhook_write_r12b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r12b_ecx, &cannoli_memhook_write_r12b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_edx, &cannoli_memhook_write_r12b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_ebx, &cannoli_memhook_write_r12b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_esp, &cannoli_memhook_write_r12b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r12b_ebp, &cannoli_memhook_write_r12b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r12b_esi, &cannoli_memhook_write_r12b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r12b_edi, &cannoli_memhook_write_r12b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r8d, &cannoli_memhook_write_r12b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r9d, &cannoli_memhook_write_r12b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r10d, &cannoli_memhook_write_r12b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r11d, &cannoli_memhook_write_r12b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r12d, &cannoli_memhook_write_r12b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r13d, &cannoli_memhook_write_r12b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r14d, &cannoli_memhook_write_r12b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r15d, &cannoli_memhook_write_r12b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13b_eax, &cannoli_memhook_write_r13b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r13b_ecx, &cannoli_memhook_write_r13b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_edx, &cannoli_memhook_write_r13b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_ebx, &cannoli_memhook_write_r13b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_esp, &cannoli_memhook_write_r13b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r13b_ebp, &cannoli_memhook_write_r13b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r13b_esi, &cannoli_memhook_write_r13b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r13b_edi, &cannoli_memhook_write_r13b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r8d, &cannoli_memhook_write_r13b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r9d, &cannoli_memhook_write_r13b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r10d, &cannoli_memhook_write_r13b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r11d, &cannoli_memhook_write_r13b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r12d, &cannoli_memhook_write_r13b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r13d, &cannoli_memhook_write_r13b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r14d, &cannoli_memhook_write_r13b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r15d, &cannoli_memhook_write_r13b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14b_eax, &cannoli_memhook_write_r14b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r14b_ecx, &cannoli_memhook_write_r14b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_edx, &cannoli_memhook_write_r14b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_ebx, &cannoli_memhook_write_r14b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_esp, &cannoli_memhook_write_r14b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r14b_ebp, &cannoli_memhook_write_r14b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r14b_esi, &cannoli_memhook_write_r14b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r14b_edi, &cannoli_memhook_write_r14b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r8d, &cannoli_memhook_write_r14b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r9d, &cannoli_memhook_write_r14b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r10d, &cannoli_memhook_write_r14b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r11d, &cannoli_memhook_write_r14b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r12d, &cannoli_memhook_write_r14b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r13d, &cannoli_memhook_write_r14b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r14d, &cannoli_memhook_write_r14b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r15d, &cannoli_memhook_write_r14b_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15b_eax, &cannoli_memhook_write_r15b_eax_end) },
                    unsafe { (&cannoli_memhook_write_r15b_ecx, &cannoli_memhook_write_r15b_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_edx, &cannoli_memhook_write_r15b_edx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_ebx, &cannoli_memhook_write_r15b_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_esp, &cannoli_memhook_write_r15b_esp_end) },
                    unsafe { (&cannoli_memhook_write_r15b_ebp, &cannoli_memhook_write_r15b_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r15b_esi, &cannoli_memhook_write_r15b_esi_end) },
                    unsafe { (&cannoli_memhook_write_r15b_edi, &cannoli_memhook_write_r15b_edi_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r8d, &cannoli_memhook_write_r15b_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r9d, &cannoli_memhook_write_r15b_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r10d, &cannoli_memhook_write_r15b_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r11d, &cannoli_memhook_write_r15b_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r12d, &cannoli_memhook_write_r15b_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r13d, &cannoli_memhook_write_r15b_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r14d, &cannoli_memhook_write_r15b_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r15d, &cannoli_memhook_write_r15b_r15d_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_ax_eax, &cannoli_memhook_read_ax_eax_end) },
                    unsafe { (&cannoli_memhook_read_ax_ecx, &cannoli_memhook_read_ax_ecx_end) },
                    unsafe { (&cannoli_memhook_read_ax_edx, &cannoli_memhook_read_ax_edx_end) },
                    unsafe { (&cannoli_memhook_read_ax_ebx, &cannoli_memhook_read_ax_ebx_end) },
                    unsafe { (&cannoli_memhook_read_ax_esp, &cannoli_memhook_read_ax_esp_end) },
                    unsafe { (&cannoli_memhook_read_ax_ebp, &cannoli_memhook_read_ax_ebp_end) },
                    unsafe { (&cannoli_memhook_read_ax_esi, &cannoli_memhook_read_ax_esi_end) },
                    unsafe { (&cannoli_memhook_read_ax_edi, &cannoli_memhook_read_ax_edi_end) },
                    unsafe { (&cannoli_memhook_read_ax_r8d, &cannoli_memhook_read_ax_r8d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r9d, &cannoli_memhook_read_ax_r9d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r10d, &cannoli_memhook_read_ax_r10d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r11d, &cannoli_memhook_read_ax_r11d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r12d, &cannoli_memhook_read_ax_r12d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r13d, &cannoli_memhook_read_ax_r13d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r14d, &cannoli_memhook_read_ax_r14d_end) },
                    unsafe { (&cannoli_memhook_read_ax_r15d, &cannoli_memhook_read_ax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_cx_eax, &cannoli_memhook_read_cx_eax_end) },
                    unsafe { (&cannoli_memhook_read_cx_ecx, &cannoli_memhook_read_cx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_cx_edx, &cannoli_memhook_read_cx_edx_end) },
                    unsafe { (&cannoli_memhook_read_cx_ebx, &cannoli_memhook_read_cx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_cx_esp, &cannoli_memhook_read_cx_esp_end) },
                    unsafe { (&cannoli_memhook_read_cx_ebp, &cannoli_memhook_read_cx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_cx_esi, &cannoli_memhook_read_cx_esi_end) },
                    unsafe { (&cannoli_memhook_read_cx_edi, &cannoli_memhook_read_cx_edi_end) },
                    unsafe { (&cannoli_memhook_read_cx_r8d, &cannoli_memhook_read_cx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r9d, &cannoli_memhook_read_cx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r10d, &cannoli_memhook_read_cx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r11d, &cannoli_memhook_read_cx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r12d, &cannoli_memhook_read_cx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r13d, &cannoli_memhook_read_cx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r14d, &cannoli_memhook_read_cx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_cx_r15d, &cannoli_memhook_read_cx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dx_eax, &cannoli_memhook_read_dx_eax_end) },
                    unsafe { (&cannoli_memhook_read_dx_ecx, &cannoli_memhook_read_dx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_dx_edx, &cannoli_memhook_read_dx_edx_end) },
                    unsafe { (&cannoli_memhook_read_dx_ebx, &cannoli_memhook_read_dx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_dx_esp, &cannoli_memhook_read_dx_esp_end) },
                    unsafe { (&cannoli_memhook_read_dx_ebp, &cannoli_memhook_read_dx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_dx_esi, &cannoli_memhook_read_dx_esi_end) },
                    unsafe { (&cannoli_memhook_read_dx_edi, &cannoli_memhook_read_dx_edi_end) },
                    unsafe { (&cannoli_memhook_read_dx_r8d, &cannoli_memhook_read_dx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r9d, &cannoli_memhook_read_dx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r10d, &cannoli_memhook_read_dx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r11d, &cannoli_memhook_read_dx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r12d, &cannoli_memhook_read_dx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r13d, &cannoli_memhook_read_dx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r14d, &cannoli_memhook_read_dx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_dx_r15d, &cannoli_memhook_read_dx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bx_eax, &cannoli_memhook_read_bx_eax_end) },
                    unsafe { (&cannoli_memhook_read_bx_ecx, &cannoli_memhook_read_bx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_bx_edx, &cannoli_memhook_read_bx_edx_end) },
                    unsafe { (&cannoli_memhook_read_bx_ebx, &cannoli_memhook_read_bx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_bx_esp, &cannoli_memhook_read_bx_esp_end) },
                    unsafe { (&cannoli_memhook_read_bx_ebp, &cannoli_memhook_read_bx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_bx_esi, &cannoli_memhook_read_bx_esi_end) },
                    unsafe { (&cannoli_memhook_read_bx_edi, &cannoli_memhook_read_bx_edi_end) },
                    unsafe { (&cannoli_memhook_read_bx_r8d, &cannoli_memhook_read_bx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r9d, &cannoli_memhook_read_bx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r10d, &cannoli_memhook_read_bx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r11d, &cannoli_memhook_read_bx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r12d, &cannoli_memhook_read_bx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r13d, &cannoli_memhook_read_bx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r14d, &cannoli_memhook_read_bx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_bx_r15d, &cannoli_memhook_read_bx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_sp_eax, &cannoli_memhook_read_sp_eax_end) },
                    unsafe { (&cannoli_memhook_read_sp_ecx, &cannoli_memhook_read_sp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_sp_edx, &cannoli_memhook_read_sp_edx_end) },
                    unsafe { (&cannoli_memhook_read_sp_ebx, &cannoli_memhook_read_sp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_sp_esp, &cannoli_memhook_read_sp_esp_end) },
                    unsafe { (&cannoli_memhook_read_sp_ebp, &cannoli_memhook_read_sp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_sp_esi, &cannoli_memhook_read_sp_esi_end) },
                    unsafe { (&cannoli_memhook_read_sp_edi, &cannoli_memhook_read_sp_edi_end) },
                    unsafe { (&cannoli_memhook_read_sp_r8d, &cannoli_memhook_read_sp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r9d, &cannoli_memhook_read_sp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r10d, &cannoli_memhook_read_sp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r11d, &cannoli_memhook_read_sp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r12d, &cannoli_memhook_read_sp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r13d, &cannoli_memhook_read_sp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r14d, &cannoli_memhook_read_sp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_sp_r15d, &cannoli_memhook_read_sp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bp_eax, &cannoli_memhook_read_bp_eax_end) },
                    unsafe { (&cannoli_memhook_read_bp_ecx, &cannoli_memhook_read_bp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_bp_edx, &cannoli_memhook_read_bp_edx_end) },
                    unsafe { (&cannoli_memhook_read_bp_ebx, &cannoli_memhook_read_bp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_bp_esp, &cannoli_memhook_read_bp_esp_end) },
                    unsafe { (&cannoli_memhook_read_bp_ebp, &cannoli_memhook_read_bp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_bp_esi, &cannoli_memhook_read_bp_esi_end) },
                    unsafe { (&cannoli_memhook_read_bp_edi, &cannoli_memhook_read_bp_edi_end) },
                    unsafe { (&cannoli_memhook_read_bp_r8d, &cannoli_memhook_read_bp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r9d, &cannoli_memhook_read_bp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r10d, &cannoli_memhook_read_bp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r11d, &cannoli_memhook_read_bp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r12d, &cannoli_memhook_read_bp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r13d, &cannoli_memhook_read_bp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r14d, &cannoli_memhook_read_bp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_bp_r15d, &cannoli_memhook_read_bp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_si_eax, &cannoli_memhook_read_si_eax_end) },
                    unsafe { (&cannoli_memhook_read_si_ecx, &cannoli_memhook_read_si_ecx_end) },
                    unsafe { (&cannoli_memhook_read_si_edx, &cannoli_memhook_read_si_edx_end) },
                    unsafe { (&cannoli_memhook_read_si_ebx, &cannoli_memhook_read_si_ebx_end) },
                    unsafe { (&cannoli_memhook_read_si_esp, &cannoli_memhook_read_si_esp_end) },
                    unsafe { (&cannoli_memhook_read_si_ebp, &cannoli_memhook_read_si_ebp_end) },
                    unsafe { (&cannoli_memhook_read_si_esi, &cannoli_memhook_read_si_esi_end) },
                    unsafe { (&cannoli_memhook_read_si_edi, &cannoli_memhook_read_si_edi_end) },
                    unsafe { (&cannoli_memhook_read_si_r8d, &cannoli_memhook_read_si_r8d_end) },
                    unsafe { (&cannoli_memhook_read_si_r9d, &cannoli_memhook_read_si_r9d_end) },
                    unsafe { (&cannoli_memhook_read_si_r10d, &cannoli_memhook_read_si_r10d_end) },
                    unsafe { (&cannoli_memhook_read_si_r11d, &cannoli_memhook_read_si_r11d_end) },
                    unsafe { (&cannoli_memhook_read_si_r12d, &cannoli_memhook_read_si_r12d_end) },
                    unsafe { (&cannoli_memhook_read_si_r13d, &cannoli_memhook_read_si_r13d_end) },
                    unsafe { (&cannoli_memhook_read_si_r14d, &cannoli_memhook_read_si_r14d_end) },
                    unsafe { (&cannoli_memhook_read_si_r15d, &cannoli_memhook_read_si_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_di_eax, &cannoli_memhook_read_di_eax_end) },
                    unsafe { (&cannoli_memhook_read_di_ecx, &cannoli_memhook_read_di_ecx_end) },
                    unsafe { (&cannoli_memhook_read_di_edx, &cannoli_memhook_read_di_edx_end) },
                    unsafe { (&cannoli_memhook_read_di_ebx, &cannoli_memhook_read_di_ebx_end) },
                    unsafe { (&cannoli_memhook_read_di_esp, &cannoli_memhook_read_di_esp_end) },
                    unsafe { (&cannoli_memhook_read_di_ebp, &cannoli_memhook_read_di_ebp_end) },
                    unsafe { (&cannoli_memhook_read_di_esi, &cannoli_memhook_read_di_esi_end) },
                    unsafe { (&cannoli_memhook_read_di_edi, &cannoli_memhook_read_di_edi_end) },
                    unsafe { (&cannoli_memhook_read_di_r8d, &cannoli_memhook_read_di_r8d_end) },
                    unsafe { (&cannoli_memhook_read_di_r9d, &cannoli_memhook_read_di_r9d_end) },
                    unsafe { (&cannoli_memhook_read_di_r10d, &cannoli_memhook_read_di_r10d_end) },
                    unsafe { (&cannoli_memhook_read_di_r11d, &cannoli_memhook_read_di_r11d_end) },
                    unsafe { (&cannoli_memhook_read_di_r12d, &cannoli_memhook_read_di_r12d_end) },
                    unsafe { (&cannoli_memhook_read_di_r13d, &cannoli_memhook_read_di_r13d_end) },
                    unsafe { (&cannoli_memhook_read_di_r14d, &cannoli_memhook_read_di_r14d_end) },
                    unsafe { (&cannoli_memhook_read_di_r15d, &cannoli_memhook_read_di_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8w_eax, &cannoli_memhook_read_r8w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r8w_ecx, &cannoli_memhook_read_r8w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_edx, &cannoli_memhook_read_r8w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_ebx, &cannoli_memhook_read_r8w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_esp, &cannoli_memhook_read_r8w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r8w_ebp, &cannoli_memhook_read_r8w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r8w_esi, &cannoli_memhook_read_r8w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r8w_edi, &cannoli_memhook_read_r8w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r8d, &cannoli_memhook_read_r8w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r9d, &cannoli_memhook_read_r8w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r10d, &cannoli_memhook_read_r8w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r11d, &cannoli_memhook_read_r8w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r12d, &cannoli_memhook_read_r8w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r13d, &cannoli_memhook_read_r8w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r14d, &cannoli_memhook_read_r8w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r15d, &cannoli_memhook_read_r8w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9w_eax, &cannoli_memhook_read_r9w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r9w_ecx, &cannoli_memhook_read_r9w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_edx, &cannoli_memhook_read_r9w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_ebx, &cannoli_memhook_read_r9w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_esp, &cannoli_memhook_read_r9w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r9w_ebp, &cannoli_memhook_read_r9w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r9w_esi, &cannoli_memhook_read_r9w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r9w_edi, &cannoli_memhook_read_r9w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r8d, &cannoli_memhook_read_r9w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r9d, &cannoli_memhook_read_r9w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r10d, &cannoli_memhook_read_r9w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r11d, &cannoli_memhook_read_r9w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r12d, &cannoli_memhook_read_r9w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r13d, &cannoli_memhook_read_r9w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r14d, &cannoli_memhook_read_r9w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r15d, &cannoli_memhook_read_r9w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10w_eax, &cannoli_memhook_read_r10w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r10w_ecx, &cannoli_memhook_read_r10w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_edx, &cannoli_memhook_read_r10w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_ebx, &cannoli_memhook_read_r10w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_esp, &cannoli_memhook_read_r10w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r10w_ebp, &cannoli_memhook_read_r10w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r10w_esi, &cannoli_memhook_read_r10w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r10w_edi, &cannoli_memhook_read_r10w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r8d, &cannoli_memhook_read_r10w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r9d, &cannoli_memhook_read_r10w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r10d, &cannoli_memhook_read_r10w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r11d, &cannoli_memhook_read_r10w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r12d, &cannoli_memhook_read_r10w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r13d, &cannoli_memhook_read_r10w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r14d, &cannoli_memhook_read_r10w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r15d, &cannoli_memhook_read_r10w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11w_eax, &cannoli_memhook_read_r11w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r11w_ecx, &cannoli_memhook_read_r11w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_edx, &cannoli_memhook_read_r11w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_ebx, &cannoli_memhook_read_r11w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_esp, &cannoli_memhook_read_r11w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r11w_ebp, &cannoli_memhook_read_r11w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r11w_esi, &cannoli_memhook_read_r11w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r11w_edi, &cannoli_memhook_read_r11w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r8d, &cannoli_memhook_read_r11w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r9d, &cannoli_memhook_read_r11w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r10d, &cannoli_memhook_read_r11w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r11d, &cannoli_memhook_read_r11w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r12d, &cannoli_memhook_read_r11w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r13d, &cannoli_memhook_read_r11w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r14d, &cannoli_memhook_read_r11w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r15d, &cannoli_memhook_read_r11w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12w_eax, &cannoli_memhook_read_r12w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r12w_ecx, &cannoli_memhook_read_r12w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_edx, &cannoli_memhook_read_r12w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_ebx, &cannoli_memhook_read_r12w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_esp, &cannoli_memhook_read_r12w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r12w_ebp, &cannoli_memhook_read_r12w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r12w_esi, &cannoli_memhook_read_r12w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r12w_edi, &cannoli_memhook_read_r12w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r8d, &cannoli_memhook_read_r12w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r9d, &cannoli_memhook_read_r12w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r10d, &cannoli_memhook_read_r12w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r11d, &cannoli_memhook_read_r12w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r12d, &cannoli_memhook_read_r12w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r13d, &cannoli_memhook_read_r12w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r14d, &cannoli_memhook_read_r12w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r15d, &cannoli_memhook_read_r12w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13w_eax, &cannoli_memhook_read_r13w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r13w_ecx, &cannoli_memhook_read_r13w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_edx, &cannoli_memhook_read_r13w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_ebx, &cannoli_memhook_read_r13w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_esp, &cannoli_memhook_read_r13w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r13w_ebp, &cannoli_memhook_read_r13w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r13w_esi, &cannoli_memhook_read_r13w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r13w_edi, &cannoli_memhook_read_r13w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r8d, &cannoli_memhook_read_r13w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r9d, &cannoli_memhook_read_r13w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r10d, &cannoli_memhook_read_r13w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r11d, &cannoli_memhook_read_r13w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r12d, &cannoli_memhook_read_r13w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r13d, &cannoli_memhook_read_r13w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r14d, &cannoli_memhook_read_r13w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r15d, &cannoli_memhook_read_r13w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14w_eax, &cannoli_memhook_read_r14w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r14w_ecx, &cannoli_memhook_read_r14w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_edx, &cannoli_memhook_read_r14w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_ebx, &cannoli_memhook_read_r14w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_esp, &cannoli_memhook_read_r14w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r14w_ebp, &cannoli_memhook_read_r14w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r14w_esi, &cannoli_memhook_read_r14w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r14w_edi, &cannoli_memhook_read_r14w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r8d, &cannoli_memhook_read_r14w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r9d, &cannoli_memhook_read_r14w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r10d, &cannoli_memhook_read_r14w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r11d, &cannoli_memhook_read_r14w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r12d, &cannoli_memhook_read_r14w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r13d, &cannoli_memhook_read_r14w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r14d, &cannoli_memhook_read_r14w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r15d, &cannoli_memhook_read_r14w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15w_eax, &cannoli_memhook_read_r15w_eax_end) },
                    unsafe { (&cannoli_memhook_read_r15w_ecx, &cannoli_memhook_read_r15w_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_edx, &cannoli_memhook_read_r15w_edx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_ebx, &cannoli_memhook_read_r15w_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_esp, &cannoli_memhook_read_r15w_esp_end) },
                    unsafe { (&cannoli_memhook_read_r15w_ebp, &cannoli_memhook_read_r15w_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r15w_esi, &cannoli_memhook_read_r15w_esi_end) },
                    unsafe { (&cannoli_memhook_read_r15w_edi, &cannoli_memhook_read_r15w_edi_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r8d, &cannoli_memhook_read_r15w_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r9d, &cannoli_memhook_read_r15w_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r10d, &cannoli_memhook_read_r15w_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r11d, &cannoli_memhook_read_r15w_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r12d, &cannoli_memhook_read_r15w_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r13d, &cannoli_memhook_read_r15w_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r14d, &cannoli_memhook_read_r15w_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r15d, &cannoli_memhook_read_r15w_r15d_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_ax_eax, &cannoli_memhook_write_ax_eax_end) },
                    unsafe { (&cannoli_memhook_write_ax_ecx, &cannoli_memhook_write_ax_ecx_end) },
                    unsafe { (&cannoli_memhook_write_ax_edx, &cannoli_memhook_write_ax_edx_end) },
                    unsafe { (&cannoli_memhook_write_ax_ebx, &cannoli_memhook_write_ax_ebx_end) },
                    unsafe { (&cannoli_memhook_write_ax_esp, &cannoli_memhook_write_ax_esp_end) },
                    unsafe { (&cannoli_memhook_write_ax_ebp, &cannoli_memhook_write_ax_ebp_end) },
                    unsafe { (&cannoli_memhook_write_ax_esi, &cannoli_memhook_write_ax_esi_end) },
                    unsafe { (&cannoli_memhook_write_ax_edi, &cannoli_memhook_write_ax_edi_end) },
                    unsafe { (&cannoli_memhook_write_ax_r8d, &cannoli_memhook_write_ax_r8d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r9d, &cannoli_memhook_write_ax_r9d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r10d, &cannoli_memhook_write_ax_r10d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r11d, &cannoli_memhook_write_ax_r11d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r12d, &cannoli_memhook_write_ax_r12d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r13d, &cannoli_memhook_write_ax_r13d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r14d, &cannoli_memhook_write_ax_r14d_end) },
                    unsafe { (&cannoli_memhook_write_ax_r15d, &cannoli_memhook_write_ax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_cx_eax, &cannoli_memhook_write_cx_eax_end) },
                    unsafe { (&cannoli_memhook_write_cx_ecx, &cannoli_memhook_write_cx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_cx_edx, &cannoli_memhook_write_cx_edx_end) },
                    unsafe { (&cannoli_memhook_write_cx_ebx, &cannoli_memhook_write_cx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_cx_esp, &cannoli_memhook_write_cx_esp_end) },
                    unsafe { (&cannoli_memhook_write_cx_ebp, &cannoli_memhook_write_cx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_cx_esi, &cannoli_memhook_write_cx_esi_end) },
                    unsafe { (&cannoli_memhook_write_cx_edi, &cannoli_memhook_write_cx_edi_end) },
                    unsafe { (&cannoli_memhook_write_cx_r8d, &cannoli_memhook_write_cx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r9d, &cannoli_memhook_write_cx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r10d, &cannoli_memhook_write_cx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r11d, &cannoli_memhook_write_cx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r12d, &cannoli_memhook_write_cx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r13d, &cannoli_memhook_write_cx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r14d, &cannoli_memhook_write_cx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_cx_r15d, &cannoli_memhook_write_cx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dx_eax, &cannoli_memhook_write_dx_eax_end) },
                    unsafe { (&cannoli_memhook_write_dx_ecx, &cannoli_memhook_write_dx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_dx_edx, &cannoli_memhook_write_dx_edx_end) },
                    unsafe { (&cannoli_memhook_write_dx_ebx, &cannoli_memhook_write_dx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_dx_esp, &cannoli_memhook_write_dx_esp_end) },
                    unsafe { (&cannoli_memhook_write_dx_ebp, &cannoli_memhook_write_dx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_dx_esi, &cannoli_memhook_write_dx_esi_end) },
                    unsafe { (&cannoli_memhook_write_dx_edi, &cannoli_memhook_write_dx_edi_end) },
                    unsafe { (&cannoli_memhook_write_dx_r8d, &cannoli_memhook_write_dx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r9d, &cannoli_memhook_write_dx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r10d, &cannoli_memhook_write_dx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r11d, &cannoli_memhook_write_dx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r12d, &cannoli_memhook_write_dx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r13d, &cannoli_memhook_write_dx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r14d, &cannoli_memhook_write_dx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_dx_r15d, &cannoli_memhook_write_dx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bx_eax, &cannoli_memhook_write_bx_eax_end) },
                    unsafe { (&cannoli_memhook_write_bx_ecx, &cannoli_memhook_write_bx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_bx_edx, &cannoli_memhook_write_bx_edx_end) },
                    unsafe { (&cannoli_memhook_write_bx_ebx, &cannoli_memhook_write_bx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_bx_esp, &cannoli_memhook_write_bx_esp_end) },
                    unsafe { (&cannoli_memhook_write_bx_ebp, &cannoli_memhook_write_bx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_bx_esi, &cannoli_memhook_write_bx_esi_end) },
                    unsafe { (&cannoli_memhook_write_bx_edi, &cannoli_memhook_write_bx_edi_end) },
                    unsafe { (&cannoli_memhook_write_bx_r8d, &cannoli_memhook_write_bx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r9d, &cannoli_memhook_write_bx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r10d, &cannoli_memhook_write_bx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r11d, &cannoli_memhook_write_bx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r12d, &cannoli_memhook_write_bx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r13d, &cannoli_memhook_write_bx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r14d, &cannoli_memhook_write_bx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_bx_r15d, &cannoli_memhook_write_bx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_sp_eax, &cannoli_memhook_write_sp_eax_end) },
                    unsafe { (&cannoli_memhook_write_sp_ecx, &cannoli_memhook_write_sp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_sp_edx, &cannoli_memhook_write_sp_edx_end) },
                    unsafe { (&cannoli_memhook_write_sp_ebx, &cannoli_memhook_write_sp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_sp_esp, &cannoli_memhook_write_sp_esp_end) },
                    unsafe { (&cannoli_memhook_write_sp_ebp, &cannoli_memhook_write_sp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_sp_esi, &cannoli_memhook_write_sp_esi_end) },
                    unsafe { (&cannoli_memhook_write_sp_edi, &cannoli_memhook_write_sp_edi_end) },
                    unsafe { (&cannoli_memhook_write_sp_r8d, &cannoli_memhook_write_sp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r9d, &cannoli_memhook_write_sp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r10d, &cannoli_memhook_write_sp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r11d, &cannoli_memhook_write_sp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r12d, &cannoli_memhook_write_sp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r13d, &cannoli_memhook_write_sp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r14d, &cannoli_memhook_write_sp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_sp_r15d, &cannoli_memhook_write_sp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bp_eax, &cannoli_memhook_write_bp_eax_end) },
                    unsafe { (&cannoli_memhook_write_bp_ecx, &cannoli_memhook_write_bp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_bp_edx, &cannoli_memhook_write_bp_edx_end) },
                    unsafe { (&cannoli_memhook_write_bp_ebx, &cannoli_memhook_write_bp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_bp_esp, &cannoli_memhook_write_bp_esp_end) },
                    unsafe { (&cannoli_memhook_write_bp_ebp, &cannoli_memhook_write_bp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_bp_esi, &cannoli_memhook_write_bp_esi_end) },
                    unsafe { (&cannoli_memhook_write_bp_edi, &cannoli_memhook_write_bp_edi_end) },
                    unsafe { (&cannoli_memhook_write_bp_r8d, &cannoli_memhook_write_bp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r9d, &cannoli_memhook_write_bp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r10d, &cannoli_memhook_write_bp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r11d, &cannoli_memhook_write_bp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r12d, &cannoli_memhook_write_bp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r13d, &cannoli_memhook_write_bp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r14d, &cannoli_memhook_write_bp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_bp_r15d, &cannoli_memhook_write_bp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_si_eax, &cannoli_memhook_write_si_eax_end) },
                    unsafe { (&cannoli_memhook_write_si_ecx, &cannoli_memhook_write_si_ecx_end) },
                    unsafe { (&cannoli_memhook_write_si_edx, &cannoli_memhook_write_si_edx_end) },
                    unsafe { (&cannoli_memhook_write_si_ebx, &cannoli_memhook_write_si_ebx_end) },
                    unsafe { (&cannoli_memhook_write_si_esp, &cannoli_memhook_write_si_esp_end) },
                    unsafe { (&cannoli_memhook_write_si_ebp, &cannoli_memhook_write_si_ebp_end) },
                    unsafe { (&cannoli_memhook_write_si_esi, &cannoli_memhook_write_si_esi_end) },
                    unsafe { (&cannoli_memhook_write_si_edi, &cannoli_memhook_write_si_edi_end) },
                    unsafe { (&cannoli_memhook_write_si_r8d, &cannoli_memhook_write_si_r8d_end) },
                    unsafe { (&cannoli_memhook_write_si_r9d, &cannoli_memhook_write_si_r9d_end) },
                    unsafe { (&cannoli_memhook_write_si_r10d, &cannoli_memhook_write_si_r10d_end) },
                    unsafe { (&cannoli_memhook_write_si_r11d, &cannoli_memhook_write_si_r11d_end) },
                    unsafe { (&cannoli_memhook_write_si_r12d, &cannoli_memhook_write_si_r12d_end) },
                    unsafe { (&cannoli_memhook_write_si_r13d, &cannoli_memhook_write_si_r13d_end) },
                    unsafe { (&cannoli_memhook_write_si_r14d, &cannoli_memhook_write_si_r14d_end) },
                    unsafe { (&cannoli_memhook_write_si_r15d, &cannoli_memhook_write_si_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_di_eax, &cannoli_memhook_write_di_eax_end) },
                    unsafe { (&cannoli_memhook_write_di_ecx, &cannoli_memhook_write_di_ecx_end) },
                    unsafe { (&cannoli_memhook_write_di_edx, &cannoli_memhook_write_di_edx_end) },
                    unsafe { (&cannoli_memhook_write_di_ebx, &cannoli_memhook_write_di_ebx_end) },
                    unsafe { (&cannoli_memhook_write_di_esp, &cannoli_memhook_write_di_esp_end) },
                    unsafe { (&cannoli_memhook_write_di_ebp, &cannoli_memhook_write_di_ebp_end) },
                    unsafe { (&cannoli_memhook_write_di_esi, &cannoli_memhook_write_di_esi_end) },
                    unsafe { (&cannoli_memhook_write_di_edi, &cannoli_memhook_write_di_edi_end) },
                    unsafe { (&cannoli_memhook_write_di_r8d, &cannoli_memhook_write_di_r8d_end) },
                    unsafe { (&cannoli_memhook_write_di_r9d, &cannoli_memhook_write_di_r9d_end) },
                    unsafe { (&cannoli_memhook_write_di_r10d, &cannoli_memhook_write_di_r10d_end) },
                    unsafe { (&cannoli_memhook_write_di_r11d, &cannoli_memhook_write_di_r11d_end) },
                    unsafe { (&cannoli_memhook_write_di_r12d, &cannoli_memhook_write_di_r12d_end) },
                    unsafe { (&cannoli_memhook_write_di_r13d, &cannoli_memhook_write_di_r13d_end) },
                    unsafe { (&cannoli_memhook_write_di_r14d, &cannoli_memhook_write_di_r14d_end) },
                    unsafe { (&cannoli_memhook_write_di_r15d, &cannoli_memhook_write_di_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8w_eax, &cannoli_memhook_write_r8w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r8w_ecx, &cannoli_memhook_write_r8w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_edx, &cannoli_memhook_write_r8w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_ebx, &cannoli_memhook_write_r8w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_esp, &cannoli_memhook_write_r8w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r8w_ebp, &cannoli_memhook_write_r8w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r8w_esi, &cannoli_memhook_write_r8w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r8w_edi, &cannoli_memhook_write_r8w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r8d, &cannoli_memhook_write_r8w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r9d, &cannoli_memhook_write_r8w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r10d, &cannoli_memhook_write_r8w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r11d, &cannoli_memhook_write_r8w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r12d, &cannoli_memhook_write_r8w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r13d, &cannoli_memhook_write_r8w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r14d, &cannoli_memhook_write_r8w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r15d, &cannoli_memhook_write_r8w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9w_eax, &cannoli_memhook_write_r9w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r9w_ecx, &cannoli_memhook_write_r9w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_edx, &cannoli_memhook_write_r9w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_ebx, &cannoli_memhook_write_r9w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_esp, &cannoli_memhook_write_r9w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r9w_ebp, &cannoli_memhook_write_r9w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r9w_esi, &cannoli_memhook_write_r9w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r9w_edi, &cannoli_memhook_write_r9w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r8d, &cannoli_memhook_write_r9w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r9d, &cannoli_memhook_write_r9w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r10d, &cannoli_memhook_write_r9w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r11d, &cannoli_memhook_write_r9w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r12d, &cannoli_memhook_write_r9w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r13d, &cannoli_memhook_write_r9w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r14d, &cannoli_memhook_write_r9w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r15d, &cannoli_memhook_write_r9w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10w_eax, &cannoli_memhook_write_r10w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r10w_ecx, &cannoli_memhook_write_r10w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_edx, &cannoli_memhook_write_r10w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_ebx, &cannoli_memhook_write_r10w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_esp, &cannoli_memhook_write_r10w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r10w_ebp, &cannoli_memhook_write_r10w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r10w_esi, &cannoli_memhook_write_r10w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r10w_edi, &cannoli_memhook_write_r10w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r8d, &cannoli_memhook_write_r10w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r9d, &cannoli_memhook_write_r10w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r10d, &cannoli_memhook_write_r10w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r11d, &cannoli_memhook_write_r10w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r12d, &cannoli_memhook_write_r10w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r13d, &cannoli_memhook_write_r10w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r14d, &cannoli_memhook_write_r10w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r15d, &cannoli_memhook_write_r10w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11w_eax, &cannoli_memhook_write_r11w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r11w_ecx, &cannoli_memhook_write_r11w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_edx, &cannoli_memhook_write_r11w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_ebx, &cannoli_memhook_write_r11w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_esp, &cannoli_memhook_write_r11w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r11w_ebp, &cannoli_memhook_write_r11w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r11w_esi, &cannoli_memhook_write_r11w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r11w_edi, &cannoli_memhook_write_r11w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r8d, &cannoli_memhook_write_r11w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r9d, &cannoli_memhook_write_r11w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r10d, &cannoli_memhook_write_r11w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r11d, &cannoli_memhook_write_r11w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r12d, &cannoli_memhook_write_r11w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r13d, &cannoli_memhook_write_r11w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r14d, &cannoli_memhook_write_r11w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r15d, &cannoli_memhook_write_r11w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12w_eax, &cannoli_memhook_write_r12w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r12w_ecx, &cannoli_memhook_write_r12w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_edx, &cannoli_memhook_write_r12w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_ebx, &cannoli_memhook_write_r12w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_esp, &cannoli_memhook_write_r12w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r12w_ebp, &cannoli_memhook_write_r12w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r12w_esi, &cannoli_memhook_write_r12w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r12w_edi, &cannoli_memhook_write_r12w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r8d, &cannoli_memhook_write_r12w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r9d, &cannoli_memhook_write_r12w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r10d, &cannoli_memhook_write_r12w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r11d, &cannoli_memhook_write_r12w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r12d, &cannoli_memhook_write_r12w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r13d, &cannoli_memhook_write_r12w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r14d, &cannoli_memhook_write_r12w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r15d, &cannoli_memhook_write_r12w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13w_eax, &cannoli_memhook_write_r13w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r13w_ecx, &cannoli_memhook_write_r13w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_edx, &cannoli_memhook_write_r13w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_ebx, &cannoli_memhook_write_r13w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_esp, &cannoli_memhook_write_r13w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r13w_ebp, &cannoli_memhook_write_r13w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r13w_esi, &cannoli_memhook_write_r13w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r13w_edi, &cannoli_memhook_write_r13w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r8d, &cannoli_memhook_write_r13w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r9d, &cannoli_memhook_write_r13w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r10d, &cannoli_memhook_write_r13w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r11d, &cannoli_memhook_write_r13w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r12d, &cannoli_memhook_write_r13w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r13d, &cannoli_memhook_write_r13w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r14d, &cannoli_memhook_write_r13w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r15d, &cannoli_memhook_write_r13w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14w_eax, &cannoli_memhook_write_r14w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r14w_ecx, &cannoli_memhook_write_r14w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_edx, &cannoli_memhook_write_r14w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_ebx, &cannoli_memhook_write_r14w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_esp, &cannoli_memhook_write_r14w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r14w_ebp, &cannoli_memhook_write_r14w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r14w_esi, &cannoli_memhook_write_r14w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r14w_edi, &cannoli_memhook_write_r14w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r8d, &cannoli_memhook_write_r14w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r9d, &cannoli_memhook_write_r14w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r10d, &cannoli_memhook_write_r14w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r11d, &cannoli_memhook_write_r14w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r12d, &cannoli_memhook_write_r14w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r13d, &cannoli_memhook_write_r14w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r14d, &cannoli_memhook_write_r14w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r15d, &cannoli_memhook_write_r14w_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15w_eax, &cannoli_memhook_write_r15w_eax_end) },
                    unsafe { (&cannoli_memhook_write_r15w_ecx, &cannoli_memhook_write_r15w_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_edx, &cannoli_memhook_write_r15w_edx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_ebx, &cannoli_memhook_write_r15w_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_esp, &cannoli_memhook_write_r15w_esp_end) },
                    unsafe { (&cannoli_memhook_write_r15w_ebp, &cannoli_memhook_write_r15w_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r15w_esi, &cannoli_memhook_write_r15w_esi_end) },
                    unsafe { (&cannoli_memhook_write_r15w_edi, &cannoli_memhook_write_r15w_edi_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r8d, &cannoli_memhook_write_r15w_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r9d, &cannoli_memhook_write_r15w_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r10d, &cannoli_memhook_write_r15w_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r11d, &cannoli_memhook_write_r15w_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r12d, &cannoli_memhook_write_r15w_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r13d, &cannoli_memhook_write_r15w_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r14d, &cannoli_memhook_write_r15w_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r15d, &cannoli_memhook_write_r15w_r15d_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_eax_eax, &cannoli_memhook_read_eax_eax_end) },
                    unsafe { (&cannoli_memhook_read_eax_ecx, &cannoli_memhook_read_eax_ecx_end) },
                    unsafe { (&cannoli_memhook_read_eax_edx, &cannoli_memhook_read_eax_edx_end) },
                    unsafe { (&cannoli_memhook_read_eax_ebx, &cannoli_memhook_read_eax_ebx_end) },
                    unsafe { (&cannoli_memhook_read_eax_esp, &cannoli_memhook_read_eax_esp_end) },
                    unsafe { (&cannoli_memhook_read_eax_ebp, &cannoli_memhook_read_eax_ebp_end) },
                    unsafe { (&cannoli_memhook_read_eax_esi, &cannoli_memhook_read_eax_esi_end) },
                    unsafe { (&cannoli_memhook_read_eax_edi, &cannoli_memhook_read_eax_edi_end) },
                    unsafe { (&cannoli_memhook_read_eax_r8d, &cannoli_memhook_read_eax_r8d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r9d, &cannoli_memhook_read_eax_r9d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r10d, &cannoli_memhook_read_eax_r10d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r11d, &cannoli_memhook_read_eax_r11d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r12d, &cannoli_memhook_read_eax_r12d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r13d, &cannoli_memhook_read_eax_r13d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r14d, &cannoli_memhook_read_eax_r14d_end) },
                    unsafe { (&cannoli_memhook_read_eax_r15d, &cannoli_memhook_read_eax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ecx_eax, &cannoli_memhook_read_ecx_eax_end) },
                    unsafe { (&cannoli_memhook_read_ecx_ecx, &cannoli_memhook_read_ecx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_edx, &cannoli_memhook_read_ecx_edx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_ebx, &cannoli_memhook_read_ecx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_esp, &cannoli_memhook_read_ecx_esp_end) },
                    unsafe { (&cannoli_memhook_read_ecx_ebp, &cannoli_memhook_read_ecx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_ecx_esi, &cannoli_memhook_read_ecx_esi_end) },
                    unsafe { (&cannoli_memhook_read_ecx_edi, &cannoli_memhook_read_ecx_edi_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r8d, &cannoli_memhook_read_ecx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r9d, &cannoli_memhook_read_ecx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r10d, &cannoli_memhook_read_ecx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r11d, &cannoli_memhook_read_ecx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r12d, &cannoli_memhook_read_ecx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r13d, &cannoli_memhook_read_ecx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r14d, &cannoli_memhook_read_ecx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r15d, &cannoli_memhook_read_ecx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_edx_eax, &cannoli_memhook_read_edx_eax_end) },
                    unsafe { (&cannoli_memhook_read_edx_ecx, &cannoli_memhook_read_edx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_edx_edx, &cannoli_memhook_read_edx_edx_end) },
                    unsafe { (&cannoli_memhook_read_edx_ebx, &cannoli_memhook_read_edx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_edx_esp, &cannoli_memhook_read_edx_esp_end) },
                    unsafe { (&cannoli_memhook_read_edx_ebp, &cannoli_memhook_read_edx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_edx_esi, &cannoli_memhook_read_edx_esi_end) },
                    unsafe { (&cannoli_memhook_read_edx_edi, &cannoli_memhook_read_edx_edi_end) },
                    unsafe { (&cannoli_memhook_read_edx_r8d, &cannoli_memhook_read_edx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r9d, &cannoli_memhook_read_edx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r10d, &cannoli_memhook_read_edx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r11d, &cannoli_memhook_read_edx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r12d, &cannoli_memhook_read_edx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r13d, &cannoli_memhook_read_edx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r14d, &cannoli_memhook_read_edx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_edx_r15d, &cannoli_memhook_read_edx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ebx_eax, &cannoli_memhook_read_ebx_eax_end) },
                    unsafe { (&cannoli_memhook_read_ebx_ecx, &cannoli_memhook_read_ebx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_edx, &cannoli_memhook_read_ebx_edx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_ebx, &cannoli_memhook_read_ebx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_esp, &cannoli_memhook_read_ebx_esp_end) },
                    unsafe { (&cannoli_memhook_read_ebx_ebp, &cannoli_memhook_read_ebx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_ebx_esi, &cannoli_memhook_read_ebx_esi_end) },
                    unsafe { (&cannoli_memhook_read_ebx_edi, &cannoli_memhook_read_ebx_edi_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r8d, &cannoli_memhook_read_ebx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r9d, &cannoli_memhook_read_ebx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r10d, &cannoli_memhook_read_ebx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r11d, &cannoli_memhook_read_ebx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r12d, &cannoli_memhook_read_ebx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r13d, &cannoli_memhook_read_ebx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r14d, &cannoli_memhook_read_ebx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r15d, &cannoli_memhook_read_ebx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_esp_eax, &cannoli_memhook_read_esp_eax_end) },
                    unsafe { (&cannoli_memhook_read_esp_ecx, &cannoli_memhook_read_esp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_esp_edx, &cannoli_memhook_read_esp_edx_end) },
                    unsafe { (&cannoli_memhook_read_esp_ebx, &cannoli_memhook_read_esp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_esp_esp, &cannoli_memhook_read_esp_esp_end) },
                    unsafe { (&cannoli_memhook_read_esp_ebp, &cannoli_memhook_read_esp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_esp_esi, &cannoli_memhook_read_esp_esi_end) },
                    unsafe { (&cannoli_memhook_read_esp_edi, &cannoli_memhook_read_esp_edi_end) },
                    unsafe { (&cannoli_memhook_read_esp_r8d, &cannoli_memhook_read_esp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r9d, &cannoli_memhook_read_esp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r10d, &cannoli_memhook_read_esp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r11d, &cannoli_memhook_read_esp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r12d, &cannoli_memhook_read_esp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r13d, &cannoli_memhook_read_esp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r14d, &cannoli_memhook_read_esp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_esp_r15d, &cannoli_memhook_read_esp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ebp_eax, &cannoli_memhook_read_ebp_eax_end) },
                    unsafe { (&cannoli_memhook_read_ebp_ecx, &cannoli_memhook_read_ebp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_edx, &cannoli_memhook_read_ebp_edx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_ebx, &cannoli_memhook_read_ebp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_esp, &cannoli_memhook_read_ebp_esp_end) },
                    unsafe { (&cannoli_memhook_read_ebp_ebp, &cannoli_memhook_read_ebp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_ebp_esi, &cannoli_memhook_read_ebp_esi_end) },
                    unsafe { (&cannoli_memhook_read_ebp_edi, &cannoli_memhook_read_ebp_edi_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r8d, &cannoli_memhook_read_ebp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r9d, &cannoli_memhook_read_ebp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r10d, &cannoli_memhook_read_ebp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r11d, &cannoli_memhook_read_ebp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r12d, &cannoli_memhook_read_ebp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r13d, &cannoli_memhook_read_ebp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r14d, &cannoli_memhook_read_ebp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r15d, &cannoli_memhook_read_ebp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_esi_eax, &cannoli_memhook_read_esi_eax_end) },
                    unsafe { (&cannoli_memhook_read_esi_ecx, &cannoli_memhook_read_esi_ecx_end) },
                    unsafe { (&cannoli_memhook_read_esi_edx, &cannoli_memhook_read_esi_edx_end) },
                    unsafe { (&cannoli_memhook_read_esi_ebx, &cannoli_memhook_read_esi_ebx_end) },
                    unsafe { (&cannoli_memhook_read_esi_esp, &cannoli_memhook_read_esi_esp_end) },
                    unsafe { (&cannoli_memhook_read_esi_ebp, &cannoli_memhook_read_esi_ebp_end) },
                    unsafe { (&cannoli_memhook_read_esi_esi, &cannoli_memhook_read_esi_esi_end) },
                    unsafe { (&cannoli_memhook_read_esi_edi, &cannoli_memhook_read_esi_edi_end) },
                    unsafe { (&cannoli_memhook_read_esi_r8d, &cannoli_memhook_read_esi_r8d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r9d, &cannoli_memhook_read_esi_r9d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r10d, &cannoli_memhook_read_esi_r10d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r11d, &cannoli_memhook_read_esi_r11d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r12d, &cannoli_memhook_read_esi_r12d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r13d, &cannoli_memhook_read_esi_r13d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r14d, &cannoli_memhook_read_esi_r14d_end) },
                    unsafe { (&cannoli_memhook_read_esi_r15d, &cannoli_memhook_read_esi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_edi_eax, &cannoli_memhook_read_edi_eax_end) },
                    unsafe { (&cannoli_memhook_read_edi_ecx, &cannoli_memhook_read_edi_ecx_end) },
                    unsafe { (&cannoli_memhook_read_edi_edx, &cannoli_memhook_read_edi_edx_end) },
                    unsafe { (&cannoli_memhook_read_edi_ebx, &cannoli_memhook_read_edi_ebx_end) },
                    unsafe { (&cannoli_memhook_read_edi_esp, &cannoli_memhook_read_edi_esp_end) },
                    unsafe { (&cannoli_memhook_read_edi_ebp, &cannoli_memhook_read_edi_ebp_end) },
                    unsafe { (&cannoli_memhook_read_edi_esi, &cannoli_memhook_read_edi_esi_end) },
                    unsafe { (&cannoli_memhook_read_edi_edi, &cannoli_memhook_read_edi_edi_end) },
                    unsafe { (&cannoli_memhook_read_edi_r8d, &cannoli_memhook_read_edi_r8d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r9d, &cannoli_memhook_read_edi_r9d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r10d, &cannoli_memhook_read_edi_r10d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r11d, &cannoli_memhook_read_edi_r11d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r12d, &cannoli_memhook_read_edi_r12d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r13d, &cannoli_memhook_read_edi_r13d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r14d, &cannoli_memhook_read_edi_r14d_end) },
                    unsafe { (&cannoli_memhook_read_edi_r15d, &cannoli_memhook_read_edi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8d_eax, &cannoli_memhook_read_r8d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r8d_ecx, &cannoli_memhook_read_r8d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_edx, &cannoli_memhook_read_r8d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_ebx, &cannoli_memhook_read_r8d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_esp, &cannoli_memhook_read_r8d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r8d_ebp, &cannoli_memhook_read_r8d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r8d_esi, &cannoli_memhook_read_r8d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r8d_edi, &cannoli_memhook_read_r8d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r8d, &cannoli_memhook_read_r8d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r9d, &cannoli_memhook_read_r8d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r10d, &cannoli_memhook_read_r8d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r11d, &cannoli_memhook_read_r8d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r12d, &cannoli_memhook_read_r8d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r13d, &cannoli_memhook_read_r8d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r14d, &cannoli_memhook_read_r8d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r15d, &cannoli_memhook_read_r8d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9d_eax, &cannoli_memhook_read_r9d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r9d_ecx, &cannoli_memhook_read_r9d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_edx, &cannoli_memhook_read_r9d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_ebx, &cannoli_memhook_read_r9d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_esp, &cannoli_memhook_read_r9d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r9d_ebp, &cannoli_memhook_read_r9d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r9d_esi, &cannoli_memhook_read_r9d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r9d_edi, &cannoli_memhook_read_r9d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r8d, &cannoli_memhook_read_r9d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r9d, &cannoli_memhook_read_r9d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r10d, &cannoli_memhook_read_r9d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r11d, &cannoli_memhook_read_r9d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r12d, &cannoli_memhook_read_r9d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r13d, &cannoli_memhook_read_r9d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r14d, &cannoli_memhook_read_r9d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r15d, &cannoli_memhook_read_r9d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10d_eax, &cannoli_memhook_read_r10d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r10d_ecx, &cannoli_memhook_read_r10d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_edx, &cannoli_memhook_read_r10d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_ebx, &cannoli_memhook_read_r10d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_esp, &cannoli_memhook_read_r10d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r10d_ebp, &cannoli_memhook_read_r10d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r10d_esi, &cannoli_memhook_read_r10d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r10d_edi, &cannoli_memhook_read_r10d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r8d, &cannoli_memhook_read_r10d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r9d, &cannoli_memhook_read_r10d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r10d, &cannoli_memhook_read_r10d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r11d, &cannoli_memhook_read_r10d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r12d, &cannoli_memhook_read_r10d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r13d, &cannoli_memhook_read_r10d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r14d, &cannoli_memhook_read_r10d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r15d, &cannoli_memhook_read_r10d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11d_eax, &cannoli_memhook_read_r11d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r11d_ecx, &cannoli_memhook_read_r11d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_edx, &cannoli_memhook_read_r11d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_ebx, &cannoli_memhook_read_r11d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_esp, &cannoli_memhook_read_r11d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r11d_ebp, &cannoli_memhook_read_r11d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r11d_esi, &cannoli_memhook_read_r11d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r11d_edi, &cannoli_memhook_read_r11d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r8d, &cannoli_memhook_read_r11d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r9d, &cannoli_memhook_read_r11d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r10d, &cannoli_memhook_read_r11d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r11d, &cannoli_memhook_read_r11d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r12d, &cannoli_memhook_read_r11d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r13d, &cannoli_memhook_read_r11d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r14d, &cannoli_memhook_read_r11d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r15d, &cannoli_memhook_read_r11d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12d_eax, &cannoli_memhook_read_r12d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r12d_ecx, &cannoli_memhook_read_r12d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_edx, &cannoli_memhook_read_r12d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_ebx, &cannoli_memhook_read_r12d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_esp, &cannoli_memhook_read_r12d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r12d_ebp, &cannoli_memhook_read_r12d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r12d_esi, &cannoli_memhook_read_r12d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r12d_edi, &cannoli_memhook_read_r12d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r8d, &cannoli_memhook_read_r12d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r9d, &cannoli_memhook_read_r12d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r10d, &cannoli_memhook_read_r12d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r11d, &cannoli_memhook_read_r12d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r12d, &cannoli_memhook_read_r12d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r13d, &cannoli_memhook_read_r12d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r14d, &cannoli_memhook_read_r12d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r15d, &cannoli_memhook_read_r12d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13d_eax, &cannoli_memhook_read_r13d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r13d_ecx, &cannoli_memhook_read_r13d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_edx, &cannoli_memhook_read_r13d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_ebx, &cannoli_memhook_read_r13d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_esp, &cannoli_memhook_read_r13d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r13d_ebp, &cannoli_memhook_read_r13d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r13d_esi, &cannoli_memhook_read_r13d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r13d_edi, &cannoli_memhook_read_r13d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r8d, &cannoli_memhook_read_r13d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r9d, &cannoli_memhook_read_r13d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r10d, &cannoli_memhook_read_r13d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r11d, &cannoli_memhook_read_r13d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r12d, &cannoli_memhook_read_r13d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r13d, &cannoli_memhook_read_r13d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r14d, &cannoli_memhook_read_r13d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r15d, &cannoli_memhook_read_r13d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14d_eax, &cannoli_memhook_read_r14d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r14d_ecx, &cannoli_memhook_read_r14d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_edx, &cannoli_memhook_read_r14d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_ebx, &cannoli_memhook_read_r14d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_esp, &cannoli_memhook_read_r14d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r14d_ebp, &cannoli_memhook_read_r14d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r14d_esi, &cannoli_memhook_read_r14d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r14d_edi, &cannoli_memhook_read_r14d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r8d, &cannoli_memhook_read_r14d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r9d, &cannoli_memhook_read_r14d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r10d, &cannoli_memhook_read_r14d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r11d, &cannoli_memhook_read_r14d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r12d, &cannoli_memhook_read_r14d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r13d, &cannoli_memhook_read_r14d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r14d, &cannoli_memhook_read_r14d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r15d, &cannoli_memhook_read_r14d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15d_eax, &cannoli_memhook_read_r15d_eax_end) },
                    unsafe { (&cannoli_memhook_read_r15d_ecx, &cannoli_memhook_read_r15d_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_edx, &cannoli_memhook_read_r15d_edx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_ebx, &cannoli_memhook_read_r15d_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_esp, &cannoli_memhook_read_r15d_esp_end) },
                    unsafe { (&cannoli_memhook_read_r15d_ebp, &cannoli_memhook_read_r15d_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r15d_esi, &cannoli_memhook_read_r15d_esi_end) },
                    unsafe { (&cannoli_memhook_read_r15d_edi, &cannoli_memhook_read_r15d_edi_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r8d, &cannoli_memhook_read_r15d_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r9d, &cannoli_memhook_read_r15d_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r10d, &cannoli_memhook_read_r15d_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r11d, &cannoli_memhook_read_r15d_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r12d, &cannoli_memhook_read_r15d_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r13d, &cannoli_memhook_read_r15d_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r14d, &cannoli_memhook_read_r15d_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r15d, &cannoli_memhook_read_r15d_r15d_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_eax_eax, &cannoli_memhook_write_eax_eax_end) },
                    unsafe { (&cannoli_memhook_write_eax_ecx, &cannoli_memhook_write_eax_ecx_end) },
                    unsafe { (&cannoli_memhook_write_eax_edx, &cannoli_memhook_write_eax_edx_end) },
                    unsafe { (&cannoli_memhook_write_eax_ebx, &cannoli_memhook_write_eax_ebx_end) },
                    unsafe { (&cannoli_memhook_write_eax_esp, &cannoli_memhook_write_eax_esp_end) },
                    unsafe { (&cannoli_memhook_write_eax_ebp, &cannoli_memhook_write_eax_ebp_end) },
                    unsafe { (&cannoli_memhook_write_eax_esi, &cannoli_memhook_write_eax_esi_end) },
                    unsafe { (&cannoli_memhook_write_eax_edi, &cannoli_memhook_write_eax_edi_end) },
                    unsafe { (&cannoli_memhook_write_eax_r8d, &cannoli_memhook_write_eax_r8d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r9d, &cannoli_memhook_write_eax_r9d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r10d, &cannoli_memhook_write_eax_r10d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r11d, &cannoli_memhook_write_eax_r11d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r12d, &cannoli_memhook_write_eax_r12d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r13d, &cannoli_memhook_write_eax_r13d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r14d, &cannoli_memhook_write_eax_r14d_end) },
                    unsafe { (&cannoli_memhook_write_eax_r15d, &cannoli_memhook_write_eax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ecx_eax, &cannoli_memhook_write_ecx_eax_end) },
                    unsafe { (&cannoli_memhook_write_ecx_ecx, &cannoli_memhook_write_ecx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_edx, &cannoli_memhook_write_ecx_edx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_ebx, &cannoli_memhook_write_ecx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_esp, &cannoli_memhook_write_ecx_esp_end) },
                    unsafe { (&cannoli_memhook_write_ecx_ebp, &cannoli_memhook_write_ecx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_ecx_esi, &cannoli_memhook_write_ecx_esi_end) },
                    unsafe { (&cannoli_memhook_write_ecx_edi, &cannoli_memhook_write_ecx_edi_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r8d, &cannoli_memhook_write_ecx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r9d, &cannoli_memhook_write_ecx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r10d, &cannoli_memhook_write_ecx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r11d, &cannoli_memhook_write_ecx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r12d, &cannoli_memhook_write_ecx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r13d, &cannoli_memhook_write_ecx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r14d, &cannoli_memhook_write_ecx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r15d, &cannoli_memhook_write_ecx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_edx_eax, &cannoli_memhook_write_edx_eax_end) },
                    unsafe { (&cannoli_memhook_write_edx_ecx, &cannoli_memhook_write_edx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_edx_edx, &cannoli_memhook_write_edx_edx_end) },
                    unsafe { (&cannoli_memhook_write_edx_ebx, &cannoli_memhook_write_edx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_edx_esp, &cannoli_memhook_write_edx_esp_end) },
                    unsafe { (&cannoli_memhook_write_edx_ebp, &cannoli_memhook_write_edx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_edx_esi, &cannoli_memhook_write_edx_esi_end) },
                    unsafe { (&cannoli_memhook_write_edx_edi, &cannoli_memhook_write_edx_edi_end) },
                    unsafe { (&cannoli_memhook_write_edx_r8d, &cannoli_memhook_write_edx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r9d, &cannoli_memhook_write_edx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r10d, &cannoli_memhook_write_edx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r11d, &cannoli_memhook_write_edx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r12d, &cannoli_memhook_write_edx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r13d, &cannoli_memhook_write_edx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r14d, &cannoli_memhook_write_edx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_edx_r15d, &cannoli_memhook_write_edx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ebx_eax, &cannoli_memhook_write_ebx_eax_end) },
                    unsafe { (&cannoli_memhook_write_ebx_ecx, &cannoli_memhook_write_ebx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_edx, &cannoli_memhook_write_ebx_edx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_ebx, &cannoli_memhook_write_ebx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_esp, &cannoli_memhook_write_ebx_esp_end) },
                    unsafe { (&cannoli_memhook_write_ebx_ebp, &cannoli_memhook_write_ebx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_ebx_esi, &cannoli_memhook_write_ebx_esi_end) },
                    unsafe { (&cannoli_memhook_write_ebx_edi, &cannoli_memhook_write_ebx_edi_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r8d, &cannoli_memhook_write_ebx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r9d, &cannoli_memhook_write_ebx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r10d, &cannoli_memhook_write_ebx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r11d, &cannoli_memhook_write_ebx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r12d, &cannoli_memhook_write_ebx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r13d, &cannoli_memhook_write_ebx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r14d, &cannoli_memhook_write_ebx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r15d, &cannoli_memhook_write_ebx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_esp_eax, &cannoli_memhook_write_esp_eax_end) },
                    unsafe { (&cannoli_memhook_write_esp_ecx, &cannoli_memhook_write_esp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_esp_edx, &cannoli_memhook_write_esp_edx_end) },
                    unsafe { (&cannoli_memhook_write_esp_ebx, &cannoli_memhook_write_esp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_esp_esp, &cannoli_memhook_write_esp_esp_end) },
                    unsafe { (&cannoli_memhook_write_esp_ebp, &cannoli_memhook_write_esp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_esp_esi, &cannoli_memhook_write_esp_esi_end) },
                    unsafe { (&cannoli_memhook_write_esp_edi, &cannoli_memhook_write_esp_edi_end) },
                    unsafe { (&cannoli_memhook_write_esp_r8d, &cannoli_memhook_write_esp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r9d, &cannoli_memhook_write_esp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r10d, &cannoli_memhook_write_esp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r11d, &cannoli_memhook_write_esp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r12d, &cannoli_memhook_write_esp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r13d, &cannoli_memhook_write_esp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r14d, &cannoli_memhook_write_esp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_esp_r15d, &cannoli_memhook_write_esp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ebp_eax, &cannoli_memhook_write_ebp_eax_end) },
                    unsafe { (&cannoli_memhook_write_ebp_ecx, &cannoli_memhook_write_ebp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_edx, &cannoli_memhook_write_ebp_edx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_ebx, &cannoli_memhook_write_ebp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_esp, &cannoli_memhook_write_ebp_esp_end) },
                    unsafe { (&cannoli_memhook_write_ebp_ebp, &cannoli_memhook_write_ebp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_ebp_esi, &cannoli_memhook_write_ebp_esi_end) },
                    unsafe { (&cannoli_memhook_write_ebp_edi, &cannoli_memhook_write_ebp_edi_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r8d, &cannoli_memhook_write_ebp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r9d, &cannoli_memhook_write_ebp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r10d, &cannoli_memhook_write_ebp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r11d, &cannoli_memhook_write_ebp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r12d, &cannoli_memhook_write_ebp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r13d, &cannoli_memhook_write_ebp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r14d, &cannoli_memhook_write_ebp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r15d, &cannoli_memhook_write_ebp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_esi_eax, &cannoli_memhook_write_esi_eax_end) },
                    unsafe { (&cannoli_memhook_write_esi_ecx, &cannoli_memhook_write_esi_ecx_end) },
                    unsafe { (&cannoli_memhook_write_esi_edx, &cannoli_memhook_write_esi_edx_end) },
                    unsafe { (&cannoli_memhook_write_esi_ebx, &cannoli_memhook_write_esi_ebx_end) },
                    unsafe { (&cannoli_memhook_write_esi_esp, &cannoli_memhook_write_esi_esp_end) },
                    unsafe { (&cannoli_memhook_write_esi_ebp, &cannoli_memhook_write_esi_ebp_end) },
                    unsafe { (&cannoli_memhook_write_esi_esi, &cannoli_memhook_write_esi_esi_end) },
                    unsafe { (&cannoli_memhook_write_esi_edi, &cannoli_memhook_write_esi_edi_end) },
                    unsafe { (&cannoli_memhook_write_esi_r8d, &cannoli_memhook_write_esi_r8d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r9d, &cannoli_memhook_write_esi_r9d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r10d, &cannoli_memhook_write_esi_r10d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r11d, &cannoli_memhook_write_esi_r11d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r12d, &cannoli_memhook_write_esi_r12d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r13d, &cannoli_memhook_write_esi_r13d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r14d, &cannoli_memhook_write_esi_r14d_end) },
                    unsafe { (&cannoli_memhook_write_esi_r15d, &cannoli_memhook_write_esi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_edi_eax, &cannoli_memhook_write_edi_eax_end) },
                    unsafe { (&cannoli_memhook_write_edi_ecx, &cannoli_memhook_write_edi_ecx_end) },
                    unsafe { (&cannoli_memhook_write_edi_edx, &cannoli_memhook_write_edi_edx_end) },
                    unsafe { (&cannoli_memhook_write_edi_ebx, &cannoli_memhook_write_edi_ebx_end) },
                    unsafe { (&cannoli_memhook_write_edi_esp, &cannoli_memhook_write_edi_esp_end) },
                    unsafe { (&cannoli_memhook_write_edi_ebp, &cannoli_memhook_write_edi_ebp_end) },
                    unsafe { (&cannoli_memhook_write_edi_esi, &cannoli_memhook_write_edi_esi_end) },
                    unsafe { (&cannoli_memhook_write_edi_edi, &cannoli_memhook_write_edi_edi_end) },
                    unsafe { (&cannoli_memhook_write_edi_r8d, &cannoli_memhook_write_edi_r8d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r9d, &cannoli_memhook_write_edi_r9d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r10d, &cannoli_memhook_write_edi_r10d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r11d, &cannoli_memhook_write_edi_r11d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r12d, &cannoli_memhook_write_edi_r12d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r13d, &cannoli_memhook_write_edi_r13d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r14d, &cannoli_memhook_write_edi_r14d_end) },
                    unsafe { (&cannoli_memhook_write_edi_r15d, &cannoli_memhook_write_edi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8d_eax, &cannoli_memhook_write_r8d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r8d_ecx, &cannoli_memhook_write_r8d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_edx, &cannoli_memhook_write_r8d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_ebx, &cannoli_memhook_write_r8d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_esp, &cannoli_memhook_write_r8d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r8d_ebp, &cannoli_memhook_write_r8d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r8d_esi, &cannoli_memhook_write_r8d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r8d_edi, &cannoli_memhook_write_r8d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r8d, &cannoli_memhook_write_r8d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r9d, &cannoli_memhook_write_r8d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r10d, &cannoli_memhook_write_r8d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r11d, &cannoli_memhook_write_r8d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r12d, &cannoli_memhook_write_r8d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r13d, &cannoli_memhook_write_r8d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r14d, &cannoli_memhook_write_r8d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r15d, &cannoli_memhook_write_r8d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9d_eax, &cannoli_memhook_write_r9d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r9d_ecx, &cannoli_memhook_write_r9d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_edx, &cannoli_memhook_write_r9d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_ebx, &cannoli_memhook_write_r9d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_esp, &cannoli_memhook_write_r9d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r9d_ebp, &cannoli_memhook_write_r9d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r9d_esi, &cannoli_memhook_write_r9d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r9d_edi, &cannoli_memhook_write_r9d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r8d, &cannoli_memhook_write_r9d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r9d, &cannoli_memhook_write_r9d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r10d, &cannoli_memhook_write_r9d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r11d, &cannoli_memhook_write_r9d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r12d, &cannoli_memhook_write_r9d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r13d, &cannoli_memhook_write_r9d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r14d, &cannoli_memhook_write_r9d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r15d, &cannoli_memhook_write_r9d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10d_eax, &cannoli_memhook_write_r10d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r10d_ecx, &cannoli_memhook_write_r10d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_edx, &cannoli_memhook_write_r10d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_ebx, &cannoli_memhook_write_r10d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_esp, &cannoli_memhook_write_r10d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r10d_ebp, &cannoli_memhook_write_r10d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r10d_esi, &cannoli_memhook_write_r10d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r10d_edi, &cannoli_memhook_write_r10d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r8d, &cannoli_memhook_write_r10d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r9d, &cannoli_memhook_write_r10d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r10d, &cannoli_memhook_write_r10d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r11d, &cannoli_memhook_write_r10d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r12d, &cannoli_memhook_write_r10d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r13d, &cannoli_memhook_write_r10d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r14d, &cannoli_memhook_write_r10d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r15d, &cannoli_memhook_write_r10d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11d_eax, &cannoli_memhook_write_r11d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r11d_ecx, &cannoli_memhook_write_r11d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_edx, &cannoli_memhook_write_r11d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_ebx, &cannoli_memhook_write_r11d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_esp, &cannoli_memhook_write_r11d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r11d_ebp, &cannoli_memhook_write_r11d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r11d_esi, &cannoli_memhook_write_r11d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r11d_edi, &cannoli_memhook_write_r11d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r8d, &cannoli_memhook_write_r11d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r9d, &cannoli_memhook_write_r11d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r10d, &cannoli_memhook_write_r11d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r11d, &cannoli_memhook_write_r11d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r12d, &cannoli_memhook_write_r11d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r13d, &cannoli_memhook_write_r11d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r14d, &cannoli_memhook_write_r11d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r15d, &cannoli_memhook_write_r11d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12d_eax, &cannoli_memhook_write_r12d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r12d_ecx, &cannoli_memhook_write_r12d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_edx, &cannoli_memhook_write_r12d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_ebx, &cannoli_memhook_write_r12d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_esp, &cannoli_memhook_write_r12d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r12d_ebp, &cannoli_memhook_write_r12d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r12d_esi, &cannoli_memhook_write_r12d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r12d_edi, &cannoli_memhook_write_r12d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r8d, &cannoli_memhook_write_r12d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r9d, &cannoli_memhook_write_r12d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r10d, &cannoli_memhook_write_r12d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r11d, &cannoli_memhook_write_r12d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r12d, &cannoli_memhook_write_r12d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r13d, &cannoli_memhook_write_r12d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r14d, &cannoli_memhook_write_r12d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r15d, &cannoli_memhook_write_r12d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13d_eax, &cannoli_memhook_write_r13d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r13d_ecx, &cannoli_memhook_write_r13d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_edx, &cannoli_memhook_write_r13d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_ebx, &cannoli_memhook_write_r13d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_esp, &cannoli_memhook_write_r13d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r13d_ebp, &cannoli_memhook_write_r13d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r13d_esi, &cannoli_memhook_write_r13d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r13d_edi, &cannoli_memhook_write_r13d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r8d, &cannoli_memhook_write_r13d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r9d, &cannoli_memhook_write_r13d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r10d, &cannoli_memhook_write_r13d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r11d, &cannoli_memhook_write_r13d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r12d, &cannoli_memhook_write_r13d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r13d, &cannoli_memhook_write_r13d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r14d, &cannoli_memhook_write_r13d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r15d, &cannoli_memhook_write_r13d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14d_eax, &cannoli_memhook_write_r14d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r14d_ecx, &cannoli_memhook_write_r14d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_edx, &cannoli_memhook_write_r14d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_ebx, &cannoli_memhook_write_r14d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_esp, &cannoli_memhook_write_r14d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r14d_ebp, &cannoli_memhook_write_r14d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r14d_esi, &cannoli_memhook_write_r14d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r14d_edi, &cannoli_memhook_write_r14d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r8d, &cannoli_memhook_write_r14d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r9d, &cannoli_memhook_write_r14d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r10d, &cannoli_memhook_write_r14d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r11d, &cannoli_memhook_write_r14d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r12d, &cannoli_memhook_write_r14d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r13d, &cannoli_memhook_write_r14d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r14d, &cannoli_memhook_write_r14d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r15d, &cannoli_memhook_write_r14d_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15d_eax, &cannoli_memhook_write_r15d_eax_end) },
                    unsafe { (&cannoli_memhook_write_r15d_ecx, &cannoli_memhook_write_r15d_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_edx, &cannoli_memhook_write_r15d_edx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_ebx, &cannoli_memhook_write_r15d_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_esp, &cannoli_memhook_write_r15d_esp_end) },
                    unsafe { (&cannoli_memhook_write_r15d_ebp, &cannoli_memhook_write_r15d_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r15d_esi, &cannoli_memhook_write_r15d_esi_end) },
                    unsafe { (&cannoli_memhook_write_r15d_edi, &cannoli_memhook_write_r15d_edi_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r8d, &cannoli_memhook_write_r15d_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r9d, &cannoli_memhook_write_r15d_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r10d, &cannoli_memhook_write_r15d_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r11d, &cannoli_memhook_write_r15d_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r12d, &cannoli_memhook_write_r15d_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r13d, &cannoli_memhook_write_r15d_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r14d, &cannoli_memhook_write_r15d_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r15d, &cannoli_memhook_write_r15d_r15d_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_rax_eax, &cannoli_memhook_read_rax_eax_end) },
                    unsafe { (&cannoli_memhook_read_rax_ecx, &cannoli_memhook_read_rax_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rax_edx, &cannoli_memhook_read_rax_edx_end) },
                    unsafe { (&cannoli_memhook_read_rax_ebx, &cannoli_memhook_read_rax_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rax_esp, &cannoli_memhook_read_rax_esp_end) },
                    unsafe { (&cannoli_memhook_read_rax_ebp, &cannoli_memhook_read_rax_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rax_esi, &cannoli_memhook_read_rax_esi_end) },
                    unsafe { (&cannoli_memhook_read_rax_edi, &cannoli_memhook_read_rax_edi_end) },
                    unsafe { (&cannoli_memhook_read_rax_r8d, &cannoli_memhook_read_rax_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r9d, &cannoli_memhook_read_rax_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r10d, &cannoli_memhook_read_rax_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r11d, &cannoli_memhook_read_rax_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r12d, &cannoli_memhook_read_rax_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r13d, &cannoli_memhook_read_rax_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r14d, &cannoli_memhook_read_rax_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rax_r15d, &cannoli_memhook_read_rax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rcx_eax, &cannoli_memhook_read_rcx_eax_end) },
                    unsafe { (&cannoli_memhook_read_rcx_ecx, &cannoli_memhook_read_rcx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_edx, &cannoli_memhook_read_rcx_edx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_ebx, &cannoli_memhook_read_rcx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_esp, &cannoli_memhook_read_rcx_esp_end) },
                    unsafe { (&cannoli_memhook_read_rcx_ebp, &cannoli_memhook_read_rcx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rcx_esi, &cannoli_memhook_read_rcx_esi_end) },
                    unsafe { (&cannoli_memhook_read_rcx_edi, &cannoli_memhook_read_rcx_edi_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r8d, &cannoli_memhook_read_rcx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r9d, &cannoli_memhook_read_rcx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r10d, &cannoli_memhook_read_rcx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r11d, &cannoli_memhook_read_rcx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r12d, &cannoli_memhook_read_rcx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r13d, &cannoli_memhook_read_rcx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r14d, &cannoli_memhook_read_rcx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r15d, &cannoli_memhook_read_rcx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rdx_eax, &cannoli_memhook_read_rdx_eax_end) },
                    unsafe { (&cannoli_memhook_read_rdx_ecx, &cannoli_memhook_read_rdx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_edx, &cannoli_memhook_read_rdx_edx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_ebx, &cannoli_memhook_read_rdx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_esp, &cannoli_memhook_read_rdx_esp_end) },
                    unsafe { (&cannoli_memhook_read_rdx_ebp, &cannoli_memhook_read_rdx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rdx_esi, &cannoli_memhook_read_rdx_esi_end) },
                    unsafe { (&cannoli_memhook_read_rdx_edi, &cannoli_memhook_read_rdx_edi_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r8d, &cannoli_memhook_read_rdx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r9d, &cannoli_memhook_read_rdx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r10d, &cannoli_memhook_read_rdx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r11d, &cannoli_memhook_read_rdx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r12d, &cannoli_memhook_read_rdx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r13d, &cannoli_memhook_read_rdx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r14d, &cannoli_memhook_read_rdx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r15d, &cannoli_memhook_read_rdx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rbx_eax, &cannoli_memhook_read_rbx_eax_end) },
                    unsafe { (&cannoli_memhook_read_rbx_ecx, &cannoli_memhook_read_rbx_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_edx, &cannoli_memhook_read_rbx_edx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_ebx, &cannoli_memhook_read_rbx_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_esp, &cannoli_memhook_read_rbx_esp_end) },
                    unsafe { (&cannoli_memhook_read_rbx_ebp, &cannoli_memhook_read_rbx_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rbx_esi, &cannoli_memhook_read_rbx_esi_end) },
                    unsafe { (&cannoli_memhook_read_rbx_edi, &cannoli_memhook_read_rbx_edi_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r8d, &cannoli_memhook_read_rbx_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r9d, &cannoli_memhook_read_rbx_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r10d, &cannoli_memhook_read_rbx_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r11d, &cannoli_memhook_read_rbx_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r12d, &cannoli_memhook_read_rbx_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r13d, &cannoli_memhook_read_rbx_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r14d, &cannoli_memhook_read_rbx_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r15d, &cannoli_memhook_read_rbx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rsp_eax, &cannoli_memhook_read_rsp_eax_end) },
                    unsafe { (&cannoli_memhook_read_rsp_ecx, &cannoli_memhook_read_rsp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_edx, &cannoli_memhook_read_rsp_edx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_ebx, &cannoli_memhook_read_rsp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_esp, &cannoli_memhook_read_rsp_esp_end) },
                    unsafe { (&cannoli_memhook_read_rsp_ebp, &cannoli_memhook_read_rsp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rsp_esi, &cannoli_memhook_read_rsp_esi_end) },
                    unsafe { (&cannoli_memhook_read_rsp_edi, &cannoli_memhook_read_rsp_edi_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r8d, &cannoli_memhook_read_rsp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r9d, &cannoli_memhook_read_rsp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r10d, &cannoli_memhook_read_rsp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r11d, &cannoli_memhook_read_rsp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r12d, &cannoli_memhook_read_rsp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r13d, &cannoli_memhook_read_rsp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r14d, &cannoli_memhook_read_rsp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r15d, &cannoli_memhook_read_rsp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rbp_eax, &cannoli_memhook_read_rbp_eax_end) },
                    unsafe { (&cannoli_memhook_read_rbp_ecx, &cannoli_memhook_read_rbp_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_edx, &cannoli_memhook_read_rbp_edx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_ebx, &cannoli_memhook_read_rbp_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_esp, &cannoli_memhook_read_rbp_esp_end) },
                    unsafe { (&cannoli_memhook_read_rbp_ebp, &cannoli_memhook_read_rbp_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rbp_esi, &cannoli_memhook_read_rbp_esi_end) },
                    unsafe { (&cannoli_memhook_read_rbp_edi, &cannoli_memhook_read_rbp_edi_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r8d, &cannoli_memhook_read_rbp_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r9d, &cannoli_memhook_read_rbp_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r10d, &cannoli_memhook_read_rbp_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r11d, &cannoli_memhook_read_rbp_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r12d, &cannoli_memhook_read_rbp_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r13d, &cannoli_memhook_read_rbp_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r14d, &cannoli_memhook_read_rbp_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r15d, &cannoli_memhook_read_rbp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rsi_eax, &cannoli_memhook_read_rsi_eax_end) },
                    unsafe { (&cannoli_memhook_read_rsi_ecx, &cannoli_memhook_read_rsi_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_edx, &cannoli_memhook_read_rsi_edx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_ebx, &cannoli_memhook_read_rsi_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_esp, &cannoli_memhook_read_rsi_esp_end) },
                    unsafe { (&cannoli_memhook_read_rsi_ebp, &cannoli_memhook_read_rsi_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rsi_esi, &cannoli_memhook_read_rsi_esi_end) },
                    unsafe { (&cannoli_memhook_read_rsi_edi, &cannoli_memhook_read_rsi_edi_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r8d, &cannoli_memhook_read_rsi_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r9d, &cannoli_memhook_read_rsi_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r10d, &cannoli_memhook_read_rsi_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r11d, &cannoli_memhook_read_rsi_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r12d, &cannoli_memhook_read_rsi_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r13d, &cannoli_memhook_read_rsi_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r14d, &cannoli_memhook_read_rsi_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r15d, &cannoli_memhook_read_rsi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rdi_eax, &cannoli_memhook_read_rdi_eax_end) },
                    unsafe { (&cannoli_memhook_read_rdi_ecx, &cannoli_memhook_read_rdi_ecx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_edx, &cannoli_memhook_read_rdi_edx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_ebx, &cannoli_memhook_read_rdi_ebx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_esp, &cannoli_memhook_read_rdi_esp_end) },
                    unsafe { (&cannoli_memhook_read_rdi_ebp, &cannoli_memhook_read_rdi_ebp_end) },
                    unsafe { (&cannoli_memhook_read_rdi_esi, &cannoli_memhook_read_rdi_esi_end) },
                    unsafe { (&cannoli_memhook_read_rdi_edi, &cannoli_memhook_read_rdi_edi_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r8d, &cannoli_memhook_read_rdi_r8d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r9d, &cannoli_memhook_read_rdi_r9d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r10d, &cannoli_memhook_read_rdi_r10d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r11d, &cannoli_memhook_read_rdi_r11d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r12d, &cannoli_memhook_read_rdi_r12d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r13d, &cannoli_memhook_read_rdi_r13d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r14d, &cannoli_memhook_read_rdi_r14d_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r15d, &cannoli_memhook_read_rdi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8_eax, &cannoli_memhook_read_r8_eax_end) },
                    unsafe { (&cannoli_memhook_read_r8_ecx, &cannoli_memhook_read_r8_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r8_edx, &cannoli_memhook_read_r8_edx_end) },
                    unsafe { (&cannoli_memhook_read_r8_ebx, &cannoli_memhook_read_r8_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r8_esp, &cannoli_memhook_read_r8_esp_end) },
                    unsafe { (&cannoli_memhook_read_r8_ebp, &cannoli_memhook_read_r8_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r8_esi, &cannoli_memhook_read_r8_esi_end) },
                    unsafe { (&cannoli_memhook_read_r8_edi, &cannoli_memhook_read_r8_edi_end) },
                    unsafe { (&cannoli_memhook_read_r8_r8d, &cannoli_memhook_read_r8_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r9d, &cannoli_memhook_read_r8_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r10d, &cannoli_memhook_read_r8_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r11d, &cannoli_memhook_read_r8_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r12d, &cannoli_memhook_read_r8_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r13d, &cannoli_memhook_read_r8_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r14d, &cannoli_memhook_read_r8_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r8_r15d, &cannoli_memhook_read_r8_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9_eax, &cannoli_memhook_read_r9_eax_end) },
                    unsafe { (&cannoli_memhook_read_r9_ecx, &cannoli_memhook_read_r9_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r9_edx, &cannoli_memhook_read_r9_edx_end) },
                    unsafe { (&cannoli_memhook_read_r9_ebx, &cannoli_memhook_read_r9_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r9_esp, &cannoli_memhook_read_r9_esp_end) },
                    unsafe { (&cannoli_memhook_read_r9_ebp, &cannoli_memhook_read_r9_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r9_esi, &cannoli_memhook_read_r9_esi_end) },
                    unsafe { (&cannoli_memhook_read_r9_edi, &cannoli_memhook_read_r9_edi_end) },
                    unsafe { (&cannoli_memhook_read_r9_r8d, &cannoli_memhook_read_r9_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r9d, &cannoli_memhook_read_r9_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r10d, &cannoli_memhook_read_r9_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r11d, &cannoli_memhook_read_r9_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r12d, &cannoli_memhook_read_r9_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r13d, &cannoli_memhook_read_r9_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r14d, &cannoli_memhook_read_r9_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r9_r15d, &cannoli_memhook_read_r9_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10_eax, &cannoli_memhook_read_r10_eax_end) },
                    unsafe { (&cannoli_memhook_read_r10_ecx, &cannoli_memhook_read_r10_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r10_edx, &cannoli_memhook_read_r10_edx_end) },
                    unsafe { (&cannoli_memhook_read_r10_ebx, &cannoli_memhook_read_r10_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r10_esp, &cannoli_memhook_read_r10_esp_end) },
                    unsafe { (&cannoli_memhook_read_r10_ebp, &cannoli_memhook_read_r10_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r10_esi, &cannoli_memhook_read_r10_esi_end) },
                    unsafe { (&cannoli_memhook_read_r10_edi, &cannoli_memhook_read_r10_edi_end) },
                    unsafe { (&cannoli_memhook_read_r10_r8d, &cannoli_memhook_read_r10_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r9d, &cannoli_memhook_read_r10_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r10d, &cannoli_memhook_read_r10_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r11d, &cannoli_memhook_read_r10_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r12d, &cannoli_memhook_read_r10_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r13d, &cannoli_memhook_read_r10_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r14d, &cannoli_memhook_read_r10_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r10_r15d, &cannoli_memhook_read_r10_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11_eax, &cannoli_memhook_read_r11_eax_end) },
                    unsafe { (&cannoli_memhook_read_r11_ecx, &cannoli_memhook_read_r11_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r11_edx, &cannoli_memhook_read_r11_edx_end) },
                    unsafe { (&cannoli_memhook_read_r11_ebx, &cannoli_memhook_read_r11_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r11_esp, &cannoli_memhook_read_r11_esp_end) },
                    unsafe { (&cannoli_memhook_read_r11_ebp, &cannoli_memhook_read_r11_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r11_esi, &cannoli_memhook_read_r11_esi_end) },
                    unsafe { (&cannoli_memhook_read_r11_edi, &cannoli_memhook_read_r11_edi_end) },
                    unsafe { (&cannoli_memhook_read_r11_r8d, &cannoli_memhook_read_r11_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r9d, &cannoli_memhook_read_r11_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r10d, &cannoli_memhook_read_r11_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r11d, &cannoli_memhook_read_r11_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r12d, &cannoli_memhook_read_r11_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r13d, &cannoli_memhook_read_r11_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r14d, &cannoli_memhook_read_r11_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r11_r15d, &cannoli_memhook_read_r11_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12_eax, &cannoli_memhook_read_r12_eax_end) },
                    unsafe { (&cannoli_memhook_read_r12_ecx, &cannoli_memhook_read_r12_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r12_edx, &cannoli_memhook_read_r12_edx_end) },
                    unsafe { (&cannoli_memhook_read_r12_ebx, &cannoli_memhook_read_r12_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r12_esp, &cannoli_memhook_read_r12_esp_end) },
                    unsafe { (&cannoli_memhook_read_r12_ebp, &cannoli_memhook_read_r12_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r12_esi, &cannoli_memhook_read_r12_esi_end) },
                    unsafe { (&cannoli_memhook_read_r12_edi, &cannoli_memhook_read_r12_edi_end) },
                    unsafe { (&cannoli_memhook_read_r12_r8d, &cannoli_memhook_read_r12_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r9d, &cannoli_memhook_read_r12_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r10d, &cannoli_memhook_read_r12_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r11d, &cannoli_memhook_read_r12_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r12d, &cannoli_memhook_read_r12_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r13d, &cannoli_memhook_read_r12_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r14d, &cannoli_memhook_read_r12_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r12_r15d, &cannoli_memhook_read_r12_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13_eax, &cannoli_memhook_read_r13_eax_end) },
                    unsafe { (&cannoli_memhook_read_r13_ecx, &cannoli_memhook_read_r13_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r13_edx, &cannoli_memhook_read_r13_edx_end) },
                    unsafe { (&cannoli_memhook_read_r13_ebx, &cannoli_memhook_read_r13_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r13_esp, &cannoli_memhook_read_r13_esp_end) },
                    unsafe { (&cannoli_memhook_read_r13_ebp, &cannoli_memhook_read_r13_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r13_esi, &cannoli_memhook_read_r13_esi_end) },
                    unsafe { (&cannoli_memhook_read_r13_edi, &cannoli_memhook_read_r13_edi_end) },
                    unsafe { (&cannoli_memhook_read_r13_r8d, &cannoli_memhook_read_r13_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r9d, &cannoli_memhook_read_r13_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r10d, &cannoli_memhook_read_r13_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r11d, &cannoli_memhook_read_r13_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r12d, &cannoli_memhook_read_r13_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r13d, &cannoli_memhook_read_r13_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r14d, &cannoli_memhook_read_r13_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r13_r15d, &cannoli_memhook_read_r13_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14_eax, &cannoli_memhook_read_r14_eax_end) },
                    unsafe { (&cannoli_memhook_read_r14_ecx, &cannoli_memhook_read_r14_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r14_edx, &cannoli_memhook_read_r14_edx_end) },
                    unsafe { (&cannoli_memhook_read_r14_ebx, &cannoli_memhook_read_r14_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r14_esp, &cannoli_memhook_read_r14_esp_end) },
                    unsafe { (&cannoli_memhook_read_r14_ebp, &cannoli_memhook_read_r14_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r14_esi, &cannoli_memhook_read_r14_esi_end) },
                    unsafe { (&cannoli_memhook_read_r14_edi, &cannoli_memhook_read_r14_edi_end) },
                    unsafe { (&cannoli_memhook_read_r14_r8d, &cannoli_memhook_read_r14_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r9d, &cannoli_memhook_read_r14_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r10d, &cannoli_memhook_read_r14_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r11d, &cannoli_memhook_read_r14_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r12d, &cannoli_memhook_read_r14_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r13d, &cannoli_memhook_read_r14_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r14d, &cannoli_memhook_read_r14_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r14_r15d, &cannoli_memhook_read_r14_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15_eax, &cannoli_memhook_read_r15_eax_end) },
                    unsafe { (&cannoli_memhook_read_r15_ecx, &cannoli_memhook_read_r15_ecx_end) },
                    unsafe { (&cannoli_memhook_read_r15_edx, &cannoli_memhook_read_r15_edx_end) },
                    unsafe { (&cannoli_memhook_read_r15_ebx, &cannoli_memhook_read_r15_ebx_end) },
                    unsafe { (&cannoli_memhook_read_r15_esp, &cannoli_memhook_read_r15_esp_end) },
                    unsafe { (&cannoli_memhook_read_r15_ebp, &cannoli_memhook_read_r15_ebp_end) },
                    unsafe { (&cannoli_memhook_read_r15_esi, &cannoli_memhook_read_r15_esi_end) },
                    unsafe { (&cannoli_memhook_read_r15_edi, &cannoli_memhook_read_r15_edi_end) },
                    unsafe { (&cannoli_memhook_read_r15_r8d, &cannoli_memhook_read_r15_r8d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r9d, &cannoli_memhook_read_r15_r9d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r10d, &cannoli_memhook_read_r15_r10d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r11d, &cannoli_memhook_read_r15_r11d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r12d, &cannoli_memhook_read_r15_r12d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r13d, &cannoli_memhook_read_r15_r13d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r14d, &cannoli_memhook_read_r15_r14d_end) },
                    unsafe { (&cannoli_memhook_read_r15_r15d, &cannoli_memhook_read_r15_r15d_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_rax_eax, &cannoli_memhook_write_rax_eax_end) },
                    unsafe { (&cannoli_memhook_write_rax_ecx, &cannoli_memhook_write_rax_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rax_edx, &cannoli_memhook_write_rax_edx_end) },
                    unsafe { (&cannoli_memhook_write_rax_ebx, &cannoli_memhook_write_rax_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rax_esp, &cannoli_memhook_write_rax_esp_end) },
                    unsafe { (&cannoli_memhook_write_rax_ebp, &cannoli_memhook_write_rax_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rax_esi, &cannoli_memhook_write_rax_esi_end) },
                    unsafe { (&cannoli_memhook_write_rax_edi, &cannoli_memhook_write_rax_edi_end) },
                    unsafe { (&cannoli_memhook_write_rax_r8d, &cannoli_memhook_write_rax_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r9d, &cannoli_memhook_write_rax_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r10d, &cannoli_memhook_write_rax_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r11d, &cannoli_memhook_write_rax_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r12d, &cannoli_memhook_write_rax_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r13d, &cannoli_memhook_write_rax_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r14d, &cannoli_memhook_write_rax_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rax_r15d, &cannoli_memhook_write_rax_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rcx_eax, &cannoli_memhook_write_rcx_eax_end) },
                    unsafe { (&cannoli_memhook_write_rcx_ecx, &cannoli_memhook_write_rcx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_edx, &cannoli_memhook_write_rcx_edx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_ebx, &cannoli_memhook_write_rcx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_esp, &cannoli_memhook_write_rcx_esp_end) },
                    unsafe { (&cannoli_memhook_write_rcx_ebp, &cannoli_memhook_write_rcx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rcx_esi, &cannoli_memhook_write_rcx_esi_end) },
                    unsafe { (&cannoli_memhook_write_rcx_edi, &cannoli_memhook_write_rcx_edi_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r8d, &cannoli_memhook_write_rcx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r9d, &cannoli_memhook_write_rcx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r10d, &cannoli_memhook_write_rcx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r11d, &cannoli_memhook_write_rcx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r12d, &cannoli_memhook_write_rcx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r13d, &cannoli_memhook_write_rcx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r14d, &cannoli_memhook_write_rcx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r15d, &cannoli_memhook_write_rcx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rdx_eax, &cannoli_memhook_write_rdx_eax_end) },
                    unsafe { (&cannoli_memhook_write_rdx_ecx, &cannoli_memhook_write_rdx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_edx, &cannoli_memhook_write_rdx_edx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_ebx, &cannoli_memhook_write_rdx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_esp, &cannoli_memhook_write_rdx_esp_end) },
                    unsafe { (&cannoli_memhook_write_rdx_ebp, &cannoli_memhook_write_rdx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rdx_esi, &cannoli_memhook_write_rdx_esi_end) },
                    unsafe { (&cannoli_memhook_write_rdx_edi, &cannoli_memhook_write_rdx_edi_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r8d, &cannoli_memhook_write_rdx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r9d, &cannoli_memhook_write_rdx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r10d, &cannoli_memhook_write_rdx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r11d, &cannoli_memhook_write_rdx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r12d, &cannoli_memhook_write_rdx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r13d, &cannoli_memhook_write_rdx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r14d, &cannoli_memhook_write_rdx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r15d, &cannoli_memhook_write_rdx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rbx_eax, &cannoli_memhook_write_rbx_eax_end) },
                    unsafe { (&cannoli_memhook_write_rbx_ecx, &cannoli_memhook_write_rbx_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_edx, &cannoli_memhook_write_rbx_edx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_ebx, &cannoli_memhook_write_rbx_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_esp, &cannoli_memhook_write_rbx_esp_end) },
                    unsafe { (&cannoli_memhook_write_rbx_ebp, &cannoli_memhook_write_rbx_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rbx_esi, &cannoli_memhook_write_rbx_esi_end) },
                    unsafe { (&cannoli_memhook_write_rbx_edi, &cannoli_memhook_write_rbx_edi_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r8d, &cannoli_memhook_write_rbx_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r9d, &cannoli_memhook_write_rbx_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r10d, &cannoli_memhook_write_rbx_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r11d, &cannoli_memhook_write_rbx_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r12d, &cannoli_memhook_write_rbx_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r13d, &cannoli_memhook_write_rbx_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r14d, &cannoli_memhook_write_rbx_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r15d, &cannoli_memhook_write_rbx_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rsp_eax, &cannoli_memhook_write_rsp_eax_end) },
                    unsafe { (&cannoli_memhook_write_rsp_ecx, &cannoli_memhook_write_rsp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_edx, &cannoli_memhook_write_rsp_edx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_ebx, &cannoli_memhook_write_rsp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_esp, &cannoli_memhook_write_rsp_esp_end) },
                    unsafe { (&cannoli_memhook_write_rsp_ebp, &cannoli_memhook_write_rsp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rsp_esi, &cannoli_memhook_write_rsp_esi_end) },
                    unsafe { (&cannoli_memhook_write_rsp_edi, &cannoli_memhook_write_rsp_edi_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r8d, &cannoli_memhook_write_rsp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r9d, &cannoli_memhook_write_rsp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r10d, &cannoli_memhook_write_rsp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r11d, &cannoli_memhook_write_rsp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r12d, &cannoli_memhook_write_rsp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r13d, &cannoli_memhook_write_rsp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r14d, &cannoli_memhook_write_rsp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r15d, &cannoli_memhook_write_rsp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rbp_eax, &cannoli_memhook_write_rbp_eax_end) },
                    unsafe { (&cannoli_memhook_write_rbp_ecx, &cannoli_memhook_write_rbp_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_edx, &cannoli_memhook_write_rbp_edx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_ebx, &cannoli_memhook_write_rbp_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_esp, &cannoli_memhook_write_rbp_esp_end) },
                    unsafe { (&cannoli_memhook_write_rbp_ebp, &cannoli_memhook_write_rbp_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rbp_esi, &cannoli_memhook_write_rbp_esi_end) },
                    unsafe { (&cannoli_memhook_write_rbp_edi, &cannoli_memhook_write_rbp_edi_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r8d, &cannoli_memhook_write_rbp_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r9d, &cannoli_memhook_write_rbp_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r10d, &cannoli_memhook_write_rbp_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r11d, &cannoli_memhook_write_rbp_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r12d, &cannoli_memhook_write_rbp_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r13d, &cannoli_memhook_write_rbp_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r14d, &cannoli_memhook_write_rbp_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r15d, &cannoli_memhook_write_rbp_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rsi_eax, &cannoli_memhook_write_rsi_eax_end) },
                    unsafe { (&cannoli_memhook_write_rsi_ecx, &cannoli_memhook_write_rsi_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_edx, &cannoli_memhook_write_rsi_edx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_ebx, &cannoli_memhook_write_rsi_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_esp, &cannoli_memhook_write_rsi_esp_end) },
                    unsafe { (&cannoli_memhook_write_rsi_ebp, &cannoli_memhook_write_rsi_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rsi_esi, &cannoli_memhook_write_rsi_esi_end) },
                    unsafe { (&cannoli_memhook_write_rsi_edi, &cannoli_memhook_write_rsi_edi_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r8d, &cannoli_memhook_write_rsi_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r9d, &cannoli_memhook_write_rsi_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r10d, &cannoli_memhook_write_rsi_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r11d, &cannoli_memhook_write_rsi_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r12d, &cannoli_memhook_write_rsi_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r13d, &cannoli_memhook_write_rsi_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r14d, &cannoli_memhook_write_rsi_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r15d, &cannoli_memhook_write_rsi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rdi_eax, &cannoli_memhook_write_rdi_eax_end) },
                    unsafe { (&cannoli_memhook_write_rdi_ecx, &cannoli_memhook_write_rdi_ecx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_edx, &cannoli_memhook_write_rdi_edx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_ebx, &cannoli_memhook_write_rdi_ebx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_esp, &cannoli_memhook_write_rdi_esp_end) },
                    unsafe { (&cannoli_memhook_write_rdi_ebp, &cannoli_memhook_write_rdi_ebp_end) },
                    unsafe { (&cannoli_memhook_write_rdi_esi, &cannoli_memhook_write_rdi_esi_end) },
                    unsafe { (&cannoli_memhook_write_rdi_edi, &cannoli_memhook_write_rdi_edi_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r8d, &cannoli_memhook_write_rdi_r8d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r9d, &cannoli_memhook_write_rdi_r9d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r10d, &cannoli_memhook_write_rdi_r10d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r11d, &cannoli_memhook_write_rdi_r11d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r12d, &cannoli_memhook_write_rdi_r12d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r13d, &cannoli_memhook_write_rdi_r13d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r14d, &cannoli_memhook_write_rdi_r14d_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r15d, &cannoli_memhook_write_rdi_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8_eax, &cannoli_memhook_write_r8_eax_end) },
                    unsafe { (&cannoli_memhook_write_r8_ecx, &cannoli_memhook_write_r8_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r8_edx, &cannoli_memhook_write_r8_edx_end) },
                    unsafe { (&cannoli_memhook_write_r8_ebx, &cannoli_memhook_write_r8_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r8_esp, &cannoli_memhook_write_r8_esp_end) },
                    unsafe { (&cannoli_memhook_write_r8_ebp, &cannoli_memhook_write_r8_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r8_esi, &cannoli_memhook_write_r8_esi_end) },
                    unsafe { (&cannoli_memhook_write_r8_edi, &cannoli_memhook_write_r8_edi_end) },
                    unsafe { (&cannoli_memhook_write_r8_r8d, &cannoli_memhook_write_r8_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r9d, &cannoli_memhook_write_r8_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r10d, &cannoli_memhook_write_r8_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r11d, &cannoli_memhook_write_r8_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r12d, &cannoli_memhook_write_r8_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r13d, &cannoli_memhook_write_r8_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r14d, &cannoli_memhook_write_r8_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r8_r15d, &cannoli_memhook_write_r8_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9_eax, &cannoli_memhook_write_r9_eax_end) },
                    unsafe { (&cannoli_memhook_write_r9_ecx, &cannoli_memhook_write_r9_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r9_edx, &cannoli_memhook_write_r9_edx_end) },
                    unsafe { (&cannoli_memhook_write_r9_ebx, &cannoli_memhook_write_r9_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r9_esp, &cannoli_memhook_write_r9_esp_end) },
                    unsafe { (&cannoli_memhook_write_r9_ebp, &cannoli_memhook_write_r9_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r9_esi, &cannoli_memhook_write_r9_esi_end) },
                    unsafe { (&cannoli_memhook_write_r9_edi, &cannoli_memhook_write_r9_edi_end) },
                    unsafe { (&cannoli_memhook_write_r9_r8d, &cannoli_memhook_write_r9_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r9d, &cannoli_memhook_write_r9_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r10d, &cannoli_memhook_write_r9_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r11d, &cannoli_memhook_write_r9_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r12d, &cannoli_memhook_write_r9_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r13d, &cannoli_memhook_write_r9_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r14d, &cannoli_memhook_write_r9_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r9_r15d, &cannoli_memhook_write_r9_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10_eax, &cannoli_memhook_write_r10_eax_end) },
                    unsafe { (&cannoli_memhook_write_r10_ecx, &cannoli_memhook_write_r10_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r10_edx, &cannoli_memhook_write_r10_edx_end) },
                    unsafe { (&cannoli_memhook_write_r10_ebx, &cannoli_memhook_write_r10_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r10_esp, &cannoli_memhook_write_r10_esp_end) },
                    unsafe { (&cannoli_memhook_write_r10_ebp, &cannoli_memhook_write_r10_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r10_esi, &cannoli_memhook_write_r10_esi_end) },
                    unsafe { (&cannoli_memhook_write_r10_edi, &cannoli_memhook_write_r10_edi_end) },
                    unsafe { (&cannoli_memhook_write_r10_r8d, &cannoli_memhook_write_r10_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r9d, &cannoli_memhook_write_r10_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r10d, &cannoli_memhook_write_r10_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r11d, &cannoli_memhook_write_r10_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r12d, &cannoli_memhook_write_r10_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r13d, &cannoli_memhook_write_r10_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r14d, &cannoli_memhook_write_r10_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r10_r15d, &cannoli_memhook_write_r10_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11_eax, &cannoli_memhook_write_r11_eax_end) },
                    unsafe { (&cannoli_memhook_write_r11_ecx, &cannoli_memhook_write_r11_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r11_edx, &cannoli_memhook_write_r11_edx_end) },
                    unsafe { (&cannoli_memhook_write_r11_ebx, &cannoli_memhook_write_r11_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r11_esp, &cannoli_memhook_write_r11_esp_end) },
                    unsafe { (&cannoli_memhook_write_r11_ebp, &cannoli_memhook_write_r11_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r11_esi, &cannoli_memhook_write_r11_esi_end) },
                    unsafe { (&cannoli_memhook_write_r11_edi, &cannoli_memhook_write_r11_edi_end) },
                    unsafe { (&cannoli_memhook_write_r11_r8d, &cannoli_memhook_write_r11_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r9d, &cannoli_memhook_write_r11_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r10d, &cannoli_memhook_write_r11_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r11d, &cannoli_memhook_write_r11_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r12d, &cannoli_memhook_write_r11_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r13d, &cannoli_memhook_write_r11_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r14d, &cannoli_memhook_write_r11_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r11_r15d, &cannoli_memhook_write_r11_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12_eax, &cannoli_memhook_write_r12_eax_end) },
                    unsafe { (&cannoli_memhook_write_r12_ecx, &cannoli_memhook_write_r12_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r12_edx, &cannoli_memhook_write_r12_edx_end) },
                    unsafe { (&cannoli_memhook_write_r12_ebx, &cannoli_memhook_write_r12_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r12_esp, &cannoli_memhook_write_r12_esp_end) },
                    unsafe { (&cannoli_memhook_write_r12_ebp, &cannoli_memhook_write_r12_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r12_esi, &cannoli_memhook_write_r12_esi_end) },
                    unsafe { (&cannoli_memhook_write_r12_edi, &cannoli_memhook_write_r12_edi_end) },
                    unsafe { (&cannoli_memhook_write_r12_r8d, &cannoli_memhook_write_r12_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r9d, &cannoli_memhook_write_r12_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r10d, &cannoli_memhook_write_r12_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r11d, &cannoli_memhook_write_r12_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r12d, &cannoli_memhook_write_r12_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r13d, &cannoli_memhook_write_r12_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r14d, &cannoli_memhook_write_r12_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r12_r15d, &cannoli_memhook_write_r12_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13_eax, &cannoli_memhook_write_r13_eax_end) },
                    unsafe { (&cannoli_memhook_write_r13_ecx, &cannoli_memhook_write_r13_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r13_edx, &cannoli_memhook_write_r13_edx_end) },
                    unsafe { (&cannoli_memhook_write_r13_ebx, &cannoli_memhook_write_r13_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r13_esp, &cannoli_memhook_write_r13_esp_end) },
                    unsafe { (&cannoli_memhook_write_r13_ebp, &cannoli_memhook_write_r13_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r13_esi, &cannoli_memhook_write_r13_esi_end) },
                    unsafe { (&cannoli_memhook_write_r13_edi, &cannoli_memhook_write_r13_edi_end) },
                    unsafe { (&cannoli_memhook_write_r13_r8d, &cannoli_memhook_write_r13_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r9d, &cannoli_memhook_write_r13_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r10d, &cannoli_memhook_write_r13_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r11d, &cannoli_memhook_write_r13_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r12d, &cannoli_memhook_write_r13_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r13d, &cannoli_memhook_write_r13_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r14d, &cannoli_memhook_write_r13_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r13_r15d, &cannoli_memhook_write_r13_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14_eax, &cannoli_memhook_write_r14_eax_end) },
                    unsafe { (&cannoli_memhook_write_r14_ecx, &cannoli_memhook_write_r14_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r14_edx, &cannoli_memhook_write_r14_edx_end) },
                    unsafe { (&cannoli_memhook_write_r14_ebx, &cannoli_memhook_write_r14_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r14_esp, &cannoli_memhook_write_r14_esp_end) },
                    unsafe { (&cannoli_memhook_write_r14_ebp, &cannoli_memhook_write_r14_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r14_esi, &cannoli_memhook_write_r14_esi_end) },
                    unsafe { (&cannoli_memhook_write_r14_edi, &cannoli_memhook_write_r14_edi_end) },
                    unsafe { (&cannoli_memhook_write_r14_r8d, &cannoli_memhook_write_r14_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r9d, &cannoli_memhook_write_r14_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r10d, &cannoli_memhook_write_r14_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r11d, &cannoli_memhook_write_r14_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r12d, &cannoli_memhook_write_r14_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r13d, &cannoli_memhook_write_r14_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r14d, &cannoli_memhook_write_r14_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r14_r15d, &cannoli_memhook_write_r14_r15d_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15_eax, &cannoli_memhook_write_r15_eax_end) },
                    unsafe { (&cannoli_memhook_write_r15_ecx, &cannoli_memhook_write_r15_ecx_end) },
                    unsafe { (&cannoli_memhook_write_r15_edx, &cannoli_memhook_write_r15_edx_end) },
                    unsafe { (&cannoli_memhook_write_r15_ebx, &cannoli_memhook_write_r15_ebx_end) },
                    unsafe { (&cannoli_memhook_write_r15_esp, &cannoli_memhook_write_r15_esp_end) },
                    unsafe { (&cannoli_memhook_write_r15_ebp, &cannoli_memhook_write_r15_ebp_end) },
                    unsafe { (&cannoli_memhook_write_r15_esi, &cannoli_memhook_write_r15_esi_end) },
                    unsafe { (&cannoli_memhook_write_r15_edi, &cannoli_memhook_write_r15_edi_end) },
                    unsafe { (&cannoli_memhook_write_r15_r8d, &cannoli_memhook_write_r15_r8d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r9d, &cannoli_memhook_write_r15_r9d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r10d, &cannoli_memhook_write_r15_r10d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r11d, &cannoli_memhook_write_r15_r11d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r12d, &cannoli_memhook_write_r15_r12d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r13d, &cannoli_memhook_write_r15_r13d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r14d, &cannoli_memhook_write_r15_r14d_end) },
                    unsafe { (&cannoli_memhook_write_r15_r15d, &cannoli_memhook_write_r15_r15d_end) },
                ],
            ],
         ],
    ],
    [
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_al_rax, &cannoli_memhook_read_al_rax_end) },
                    unsafe { (&cannoli_memhook_read_al_rcx, &cannoli_memhook_read_al_rcx_end) },
                    unsafe { (&cannoli_memhook_read_al_rdx, &cannoli_memhook_read_al_rdx_end) },
                    unsafe { (&cannoli_memhook_read_al_rbx, &cannoli_memhook_read_al_rbx_end) },
                    unsafe { (&cannoli_memhook_read_al_rsp, &cannoli_memhook_read_al_rsp_end) },
                    unsafe { (&cannoli_memhook_read_al_rbp, &cannoli_memhook_read_al_rbp_end) },
                    unsafe { (&cannoli_memhook_read_al_rsi, &cannoli_memhook_read_al_rsi_end) },
                    unsafe { (&cannoli_memhook_read_al_rdi, &cannoli_memhook_read_al_rdi_end) },
                    unsafe { (&cannoli_memhook_read_al_r8, &cannoli_memhook_read_al_r8_end) },
                    unsafe { (&cannoli_memhook_read_al_r9, &cannoli_memhook_read_al_r9_end) },
                    unsafe { (&cannoli_memhook_read_al_r10, &cannoli_memhook_read_al_r10_end) },
                    unsafe { (&cannoli_memhook_read_al_r11, &cannoli_memhook_read_al_r11_end) },
                    unsafe { (&cannoli_memhook_read_al_r12, &cannoli_memhook_read_al_r12_end) },
                    unsafe { (&cannoli_memhook_read_al_r13, &cannoli_memhook_read_al_r13_end) },
                    unsafe { (&cannoli_memhook_read_al_r14, &cannoli_memhook_read_al_r14_end) },
                    unsafe { (&cannoli_memhook_read_al_r15, &cannoli_memhook_read_al_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_cl_rax, &cannoli_memhook_read_cl_rax_end) },
                    unsafe { (&cannoli_memhook_read_cl_rcx, &cannoli_memhook_read_cl_rcx_end) },
                    unsafe { (&cannoli_memhook_read_cl_rdx, &cannoli_memhook_read_cl_rdx_end) },
                    unsafe { (&cannoli_memhook_read_cl_rbx, &cannoli_memhook_read_cl_rbx_end) },
                    unsafe { (&cannoli_memhook_read_cl_rsp, &cannoli_memhook_read_cl_rsp_end) },
                    unsafe { (&cannoli_memhook_read_cl_rbp, &cannoli_memhook_read_cl_rbp_end) },
                    unsafe { (&cannoli_memhook_read_cl_rsi, &cannoli_memhook_read_cl_rsi_end) },
                    unsafe { (&cannoli_memhook_read_cl_rdi, &cannoli_memhook_read_cl_rdi_end) },
                    unsafe { (&cannoli_memhook_read_cl_r8, &cannoli_memhook_read_cl_r8_end) },
                    unsafe { (&cannoli_memhook_read_cl_r9, &cannoli_memhook_read_cl_r9_end) },
                    unsafe { (&cannoli_memhook_read_cl_r10, &cannoli_memhook_read_cl_r10_end) },
                    unsafe { (&cannoli_memhook_read_cl_r11, &cannoli_memhook_read_cl_r11_end) },
                    unsafe { (&cannoli_memhook_read_cl_r12, &cannoli_memhook_read_cl_r12_end) },
                    unsafe { (&cannoli_memhook_read_cl_r13, &cannoli_memhook_read_cl_r13_end) },
                    unsafe { (&cannoli_memhook_read_cl_r14, &cannoli_memhook_read_cl_r14_end) },
                    unsafe { (&cannoli_memhook_read_cl_r15, &cannoli_memhook_read_cl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dl_rax, &cannoli_memhook_read_dl_rax_end) },
                    unsafe { (&cannoli_memhook_read_dl_rcx, &cannoli_memhook_read_dl_rcx_end) },
                    unsafe { (&cannoli_memhook_read_dl_rdx, &cannoli_memhook_read_dl_rdx_end) },
                    unsafe { (&cannoli_memhook_read_dl_rbx, &cannoli_memhook_read_dl_rbx_end) },
                    unsafe { (&cannoli_memhook_read_dl_rsp, &cannoli_memhook_read_dl_rsp_end) },
                    unsafe { (&cannoli_memhook_read_dl_rbp, &cannoli_memhook_read_dl_rbp_end) },
                    unsafe { (&cannoli_memhook_read_dl_rsi, &cannoli_memhook_read_dl_rsi_end) },
                    unsafe { (&cannoli_memhook_read_dl_rdi, &cannoli_memhook_read_dl_rdi_end) },
                    unsafe { (&cannoli_memhook_read_dl_r8, &cannoli_memhook_read_dl_r8_end) },
                    unsafe { (&cannoli_memhook_read_dl_r9, &cannoli_memhook_read_dl_r9_end) },
                    unsafe { (&cannoli_memhook_read_dl_r10, &cannoli_memhook_read_dl_r10_end) },
                    unsafe { (&cannoli_memhook_read_dl_r11, &cannoli_memhook_read_dl_r11_end) },
                    unsafe { (&cannoli_memhook_read_dl_r12, &cannoli_memhook_read_dl_r12_end) },
                    unsafe { (&cannoli_memhook_read_dl_r13, &cannoli_memhook_read_dl_r13_end) },
                    unsafe { (&cannoli_memhook_read_dl_r14, &cannoli_memhook_read_dl_r14_end) },
                    unsafe { (&cannoli_memhook_read_dl_r15, &cannoli_memhook_read_dl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bl_rax, &cannoli_memhook_read_bl_rax_end) },
                    unsafe { (&cannoli_memhook_read_bl_rcx, &cannoli_memhook_read_bl_rcx_end) },
                    unsafe { (&cannoli_memhook_read_bl_rdx, &cannoli_memhook_read_bl_rdx_end) },
                    unsafe { (&cannoli_memhook_read_bl_rbx, &cannoli_memhook_read_bl_rbx_end) },
                    unsafe { (&cannoli_memhook_read_bl_rsp, &cannoli_memhook_read_bl_rsp_end) },
                    unsafe { (&cannoli_memhook_read_bl_rbp, &cannoli_memhook_read_bl_rbp_end) },
                    unsafe { (&cannoli_memhook_read_bl_rsi, &cannoli_memhook_read_bl_rsi_end) },
                    unsafe { (&cannoli_memhook_read_bl_rdi, &cannoli_memhook_read_bl_rdi_end) },
                    unsafe { (&cannoli_memhook_read_bl_r8, &cannoli_memhook_read_bl_r8_end) },
                    unsafe { (&cannoli_memhook_read_bl_r9, &cannoli_memhook_read_bl_r9_end) },
                    unsafe { (&cannoli_memhook_read_bl_r10, &cannoli_memhook_read_bl_r10_end) },
                    unsafe { (&cannoli_memhook_read_bl_r11, &cannoli_memhook_read_bl_r11_end) },
                    unsafe { (&cannoli_memhook_read_bl_r12, &cannoli_memhook_read_bl_r12_end) },
                    unsafe { (&cannoli_memhook_read_bl_r13, &cannoli_memhook_read_bl_r13_end) },
                    unsafe { (&cannoli_memhook_read_bl_r14, &cannoli_memhook_read_bl_r14_end) },
                    unsafe { (&cannoli_memhook_read_bl_r15, &cannoli_memhook_read_bl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_spl_rax, &cannoli_memhook_read_spl_rax_end) },
                    unsafe { (&cannoli_memhook_read_spl_rcx, &cannoli_memhook_read_spl_rcx_end) },
                    unsafe { (&cannoli_memhook_read_spl_rdx, &cannoli_memhook_read_spl_rdx_end) },
                    unsafe { (&cannoli_memhook_read_spl_rbx, &cannoli_memhook_read_spl_rbx_end) },
                    unsafe { (&cannoli_memhook_read_spl_rsp, &cannoli_memhook_read_spl_rsp_end) },
                    unsafe { (&cannoli_memhook_read_spl_rbp, &cannoli_memhook_read_spl_rbp_end) },
                    unsafe { (&cannoli_memhook_read_spl_rsi, &cannoli_memhook_read_spl_rsi_end) },
                    unsafe { (&cannoli_memhook_read_spl_rdi, &cannoli_memhook_read_spl_rdi_end) },
                    unsafe { (&cannoli_memhook_read_spl_r8, &cannoli_memhook_read_spl_r8_end) },
                    unsafe { (&cannoli_memhook_read_spl_r9, &cannoli_memhook_read_spl_r9_end) },
                    unsafe { (&cannoli_memhook_read_spl_r10, &cannoli_memhook_read_spl_r10_end) },
                    unsafe { (&cannoli_memhook_read_spl_r11, &cannoli_memhook_read_spl_r11_end) },
                    unsafe { (&cannoli_memhook_read_spl_r12, &cannoli_memhook_read_spl_r12_end) },
                    unsafe { (&cannoli_memhook_read_spl_r13, &cannoli_memhook_read_spl_r13_end) },
                    unsafe { (&cannoli_memhook_read_spl_r14, &cannoli_memhook_read_spl_r14_end) },
                    unsafe { (&cannoli_memhook_read_spl_r15, &cannoli_memhook_read_spl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bpl_rax, &cannoli_memhook_read_bpl_rax_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rcx, &cannoli_memhook_read_bpl_rcx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rdx, &cannoli_memhook_read_bpl_rdx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rbx, &cannoli_memhook_read_bpl_rbx_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rsp, &cannoli_memhook_read_bpl_rsp_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rbp, &cannoli_memhook_read_bpl_rbp_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rsi, &cannoli_memhook_read_bpl_rsi_end) },
                    unsafe { (&cannoli_memhook_read_bpl_rdi, &cannoli_memhook_read_bpl_rdi_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r8, &cannoli_memhook_read_bpl_r8_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r9, &cannoli_memhook_read_bpl_r9_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r10, &cannoli_memhook_read_bpl_r10_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r11, &cannoli_memhook_read_bpl_r11_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r12, &cannoli_memhook_read_bpl_r12_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r13, &cannoli_memhook_read_bpl_r13_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r14, &cannoli_memhook_read_bpl_r14_end) },
                    unsafe { (&cannoli_memhook_read_bpl_r15, &cannoli_memhook_read_bpl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_sil_rax, &cannoli_memhook_read_sil_rax_end) },
                    unsafe { (&cannoli_memhook_read_sil_rcx, &cannoli_memhook_read_sil_rcx_end) },
                    unsafe { (&cannoli_memhook_read_sil_rdx, &cannoli_memhook_read_sil_rdx_end) },
                    unsafe { (&cannoli_memhook_read_sil_rbx, &cannoli_memhook_read_sil_rbx_end) },
                    unsafe { (&cannoli_memhook_read_sil_rsp, &cannoli_memhook_read_sil_rsp_end) },
                    unsafe { (&cannoli_memhook_read_sil_rbp, &cannoli_memhook_read_sil_rbp_end) },
                    unsafe { (&cannoli_memhook_read_sil_rsi, &cannoli_memhook_read_sil_rsi_end) },
                    unsafe { (&cannoli_memhook_read_sil_rdi, &cannoli_memhook_read_sil_rdi_end) },
                    unsafe { (&cannoli_memhook_read_sil_r8, &cannoli_memhook_read_sil_r8_end) },
                    unsafe { (&cannoli_memhook_read_sil_r9, &cannoli_memhook_read_sil_r9_end) },
                    unsafe { (&cannoli_memhook_read_sil_r10, &cannoli_memhook_read_sil_r10_end) },
                    unsafe { (&cannoli_memhook_read_sil_r11, &cannoli_memhook_read_sil_r11_end) },
                    unsafe { (&cannoli_memhook_read_sil_r12, &cannoli_memhook_read_sil_r12_end) },
                    unsafe { (&cannoli_memhook_read_sil_r13, &cannoli_memhook_read_sil_r13_end) },
                    unsafe { (&cannoli_memhook_read_sil_r14, &cannoli_memhook_read_sil_r14_end) },
                    unsafe { (&cannoli_memhook_read_sil_r15, &cannoli_memhook_read_sil_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dil_rax, &cannoli_memhook_read_dil_rax_end) },
                    unsafe { (&cannoli_memhook_read_dil_rcx, &cannoli_memhook_read_dil_rcx_end) },
                    unsafe { (&cannoli_memhook_read_dil_rdx, &cannoli_memhook_read_dil_rdx_end) },
                    unsafe { (&cannoli_memhook_read_dil_rbx, &cannoli_memhook_read_dil_rbx_end) },
                    unsafe { (&cannoli_memhook_read_dil_rsp, &cannoli_memhook_read_dil_rsp_end) },
                    unsafe { (&cannoli_memhook_read_dil_rbp, &cannoli_memhook_read_dil_rbp_end) },
                    unsafe { (&cannoli_memhook_read_dil_rsi, &cannoli_memhook_read_dil_rsi_end) },
                    unsafe { (&cannoli_memhook_read_dil_rdi, &cannoli_memhook_read_dil_rdi_end) },
                    unsafe { (&cannoli_memhook_read_dil_r8, &cannoli_memhook_read_dil_r8_end) },
                    unsafe { (&cannoli_memhook_read_dil_r9, &cannoli_memhook_read_dil_r9_end) },
                    unsafe { (&cannoli_memhook_read_dil_r10, &cannoli_memhook_read_dil_r10_end) },
                    unsafe { (&cannoli_memhook_read_dil_r11, &cannoli_memhook_read_dil_r11_end) },
                    unsafe { (&cannoli_memhook_read_dil_r12, &cannoli_memhook_read_dil_r12_end) },
                    unsafe { (&cannoli_memhook_read_dil_r13, &cannoli_memhook_read_dil_r13_end) },
                    unsafe { (&cannoli_memhook_read_dil_r14, &cannoli_memhook_read_dil_r14_end) },
                    unsafe { (&cannoli_memhook_read_dil_r15, &cannoli_memhook_read_dil_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8b_rax, &cannoli_memhook_read_r8b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rcx, &cannoli_memhook_read_r8b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rdx, &cannoli_memhook_read_r8b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rbx, &cannoli_memhook_read_r8b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rsp, &cannoli_memhook_read_r8b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rbp, &cannoli_memhook_read_r8b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rsi, &cannoli_memhook_read_r8b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r8b_rdi, &cannoli_memhook_read_r8b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r8, &cannoli_memhook_read_r8b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r9, &cannoli_memhook_read_r8b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r10, &cannoli_memhook_read_r8b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r11, &cannoli_memhook_read_r8b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r12, &cannoli_memhook_read_r8b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r13, &cannoli_memhook_read_r8b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r14, &cannoli_memhook_read_r8b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r8b_r15, &cannoli_memhook_read_r8b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9b_rax, &cannoli_memhook_read_r9b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rcx, &cannoli_memhook_read_r9b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rdx, &cannoli_memhook_read_r9b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rbx, &cannoli_memhook_read_r9b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rsp, &cannoli_memhook_read_r9b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rbp, &cannoli_memhook_read_r9b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rsi, &cannoli_memhook_read_r9b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r9b_rdi, &cannoli_memhook_read_r9b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r8, &cannoli_memhook_read_r9b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r9, &cannoli_memhook_read_r9b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r10, &cannoli_memhook_read_r9b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r11, &cannoli_memhook_read_r9b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r12, &cannoli_memhook_read_r9b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r13, &cannoli_memhook_read_r9b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r14, &cannoli_memhook_read_r9b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r9b_r15, &cannoli_memhook_read_r9b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10b_rax, &cannoli_memhook_read_r10b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rcx, &cannoli_memhook_read_r10b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rdx, &cannoli_memhook_read_r10b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rbx, &cannoli_memhook_read_r10b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rsp, &cannoli_memhook_read_r10b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rbp, &cannoli_memhook_read_r10b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rsi, &cannoli_memhook_read_r10b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r10b_rdi, &cannoli_memhook_read_r10b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r8, &cannoli_memhook_read_r10b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r9, &cannoli_memhook_read_r10b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r10, &cannoli_memhook_read_r10b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r11, &cannoli_memhook_read_r10b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r12, &cannoli_memhook_read_r10b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r13, &cannoli_memhook_read_r10b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r14, &cannoli_memhook_read_r10b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r10b_r15, &cannoli_memhook_read_r10b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11b_rax, &cannoli_memhook_read_r11b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rcx, &cannoli_memhook_read_r11b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rdx, &cannoli_memhook_read_r11b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rbx, &cannoli_memhook_read_r11b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rsp, &cannoli_memhook_read_r11b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rbp, &cannoli_memhook_read_r11b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rsi, &cannoli_memhook_read_r11b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r11b_rdi, &cannoli_memhook_read_r11b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r8, &cannoli_memhook_read_r11b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r9, &cannoli_memhook_read_r11b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r10, &cannoli_memhook_read_r11b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r11, &cannoli_memhook_read_r11b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r12, &cannoli_memhook_read_r11b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r13, &cannoli_memhook_read_r11b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r14, &cannoli_memhook_read_r11b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r11b_r15, &cannoli_memhook_read_r11b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12b_rax, &cannoli_memhook_read_r12b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rcx, &cannoli_memhook_read_r12b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rdx, &cannoli_memhook_read_r12b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rbx, &cannoli_memhook_read_r12b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rsp, &cannoli_memhook_read_r12b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rbp, &cannoli_memhook_read_r12b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rsi, &cannoli_memhook_read_r12b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r12b_rdi, &cannoli_memhook_read_r12b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r8, &cannoli_memhook_read_r12b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r9, &cannoli_memhook_read_r12b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r10, &cannoli_memhook_read_r12b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r11, &cannoli_memhook_read_r12b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r12, &cannoli_memhook_read_r12b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r13, &cannoli_memhook_read_r12b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r14, &cannoli_memhook_read_r12b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r12b_r15, &cannoli_memhook_read_r12b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13b_rax, &cannoli_memhook_read_r13b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rcx, &cannoli_memhook_read_r13b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rdx, &cannoli_memhook_read_r13b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rbx, &cannoli_memhook_read_r13b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rsp, &cannoli_memhook_read_r13b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rbp, &cannoli_memhook_read_r13b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rsi, &cannoli_memhook_read_r13b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r13b_rdi, &cannoli_memhook_read_r13b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r8, &cannoli_memhook_read_r13b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r9, &cannoli_memhook_read_r13b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r10, &cannoli_memhook_read_r13b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r11, &cannoli_memhook_read_r13b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r12, &cannoli_memhook_read_r13b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r13, &cannoli_memhook_read_r13b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r14, &cannoli_memhook_read_r13b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r13b_r15, &cannoli_memhook_read_r13b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14b_rax, &cannoli_memhook_read_r14b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rcx, &cannoli_memhook_read_r14b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rdx, &cannoli_memhook_read_r14b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rbx, &cannoli_memhook_read_r14b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rsp, &cannoli_memhook_read_r14b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rbp, &cannoli_memhook_read_r14b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rsi, &cannoli_memhook_read_r14b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r14b_rdi, &cannoli_memhook_read_r14b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r8, &cannoli_memhook_read_r14b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r9, &cannoli_memhook_read_r14b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r10, &cannoli_memhook_read_r14b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r11, &cannoli_memhook_read_r14b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r12, &cannoli_memhook_read_r14b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r13, &cannoli_memhook_read_r14b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r14, &cannoli_memhook_read_r14b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r14b_r15, &cannoli_memhook_read_r14b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15b_rax, &cannoli_memhook_read_r15b_rax_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rcx, &cannoli_memhook_read_r15b_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rdx, &cannoli_memhook_read_r15b_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rbx, &cannoli_memhook_read_r15b_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rsp, &cannoli_memhook_read_r15b_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rbp, &cannoli_memhook_read_r15b_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rsi, &cannoli_memhook_read_r15b_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r15b_rdi, &cannoli_memhook_read_r15b_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r8, &cannoli_memhook_read_r15b_r8_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r9, &cannoli_memhook_read_r15b_r9_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r10, &cannoli_memhook_read_r15b_r10_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r11, &cannoli_memhook_read_r15b_r11_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r12, &cannoli_memhook_read_r15b_r12_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r13, &cannoli_memhook_read_r15b_r13_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r14, &cannoli_memhook_read_r15b_r14_end) },
                    unsafe { (&cannoli_memhook_read_r15b_r15, &cannoli_memhook_read_r15b_r15_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_al_rax, &cannoli_memhook_write_al_rax_end) },
                    unsafe { (&cannoli_memhook_write_al_rcx, &cannoli_memhook_write_al_rcx_end) },
                    unsafe { (&cannoli_memhook_write_al_rdx, &cannoli_memhook_write_al_rdx_end) },
                    unsafe { (&cannoli_memhook_write_al_rbx, &cannoli_memhook_write_al_rbx_end) },
                    unsafe { (&cannoli_memhook_write_al_rsp, &cannoli_memhook_write_al_rsp_end) },
                    unsafe { (&cannoli_memhook_write_al_rbp, &cannoli_memhook_write_al_rbp_end) },
                    unsafe { (&cannoli_memhook_write_al_rsi, &cannoli_memhook_write_al_rsi_end) },
                    unsafe { (&cannoli_memhook_write_al_rdi, &cannoli_memhook_write_al_rdi_end) },
                    unsafe { (&cannoli_memhook_write_al_r8, &cannoli_memhook_write_al_r8_end) },
                    unsafe { (&cannoli_memhook_write_al_r9, &cannoli_memhook_write_al_r9_end) },
                    unsafe { (&cannoli_memhook_write_al_r10, &cannoli_memhook_write_al_r10_end) },
                    unsafe { (&cannoli_memhook_write_al_r11, &cannoli_memhook_write_al_r11_end) },
                    unsafe { (&cannoli_memhook_write_al_r12, &cannoli_memhook_write_al_r12_end) },
                    unsafe { (&cannoli_memhook_write_al_r13, &cannoli_memhook_write_al_r13_end) },
                    unsafe { (&cannoli_memhook_write_al_r14, &cannoli_memhook_write_al_r14_end) },
                    unsafe { (&cannoli_memhook_write_al_r15, &cannoli_memhook_write_al_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_cl_rax, &cannoli_memhook_write_cl_rax_end) },
                    unsafe { (&cannoli_memhook_write_cl_rcx, &cannoli_memhook_write_cl_rcx_end) },
                    unsafe { (&cannoli_memhook_write_cl_rdx, &cannoli_memhook_write_cl_rdx_end) },
                    unsafe { (&cannoli_memhook_write_cl_rbx, &cannoli_memhook_write_cl_rbx_end) },
                    unsafe { (&cannoli_memhook_write_cl_rsp, &cannoli_memhook_write_cl_rsp_end) },
                    unsafe { (&cannoli_memhook_write_cl_rbp, &cannoli_memhook_write_cl_rbp_end) },
                    unsafe { (&cannoli_memhook_write_cl_rsi, &cannoli_memhook_write_cl_rsi_end) },
                    unsafe { (&cannoli_memhook_write_cl_rdi, &cannoli_memhook_write_cl_rdi_end) },
                    unsafe { (&cannoli_memhook_write_cl_r8, &cannoli_memhook_write_cl_r8_end) },
                    unsafe { (&cannoli_memhook_write_cl_r9, &cannoli_memhook_write_cl_r9_end) },
                    unsafe { (&cannoli_memhook_write_cl_r10, &cannoli_memhook_write_cl_r10_end) },
                    unsafe { (&cannoli_memhook_write_cl_r11, &cannoli_memhook_write_cl_r11_end) },
                    unsafe { (&cannoli_memhook_write_cl_r12, &cannoli_memhook_write_cl_r12_end) },
                    unsafe { (&cannoli_memhook_write_cl_r13, &cannoli_memhook_write_cl_r13_end) },
                    unsafe { (&cannoli_memhook_write_cl_r14, &cannoli_memhook_write_cl_r14_end) },
                    unsafe { (&cannoli_memhook_write_cl_r15, &cannoli_memhook_write_cl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dl_rax, &cannoli_memhook_write_dl_rax_end) },
                    unsafe { (&cannoli_memhook_write_dl_rcx, &cannoli_memhook_write_dl_rcx_end) },
                    unsafe { (&cannoli_memhook_write_dl_rdx, &cannoli_memhook_write_dl_rdx_end) },
                    unsafe { (&cannoli_memhook_write_dl_rbx, &cannoli_memhook_write_dl_rbx_end) },
                    unsafe { (&cannoli_memhook_write_dl_rsp, &cannoli_memhook_write_dl_rsp_end) },
                    unsafe { (&cannoli_memhook_write_dl_rbp, &cannoli_memhook_write_dl_rbp_end) },
                    unsafe { (&cannoli_memhook_write_dl_rsi, &cannoli_memhook_write_dl_rsi_end) },
                    unsafe { (&cannoli_memhook_write_dl_rdi, &cannoli_memhook_write_dl_rdi_end) },
                    unsafe { (&cannoli_memhook_write_dl_r8, &cannoli_memhook_write_dl_r8_end) },
                    unsafe { (&cannoli_memhook_write_dl_r9, &cannoli_memhook_write_dl_r9_end) },
                    unsafe { (&cannoli_memhook_write_dl_r10, &cannoli_memhook_write_dl_r10_end) },
                    unsafe { (&cannoli_memhook_write_dl_r11, &cannoli_memhook_write_dl_r11_end) },
                    unsafe { (&cannoli_memhook_write_dl_r12, &cannoli_memhook_write_dl_r12_end) },
                    unsafe { (&cannoli_memhook_write_dl_r13, &cannoli_memhook_write_dl_r13_end) },
                    unsafe { (&cannoli_memhook_write_dl_r14, &cannoli_memhook_write_dl_r14_end) },
                    unsafe { (&cannoli_memhook_write_dl_r15, &cannoli_memhook_write_dl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bl_rax, &cannoli_memhook_write_bl_rax_end) },
                    unsafe { (&cannoli_memhook_write_bl_rcx, &cannoli_memhook_write_bl_rcx_end) },
                    unsafe { (&cannoli_memhook_write_bl_rdx, &cannoli_memhook_write_bl_rdx_end) },
                    unsafe { (&cannoli_memhook_write_bl_rbx, &cannoli_memhook_write_bl_rbx_end) },
                    unsafe { (&cannoli_memhook_write_bl_rsp, &cannoli_memhook_write_bl_rsp_end) },
                    unsafe { (&cannoli_memhook_write_bl_rbp, &cannoli_memhook_write_bl_rbp_end) },
                    unsafe { (&cannoli_memhook_write_bl_rsi, &cannoli_memhook_write_bl_rsi_end) },
                    unsafe { (&cannoli_memhook_write_bl_rdi, &cannoli_memhook_write_bl_rdi_end) },
                    unsafe { (&cannoli_memhook_write_bl_r8, &cannoli_memhook_write_bl_r8_end) },
                    unsafe { (&cannoli_memhook_write_bl_r9, &cannoli_memhook_write_bl_r9_end) },
                    unsafe { (&cannoli_memhook_write_bl_r10, &cannoli_memhook_write_bl_r10_end) },
                    unsafe { (&cannoli_memhook_write_bl_r11, &cannoli_memhook_write_bl_r11_end) },
                    unsafe { (&cannoli_memhook_write_bl_r12, &cannoli_memhook_write_bl_r12_end) },
                    unsafe { (&cannoli_memhook_write_bl_r13, &cannoli_memhook_write_bl_r13_end) },
                    unsafe { (&cannoli_memhook_write_bl_r14, &cannoli_memhook_write_bl_r14_end) },
                    unsafe { (&cannoli_memhook_write_bl_r15, &cannoli_memhook_write_bl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_spl_rax, &cannoli_memhook_write_spl_rax_end) },
                    unsafe { (&cannoli_memhook_write_spl_rcx, &cannoli_memhook_write_spl_rcx_end) },
                    unsafe { (&cannoli_memhook_write_spl_rdx, &cannoli_memhook_write_spl_rdx_end) },
                    unsafe { (&cannoli_memhook_write_spl_rbx, &cannoli_memhook_write_spl_rbx_end) },
                    unsafe { (&cannoli_memhook_write_spl_rsp, &cannoli_memhook_write_spl_rsp_end) },
                    unsafe { (&cannoli_memhook_write_spl_rbp, &cannoli_memhook_write_spl_rbp_end) },
                    unsafe { (&cannoli_memhook_write_spl_rsi, &cannoli_memhook_write_spl_rsi_end) },
                    unsafe { (&cannoli_memhook_write_spl_rdi, &cannoli_memhook_write_spl_rdi_end) },
                    unsafe { (&cannoli_memhook_write_spl_r8, &cannoli_memhook_write_spl_r8_end) },
                    unsafe { (&cannoli_memhook_write_spl_r9, &cannoli_memhook_write_spl_r9_end) },
                    unsafe { (&cannoli_memhook_write_spl_r10, &cannoli_memhook_write_spl_r10_end) },
                    unsafe { (&cannoli_memhook_write_spl_r11, &cannoli_memhook_write_spl_r11_end) },
                    unsafe { (&cannoli_memhook_write_spl_r12, &cannoli_memhook_write_spl_r12_end) },
                    unsafe { (&cannoli_memhook_write_spl_r13, &cannoli_memhook_write_spl_r13_end) },
                    unsafe { (&cannoli_memhook_write_spl_r14, &cannoli_memhook_write_spl_r14_end) },
                    unsafe { (&cannoli_memhook_write_spl_r15, &cannoli_memhook_write_spl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bpl_rax, &cannoli_memhook_write_bpl_rax_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rcx, &cannoli_memhook_write_bpl_rcx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rdx, &cannoli_memhook_write_bpl_rdx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rbx, &cannoli_memhook_write_bpl_rbx_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rsp, &cannoli_memhook_write_bpl_rsp_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rbp, &cannoli_memhook_write_bpl_rbp_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rsi, &cannoli_memhook_write_bpl_rsi_end) },
                    unsafe { (&cannoli_memhook_write_bpl_rdi, &cannoli_memhook_write_bpl_rdi_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r8, &cannoli_memhook_write_bpl_r8_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r9, &cannoli_memhook_write_bpl_r9_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r10, &cannoli_memhook_write_bpl_r10_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r11, &cannoli_memhook_write_bpl_r11_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r12, &cannoli_memhook_write_bpl_r12_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r13, &cannoli_memhook_write_bpl_r13_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r14, &cannoli_memhook_write_bpl_r14_end) },
                    unsafe { (&cannoli_memhook_write_bpl_r15, &cannoli_memhook_write_bpl_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_sil_rax, &cannoli_memhook_write_sil_rax_end) },
                    unsafe { (&cannoli_memhook_write_sil_rcx, &cannoli_memhook_write_sil_rcx_end) },
                    unsafe { (&cannoli_memhook_write_sil_rdx, &cannoli_memhook_write_sil_rdx_end) },
                    unsafe { (&cannoli_memhook_write_sil_rbx, &cannoli_memhook_write_sil_rbx_end) },
                    unsafe { (&cannoli_memhook_write_sil_rsp, &cannoli_memhook_write_sil_rsp_end) },
                    unsafe { (&cannoli_memhook_write_sil_rbp, &cannoli_memhook_write_sil_rbp_end) },
                    unsafe { (&cannoli_memhook_write_sil_rsi, &cannoli_memhook_write_sil_rsi_end) },
                    unsafe { (&cannoli_memhook_write_sil_rdi, &cannoli_memhook_write_sil_rdi_end) },
                    unsafe { (&cannoli_memhook_write_sil_r8, &cannoli_memhook_write_sil_r8_end) },
                    unsafe { (&cannoli_memhook_write_sil_r9, &cannoli_memhook_write_sil_r9_end) },
                    unsafe { (&cannoli_memhook_write_sil_r10, &cannoli_memhook_write_sil_r10_end) },
                    unsafe { (&cannoli_memhook_write_sil_r11, &cannoli_memhook_write_sil_r11_end) },
                    unsafe { (&cannoli_memhook_write_sil_r12, &cannoli_memhook_write_sil_r12_end) },
                    unsafe { (&cannoli_memhook_write_sil_r13, &cannoli_memhook_write_sil_r13_end) },
                    unsafe { (&cannoli_memhook_write_sil_r14, &cannoli_memhook_write_sil_r14_end) },
                    unsafe { (&cannoli_memhook_write_sil_r15, &cannoli_memhook_write_sil_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dil_rax, &cannoli_memhook_write_dil_rax_end) },
                    unsafe { (&cannoli_memhook_write_dil_rcx, &cannoli_memhook_write_dil_rcx_end) },
                    unsafe { (&cannoli_memhook_write_dil_rdx, &cannoli_memhook_write_dil_rdx_end) },
                    unsafe { (&cannoli_memhook_write_dil_rbx, &cannoli_memhook_write_dil_rbx_end) },
                    unsafe { (&cannoli_memhook_write_dil_rsp, &cannoli_memhook_write_dil_rsp_end) },
                    unsafe { (&cannoli_memhook_write_dil_rbp, &cannoli_memhook_write_dil_rbp_end) },
                    unsafe { (&cannoli_memhook_write_dil_rsi, &cannoli_memhook_write_dil_rsi_end) },
                    unsafe { (&cannoli_memhook_write_dil_rdi, &cannoli_memhook_write_dil_rdi_end) },
                    unsafe { (&cannoli_memhook_write_dil_r8, &cannoli_memhook_write_dil_r8_end) },
                    unsafe { (&cannoli_memhook_write_dil_r9, &cannoli_memhook_write_dil_r9_end) },
                    unsafe { (&cannoli_memhook_write_dil_r10, &cannoli_memhook_write_dil_r10_end) },
                    unsafe { (&cannoli_memhook_write_dil_r11, &cannoli_memhook_write_dil_r11_end) },
                    unsafe { (&cannoli_memhook_write_dil_r12, &cannoli_memhook_write_dil_r12_end) },
                    unsafe { (&cannoli_memhook_write_dil_r13, &cannoli_memhook_write_dil_r13_end) },
                    unsafe { (&cannoli_memhook_write_dil_r14, &cannoli_memhook_write_dil_r14_end) },
                    unsafe { (&cannoli_memhook_write_dil_r15, &cannoli_memhook_write_dil_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8b_rax, &cannoli_memhook_write_r8b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rcx, &cannoli_memhook_write_r8b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rdx, &cannoli_memhook_write_r8b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rbx, &cannoli_memhook_write_r8b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rsp, &cannoli_memhook_write_r8b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rbp, &cannoli_memhook_write_r8b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rsi, &cannoli_memhook_write_r8b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r8b_rdi, &cannoli_memhook_write_r8b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r8, &cannoli_memhook_write_r8b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r9, &cannoli_memhook_write_r8b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r10, &cannoli_memhook_write_r8b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r11, &cannoli_memhook_write_r8b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r12, &cannoli_memhook_write_r8b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r13, &cannoli_memhook_write_r8b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r14, &cannoli_memhook_write_r8b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r8b_r15, &cannoli_memhook_write_r8b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9b_rax, &cannoli_memhook_write_r9b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rcx, &cannoli_memhook_write_r9b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rdx, &cannoli_memhook_write_r9b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rbx, &cannoli_memhook_write_r9b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rsp, &cannoli_memhook_write_r9b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rbp, &cannoli_memhook_write_r9b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rsi, &cannoli_memhook_write_r9b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r9b_rdi, &cannoli_memhook_write_r9b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r8, &cannoli_memhook_write_r9b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r9, &cannoli_memhook_write_r9b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r10, &cannoli_memhook_write_r9b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r11, &cannoli_memhook_write_r9b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r12, &cannoli_memhook_write_r9b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r13, &cannoli_memhook_write_r9b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r14, &cannoli_memhook_write_r9b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r9b_r15, &cannoli_memhook_write_r9b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10b_rax, &cannoli_memhook_write_r10b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rcx, &cannoli_memhook_write_r10b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rdx, &cannoli_memhook_write_r10b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rbx, &cannoli_memhook_write_r10b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rsp, &cannoli_memhook_write_r10b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rbp, &cannoli_memhook_write_r10b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rsi, &cannoli_memhook_write_r10b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r10b_rdi, &cannoli_memhook_write_r10b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r8, &cannoli_memhook_write_r10b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r9, &cannoli_memhook_write_r10b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r10, &cannoli_memhook_write_r10b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r11, &cannoli_memhook_write_r10b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r12, &cannoli_memhook_write_r10b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r13, &cannoli_memhook_write_r10b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r14, &cannoli_memhook_write_r10b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r10b_r15, &cannoli_memhook_write_r10b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11b_rax, &cannoli_memhook_write_r11b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rcx, &cannoli_memhook_write_r11b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rdx, &cannoli_memhook_write_r11b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rbx, &cannoli_memhook_write_r11b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rsp, &cannoli_memhook_write_r11b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rbp, &cannoli_memhook_write_r11b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rsi, &cannoli_memhook_write_r11b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r11b_rdi, &cannoli_memhook_write_r11b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r8, &cannoli_memhook_write_r11b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r9, &cannoli_memhook_write_r11b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r10, &cannoli_memhook_write_r11b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r11, &cannoli_memhook_write_r11b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r12, &cannoli_memhook_write_r11b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r13, &cannoli_memhook_write_r11b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r14, &cannoli_memhook_write_r11b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r11b_r15, &cannoli_memhook_write_r11b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12b_rax, &cannoli_memhook_write_r12b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rcx, &cannoli_memhook_write_r12b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rdx, &cannoli_memhook_write_r12b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rbx, &cannoli_memhook_write_r12b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rsp, &cannoli_memhook_write_r12b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rbp, &cannoli_memhook_write_r12b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rsi, &cannoli_memhook_write_r12b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r12b_rdi, &cannoli_memhook_write_r12b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r8, &cannoli_memhook_write_r12b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r9, &cannoli_memhook_write_r12b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r10, &cannoli_memhook_write_r12b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r11, &cannoli_memhook_write_r12b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r12, &cannoli_memhook_write_r12b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r13, &cannoli_memhook_write_r12b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r14, &cannoli_memhook_write_r12b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r12b_r15, &cannoli_memhook_write_r12b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13b_rax, &cannoli_memhook_write_r13b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rcx, &cannoli_memhook_write_r13b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rdx, &cannoli_memhook_write_r13b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rbx, &cannoli_memhook_write_r13b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rsp, &cannoli_memhook_write_r13b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rbp, &cannoli_memhook_write_r13b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rsi, &cannoli_memhook_write_r13b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r13b_rdi, &cannoli_memhook_write_r13b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r8, &cannoli_memhook_write_r13b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r9, &cannoli_memhook_write_r13b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r10, &cannoli_memhook_write_r13b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r11, &cannoli_memhook_write_r13b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r12, &cannoli_memhook_write_r13b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r13, &cannoli_memhook_write_r13b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r14, &cannoli_memhook_write_r13b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r13b_r15, &cannoli_memhook_write_r13b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14b_rax, &cannoli_memhook_write_r14b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rcx, &cannoli_memhook_write_r14b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rdx, &cannoli_memhook_write_r14b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rbx, &cannoli_memhook_write_r14b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rsp, &cannoli_memhook_write_r14b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rbp, &cannoli_memhook_write_r14b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rsi, &cannoli_memhook_write_r14b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r14b_rdi, &cannoli_memhook_write_r14b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r8, &cannoli_memhook_write_r14b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r9, &cannoli_memhook_write_r14b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r10, &cannoli_memhook_write_r14b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r11, &cannoli_memhook_write_r14b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r12, &cannoli_memhook_write_r14b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r13, &cannoli_memhook_write_r14b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r14, &cannoli_memhook_write_r14b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r14b_r15, &cannoli_memhook_write_r14b_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15b_rax, &cannoli_memhook_write_r15b_rax_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rcx, &cannoli_memhook_write_r15b_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rdx, &cannoli_memhook_write_r15b_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rbx, &cannoli_memhook_write_r15b_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rsp, &cannoli_memhook_write_r15b_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rbp, &cannoli_memhook_write_r15b_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rsi, &cannoli_memhook_write_r15b_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r15b_rdi, &cannoli_memhook_write_r15b_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r8, &cannoli_memhook_write_r15b_r8_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r9, &cannoli_memhook_write_r15b_r9_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r10, &cannoli_memhook_write_r15b_r10_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r11, &cannoli_memhook_write_r15b_r11_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r12, &cannoli_memhook_write_r15b_r12_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r13, &cannoli_memhook_write_r15b_r13_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r14, &cannoli_memhook_write_r15b_r14_end) },
                    unsafe { (&cannoli_memhook_write_r15b_r15, &cannoli_memhook_write_r15b_r15_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_ax_rax, &cannoli_memhook_read_ax_rax_end) },
                    unsafe { (&cannoli_memhook_read_ax_rcx, &cannoli_memhook_read_ax_rcx_end) },
                    unsafe { (&cannoli_memhook_read_ax_rdx, &cannoli_memhook_read_ax_rdx_end) },
                    unsafe { (&cannoli_memhook_read_ax_rbx, &cannoli_memhook_read_ax_rbx_end) },
                    unsafe { (&cannoli_memhook_read_ax_rsp, &cannoli_memhook_read_ax_rsp_end) },
                    unsafe { (&cannoli_memhook_read_ax_rbp, &cannoli_memhook_read_ax_rbp_end) },
                    unsafe { (&cannoli_memhook_read_ax_rsi, &cannoli_memhook_read_ax_rsi_end) },
                    unsafe { (&cannoli_memhook_read_ax_rdi, &cannoli_memhook_read_ax_rdi_end) },
                    unsafe { (&cannoli_memhook_read_ax_r8, &cannoli_memhook_read_ax_r8_end) },
                    unsafe { (&cannoli_memhook_read_ax_r9, &cannoli_memhook_read_ax_r9_end) },
                    unsafe { (&cannoli_memhook_read_ax_r10, &cannoli_memhook_read_ax_r10_end) },
                    unsafe { (&cannoli_memhook_read_ax_r11, &cannoli_memhook_read_ax_r11_end) },
                    unsafe { (&cannoli_memhook_read_ax_r12, &cannoli_memhook_read_ax_r12_end) },
                    unsafe { (&cannoli_memhook_read_ax_r13, &cannoli_memhook_read_ax_r13_end) },
                    unsafe { (&cannoli_memhook_read_ax_r14, &cannoli_memhook_read_ax_r14_end) },
                    unsafe { (&cannoli_memhook_read_ax_r15, &cannoli_memhook_read_ax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_cx_rax, &cannoli_memhook_read_cx_rax_end) },
                    unsafe { (&cannoli_memhook_read_cx_rcx, &cannoli_memhook_read_cx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_cx_rdx, &cannoli_memhook_read_cx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_cx_rbx, &cannoli_memhook_read_cx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_cx_rsp, &cannoli_memhook_read_cx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_cx_rbp, &cannoli_memhook_read_cx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_cx_rsi, &cannoli_memhook_read_cx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_cx_rdi, &cannoli_memhook_read_cx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_cx_r8, &cannoli_memhook_read_cx_r8_end) },
                    unsafe { (&cannoli_memhook_read_cx_r9, &cannoli_memhook_read_cx_r9_end) },
                    unsafe { (&cannoli_memhook_read_cx_r10, &cannoli_memhook_read_cx_r10_end) },
                    unsafe { (&cannoli_memhook_read_cx_r11, &cannoli_memhook_read_cx_r11_end) },
                    unsafe { (&cannoli_memhook_read_cx_r12, &cannoli_memhook_read_cx_r12_end) },
                    unsafe { (&cannoli_memhook_read_cx_r13, &cannoli_memhook_read_cx_r13_end) },
                    unsafe { (&cannoli_memhook_read_cx_r14, &cannoli_memhook_read_cx_r14_end) },
                    unsafe { (&cannoli_memhook_read_cx_r15, &cannoli_memhook_read_cx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_dx_rax, &cannoli_memhook_read_dx_rax_end) },
                    unsafe { (&cannoli_memhook_read_dx_rcx, &cannoli_memhook_read_dx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_dx_rdx, &cannoli_memhook_read_dx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_dx_rbx, &cannoli_memhook_read_dx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_dx_rsp, &cannoli_memhook_read_dx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_dx_rbp, &cannoli_memhook_read_dx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_dx_rsi, &cannoli_memhook_read_dx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_dx_rdi, &cannoli_memhook_read_dx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_dx_r8, &cannoli_memhook_read_dx_r8_end) },
                    unsafe { (&cannoli_memhook_read_dx_r9, &cannoli_memhook_read_dx_r9_end) },
                    unsafe { (&cannoli_memhook_read_dx_r10, &cannoli_memhook_read_dx_r10_end) },
                    unsafe { (&cannoli_memhook_read_dx_r11, &cannoli_memhook_read_dx_r11_end) },
                    unsafe { (&cannoli_memhook_read_dx_r12, &cannoli_memhook_read_dx_r12_end) },
                    unsafe { (&cannoli_memhook_read_dx_r13, &cannoli_memhook_read_dx_r13_end) },
                    unsafe { (&cannoli_memhook_read_dx_r14, &cannoli_memhook_read_dx_r14_end) },
                    unsafe { (&cannoli_memhook_read_dx_r15, &cannoli_memhook_read_dx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bx_rax, &cannoli_memhook_read_bx_rax_end) },
                    unsafe { (&cannoli_memhook_read_bx_rcx, &cannoli_memhook_read_bx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_bx_rdx, &cannoli_memhook_read_bx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_bx_rbx, &cannoli_memhook_read_bx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_bx_rsp, &cannoli_memhook_read_bx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_bx_rbp, &cannoli_memhook_read_bx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_bx_rsi, &cannoli_memhook_read_bx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_bx_rdi, &cannoli_memhook_read_bx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_bx_r8, &cannoli_memhook_read_bx_r8_end) },
                    unsafe { (&cannoli_memhook_read_bx_r9, &cannoli_memhook_read_bx_r9_end) },
                    unsafe { (&cannoli_memhook_read_bx_r10, &cannoli_memhook_read_bx_r10_end) },
                    unsafe { (&cannoli_memhook_read_bx_r11, &cannoli_memhook_read_bx_r11_end) },
                    unsafe { (&cannoli_memhook_read_bx_r12, &cannoli_memhook_read_bx_r12_end) },
                    unsafe { (&cannoli_memhook_read_bx_r13, &cannoli_memhook_read_bx_r13_end) },
                    unsafe { (&cannoli_memhook_read_bx_r14, &cannoli_memhook_read_bx_r14_end) },
                    unsafe { (&cannoli_memhook_read_bx_r15, &cannoli_memhook_read_bx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_sp_rax, &cannoli_memhook_read_sp_rax_end) },
                    unsafe { (&cannoli_memhook_read_sp_rcx, &cannoli_memhook_read_sp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_sp_rdx, &cannoli_memhook_read_sp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_sp_rbx, &cannoli_memhook_read_sp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_sp_rsp, &cannoli_memhook_read_sp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_sp_rbp, &cannoli_memhook_read_sp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_sp_rsi, &cannoli_memhook_read_sp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_sp_rdi, &cannoli_memhook_read_sp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_sp_r8, &cannoli_memhook_read_sp_r8_end) },
                    unsafe { (&cannoli_memhook_read_sp_r9, &cannoli_memhook_read_sp_r9_end) },
                    unsafe { (&cannoli_memhook_read_sp_r10, &cannoli_memhook_read_sp_r10_end) },
                    unsafe { (&cannoli_memhook_read_sp_r11, &cannoli_memhook_read_sp_r11_end) },
                    unsafe { (&cannoli_memhook_read_sp_r12, &cannoli_memhook_read_sp_r12_end) },
                    unsafe { (&cannoli_memhook_read_sp_r13, &cannoli_memhook_read_sp_r13_end) },
                    unsafe { (&cannoli_memhook_read_sp_r14, &cannoli_memhook_read_sp_r14_end) },
                    unsafe { (&cannoli_memhook_read_sp_r15, &cannoli_memhook_read_sp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_bp_rax, &cannoli_memhook_read_bp_rax_end) },
                    unsafe { (&cannoli_memhook_read_bp_rcx, &cannoli_memhook_read_bp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_bp_rdx, &cannoli_memhook_read_bp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_bp_rbx, &cannoli_memhook_read_bp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_bp_rsp, &cannoli_memhook_read_bp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_bp_rbp, &cannoli_memhook_read_bp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_bp_rsi, &cannoli_memhook_read_bp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_bp_rdi, &cannoli_memhook_read_bp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_bp_r8, &cannoli_memhook_read_bp_r8_end) },
                    unsafe { (&cannoli_memhook_read_bp_r9, &cannoli_memhook_read_bp_r9_end) },
                    unsafe { (&cannoli_memhook_read_bp_r10, &cannoli_memhook_read_bp_r10_end) },
                    unsafe { (&cannoli_memhook_read_bp_r11, &cannoli_memhook_read_bp_r11_end) },
                    unsafe { (&cannoli_memhook_read_bp_r12, &cannoli_memhook_read_bp_r12_end) },
                    unsafe { (&cannoli_memhook_read_bp_r13, &cannoli_memhook_read_bp_r13_end) },
                    unsafe { (&cannoli_memhook_read_bp_r14, &cannoli_memhook_read_bp_r14_end) },
                    unsafe { (&cannoli_memhook_read_bp_r15, &cannoli_memhook_read_bp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_si_rax, &cannoli_memhook_read_si_rax_end) },
                    unsafe { (&cannoli_memhook_read_si_rcx, &cannoli_memhook_read_si_rcx_end) },
                    unsafe { (&cannoli_memhook_read_si_rdx, &cannoli_memhook_read_si_rdx_end) },
                    unsafe { (&cannoli_memhook_read_si_rbx, &cannoli_memhook_read_si_rbx_end) },
                    unsafe { (&cannoli_memhook_read_si_rsp, &cannoli_memhook_read_si_rsp_end) },
                    unsafe { (&cannoli_memhook_read_si_rbp, &cannoli_memhook_read_si_rbp_end) },
                    unsafe { (&cannoli_memhook_read_si_rsi, &cannoli_memhook_read_si_rsi_end) },
                    unsafe { (&cannoli_memhook_read_si_rdi, &cannoli_memhook_read_si_rdi_end) },
                    unsafe { (&cannoli_memhook_read_si_r8, &cannoli_memhook_read_si_r8_end) },
                    unsafe { (&cannoli_memhook_read_si_r9, &cannoli_memhook_read_si_r9_end) },
                    unsafe { (&cannoli_memhook_read_si_r10, &cannoli_memhook_read_si_r10_end) },
                    unsafe { (&cannoli_memhook_read_si_r11, &cannoli_memhook_read_si_r11_end) },
                    unsafe { (&cannoli_memhook_read_si_r12, &cannoli_memhook_read_si_r12_end) },
                    unsafe { (&cannoli_memhook_read_si_r13, &cannoli_memhook_read_si_r13_end) },
                    unsafe { (&cannoli_memhook_read_si_r14, &cannoli_memhook_read_si_r14_end) },
                    unsafe { (&cannoli_memhook_read_si_r15, &cannoli_memhook_read_si_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_di_rax, &cannoli_memhook_read_di_rax_end) },
                    unsafe { (&cannoli_memhook_read_di_rcx, &cannoli_memhook_read_di_rcx_end) },
                    unsafe { (&cannoli_memhook_read_di_rdx, &cannoli_memhook_read_di_rdx_end) },
                    unsafe { (&cannoli_memhook_read_di_rbx, &cannoli_memhook_read_di_rbx_end) },
                    unsafe { (&cannoli_memhook_read_di_rsp, &cannoli_memhook_read_di_rsp_end) },
                    unsafe { (&cannoli_memhook_read_di_rbp, &cannoli_memhook_read_di_rbp_end) },
                    unsafe { (&cannoli_memhook_read_di_rsi, &cannoli_memhook_read_di_rsi_end) },
                    unsafe { (&cannoli_memhook_read_di_rdi, &cannoli_memhook_read_di_rdi_end) },
                    unsafe { (&cannoli_memhook_read_di_r8, &cannoli_memhook_read_di_r8_end) },
                    unsafe { (&cannoli_memhook_read_di_r9, &cannoli_memhook_read_di_r9_end) },
                    unsafe { (&cannoli_memhook_read_di_r10, &cannoli_memhook_read_di_r10_end) },
                    unsafe { (&cannoli_memhook_read_di_r11, &cannoli_memhook_read_di_r11_end) },
                    unsafe { (&cannoli_memhook_read_di_r12, &cannoli_memhook_read_di_r12_end) },
                    unsafe { (&cannoli_memhook_read_di_r13, &cannoli_memhook_read_di_r13_end) },
                    unsafe { (&cannoli_memhook_read_di_r14, &cannoli_memhook_read_di_r14_end) },
                    unsafe { (&cannoli_memhook_read_di_r15, &cannoli_memhook_read_di_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8w_rax, &cannoli_memhook_read_r8w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rcx, &cannoli_memhook_read_r8w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rdx, &cannoli_memhook_read_r8w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rbx, &cannoli_memhook_read_r8w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rsp, &cannoli_memhook_read_r8w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rbp, &cannoli_memhook_read_r8w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rsi, &cannoli_memhook_read_r8w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r8w_rdi, &cannoli_memhook_read_r8w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r8, &cannoli_memhook_read_r8w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r9, &cannoli_memhook_read_r8w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r10, &cannoli_memhook_read_r8w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r11, &cannoli_memhook_read_r8w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r12, &cannoli_memhook_read_r8w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r13, &cannoli_memhook_read_r8w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r14, &cannoli_memhook_read_r8w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r8w_r15, &cannoli_memhook_read_r8w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9w_rax, &cannoli_memhook_read_r9w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rcx, &cannoli_memhook_read_r9w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rdx, &cannoli_memhook_read_r9w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rbx, &cannoli_memhook_read_r9w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rsp, &cannoli_memhook_read_r9w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rbp, &cannoli_memhook_read_r9w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rsi, &cannoli_memhook_read_r9w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r9w_rdi, &cannoli_memhook_read_r9w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r8, &cannoli_memhook_read_r9w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r9, &cannoli_memhook_read_r9w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r10, &cannoli_memhook_read_r9w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r11, &cannoli_memhook_read_r9w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r12, &cannoli_memhook_read_r9w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r13, &cannoli_memhook_read_r9w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r14, &cannoli_memhook_read_r9w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r9w_r15, &cannoli_memhook_read_r9w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10w_rax, &cannoli_memhook_read_r10w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rcx, &cannoli_memhook_read_r10w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rdx, &cannoli_memhook_read_r10w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rbx, &cannoli_memhook_read_r10w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rsp, &cannoli_memhook_read_r10w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rbp, &cannoli_memhook_read_r10w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rsi, &cannoli_memhook_read_r10w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r10w_rdi, &cannoli_memhook_read_r10w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r8, &cannoli_memhook_read_r10w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r9, &cannoli_memhook_read_r10w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r10, &cannoli_memhook_read_r10w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r11, &cannoli_memhook_read_r10w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r12, &cannoli_memhook_read_r10w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r13, &cannoli_memhook_read_r10w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r14, &cannoli_memhook_read_r10w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r10w_r15, &cannoli_memhook_read_r10w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11w_rax, &cannoli_memhook_read_r11w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rcx, &cannoli_memhook_read_r11w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rdx, &cannoli_memhook_read_r11w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rbx, &cannoli_memhook_read_r11w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rsp, &cannoli_memhook_read_r11w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rbp, &cannoli_memhook_read_r11w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rsi, &cannoli_memhook_read_r11w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r11w_rdi, &cannoli_memhook_read_r11w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r8, &cannoli_memhook_read_r11w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r9, &cannoli_memhook_read_r11w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r10, &cannoli_memhook_read_r11w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r11, &cannoli_memhook_read_r11w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r12, &cannoli_memhook_read_r11w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r13, &cannoli_memhook_read_r11w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r14, &cannoli_memhook_read_r11w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r11w_r15, &cannoli_memhook_read_r11w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12w_rax, &cannoli_memhook_read_r12w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rcx, &cannoli_memhook_read_r12w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rdx, &cannoli_memhook_read_r12w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rbx, &cannoli_memhook_read_r12w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rsp, &cannoli_memhook_read_r12w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rbp, &cannoli_memhook_read_r12w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rsi, &cannoli_memhook_read_r12w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r12w_rdi, &cannoli_memhook_read_r12w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r8, &cannoli_memhook_read_r12w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r9, &cannoli_memhook_read_r12w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r10, &cannoli_memhook_read_r12w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r11, &cannoli_memhook_read_r12w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r12, &cannoli_memhook_read_r12w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r13, &cannoli_memhook_read_r12w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r14, &cannoli_memhook_read_r12w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r12w_r15, &cannoli_memhook_read_r12w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13w_rax, &cannoli_memhook_read_r13w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rcx, &cannoli_memhook_read_r13w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rdx, &cannoli_memhook_read_r13w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rbx, &cannoli_memhook_read_r13w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rsp, &cannoli_memhook_read_r13w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rbp, &cannoli_memhook_read_r13w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rsi, &cannoli_memhook_read_r13w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r13w_rdi, &cannoli_memhook_read_r13w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r8, &cannoli_memhook_read_r13w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r9, &cannoli_memhook_read_r13w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r10, &cannoli_memhook_read_r13w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r11, &cannoli_memhook_read_r13w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r12, &cannoli_memhook_read_r13w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r13, &cannoli_memhook_read_r13w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r14, &cannoli_memhook_read_r13w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r13w_r15, &cannoli_memhook_read_r13w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14w_rax, &cannoli_memhook_read_r14w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rcx, &cannoli_memhook_read_r14w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rdx, &cannoli_memhook_read_r14w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rbx, &cannoli_memhook_read_r14w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rsp, &cannoli_memhook_read_r14w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rbp, &cannoli_memhook_read_r14w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rsi, &cannoli_memhook_read_r14w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r14w_rdi, &cannoli_memhook_read_r14w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r8, &cannoli_memhook_read_r14w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r9, &cannoli_memhook_read_r14w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r10, &cannoli_memhook_read_r14w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r11, &cannoli_memhook_read_r14w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r12, &cannoli_memhook_read_r14w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r13, &cannoli_memhook_read_r14w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r14, &cannoli_memhook_read_r14w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r14w_r15, &cannoli_memhook_read_r14w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15w_rax, &cannoli_memhook_read_r15w_rax_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rcx, &cannoli_memhook_read_r15w_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rdx, &cannoli_memhook_read_r15w_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rbx, &cannoli_memhook_read_r15w_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rsp, &cannoli_memhook_read_r15w_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rbp, &cannoli_memhook_read_r15w_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rsi, &cannoli_memhook_read_r15w_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r15w_rdi, &cannoli_memhook_read_r15w_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r8, &cannoli_memhook_read_r15w_r8_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r9, &cannoli_memhook_read_r15w_r9_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r10, &cannoli_memhook_read_r15w_r10_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r11, &cannoli_memhook_read_r15w_r11_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r12, &cannoli_memhook_read_r15w_r12_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r13, &cannoli_memhook_read_r15w_r13_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r14, &cannoli_memhook_read_r15w_r14_end) },
                    unsafe { (&cannoli_memhook_read_r15w_r15, &cannoli_memhook_read_r15w_r15_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_ax_rax, &cannoli_memhook_write_ax_rax_end) },
                    unsafe { (&cannoli_memhook_write_ax_rcx, &cannoli_memhook_write_ax_rcx_end) },
                    unsafe { (&cannoli_memhook_write_ax_rdx, &cannoli_memhook_write_ax_rdx_end) },
                    unsafe { (&cannoli_memhook_write_ax_rbx, &cannoli_memhook_write_ax_rbx_end) },
                    unsafe { (&cannoli_memhook_write_ax_rsp, &cannoli_memhook_write_ax_rsp_end) },
                    unsafe { (&cannoli_memhook_write_ax_rbp, &cannoli_memhook_write_ax_rbp_end) },
                    unsafe { (&cannoli_memhook_write_ax_rsi, &cannoli_memhook_write_ax_rsi_end) },
                    unsafe { (&cannoli_memhook_write_ax_rdi, &cannoli_memhook_write_ax_rdi_end) },
                    unsafe { (&cannoli_memhook_write_ax_r8, &cannoli_memhook_write_ax_r8_end) },
                    unsafe { (&cannoli_memhook_write_ax_r9, &cannoli_memhook_write_ax_r9_end) },
                    unsafe { (&cannoli_memhook_write_ax_r10, &cannoli_memhook_write_ax_r10_end) },
                    unsafe { (&cannoli_memhook_write_ax_r11, &cannoli_memhook_write_ax_r11_end) },
                    unsafe { (&cannoli_memhook_write_ax_r12, &cannoli_memhook_write_ax_r12_end) },
                    unsafe { (&cannoli_memhook_write_ax_r13, &cannoli_memhook_write_ax_r13_end) },
                    unsafe { (&cannoli_memhook_write_ax_r14, &cannoli_memhook_write_ax_r14_end) },
                    unsafe { (&cannoli_memhook_write_ax_r15, &cannoli_memhook_write_ax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_cx_rax, &cannoli_memhook_write_cx_rax_end) },
                    unsafe { (&cannoli_memhook_write_cx_rcx, &cannoli_memhook_write_cx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_cx_rdx, &cannoli_memhook_write_cx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_cx_rbx, &cannoli_memhook_write_cx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_cx_rsp, &cannoli_memhook_write_cx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_cx_rbp, &cannoli_memhook_write_cx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_cx_rsi, &cannoli_memhook_write_cx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_cx_rdi, &cannoli_memhook_write_cx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_cx_r8, &cannoli_memhook_write_cx_r8_end) },
                    unsafe { (&cannoli_memhook_write_cx_r9, &cannoli_memhook_write_cx_r9_end) },
                    unsafe { (&cannoli_memhook_write_cx_r10, &cannoli_memhook_write_cx_r10_end) },
                    unsafe { (&cannoli_memhook_write_cx_r11, &cannoli_memhook_write_cx_r11_end) },
                    unsafe { (&cannoli_memhook_write_cx_r12, &cannoli_memhook_write_cx_r12_end) },
                    unsafe { (&cannoli_memhook_write_cx_r13, &cannoli_memhook_write_cx_r13_end) },
                    unsafe { (&cannoli_memhook_write_cx_r14, &cannoli_memhook_write_cx_r14_end) },
                    unsafe { (&cannoli_memhook_write_cx_r15, &cannoli_memhook_write_cx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_dx_rax, &cannoli_memhook_write_dx_rax_end) },
                    unsafe { (&cannoli_memhook_write_dx_rcx, &cannoli_memhook_write_dx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_dx_rdx, &cannoli_memhook_write_dx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_dx_rbx, &cannoli_memhook_write_dx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_dx_rsp, &cannoli_memhook_write_dx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_dx_rbp, &cannoli_memhook_write_dx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_dx_rsi, &cannoli_memhook_write_dx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_dx_rdi, &cannoli_memhook_write_dx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_dx_r8, &cannoli_memhook_write_dx_r8_end) },
                    unsafe { (&cannoli_memhook_write_dx_r9, &cannoli_memhook_write_dx_r9_end) },
                    unsafe { (&cannoli_memhook_write_dx_r10, &cannoli_memhook_write_dx_r10_end) },
                    unsafe { (&cannoli_memhook_write_dx_r11, &cannoli_memhook_write_dx_r11_end) },
                    unsafe { (&cannoli_memhook_write_dx_r12, &cannoli_memhook_write_dx_r12_end) },
                    unsafe { (&cannoli_memhook_write_dx_r13, &cannoli_memhook_write_dx_r13_end) },
                    unsafe { (&cannoli_memhook_write_dx_r14, &cannoli_memhook_write_dx_r14_end) },
                    unsafe { (&cannoli_memhook_write_dx_r15, &cannoli_memhook_write_dx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bx_rax, &cannoli_memhook_write_bx_rax_end) },
                    unsafe { (&cannoli_memhook_write_bx_rcx, &cannoli_memhook_write_bx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_bx_rdx, &cannoli_memhook_write_bx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_bx_rbx, &cannoli_memhook_write_bx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_bx_rsp, &cannoli_memhook_write_bx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_bx_rbp, &cannoli_memhook_write_bx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_bx_rsi, &cannoli_memhook_write_bx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_bx_rdi, &cannoli_memhook_write_bx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_bx_r8, &cannoli_memhook_write_bx_r8_end) },
                    unsafe { (&cannoli_memhook_write_bx_r9, &cannoli_memhook_write_bx_r9_end) },
                    unsafe { (&cannoli_memhook_write_bx_r10, &cannoli_memhook_write_bx_r10_end) },
                    unsafe { (&cannoli_memhook_write_bx_r11, &cannoli_memhook_write_bx_r11_end) },
                    unsafe { (&cannoli_memhook_write_bx_r12, &cannoli_memhook_write_bx_r12_end) },
                    unsafe { (&cannoli_memhook_write_bx_r13, &cannoli_memhook_write_bx_r13_end) },
                    unsafe { (&cannoli_memhook_write_bx_r14, &cannoli_memhook_write_bx_r14_end) },
                    unsafe { (&cannoli_memhook_write_bx_r15, &cannoli_memhook_write_bx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_sp_rax, &cannoli_memhook_write_sp_rax_end) },
                    unsafe { (&cannoli_memhook_write_sp_rcx, &cannoli_memhook_write_sp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_sp_rdx, &cannoli_memhook_write_sp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_sp_rbx, &cannoli_memhook_write_sp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_sp_rsp, &cannoli_memhook_write_sp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_sp_rbp, &cannoli_memhook_write_sp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_sp_rsi, &cannoli_memhook_write_sp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_sp_rdi, &cannoli_memhook_write_sp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_sp_r8, &cannoli_memhook_write_sp_r8_end) },
                    unsafe { (&cannoli_memhook_write_sp_r9, &cannoli_memhook_write_sp_r9_end) },
                    unsafe { (&cannoli_memhook_write_sp_r10, &cannoli_memhook_write_sp_r10_end) },
                    unsafe { (&cannoli_memhook_write_sp_r11, &cannoli_memhook_write_sp_r11_end) },
                    unsafe { (&cannoli_memhook_write_sp_r12, &cannoli_memhook_write_sp_r12_end) },
                    unsafe { (&cannoli_memhook_write_sp_r13, &cannoli_memhook_write_sp_r13_end) },
                    unsafe { (&cannoli_memhook_write_sp_r14, &cannoli_memhook_write_sp_r14_end) },
                    unsafe { (&cannoli_memhook_write_sp_r15, &cannoli_memhook_write_sp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_bp_rax, &cannoli_memhook_write_bp_rax_end) },
                    unsafe { (&cannoli_memhook_write_bp_rcx, &cannoli_memhook_write_bp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_bp_rdx, &cannoli_memhook_write_bp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_bp_rbx, &cannoli_memhook_write_bp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_bp_rsp, &cannoli_memhook_write_bp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_bp_rbp, &cannoli_memhook_write_bp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_bp_rsi, &cannoli_memhook_write_bp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_bp_rdi, &cannoli_memhook_write_bp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_bp_r8, &cannoli_memhook_write_bp_r8_end) },
                    unsafe { (&cannoli_memhook_write_bp_r9, &cannoli_memhook_write_bp_r9_end) },
                    unsafe { (&cannoli_memhook_write_bp_r10, &cannoli_memhook_write_bp_r10_end) },
                    unsafe { (&cannoli_memhook_write_bp_r11, &cannoli_memhook_write_bp_r11_end) },
                    unsafe { (&cannoli_memhook_write_bp_r12, &cannoli_memhook_write_bp_r12_end) },
                    unsafe { (&cannoli_memhook_write_bp_r13, &cannoli_memhook_write_bp_r13_end) },
                    unsafe { (&cannoli_memhook_write_bp_r14, &cannoli_memhook_write_bp_r14_end) },
                    unsafe { (&cannoli_memhook_write_bp_r15, &cannoli_memhook_write_bp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_si_rax, &cannoli_memhook_write_si_rax_end) },
                    unsafe { (&cannoli_memhook_write_si_rcx, &cannoli_memhook_write_si_rcx_end) },
                    unsafe { (&cannoli_memhook_write_si_rdx, &cannoli_memhook_write_si_rdx_end) },
                    unsafe { (&cannoli_memhook_write_si_rbx, &cannoli_memhook_write_si_rbx_end) },
                    unsafe { (&cannoli_memhook_write_si_rsp, &cannoli_memhook_write_si_rsp_end) },
                    unsafe { (&cannoli_memhook_write_si_rbp, &cannoli_memhook_write_si_rbp_end) },
                    unsafe { (&cannoli_memhook_write_si_rsi, &cannoli_memhook_write_si_rsi_end) },
                    unsafe { (&cannoli_memhook_write_si_rdi, &cannoli_memhook_write_si_rdi_end) },
                    unsafe { (&cannoli_memhook_write_si_r8, &cannoli_memhook_write_si_r8_end) },
                    unsafe { (&cannoli_memhook_write_si_r9, &cannoli_memhook_write_si_r9_end) },
                    unsafe { (&cannoli_memhook_write_si_r10, &cannoli_memhook_write_si_r10_end) },
                    unsafe { (&cannoli_memhook_write_si_r11, &cannoli_memhook_write_si_r11_end) },
                    unsafe { (&cannoli_memhook_write_si_r12, &cannoli_memhook_write_si_r12_end) },
                    unsafe { (&cannoli_memhook_write_si_r13, &cannoli_memhook_write_si_r13_end) },
                    unsafe { (&cannoli_memhook_write_si_r14, &cannoli_memhook_write_si_r14_end) },
                    unsafe { (&cannoli_memhook_write_si_r15, &cannoli_memhook_write_si_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_di_rax, &cannoli_memhook_write_di_rax_end) },
                    unsafe { (&cannoli_memhook_write_di_rcx, &cannoli_memhook_write_di_rcx_end) },
                    unsafe { (&cannoli_memhook_write_di_rdx, &cannoli_memhook_write_di_rdx_end) },
                    unsafe { (&cannoli_memhook_write_di_rbx, &cannoli_memhook_write_di_rbx_end) },
                    unsafe { (&cannoli_memhook_write_di_rsp, &cannoli_memhook_write_di_rsp_end) },
                    unsafe { (&cannoli_memhook_write_di_rbp, &cannoli_memhook_write_di_rbp_end) },
                    unsafe { (&cannoli_memhook_write_di_rsi, &cannoli_memhook_write_di_rsi_end) },
                    unsafe { (&cannoli_memhook_write_di_rdi, &cannoli_memhook_write_di_rdi_end) },
                    unsafe { (&cannoli_memhook_write_di_r8, &cannoli_memhook_write_di_r8_end) },
                    unsafe { (&cannoli_memhook_write_di_r9, &cannoli_memhook_write_di_r9_end) },
                    unsafe { (&cannoli_memhook_write_di_r10, &cannoli_memhook_write_di_r10_end) },
                    unsafe { (&cannoli_memhook_write_di_r11, &cannoli_memhook_write_di_r11_end) },
                    unsafe { (&cannoli_memhook_write_di_r12, &cannoli_memhook_write_di_r12_end) },
                    unsafe { (&cannoli_memhook_write_di_r13, &cannoli_memhook_write_di_r13_end) },
                    unsafe { (&cannoli_memhook_write_di_r14, &cannoli_memhook_write_di_r14_end) },
                    unsafe { (&cannoli_memhook_write_di_r15, &cannoli_memhook_write_di_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8w_rax, &cannoli_memhook_write_r8w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rcx, &cannoli_memhook_write_r8w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rdx, &cannoli_memhook_write_r8w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rbx, &cannoli_memhook_write_r8w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rsp, &cannoli_memhook_write_r8w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rbp, &cannoli_memhook_write_r8w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rsi, &cannoli_memhook_write_r8w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r8w_rdi, &cannoli_memhook_write_r8w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r8, &cannoli_memhook_write_r8w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r9, &cannoli_memhook_write_r8w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r10, &cannoli_memhook_write_r8w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r11, &cannoli_memhook_write_r8w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r12, &cannoli_memhook_write_r8w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r13, &cannoli_memhook_write_r8w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r14, &cannoli_memhook_write_r8w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r8w_r15, &cannoli_memhook_write_r8w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9w_rax, &cannoli_memhook_write_r9w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rcx, &cannoli_memhook_write_r9w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rdx, &cannoli_memhook_write_r9w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rbx, &cannoli_memhook_write_r9w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rsp, &cannoli_memhook_write_r9w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rbp, &cannoli_memhook_write_r9w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rsi, &cannoli_memhook_write_r9w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r9w_rdi, &cannoli_memhook_write_r9w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r8, &cannoli_memhook_write_r9w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r9, &cannoli_memhook_write_r9w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r10, &cannoli_memhook_write_r9w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r11, &cannoli_memhook_write_r9w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r12, &cannoli_memhook_write_r9w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r13, &cannoli_memhook_write_r9w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r14, &cannoli_memhook_write_r9w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r9w_r15, &cannoli_memhook_write_r9w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10w_rax, &cannoli_memhook_write_r10w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rcx, &cannoli_memhook_write_r10w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rdx, &cannoli_memhook_write_r10w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rbx, &cannoli_memhook_write_r10w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rsp, &cannoli_memhook_write_r10w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rbp, &cannoli_memhook_write_r10w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rsi, &cannoli_memhook_write_r10w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r10w_rdi, &cannoli_memhook_write_r10w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r8, &cannoli_memhook_write_r10w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r9, &cannoli_memhook_write_r10w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r10, &cannoli_memhook_write_r10w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r11, &cannoli_memhook_write_r10w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r12, &cannoli_memhook_write_r10w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r13, &cannoli_memhook_write_r10w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r14, &cannoli_memhook_write_r10w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r10w_r15, &cannoli_memhook_write_r10w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11w_rax, &cannoli_memhook_write_r11w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rcx, &cannoli_memhook_write_r11w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rdx, &cannoli_memhook_write_r11w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rbx, &cannoli_memhook_write_r11w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rsp, &cannoli_memhook_write_r11w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rbp, &cannoli_memhook_write_r11w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rsi, &cannoli_memhook_write_r11w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r11w_rdi, &cannoli_memhook_write_r11w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r8, &cannoli_memhook_write_r11w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r9, &cannoli_memhook_write_r11w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r10, &cannoli_memhook_write_r11w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r11, &cannoli_memhook_write_r11w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r12, &cannoli_memhook_write_r11w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r13, &cannoli_memhook_write_r11w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r14, &cannoli_memhook_write_r11w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r11w_r15, &cannoli_memhook_write_r11w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12w_rax, &cannoli_memhook_write_r12w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rcx, &cannoli_memhook_write_r12w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rdx, &cannoli_memhook_write_r12w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rbx, &cannoli_memhook_write_r12w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rsp, &cannoli_memhook_write_r12w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rbp, &cannoli_memhook_write_r12w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rsi, &cannoli_memhook_write_r12w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r12w_rdi, &cannoli_memhook_write_r12w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r8, &cannoli_memhook_write_r12w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r9, &cannoli_memhook_write_r12w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r10, &cannoli_memhook_write_r12w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r11, &cannoli_memhook_write_r12w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r12, &cannoli_memhook_write_r12w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r13, &cannoli_memhook_write_r12w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r14, &cannoli_memhook_write_r12w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r12w_r15, &cannoli_memhook_write_r12w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13w_rax, &cannoli_memhook_write_r13w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rcx, &cannoli_memhook_write_r13w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rdx, &cannoli_memhook_write_r13w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rbx, &cannoli_memhook_write_r13w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rsp, &cannoli_memhook_write_r13w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rbp, &cannoli_memhook_write_r13w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rsi, &cannoli_memhook_write_r13w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r13w_rdi, &cannoli_memhook_write_r13w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r8, &cannoli_memhook_write_r13w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r9, &cannoli_memhook_write_r13w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r10, &cannoli_memhook_write_r13w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r11, &cannoli_memhook_write_r13w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r12, &cannoli_memhook_write_r13w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r13, &cannoli_memhook_write_r13w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r14, &cannoli_memhook_write_r13w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r13w_r15, &cannoli_memhook_write_r13w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14w_rax, &cannoli_memhook_write_r14w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rcx, &cannoli_memhook_write_r14w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rdx, &cannoli_memhook_write_r14w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rbx, &cannoli_memhook_write_r14w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rsp, &cannoli_memhook_write_r14w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rbp, &cannoli_memhook_write_r14w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rsi, &cannoli_memhook_write_r14w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r14w_rdi, &cannoli_memhook_write_r14w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r8, &cannoli_memhook_write_r14w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r9, &cannoli_memhook_write_r14w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r10, &cannoli_memhook_write_r14w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r11, &cannoli_memhook_write_r14w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r12, &cannoli_memhook_write_r14w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r13, &cannoli_memhook_write_r14w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r14, &cannoli_memhook_write_r14w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r14w_r15, &cannoli_memhook_write_r14w_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15w_rax, &cannoli_memhook_write_r15w_rax_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rcx, &cannoli_memhook_write_r15w_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rdx, &cannoli_memhook_write_r15w_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rbx, &cannoli_memhook_write_r15w_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rsp, &cannoli_memhook_write_r15w_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rbp, &cannoli_memhook_write_r15w_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rsi, &cannoli_memhook_write_r15w_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r15w_rdi, &cannoli_memhook_write_r15w_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r8, &cannoli_memhook_write_r15w_r8_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r9, &cannoli_memhook_write_r15w_r9_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r10, &cannoli_memhook_write_r15w_r10_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r11, &cannoli_memhook_write_r15w_r11_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r12, &cannoli_memhook_write_r15w_r12_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r13, &cannoli_memhook_write_r15w_r13_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r14, &cannoli_memhook_write_r15w_r14_end) },
                    unsafe { (&cannoli_memhook_write_r15w_r15, &cannoli_memhook_write_r15w_r15_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_eax_rax, &cannoli_memhook_read_eax_rax_end) },
                    unsafe { (&cannoli_memhook_read_eax_rcx, &cannoli_memhook_read_eax_rcx_end) },
                    unsafe { (&cannoli_memhook_read_eax_rdx, &cannoli_memhook_read_eax_rdx_end) },
                    unsafe { (&cannoli_memhook_read_eax_rbx, &cannoli_memhook_read_eax_rbx_end) },
                    unsafe { (&cannoli_memhook_read_eax_rsp, &cannoli_memhook_read_eax_rsp_end) },
                    unsafe { (&cannoli_memhook_read_eax_rbp, &cannoli_memhook_read_eax_rbp_end) },
                    unsafe { (&cannoli_memhook_read_eax_rsi, &cannoli_memhook_read_eax_rsi_end) },
                    unsafe { (&cannoli_memhook_read_eax_rdi, &cannoli_memhook_read_eax_rdi_end) },
                    unsafe { (&cannoli_memhook_read_eax_r8, &cannoli_memhook_read_eax_r8_end) },
                    unsafe { (&cannoli_memhook_read_eax_r9, &cannoli_memhook_read_eax_r9_end) },
                    unsafe { (&cannoli_memhook_read_eax_r10, &cannoli_memhook_read_eax_r10_end) },
                    unsafe { (&cannoli_memhook_read_eax_r11, &cannoli_memhook_read_eax_r11_end) },
                    unsafe { (&cannoli_memhook_read_eax_r12, &cannoli_memhook_read_eax_r12_end) },
                    unsafe { (&cannoli_memhook_read_eax_r13, &cannoli_memhook_read_eax_r13_end) },
                    unsafe { (&cannoli_memhook_read_eax_r14, &cannoli_memhook_read_eax_r14_end) },
                    unsafe { (&cannoli_memhook_read_eax_r15, &cannoli_memhook_read_eax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ecx_rax, &cannoli_memhook_read_ecx_rax_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rcx, &cannoli_memhook_read_ecx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rdx, &cannoli_memhook_read_ecx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rbx, &cannoli_memhook_read_ecx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rsp, &cannoli_memhook_read_ecx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rbp, &cannoli_memhook_read_ecx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rsi, &cannoli_memhook_read_ecx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_ecx_rdi, &cannoli_memhook_read_ecx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r8, &cannoli_memhook_read_ecx_r8_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r9, &cannoli_memhook_read_ecx_r9_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r10, &cannoli_memhook_read_ecx_r10_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r11, &cannoli_memhook_read_ecx_r11_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r12, &cannoli_memhook_read_ecx_r12_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r13, &cannoli_memhook_read_ecx_r13_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r14, &cannoli_memhook_read_ecx_r14_end) },
                    unsafe { (&cannoli_memhook_read_ecx_r15, &cannoli_memhook_read_ecx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_edx_rax, &cannoli_memhook_read_edx_rax_end) },
                    unsafe { (&cannoli_memhook_read_edx_rcx, &cannoli_memhook_read_edx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_edx_rdx, &cannoli_memhook_read_edx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_edx_rbx, &cannoli_memhook_read_edx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_edx_rsp, &cannoli_memhook_read_edx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_edx_rbp, &cannoli_memhook_read_edx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_edx_rsi, &cannoli_memhook_read_edx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_edx_rdi, &cannoli_memhook_read_edx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_edx_r8, &cannoli_memhook_read_edx_r8_end) },
                    unsafe { (&cannoli_memhook_read_edx_r9, &cannoli_memhook_read_edx_r9_end) },
                    unsafe { (&cannoli_memhook_read_edx_r10, &cannoli_memhook_read_edx_r10_end) },
                    unsafe { (&cannoli_memhook_read_edx_r11, &cannoli_memhook_read_edx_r11_end) },
                    unsafe { (&cannoli_memhook_read_edx_r12, &cannoli_memhook_read_edx_r12_end) },
                    unsafe { (&cannoli_memhook_read_edx_r13, &cannoli_memhook_read_edx_r13_end) },
                    unsafe { (&cannoli_memhook_read_edx_r14, &cannoli_memhook_read_edx_r14_end) },
                    unsafe { (&cannoli_memhook_read_edx_r15, &cannoli_memhook_read_edx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ebx_rax, &cannoli_memhook_read_ebx_rax_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rcx, &cannoli_memhook_read_ebx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rdx, &cannoli_memhook_read_ebx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rbx, &cannoli_memhook_read_ebx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rsp, &cannoli_memhook_read_ebx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rbp, &cannoli_memhook_read_ebx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rsi, &cannoli_memhook_read_ebx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_ebx_rdi, &cannoli_memhook_read_ebx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r8, &cannoli_memhook_read_ebx_r8_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r9, &cannoli_memhook_read_ebx_r9_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r10, &cannoli_memhook_read_ebx_r10_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r11, &cannoli_memhook_read_ebx_r11_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r12, &cannoli_memhook_read_ebx_r12_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r13, &cannoli_memhook_read_ebx_r13_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r14, &cannoli_memhook_read_ebx_r14_end) },
                    unsafe { (&cannoli_memhook_read_ebx_r15, &cannoli_memhook_read_ebx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_esp_rax, &cannoli_memhook_read_esp_rax_end) },
                    unsafe { (&cannoli_memhook_read_esp_rcx, &cannoli_memhook_read_esp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_esp_rdx, &cannoli_memhook_read_esp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_esp_rbx, &cannoli_memhook_read_esp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_esp_rsp, &cannoli_memhook_read_esp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_esp_rbp, &cannoli_memhook_read_esp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_esp_rsi, &cannoli_memhook_read_esp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_esp_rdi, &cannoli_memhook_read_esp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_esp_r8, &cannoli_memhook_read_esp_r8_end) },
                    unsafe { (&cannoli_memhook_read_esp_r9, &cannoli_memhook_read_esp_r9_end) },
                    unsafe { (&cannoli_memhook_read_esp_r10, &cannoli_memhook_read_esp_r10_end) },
                    unsafe { (&cannoli_memhook_read_esp_r11, &cannoli_memhook_read_esp_r11_end) },
                    unsafe { (&cannoli_memhook_read_esp_r12, &cannoli_memhook_read_esp_r12_end) },
                    unsafe { (&cannoli_memhook_read_esp_r13, &cannoli_memhook_read_esp_r13_end) },
                    unsafe { (&cannoli_memhook_read_esp_r14, &cannoli_memhook_read_esp_r14_end) },
                    unsafe { (&cannoli_memhook_read_esp_r15, &cannoli_memhook_read_esp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_ebp_rax, &cannoli_memhook_read_ebp_rax_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rcx, &cannoli_memhook_read_ebp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rdx, &cannoli_memhook_read_ebp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rbx, &cannoli_memhook_read_ebp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rsp, &cannoli_memhook_read_ebp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rbp, &cannoli_memhook_read_ebp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rsi, &cannoli_memhook_read_ebp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_ebp_rdi, &cannoli_memhook_read_ebp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r8, &cannoli_memhook_read_ebp_r8_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r9, &cannoli_memhook_read_ebp_r9_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r10, &cannoli_memhook_read_ebp_r10_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r11, &cannoli_memhook_read_ebp_r11_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r12, &cannoli_memhook_read_ebp_r12_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r13, &cannoli_memhook_read_ebp_r13_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r14, &cannoli_memhook_read_ebp_r14_end) },
                    unsafe { (&cannoli_memhook_read_ebp_r15, &cannoli_memhook_read_ebp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_esi_rax, &cannoli_memhook_read_esi_rax_end) },
                    unsafe { (&cannoli_memhook_read_esi_rcx, &cannoli_memhook_read_esi_rcx_end) },
                    unsafe { (&cannoli_memhook_read_esi_rdx, &cannoli_memhook_read_esi_rdx_end) },
                    unsafe { (&cannoli_memhook_read_esi_rbx, &cannoli_memhook_read_esi_rbx_end) },
                    unsafe { (&cannoli_memhook_read_esi_rsp, &cannoli_memhook_read_esi_rsp_end) },
                    unsafe { (&cannoli_memhook_read_esi_rbp, &cannoli_memhook_read_esi_rbp_end) },
                    unsafe { (&cannoli_memhook_read_esi_rsi, &cannoli_memhook_read_esi_rsi_end) },
                    unsafe { (&cannoli_memhook_read_esi_rdi, &cannoli_memhook_read_esi_rdi_end) },
                    unsafe { (&cannoli_memhook_read_esi_r8, &cannoli_memhook_read_esi_r8_end) },
                    unsafe { (&cannoli_memhook_read_esi_r9, &cannoli_memhook_read_esi_r9_end) },
                    unsafe { (&cannoli_memhook_read_esi_r10, &cannoli_memhook_read_esi_r10_end) },
                    unsafe { (&cannoli_memhook_read_esi_r11, &cannoli_memhook_read_esi_r11_end) },
                    unsafe { (&cannoli_memhook_read_esi_r12, &cannoli_memhook_read_esi_r12_end) },
                    unsafe { (&cannoli_memhook_read_esi_r13, &cannoli_memhook_read_esi_r13_end) },
                    unsafe { (&cannoli_memhook_read_esi_r14, &cannoli_memhook_read_esi_r14_end) },
                    unsafe { (&cannoli_memhook_read_esi_r15, &cannoli_memhook_read_esi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_edi_rax, &cannoli_memhook_read_edi_rax_end) },
                    unsafe { (&cannoli_memhook_read_edi_rcx, &cannoli_memhook_read_edi_rcx_end) },
                    unsafe { (&cannoli_memhook_read_edi_rdx, &cannoli_memhook_read_edi_rdx_end) },
                    unsafe { (&cannoli_memhook_read_edi_rbx, &cannoli_memhook_read_edi_rbx_end) },
                    unsafe { (&cannoli_memhook_read_edi_rsp, &cannoli_memhook_read_edi_rsp_end) },
                    unsafe { (&cannoli_memhook_read_edi_rbp, &cannoli_memhook_read_edi_rbp_end) },
                    unsafe { (&cannoli_memhook_read_edi_rsi, &cannoli_memhook_read_edi_rsi_end) },
                    unsafe { (&cannoli_memhook_read_edi_rdi, &cannoli_memhook_read_edi_rdi_end) },
                    unsafe { (&cannoli_memhook_read_edi_r8, &cannoli_memhook_read_edi_r8_end) },
                    unsafe { (&cannoli_memhook_read_edi_r9, &cannoli_memhook_read_edi_r9_end) },
                    unsafe { (&cannoli_memhook_read_edi_r10, &cannoli_memhook_read_edi_r10_end) },
                    unsafe { (&cannoli_memhook_read_edi_r11, &cannoli_memhook_read_edi_r11_end) },
                    unsafe { (&cannoli_memhook_read_edi_r12, &cannoli_memhook_read_edi_r12_end) },
                    unsafe { (&cannoli_memhook_read_edi_r13, &cannoli_memhook_read_edi_r13_end) },
                    unsafe { (&cannoli_memhook_read_edi_r14, &cannoli_memhook_read_edi_r14_end) },
                    unsafe { (&cannoli_memhook_read_edi_r15, &cannoli_memhook_read_edi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8d_rax, &cannoli_memhook_read_r8d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rcx, &cannoli_memhook_read_r8d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rdx, &cannoli_memhook_read_r8d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rbx, &cannoli_memhook_read_r8d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rsp, &cannoli_memhook_read_r8d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rbp, &cannoli_memhook_read_r8d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rsi, &cannoli_memhook_read_r8d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r8d_rdi, &cannoli_memhook_read_r8d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r8, &cannoli_memhook_read_r8d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r9, &cannoli_memhook_read_r8d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r10, &cannoli_memhook_read_r8d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r11, &cannoli_memhook_read_r8d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r12, &cannoli_memhook_read_r8d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r13, &cannoli_memhook_read_r8d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r14, &cannoli_memhook_read_r8d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r8d_r15, &cannoli_memhook_read_r8d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9d_rax, &cannoli_memhook_read_r9d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rcx, &cannoli_memhook_read_r9d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rdx, &cannoli_memhook_read_r9d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rbx, &cannoli_memhook_read_r9d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rsp, &cannoli_memhook_read_r9d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rbp, &cannoli_memhook_read_r9d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rsi, &cannoli_memhook_read_r9d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r9d_rdi, &cannoli_memhook_read_r9d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r8, &cannoli_memhook_read_r9d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r9, &cannoli_memhook_read_r9d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r10, &cannoli_memhook_read_r9d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r11, &cannoli_memhook_read_r9d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r12, &cannoli_memhook_read_r9d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r13, &cannoli_memhook_read_r9d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r14, &cannoli_memhook_read_r9d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r9d_r15, &cannoli_memhook_read_r9d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10d_rax, &cannoli_memhook_read_r10d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rcx, &cannoli_memhook_read_r10d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rdx, &cannoli_memhook_read_r10d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rbx, &cannoli_memhook_read_r10d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rsp, &cannoli_memhook_read_r10d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rbp, &cannoli_memhook_read_r10d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rsi, &cannoli_memhook_read_r10d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r10d_rdi, &cannoli_memhook_read_r10d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r8, &cannoli_memhook_read_r10d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r9, &cannoli_memhook_read_r10d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r10, &cannoli_memhook_read_r10d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r11, &cannoli_memhook_read_r10d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r12, &cannoli_memhook_read_r10d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r13, &cannoli_memhook_read_r10d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r14, &cannoli_memhook_read_r10d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r10d_r15, &cannoli_memhook_read_r10d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11d_rax, &cannoli_memhook_read_r11d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rcx, &cannoli_memhook_read_r11d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rdx, &cannoli_memhook_read_r11d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rbx, &cannoli_memhook_read_r11d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rsp, &cannoli_memhook_read_r11d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rbp, &cannoli_memhook_read_r11d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rsi, &cannoli_memhook_read_r11d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r11d_rdi, &cannoli_memhook_read_r11d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r8, &cannoli_memhook_read_r11d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r9, &cannoli_memhook_read_r11d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r10, &cannoli_memhook_read_r11d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r11, &cannoli_memhook_read_r11d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r12, &cannoli_memhook_read_r11d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r13, &cannoli_memhook_read_r11d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r14, &cannoli_memhook_read_r11d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r11d_r15, &cannoli_memhook_read_r11d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12d_rax, &cannoli_memhook_read_r12d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rcx, &cannoli_memhook_read_r12d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rdx, &cannoli_memhook_read_r12d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rbx, &cannoli_memhook_read_r12d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rsp, &cannoli_memhook_read_r12d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rbp, &cannoli_memhook_read_r12d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rsi, &cannoli_memhook_read_r12d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r12d_rdi, &cannoli_memhook_read_r12d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r8, &cannoli_memhook_read_r12d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r9, &cannoli_memhook_read_r12d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r10, &cannoli_memhook_read_r12d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r11, &cannoli_memhook_read_r12d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r12, &cannoli_memhook_read_r12d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r13, &cannoli_memhook_read_r12d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r14, &cannoli_memhook_read_r12d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r12d_r15, &cannoli_memhook_read_r12d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13d_rax, &cannoli_memhook_read_r13d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rcx, &cannoli_memhook_read_r13d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rdx, &cannoli_memhook_read_r13d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rbx, &cannoli_memhook_read_r13d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rsp, &cannoli_memhook_read_r13d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rbp, &cannoli_memhook_read_r13d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rsi, &cannoli_memhook_read_r13d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r13d_rdi, &cannoli_memhook_read_r13d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r8, &cannoli_memhook_read_r13d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r9, &cannoli_memhook_read_r13d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r10, &cannoli_memhook_read_r13d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r11, &cannoli_memhook_read_r13d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r12, &cannoli_memhook_read_r13d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r13, &cannoli_memhook_read_r13d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r14, &cannoli_memhook_read_r13d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r13d_r15, &cannoli_memhook_read_r13d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14d_rax, &cannoli_memhook_read_r14d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rcx, &cannoli_memhook_read_r14d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rdx, &cannoli_memhook_read_r14d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rbx, &cannoli_memhook_read_r14d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rsp, &cannoli_memhook_read_r14d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rbp, &cannoli_memhook_read_r14d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rsi, &cannoli_memhook_read_r14d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r14d_rdi, &cannoli_memhook_read_r14d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r8, &cannoli_memhook_read_r14d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r9, &cannoli_memhook_read_r14d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r10, &cannoli_memhook_read_r14d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r11, &cannoli_memhook_read_r14d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r12, &cannoli_memhook_read_r14d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r13, &cannoli_memhook_read_r14d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r14, &cannoli_memhook_read_r14d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r14d_r15, &cannoli_memhook_read_r14d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15d_rax, &cannoli_memhook_read_r15d_rax_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rcx, &cannoli_memhook_read_r15d_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rdx, &cannoli_memhook_read_r15d_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rbx, &cannoli_memhook_read_r15d_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rsp, &cannoli_memhook_read_r15d_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rbp, &cannoli_memhook_read_r15d_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rsi, &cannoli_memhook_read_r15d_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r15d_rdi, &cannoli_memhook_read_r15d_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r8, &cannoli_memhook_read_r15d_r8_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r9, &cannoli_memhook_read_r15d_r9_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r10, &cannoli_memhook_read_r15d_r10_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r11, &cannoli_memhook_read_r15d_r11_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r12, &cannoli_memhook_read_r15d_r12_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r13, &cannoli_memhook_read_r15d_r13_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r14, &cannoli_memhook_read_r15d_r14_end) },
                    unsafe { (&cannoli_memhook_read_r15d_r15, &cannoli_memhook_read_r15d_r15_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_eax_rax, &cannoli_memhook_write_eax_rax_end) },
                    unsafe { (&cannoli_memhook_write_eax_rcx, &cannoli_memhook_write_eax_rcx_end) },
                    unsafe { (&cannoli_memhook_write_eax_rdx, &cannoli_memhook_write_eax_rdx_end) },
                    unsafe { (&cannoli_memhook_write_eax_rbx, &cannoli_memhook_write_eax_rbx_end) },
                    unsafe { (&cannoli_memhook_write_eax_rsp, &cannoli_memhook_write_eax_rsp_end) },
                    unsafe { (&cannoli_memhook_write_eax_rbp, &cannoli_memhook_write_eax_rbp_end) },
                    unsafe { (&cannoli_memhook_write_eax_rsi, &cannoli_memhook_write_eax_rsi_end) },
                    unsafe { (&cannoli_memhook_write_eax_rdi, &cannoli_memhook_write_eax_rdi_end) },
                    unsafe { (&cannoli_memhook_write_eax_r8, &cannoli_memhook_write_eax_r8_end) },
                    unsafe { (&cannoli_memhook_write_eax_r9, &cannoli_memhook_write_eax_r9_end) },
                    unsafe { (&cannoli_memhook_write_eax_r10, &cannoli_memhook_write_eax_r10_end) },
                    unsafe { (&cannoli_memhook_write_eax_r11, &cannoli_memhook_write_eax_r11_end) },
                    unsafe { (&cannoli_memhook_write_eax_r12, &cannoli_memhook_write_eax_r12_end) },
                    unsafe { (&cannoli_memhook_write_eax_r13, &cannoli_memhook_write_eax_r13_end) },
                    unsafe { (&cannoli_memhook_write_eax_r14, &cannoli_memhook_write_eax_r14_end) },
                    unsafe { (&cannoli_memhook_write_eax_r15, &cannoli_memhook_write_eax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ecx_rax, &cannoli_memhook_write_ecx_rax_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rcx, &cannoli_memhook_write_ecx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rdx, &cannoli_memhook_write_ecx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rbx, &cannoli_memhook_write_ecx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rsp, &cannoli_memhook_write_ecx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rbp, &cannoli_memhook_write_ecx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rsi, &cannoli_memhook_write_ecx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_ecx_rdi, &cannoli_memhook_write_ecx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r8, &cannoli_memhook_write_ecx_r8_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r9, &cannoli_memhook_write_ecx_r9_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r10, &cannoli_memhook_write_ecx_r10_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r11, &cannoli_memhook_write_ecx_r11_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r12, &cannoli_memhook_write_ecx_r12_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r13, &cannoli_memhook_write_ecx_r13_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r14, &cannoli_memhook_write_ecx_r14_end) },
                    unsafe { (&cannoli_memhook_write_ecx_r15, &cannoli_memhook_write_ecx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_edx_rax, &cannoli_memhook_write_edx_rax_end) },
                    unsafe { (&cannoli_memhook_write_edx_rcx, &cannoli_memhook_write_edx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_edx_rdx, &cannoli_memhook_write_edx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_edx_rbx, &cannoli_memhook_write_edx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_edx_rsp, &cannoli_memhook_write_edx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_edx_rbp, &cannoli_memhook_write_edx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_edx_rsi, &cannoli_memhook_write_edx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_edx_rdi, &cannoli_memhook_write_edx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_edx_r8, &cannoli_memhook_write_edx_r8_end) },
                    unsafe { (&cannoli_memhook_write_edx_r9, &cannoli_memhook_write_edx_r9_end) },
                    unsafe { (&cannoli_memhook_write_edx_r10, &cannoli_memhook_write_edx_r10_end) },
                    unsafe { (&cannoli_memhook_write_edx_r11, &cannoli_memhook_write_edx_r11_end) },
                    unsafe { (&cannoli_memhook_write_edx_r12, &cannoli_memhook_write_edx_r12_end) },
                    unsafe { (&cannoli_memhook_write_edx_r13, &cannoli_memhook_write_edx_r13_end) },
                    unsafe { (&cannoli_memhook_write_edx_r14, &cannoli_memhook_write_edx_r14_end) },
                    unsafe { (&cannoli_memhook_write_edx_r15, &cannoli_memhook_write_edx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ebx_rax, &cannoli_memhook_write_ebx_rax_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rcx, &cannoli_memhook_write_ebx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rdx, &cannoli_memhook_write_ebx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rbx, &cannoli_memhook_write_ebx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rsp, &cannoli_memhook_write_ebx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rbp, &cannoli_memhook_write_ebx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rsi, &cannoli_memhook_write_ebx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_ebx_rdi, &cannoli_memhook_write_ebx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r8, &cannoli_memhook_write_ebx_r8_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r9, &cannoli_memhook_write_ebx_r9_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r10, &cannoli_memhook_write_ebx_r10_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r11, &cannoli_memhook_write_ebx_r11_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r12, &cannoli_memhook_write_ebx_r12_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r13, &cannoli_memhook_write_ebx_r13_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r14, &cannoli_memhook_write_ebx_r14_end) },
                    unsafe { (&cannoli_memhook_write_ebx_r15, &cannoli_memhook_write_ebx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_esp_rax, &cannoli_memhook_write_esp_rax_end) },
                    unsafe { (&cannoli_memhook_write_esp_rcx, &cannoli_memhook_write_esp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_esp_rdx, &cannoli_memhook_write_esp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_esp_rbx, &cannoli_memhook_write_esp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_esp_rsp, &cannoli_memhook_write_esp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_esp_rbp, &cannoli_memhook_write_esp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_esp_rsi, &cannoli_memhook_write_esp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_esp_rdi, &cannoli_memhook_write_esp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_esp_r8, &cannoli_memhook_write_esp_r8_end) },
                    unsafe { (&cannoli_memhook_write_esp_r9, &cannoli_memhook_write_esp_r9_end) },
                    unsafe { (&cannoli_memhook_write_esp_r10, &cannoli_memhook_write_esp_r10_end) },
                    unsafe { (&cannoli_memhook_write_esp_r11, &cannoli_memhook_write_esp_r11_end) },
                    unsafe { (&cannoli_memhook_write_esp_r12, &cannoli_memhook_write_esp_r12_end) },
                    unsafe { (&cannoli_memhook_write_esp_r13, &cannoli_memhook_write_esp_r13_end) },
                    unsafe { (&cannoli_memhook_write_esp_r14, &cannoli_memhook_write_esp_r14_end) },
                    unsafe { (&cannoli_memhook_write_esp_r15, &cannoli_memhook_write_esp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_ebp_rax, &cannoli_memhook_write_ebp_rax_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rcx, &cannoli_memhook_write_ebp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rdx, &cannoli_memhook_write_ebp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rbx, &cannoli_memhook_write_ebp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rsp, &cannoli_memhook_write_ebp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rbp, &cannoli_memhook_write_ebp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rsi, &cannoli_memhook_write_ebp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_ebp_rdi, &cannoli_memhook_write_ebp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r8, &cannoli_memhook_write_ebp_r8_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r9, &cannoli_memhook_write_ebp_r9_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r10, &cannoli_memhook_write_ebp_r10_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r11, &cannoli_memhook_write_ebp_r11_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r12, &cannoli_memhook_write_ebp_r12_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r13, &cannoli_memhook_write_ebp_r13_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r14, &cannoli_memhook_write_ebp_r14_end) },
                    unsafe { (&cannoli_memhook_write_ebp_r15, &cannoli_memhook_write_ebp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_esi_rax, &cannoli_memhook_write_esi_rax_end) },
                    unsafe { (&cannoli_memhook_write_esi_rcx, &cannoli_memhook_write_esi_rcx_end) },
                    unsafe { (&cannoli_memhook_write_esi_rdx, &cannoli_memhook_write_esi_rdx_end) },
                    unsafe { (&cannoli_memhook_write_esi_rbx, &cannoli_memhook_write_esi_rbx_end) },
                    unsafe { (&cannoli_memhook_write_esi_rsp, &cannoli_memhook_write_esi_rsp_end) },
                    unsafe { (&cannoli_memhook_write_esi_rbp, &cannoli_memhook_write_esi_rbp_end) },
                    unsafe { (&cannoli_memhook_write_esi_rsi, &cannoli_memhook_write_esi_rsi_end) },
                    unsafe { (&cannoli_memhook_write_esi_rdi, &cannoli_memhook_write_esi_rdi_end) },
                    unsafe { (&cannoli_memhook_write_esi_r8, &cannoli_memhook_write_esi_r8_end) },
                    unsafe { (&cannoli_memhook_write_esi_r9, &cannoli_memhook_write_esi_r9_end) },
                    unsafe { (&cannoli_memhook_write_esi_r10, &cannoli_memhook_write_esi_r10_end) },
                    unsafe { (&cannoli_memhook_write_esi_r11, &cannoli_memhook_write_esi_r11_end) },
                    unsafe { (&cannoli_memhook_write_esi_r12, &cannoli_memhook_write_esi_r12_end) },
                    unsafe { (&cannoli_memhook_write_esi_r13, &cannoli_memhook_write_esi_r13_end) },
                    unsafe { (&cannoli_memhook_write_esi_r14, &cannoli_memhook_write_esi_r14_end) },
                    unsafe { (&cannoli_memhook_write_esi_r15, &cannoli_memhook_write_esi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_edi_rax, &cannoli_memhook_write_edi_rax_end) },
                    unsafe { (&cannoli_memhook_write_edi_rcx, &cannoli_memhook_write_edi_rcx_end) },
                    unsafe { (&cannoli_memhook_write_edi_rdx, &cannoli_memhook_write_edi_rdx_end) },
                    unsafe { (&cannoli_memhook_write_edi_rbx, &cannoli_memhook_write_edi_rbx_end) },
                    unsafe { (&cannoli_memhook_write_edi_rsp, &cannoli_memhook_write_edi_rsp_end) },
                    unsafe { (&cannoli_memhook_write_edi_rbp, &cannoli_memhook_write_edi_rbp_end) },
                    unsafe { (&cannoli_memhook_write_edi_rsi, &cannoli_memhook_write_edi_rsi_end) },
                    unsafe { (&cannoli_memhook_write_edi_rdi, &cannoli_memhook_write_edi_rdi_end) },
                    unsafe { (&cannoli_memhook_write_edi_r8, &cannoli_memhook_write_edi_r8_end) },
                    unsafe { (&cannoli_memhook_write_edi_r9, &cannoli_memhook_write_edi_r9_end) },
                    unsafe { (&cannoli_memhook_write_edi_r10, &cannoli_memhook_write_edi_r10_end) },
                    unsafe { (&cannoli_memhook_write_edi_r11, &cannoli_memhook_write_edi_r11_end) },
                    unsafe { (&cannoli_memhook_write_edi_r12, &cannoli_memhook_write_edi_r12_end) },
                    unsafe { (&cannoli_memhook_write_edi_r13, &cannoli_memhook_write_edi_r13_end) },
                    unsafe { (&cannoli_memhook_write_edi_r14, &cannoli_memhook_write_edi_r14_end) },
                    unsafe { (&cannoli_memhook_write_edi_r15, &cannoli_memhook_write_edi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8d_rax, &cannoli_memhook_write_r8d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rcx, &cannoli_memhook_write_r8d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rdx, &cannoli_memhook_write_r8d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rbx, &cannoli_memhook_write_r8d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rsp, &cannoli_memhook_write_r8d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rbp, &cannoli_memhook_write_r8d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rsi, &cannoli_memhook_write_r8d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r8d_rdi, &cannoli_memhook_write_r8d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r8, &cannoli_memhook_write_r8d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r9, &cannoli_memhook_write_r8d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r10, &cannoli_memhook_write_r8d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r11, &cannoli_memhook_write_r8d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r12, &cannoli_memhook_write_r8d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r13, &cannoli_memhook_write_r8d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r14, &cannoli_memhook_write_r8d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r8d_r15, &cannoli_memhook_write_r8d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9d_rax, &cannoli_memhook_write_r9d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rcx, &cannoli_memhook_write_r9d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rdx, &cannoli_memhook_write_r9d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rbx, &cannoli_memhook_write_r9d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rsp, &cannoli_memhook_write_r9d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rbp, &cannoli_memhook_write_r9d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rsi, &cannoli_memhook_write_r9d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r9d_rdi, &cannoli_memhook_write_r9d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r8, &cannoli_memhook_write_r9d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r9, &cannoli_memhook_write_r9d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r10, &cannoli_memhook_write_r9d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r11, &cannoli_memhook_write_r9d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r12, &cannoli_memhook_write_r9d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r13, &cannoli_memhook_write_r9d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r14, &cannoli_memhook_write_r9d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r9d_r15, &cannoli_memhook_write_r9d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10d_rax, &cannoli_memhook_write_r10d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rcx, &cannoli_memhook_write_r10d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rdx, &cannoli_memhook_write_r10d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rbx, &cannoli_memhook_write_r10d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rsp, &cannoli_memhook_write_r10d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rbp, &cannoli_memhook_write_r10d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rsi, &cannoli_memhook_write_r10d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r10d_rdi, &cannoli_memhook_write_r10d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r8, &cannoli_memhook_write_r10d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r9, &cannoli_memhook_write_r10d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r10, &cannoli_memhook_write_r10d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r11, &cannoli_memhook_write_r10d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r12, &cannoli_memhook_write_r10d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r13, &cannoli_memhook_write_r10d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r14, &cannoli_memhook_write_r10d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r10d_r15, &cannoli_memhook_write_r10d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11d_rax, &cannoli_memhook_write_r11d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rcx, &cannoli_memhook_write_r11d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rdx, &cannoli_memhook_write_r11d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rbx, &cannoli_memhook_write_r11d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rsp, &cannoli_memhook_write_r11d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rbp, &cannoli_memhook_write_r11d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rsi, &cannoli_memhook_write_r11d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r11d_rdi, &cannoli_memhook_write_r11d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r8, &cannoli_memhook_write_r11d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r9, &cannoli_memhook_write_r11d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r10, &cannoli_memhook_write_r11d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r11, &cannoli_memhook_write_r11d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r12, &cannoli_memhook_write_r11d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r13, &cannoli_memhook_write_r11d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r14, &cannoli_memhook_write_r11d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r11d_r15, &cannoli_memhook_write_r11d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12d_rax, &cannoli_memhook_write_r12d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rcx, &cannoli_memhook_write_r12d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rdx, &cannoli_memhook_write_r12d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rbx, &cannoli_memhook_write_r12d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rsp, &cannoli_memhook_write_r12d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rbp, &cannoli_memhook_write_r12d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rsi, &cannoli_memhook_write_r12d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r12d_rdi, &cannoli_memhook_write_r12d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r8, &cannoli_memhook_write_r12d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r9, &cannoli_memhook_write_r12d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r10, &cannoli_memhook_write_r12d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r11, &cannoli_memhook_write_r12d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r12, &cannoli_memhook_write_r12d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r13, &cannoli_memhook_write_r12d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r14, &cannoli_memhook_write_r12d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r12d_r15, &cannoli_memhook_write_r12d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13d_rax, &cannoli_memhook_write_r13d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rcx, &cannoli_memhook_write_r13d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rdx, &cannoli_memhook_write_r13d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rbx, &cannoli_memhook_write_r13d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rsp, &cannoli_memhook_write_r13d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rbp, &cannoli_memhook_write_r13d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rsi, &cannoli_memhook_write_r13d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r13d_rdi, &cannoli_memhook_write_r13d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r8, &cannoli_memhook_write_r13d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r9, &cannoli_memhook_write_r13d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r10, &cannoli_memhook_write_r13d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r11, &cannoli_memhook_write_r13d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r12, &cannoli_memhook_write_r13d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r13, &cannoli_memhook_write_r13d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r14, &cannoli_memhook_write_r13d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r13d_r15, &cannoli_memhook_write_r13d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14d_rax, &cannoli_memhook_write_r14d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rcx, &cannoli_memhook_write_r14d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rdx, &cannoli_memhook_write_r14d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rbx, &cannoli_memhook_write_r14d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rsp, &cannoli_memhook_write_r14d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rbp, &cannoli_memhook_write_r14d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rsi, &cannoli_memhook_write_r14d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r14d_rdi, &cannoli_memhook_write_r14d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r8, &cannoli_memhook_write_r14d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r9, &cannoli_memhook_write_r14d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r10, &cannoli_memhook_write_r14d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r11, &cannoli_memhook_write_r14d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r12, &cannoli_memhook_write_r14d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r13, &cannoli_memhook_write_r14d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r14, &cannoli_memhook_write_r14d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r14d_r15, &cannoli_memhook_write_r14d_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15d_rax, &cannoli_memhook_write_r15d_rax_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rcx, &cannoli_memhook_write_r15d_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rdx, &cannoli_memhook_write_r15d_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rbx, &cannoli_memhook_write_r15d_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rsp, &cannoli_memhook_write_r15d_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rbp, &cannoli_memhook_write_r15d_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rsi, &cannoli_memhook_write_r15d_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r15d_rdi, &cannoli_memhook_write_r15d_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r8, &cannoli_memhook_write_r15d_r8_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r9, &cannoli_memhook_write_r15d_r9_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r10, &cannoli_memhook_write_r15d_r10_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r11, &cannoli_memhook_write_r15d_r11_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r12, &cannoli_memhook_write_r15d_r12_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r13, &cannoli_memhook_write_r15d_r13_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r14, &cannoli_memhook_write_r15d_r14_end) },
                    unsafe { (&cannoli_memhook_write_r15d_r15, &cannoli_memhook_write_r15d_r15_end) },
                ],
            ],
         ],
        [
            [
                [
                    unsafe { (&cannoli_memhook_read_rax_rax, &cannoli_memhook_read_rax_rax_end) },
                    unsafe { (&cannoli_memhook_read_rax_rcx, &cannoli_memhook_read_rax_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rax_rdx, &cannoli_memhook_read_rax_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rax_rbx, &cannoli_memhook_read_rax_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rax_rsp, &cannoli_memhook_read_rax_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rax_rbp, &cannoli_memhook_read_rax_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rax_rsi, &cannoli_memhook_read_rax_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rax_rdi, &cannoli_memhook_read_rax_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rax_r8, &cannoli_memhook_read_rax_r8_end) },
                    unsafe { (&cannoli_memhook_read_rax_r9, &cannoli_memhook_read_rax_r9_end) },
                    unsafe { (&cannoli_memhook_read_rax_r10, &cannoli_memhook_read_rax_r10_end) },
                    unsafe { (&cannoli_memhook_read_rax_r11, &cannoli_memhook_read_rax_r11_end) },
                    unsafe { (&cannoli_memhook_read_rax_r12, &cannoli_memhook_read_rax_r12_end) },
                    unsafe { (&cannoli_memhook_read_rax_r13, &cannoli_memhook_read_rax_r13_end) },
                    unsafe { (&cannoli_memhook_read_rax_r14, &cannoli_memhook_read_rax_r14_end) },
                    unsafe { (&cannoli_memhook_read_rax_r15, &cannoli_memhook_read_rax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rcx_rax, &cannoli_memhook_read_rcx_rax_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rcx, &cannoli_memhook_read_rcx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rdx, &cannoli_memhook_read_rcx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rbx, &cannoli_memhook_read_rcx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rsp, &cannoli_memhook_read_rcx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rbp, &cannoli_memhook_read_rcx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rsi, &cannoli_memhook_read_rcx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rcx_rdi, &cannoli_memhook_read_rcx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r8, &cannoli_memhook_read_rcx_r8_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r9, &cannoli_memhook_read_rcx_r9_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r10, &cannoli_memhook_read_rcx_r10_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r11, &cannoli_memhook_read_rcx_r11_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r12, &cannoli_memhook_read_rcx_r12_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r13, &cannoli_memhook_read_rcx_r13_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r14, &cannoli_memhook_read_rcx_r14_end) },
                    unsafe { (&cannoli_memhook_read_rcx_r15, &cannoli_memhook_read_rcx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rdx_rax, &cannoli_memhook_read_rdx_rax_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rcx, &cannoli_memhook_read_rdx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rdx, &cannoli_memhook_read_rdx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rbx, &cannoli_memhook_read_rdx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rsp, &cannoli_memhook_read_rdx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rbp, &cannoli_memhook_read_rdx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rsi, &cannoli_memhook_read_rdx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rdx_rdi, &cannoli_memhook_read_rdx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r8, &cannoli_memhook_read_rdx_r8_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r9, &cannoli_memhook_read_rdx_r9_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r10, &cannoli_memhook_read_rdx_r10_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r11, &cannoli_memhook_read_rdx_r11_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r12, &cannoli_memhook_read_rdx_r12_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r13, &cannoli_memhook_read_rdx_r13_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r14, &cannoli_memhook_read_rdx_r14_end) },
                    unsafe { (&cannoli_memhook_read_rdx_r15, &cannoli_memhook_read_rdx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rbx_rax, &cannoli_memhook_read_rbx_rax_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rcx, &cannoli_memhook_read_rbx_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rdx, &cannoli_memhook_read_rbx_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rbx, &cannoli_memhook_read_rbx_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rsp, &cannoli_memhook_read_rbx_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rbp, &cannoli_memhook_read_rbx_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rsi, &cannoli_memhook_read_rbx_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rbx_rdi, &cannoli_memhook_read_rbx_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r8, &cannoli_memhook_read_rbx_r8_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r9, &cannoli_memhook_read_rbx_r9_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r10, &cannoli_memhook_read_rbx_r10_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r11, &cannoli_memhook_read_rbx_r11_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r12, &cannoli_memhook_read_rbx_r12_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r13, &cannoli_memhook_read_rbx_r13_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r14, &cannoli_memhook_read_rbx_r14_end) },
                    unsafe { (&cannoli_memhook_read_rbx_r15, &cannoli_memhook_read_rbx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rsp_rax, &cannoli_memhook_read_rsp_rax_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rcx, &cannoli_memhook_read_rsp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rdx, &cannoli_memhook_read_rsp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rbx, &cannoli_memhook_read_rsp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rsp, &cannoli_memhook_read_rsp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rbp, &cannoli_memhook_read_rsp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rsi, &cannoli_memhook_read_rsp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rsp_rdi, &cannoli_memhook_read_rsp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r8, &cannoli_memhook_read_rsp_r8_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r9, &cannoli_memhook_read_rsp_r9_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r10, &cannoli_memhook_read_rsp_r10_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r11, &cannoli_memhook_read_rsp_r11_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r12, &cannoli_memhook_read_rsp_r12_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r13, &cannoli_memhook_read_rsp_r13_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r14, &cannoli_memhook_read_rsp_r14_end) },
                    unsafe { (&cannoli_memhook_read_rsp_r15, &cannoli_memhook_read_rsp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rbp_rax, &cannoli_memhook_read_rbp_rax_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rcx, &cannoli_memhook_read_rbp_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rdx, &cannoli_memhook_read_rbp_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rbx, &cannoli_memhook_read_rbp_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rsp, &cannoli_memhook_read_rbp_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rbp, &cannoli_memhook_read_rbp_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rsi, &cannoli_memhook_read_rbp_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rbp_rdi, &cannoli_memhook_read_rbp_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r8, &cannoli_memhook_read_rbp_r8_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r9, &cannoli_memhook_read_rbp_r9_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r10, &cannoli_memhook_read_rbp_r10_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r11, &cannoli_memhook_read_rbp_r11_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r12, &cannoli_memhook_read_rbp_r12_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r13, &cannoli_memhook_read_rbp_r13_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r14, &cannoli_memhook_read_rbp_r14_end) },
                    unsafe { (&cannoli_memhook_read_rbp_r15, &cannoli_memhook_read_rbp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rsi_rax, &cannoli_memhook_read_rsi_rax_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rcx, &cannoli_memhook_read_rsi_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rdx, &cannoli_memhook_read_rsi_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rbx, &cannoli_memhook_read_rsi_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rsp, &cannoli_memhook_read_rsi_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rbp, &cannoli_memhook_read_rsi_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rsi, &cannoli_memhook_read_rsi_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rsi_rdi, &cannoli_memhook_read_rsi_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r8, &cannoli_memhook_read_rsi_r8_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r9, &cannoli_memhook_read_rsi_r9_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r10, &cannoli_memhook_read_rsi_r10_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r11, &cannoli_memhook_read_rsi_r11_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r12, &cannoli_memhook_read_rsi_r12_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r13, &cannoli_memhook_read_rsi_r13_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r14, &cannoli_memhook_read_rsi_r14_end) },
                    unsafe { (&cannoli_memhook_read_rsi_r15, &cannoli_memhook_read_rsi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_rdi_rax, &cannoli_memhook_read_rdi_rax_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rcx, &cannoli_memhook_read_rdi_rcx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rdx, &cannoli_memhook_read_rdi_rdx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rbx, &cannoli_memhook_read_rdi_rbx_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rsp, &cannoli_memhook_read_rdi_rsp_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rbp, &cannoli_memhook_read_rdi_rbp_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rsi, &cannoli_memhook_read_rdi_rsi_end) },
                    unsafe { (&cannoli_memhook_read_rdi_rdi, &cannoli_memhook_read_rdi_rdi_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r8, &cannoli_memhook_read_rdi_r8_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r9, &cannoli_memhook_read_rdi_r9_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r10, &cannoli_memhook_read_rdi_r10_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r11, &cannoli_memhook_read_rdi_r11_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r12, &cannoli_memhook_read_rdi_r12_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r13, &cannoli_memhook_read_rdi_r13_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r14, &cannoli_memhook_read_rdi_r14_end) },
                    unsafe { (&cannoli_memhook_read_rdi_r15, &cannoli_memhook_read_rdi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r8_rax, &cannoli_memhook_read_r8_rax_end) },
                    unsafe { (&cannoli_memhook_read_r8_rcx, &cannoli_memhook_read_r8_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r8_rdx, &cannoli_memhook_read_r8_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r8_rbx, &cannoli_memhook_read_r8_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r8_rsp, &cannoli_memhook_read_r8_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r8_rbp, &cannoli_memhook_read_r8_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r8_rsi, &cannoli_memhook_read_r8_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r8_rdi, &cannoli_memhook_read_r8_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r8_r8, &cannoli_memhook_read_r8_r8_end) },
                    unsafe { (&cannoli_memhook_read_r8_r9, &cannoli_memhook_read_r8_r9_end) },
                    unsafe { (&cannoli_memhook_read_r8_r10, &cannoli_memhook_read_r8_r10_end) },
                    unsafe { (&cannoli_memhook_read_r8_r11, &cannoli_memhook_read_r8_r11_end) },
                    unsafe { (&cannoli_memhook_read_r8_r12, &cannoli_memhook_read_r8_r12_end) },
                    unsafe { (&cannoli_memhook_read_r8_r13, &cannoli_memhook_read_r8_r13_end) },
                    unsafe { (&cannoli_memhook_read_r8_r14, &cannoli_memhook_read_r8_r14_end) },
                    unsafe { (&cannoli_memhook_read_r8_r15, &cannoli_memhook_read_r8_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r9_rax, &cannoli_memhook_read_r9_rax_end) },
                    unsafe { (&cannoli_memhook_read_r9_rcx, &cannoli_memhook_read_r9_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r9_rdx, &cannoli_memhook_read_r9_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r9_rbx, &cannoli_memhook_read_r9_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r9_rsp, &cannoli_memhook_read_r9_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r9_rbp, &cannoli_memhook_read_r9_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r9_rsi, &cannoli_memhook_read_r9_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r9_rdi, &cannoli_memhook_read_r9_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r9_r8, &cannoli_memhook_read_r9_r8_end) },
                    unsafe { (&cannoli_memhook_read_r9_r9, &cannoli_memhook_read_r9_r9_end) },
                    unsafe { (&cannoli_memhook_read_r9_r10, &cannoli_memhook_read_r9_r10_end) },
                    unsafe { (&cannoli_memhook_read_r9_r11, &cannoli_memhook_read_r9_r11_end) },
                    unsafe { (&cannoli_memhook_read_r9_r12, &cannoli_memhook_read_r9_r12_end) },
                    unsafe { (&cannoli_memhook_read_r9_r13, &cannoli_memhook_read_r9_r13_end) },
                    unsafe { (&cannoli_memhook_read_r9_r14, &cannoli_memhook_read_r9_r14_end) },
                    unsafe { (&cannoli_memhook_read_r9_r15, &cannoli_memhook_read_r9_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r10_rax, &cannoli_memhook_read_r10_rax_end) },
                    unsafe { (&cannoli_memhook_read_r10_rcx, &cannoli_memhook_read_r10_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r10_rdx, &cannoli_memhook_read_r10_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r10_rbx, &cannoli_memhook_read_r10_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r10_rsp, &cannoli_memhook_read_r10_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r10_rbp, &cannoli_memhook_read_r10_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r10_rsi, &cannoli_memhook_read_r10_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r10_rdi, &cannoli_memhook_read_r10_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r10_r8, &cannoli_memhook_read_r10_r8_end) },
                    unsafe { (&cannoli_memhook_read_r10_r9, &cannoli_memhook_read_r10_r9_end) },
                    unsafe { (&cannoli_memhook_read_r10_r10, &cannoli_memhook_read_r10_r10_end) },
                    unsafe { (&cannoli_memhook_read_r10_r11, &cannoli_memhook_read_r10_r11_end) },
                    unsafe { (&cannoli_memhook_read_r10_r12, &cannoli_memhook_read_r10_r12_end) },
                    unsafe { (&cannoli_memhook_read_r10_r13, &cannoli_memhook_read_r10_r13_end) },
                    unsafe { (&cannoli_memhook_read_r10_r14, &cannoli_memhook_read_r10_r14_end) },
                    unsafe { (&cannoli_memhook_read_r10_r15, &cannoli_memhook_read_r10_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r11_rax, &cannoli_memhook_read_r11_rax_end) },
                    unsafe { (&cannoli_memhook_read_r11_rcx, &cannoli_memhook_read_r11_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r11_rdx, &cannoli_memhook_read_r11_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r11_rbx, &cannoli_memhook_read_r11_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r11_rsp, &cannoli_memhook_read_r11_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r11_rbp, &cannoli_memhook_read_r11_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r11_rsi, &cannoli_memhook_read_r11_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r11_rdi, &cannoli_memhook_read_r11_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r11_r8, &cannoli_memhook_read_r11_r8_end) },
                    unsafe { (&cannoli_memhook_read_r11_r9, &cannoli_memhook_read_r11_r9_end) },
                    unsafe { (&cannoli_memhook_read_r11_r10, &cannoli_memhook_read_r11_r10_end) },
                    unsafe { (&cannoli_memhook_read_r11_r11, &cannoli_memhook_read_r11_r11_end) },
                    unsafe { (&cannoli_memhook_read_r11_r12, &cannoli_memhook_read_r11_r12_end) },
                    unsafe { (&cannoli_memhook_read_r11_r13, &cannoli_memhook_read_r11_r13_end) },
                    unsafe { (&cannoli_memhook_read_r11_r14, &cannoli_memhook_read_r11_r14_end) },
                    unsafe { (&cannoli_memhook_read_r11_r15, &cannoli_memhook_read_r11_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r12_rax, &cannoli_memhook_read_r12_rax_end) },
                    unsafe { (&cannoli_memhook_read_r12_rcx, &cannoli_memhook_read_r12_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r12_rdx, &cannoli_memhook_read_r12_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r12_rbx, &cannoli_memhook_read_r12_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r12_rsp, &cannoli_memhook_read_r12_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r12_rbp, &cannoli_memhook_read_r12_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r12_rsi, &cannoli_memhook_read_r12_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r12_rdi, &cannoli_memhook_read_r12_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r12_r8, &cannoli_memhook_read_r12_r8_end) },
                    unsafe { (&cannoli_memhook_read_r12_r9, &cannoli_memhook_read_r12_r9_end) },
                    unsafe { (&cannoli_memhook_read_r12_r10, &cannoli_memhook_read_r12_r10_end) },
                    unsafe { (&cannoli_memhook_read_r12_r11, &cannoli_memhook_read_r12_r11_end) },
                    unsafe { (&cannoli_memhook_read_r12_r12, &cannoli_memhook_read_r12_r12_end) },
                    unsafe { (&cannoli_memhook_read_r12_r13, &cannoli_memhook_read_r12_r13_end) },
                    unsafe { (&cannoli_memhook_read_r12_r14, &cannoli_memhook_read_r12_r14_end) },
                    unsafe { (&cannoli_memhook_read_r12_r15, &cannoli_memhook_read_r12_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r13_rax, &cannoli_memhook_read_r13_rax_end) },
                    unsafe { (&cannoli_memhook_read_r13_rcx, &cannoli_memhook_read_r13_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r13_rdx, &cannoli_memhook_read_r13_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r13_rbx, &cannoli_memhook_read_r13_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r13_rsp, &cannoli_memhook_read_r13_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r13_rbp, &cannoli_memhook_read_r13_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r13_rsi, &cannoli_memhook_read_r13_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r13_rdi, &cannoli_memhook_read_r13_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r13_r8, &cannoli_memhook_read_r13_r8_end) },
                    unsafe { (&cannoli_memhook_read_r13_r9, &cannoli_memhook_read_r13_r9_end) },
                    unsafe { (&cannoli_memhook_read_r13_r10, &cannoli_memhook_read_r13_r10_end) },
                    unsafe { (&cannoli_memhook_read_r13_r11, &cannoli_memhook_read_r13_r11_end) },
                    unsafe { (&cannoli_memhook_read_r13_r12, &cannoli_memhook_read_r13_r12_end) },
                    unsafe { (&cannoli_memhook_read_r13_r13, &cannoli_memhook_read_r13_r13_end) },
                    unsafe { (&cannoli_memhook_read_r13_r14, &cannoli_memhook_read_r13_r14_end) },
                    unsafe { (&cannoli_memhook_read_r13_r15, &cannoli_memhook_read_r13_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r14_rax, &cannoli_memhook_read_r14_rax_end) },
                    unsafe { (&cannoli_memhook_read_r14_rcx, &cannoli_memhook_read_r14_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r14_rdx, &cannoli_memhook_read_r14_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r14_rbx, &cannoli_memhook_read_r14_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r14_rsp, &cannoli_memhook_read_r14_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r14_rbp, &cannoli_memhook_read_r14_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r14_rsi, &cannoli_memhook_read_r14_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r14_rdi, &cannoli_memhook_read_r14_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r14_r8, &cannoli_memhook_read_r14_r8_end) },
                    unsafe { (&cannoli_memhook_read_r14_r9, &cannoli_memhook_read_r14_r9_end) },
                    unsafe { (&cannoli_memhook_read_r14_r10, &cannoli_memhook_read_r14_r10_end) },
                    unsafe { (&cannoli_memhook_read_r14_r11, &cannoli_memhook_read_r14_r11_end) },
                    unsafe { (&cannoli_memhook_read_r14_r12, &cannoli_memhook_read_r14_r12_end) },
                    unsafe { (&cannoli_memhook_read_r14_r13, &cannoli_memhook_read_r14_r13_end) },
                    unsafe { (&cannoli_memhook_read_r14_r14, &cannoli_memhook_read_r14_r14_end) },
                    unsafe { (&cannoli_memhook_read_r14_r15, &cannoli_memhook_read_r14_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_read_r15_rax, &cannoli_memhook_read_r15_rax_end) },
                    unsafe { (&cannoli_memhook_read_r15_rcx, &cannoli_memhook_read_r15_rcx_end) },
                    unsafe { (&cannoli_memhook_read_r15_rdx, &cannoli_memhook_read_r15_rdx_end) },
                    unsafe { (&cannoli_memhook_read_r15_rbx, &cannoli_memhook_read_r15_rbx_end) },
                    unsafe { (&cannoli_memhook_read_r15_rsp, &cannoli_memhook_read_r15_rsp_end) },
                    unsafe { (&cannoli_memhook_read_r15_rbp, &cannoli_memhook_read_r15_rbp_end) },
                    unsafe { (&cannoli_memhook_read_r15_rsi, &cannoli_memhook_read_r15_rsi_end) },
                    unsafe { (&cannoli_memhook_read_r15_rdi, &cannoli_memhook_read_r15_rdi_end) },
                    unsafe { (&cannoli_memhook_read_r15_r8, &cannoli_memhook_read_r15_r8_end) },
                    unsafe { (&cannoli_memhook_read_r15_r9, &cannoli_memhook_read_r15_r9_end) },
                    unsafe { (&cannoli_memhook_read_r15_r10, &cannoli_memhook_read_r15_r10_end) },
                    unsafe { (&cannoli_memhook_read_r15_r11, &cannoli_memhook_read_r15_r11_end) },
                    unsafe { (&cannoli_memhook_read_r15_r12, &cannoli_memhook_read_r15_r12_end) },
                    unsafe { (&cannoli_memhook_read_r15_r13, &cannoli_memhook_read_r15_r13_end) },
                    unsafe { (&cannoli_memhook_read_r15_r14, &cannoli_memhook_read_r15_r14_end) },
                    unsafe { (&cannoli_memhook_read_r15_r15, &cannoli_memhook_read_r15_r15_end) },
                ],
            ],
            [
                [
                    unsafe { (&cannoli_memhook_write_rax_rax, &cannoli_memhook_write_rax_rax_end) },
                    unsafe { (&cannoli_memhook_write_rax_rcx, &cannoli_memhook_write_rax_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rax_rdx, &cannoli_memhook_write_rax_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rax_rbx, &cannoli_memhook_write_rax_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rax_rsp, &cannoli_memhook_write_rax_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rax_rbp, &cannoli_memhook_write_rax_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rax_rsi, &cannoli_memhook_write_rax_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rax_rdi, &cannoli_memhook_write_rax_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rax_r8, &cannoli_memhook_write_rax_r8_end) },
                    unsafe { (&cannoli_memhook_write_rax_r9, &cannoli_memhook_write_rax_r9_end) },
                    unsafe { (&cannoli_memhook_write_rax_r10, &cannoli_memhook_write_rax_r10_end) },
                    unsafe { (&cannoli_memhook_write_rax_r11, &cannoli_memhook_write_rax_r11_end) },
                    unsafe { (&cannoli_memhook_write_rax_r12, &cannoli_memhook_write_rax_r12_end) },
                    unsafe { (&cannoli_memhook_write_rax_r13, &cannoli_memhook_write_rax_r13_end) },
                    unsafe { (&cannoli_memhook_write_rax_r14, &cannoli_memhook_write_rax_r14_end) },
                    unsafe { (&cannoli_memhook_write_rax_r15, &cannoli_memhook_write_rax_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rcx_rax, &cannoli_memhook_write_rcx_rax_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rcx, &cannoli_memhook_write_rcx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rdx, &cannoli_memhook_write_rcx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rbx, &cannoli_memhook_write_rcx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rsp, &cannoli_memhook_write_rcx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rbp, &cannoli_memhook_write_rcx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rsi, &cannoli_memhook_write_rcx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rcx_rdi, &cannoli_memhook_write_rcx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r8, &cannoli_memhook_write_rcx_r8_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r9, &cannoli_memhook_write_rcx_r9_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r10, &cannoli_memhook_write_rcx_r10_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r11, &cannoli_memhook_write_rcx_r11_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r12, &cannoli_memhook_write_rcx_r12_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r13, &cannoli_memhook_write_rcx_r13_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r14, &cannoli_memhook_write_rcx_r14_end) },
                    unsafe { (&cannoli_memhook_write_rcx_r15, &cannoli_memhook_write_rcx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rdx_rax, &cannoli_memhook_write_rdx_rax_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rcx, &cannoli_memhook_write_rdx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rdx, &cannoli_memhook_write_rdx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rbx, &cannoli_memhook_write_rdx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rsp, &cannoli_memhook_write_rdx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rbp, &cannoli_memhook_write_rdx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rsi, &cannoli_memhook_write_rdx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rdx_rdi, &cannoli_memhook_write_rdx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r8, &cannoli_memhook_write_rdx_r8_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r9, &cannoli_memhook_write_rdx_r9_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r10, &cannoli_memhook_write_rdx_r10_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r11, &cannoli_memhook_write_rdx_r11_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r12, &cannoli_memhook_write_rdx_r12_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r13, &cannoli_memhook_write_rdx_r13_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r14, &cannoli_memhook_write_rdx_r14_end) },
                    unsafe { (&cannoli_memhook_write_rdx_r15, &cannoli_memhook_write_rdx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rbx_rax, &cannoli_memhook_write_rbx_rax_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rcx, &cannoli_memhook_write_rbx_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rdx, &cannoli_memhook_write_rbx_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rbx, &cannoli_memhook_write_rbx_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rsp, &cannoli_memhook_write_rbx_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rbp, &cannoli_memhook_write_rbx_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rsi, &cannoli_memhook_write_rbx_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rbx_rdi, &cannoli_memhook_write_rbx_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r8, &cannoli_memhook_write_rbx_r8_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r9, &cannoli_memhook_write_rbx_r9_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r10, &cannoli_memhook_write_rbx_r10_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r11, &cannoli_memhook_write_rbx_r11_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r12, &cannoli_memhook_write_rbx_r12_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r13, &cannoli_memhook_write_rbx_r13_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r14, &cannoli_memhook_write_rbx_r14_end) },
                    unsafe { (&cannoli_memhook_write_rbx_r15, &cannoli_memhook_write_rbx_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rsp_rax, &cannoli_memhook_write_rsp_rax_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rcx, &cannoli_memhook_write_rsp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rdx, &cannoli_memhook_write_rsp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rbx, &cannoli_memhook_write_rsp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rsp, &cannoli_memhook_write_rsp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rbp, &cannoli_memhook_write_rsp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rsi, &cannoli_memhook_write_rsp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rsp_rdi, &cannoli_memhook_write_rsp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r8, &cannoli_memhook_write_rsp_r8_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r9, &cannoli_memhook_write_rsp_r9_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r10, &cannoli_memhook_write_rsp_r10_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r11, &cannoli_memhook_write_rsp_r11_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r12, &cannoli_memhook_write_rsp_r12_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r13, &cannoli_memhook_write_rsp_r13_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r14, &cannoli_memhook_write_rsp_r14_end) },
                    unsafe { (&cannoli_memhook_write_rsp_r15, &cannoli_memhook_write_rsp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rbp_rax, &cannoli_memhook_write_rbp_rax_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rcx, &cannoli_memhook_write_rbp_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rdx, &cannoli_memhook_write_rbp_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rbx, &cannoli_memhook_write_rbp_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rsp, &cannoli_memhook_write_rbp_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rbp, &cannoli_memhook_write_rbp_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rsi, &cannoli_memhook_write_rbp_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rbp_rdi, &cannoli_memhook_write_rbp_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r8, &cannoli_memhook_write_rbp_r8_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r9, &cannoli_memhook_write_rbp_r9_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r10, &cannoli_memhook_write_rbp_r10_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r11, &cannoli_memhook_write_rbp_r11_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r12, &cannoli_memhook_write_rbp_r12_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r13, &cannoli_memhook_write_rbp_r13_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r14, &cannoli_memhook_write_rbp_r14_end) },
                    unsafe { (&cannoli_memhook_write_rbp_r15, &cannoli_memhook_write_rbp_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rsi_rax, &cannoli_memhook_write_rsi_rax_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rcx, &cannoli_memhook_write_rsi_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rdx, &cannoli_memhook_write_rsi_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rbx, &cannoli_memhook_write_rsi_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rsp, &cannoli_memhook_write_rsi_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rbp, &cannoli_memhook_write_rsi_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rsi, &cannoli_memhook_write_rsi_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rsi_rdi, &cannoli_memhook_write_rsi_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r8, &cannoli_memhook_write_rsi_r8_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r9, &cannoli_memhook_write_rsi_r9_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r10, &cannoli_memhook_write_rsi_r10_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r11, &cannoli_memhook_write_rsi_r11_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r12, &cannoli_memhook_write_rsi_r12_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r13, &cannoli_memhook_write_rsi_r13_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r14, &cannoli_memhook_write_rsi_r14_end) },
                    unsafe { (&cannoli_memhook_write_rsi_r15, &cannoli_memhook_write_rsi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_rdi_rax, &cannoli_memhook_write_rdi_rax_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rcx, &cannoli_memhook_write_rdi_rcx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rdx, &cannoli_memhook_write_rdi_rdx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rbx, &cannoli_memhook_write_rdi_rbx_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rsp, &cannoli_memhook_write_rdi_rsp_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rbp, &cannoli_memhook_write_rdi_rbp_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rsi, &cannoli_memhook_write_rdi_rsi_end) },
                    unsafe { (&cannoli_memhook_write_rdi_rdi, &cannoli_memhook_write_rdi_rdi_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r8, &cannoli_memhook_write_rdi_r8_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r9, &cannoli_memhook_write_rdi_r9_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r10, &cannoli_memhook_write_rdi_r10_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r11, &cannoli_memhook_write_rdi_r11_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r12, &cannoli_memhook_write_rdi_r12_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r13, &cannoli_memhook_write_rdi_r13_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r14, &cannoli_memhook_write_rdi_r14_end) },
                    unsafe { (&cannoli_memhook_write_rdi_r15, &cannoli_memhook_write_rdi_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r8_rax, &cannoli_memhook_write_r8_rax_end) },
                    unsafe { (&cannoli_memhook_write_r8_rcx, &cannoli_memhook_write_r8_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r8_rdx, &cannoli_memhook_write_r8_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r8_rbx, &cannoli_memhook_write_r8_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r8_rsp, &cannoli_memhook_write_r8_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r8_rbp, &cannoli_memhook_write_r8_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r8_rsi, &cannoli_memhook_write_r8_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r8_rdi, &cannoli_memhook_write_r8_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r8_r8, &cannoli_memhook_write_r8_r8_end) },
                    unsafe { (&cannoli_memhook_write_r8_r9, &cannoli_memhook_write_r8_r9_end) },
                    unsafe { (&cannoli_memhook_write_r8_r10, &cannoli_memhook_write_r8_r10_end) },
                    unsafe { (&cannoli_memhook_write_r8_r11, &cannoli_memhook_write_r8_r11_end) },
                    unsafe { (&cannoli_memhook_write_r8_r12, &cannoli_memhook_write_r8_r12_end) },
                    unsafe { (&cannoli_memhook_write_r8_r13, &cannoli_memhook_write_r8_r13_end) },
                    unsafe { (&cannoli_memhook_write_r8_r14, &cannoli_memhook_write_r8_r14_end) },
                    unsafe { (&cannoli_memhook_write_r8_r15, &cannoli_memhook_write_r8_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r9_rax, &cannoli_memhook_write_r9_rax_end) },
                    unsafe { (&cannoli_memhook_write_r9_rcx, &cannoli_memhook_write_r9_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r9_rdx, &cannoli_memhook_write_r9_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r9_rbx, &cannoli_memhook_write_r9_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r9_rsp, &cannoli_memhook_write_r9_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r9_rbp, &cannoli_memhook_write_r9_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r9_rsi, &cannoli_memhook_write_r9_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r9_rdi, &cannoli_memhook_write_r9_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r9_r8, &cannoli_memhook_write_r9_r8_end) },
                    unsafe { (&cannoli_memhook_write_r9_r9, &cannoli_memhook_write_r9_r9_end) },
                    unsafe { (&cannoli_memhook_write_r9_r10, &cannoli_memhook_write_r9_r10_end) },
                    unsafe { (&cannoli_memhook_write_r9_r11, &cannoli_memhook_write_r9_r11_end) },
                    unsafe { (&cannoli_memhook_write_r9_r12, &cannoli_memhook_write_r9_r12_end) },
                    unsafe { (&cannoli_memhook_write_r9_r13, &cannoli_memhook_write_r9_r13_end) },
                    unsafe { (&cannoli_memhook_write_r9_r14, &cannoli_memhook_write_r9_r14_end) },
                    unsafe { (&cannoli_memhook_write_r9_r15, &cannoli_memhook_write_r9_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r10_rax, &cannoli_memhook_write_r10_rax_end) },
                    unsafe { (&cannoli_memhook_write_r10_rcx, &cannoli_memhook_write_r10_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r10_rdx, &cannoli_memhook_write_r10_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r10_rbx, &cannoli_memhook_write_r10_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r10_rsp, &cannoli_memhook_write_r10_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r10_rbp, &cannoli_memhook_write_r10_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r10_rsi, &cannoli_memhook_write_r10_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r10_rdi, &cannoli_memhook_write_r10_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r10_r8, &cannoli_memhook_write_r10_r8_end) },
                    unsafe { (&cannoli_memhook_write_r10_r9, &cannoli_memhook_write_r10_r9_end) },
                    unsafe { (&cannoli_memhook_write_r10_r10, &cannoli_memhook_write_r10_r10_end) },
                    unsafe { (&cannoli_memhook_write_r10_r11, &cannoli_memhook_write_r10_r11_end) },
                    unsafe { (&cannoli_memhook_write_r10_r12, &cannoli_memhook_write_r10_r12_end) },
                    unsafe { (&cannoli_memhook_write_r10_r13, &cannoli_memhook_write_r10_r13_end) },
                    unsafe { (&cannoli_memhook_write_r10_r14, &cannoli_memhook_write_r10_r14_end) },
                    unsafe { (&cannoli_memhook_write_r10_r15, &cannoli_memhook_write_r10_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r11_rax, &cannoli_memhook_write_r11_rax_end) },
                    unsafe { (&cannoli_memhook_write_r11_rcx, &cannoli_memhook_write_r11_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r11_rdx, &cannoli_memhook_write_r11_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r11_rbx, &cannoli_memhook_write_r11_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r11_rsp, &cannoli_memhook_write_r11_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r11_rbp, &cannoli_memhook_write_r11_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r11_rsi, &cannoli_memhook_write_r11_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r11_rdi, &cannoli_memhook_write_r11_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r11_r8, &cannoli_memhook_write_r11_r8_end) },
                    unsafe { (&cannoli_memhook_write_r11_r9, &cannoli_memhook_write_r11_r9_end) },
                    unsafe { (&cannoli_memhook_write_r11_r10, &cannoli_memhook_write_r11_r10_end) },
                    unsafe { (&cannoli_memhook_write_r11_r11, &cannoli_memhook_write_r11_r11_end) },
                    unsafe { (&cannoli_memhook_write_r11_r12, &cannoli_memhook_write_r11_r12_end) },
                    unsafe { (&cannoli_memhook_write_r11_r13, &cannoli_memhook_write_r11_r13_end) },
                    unsafe { (&cannoli_memhook_write_r11_r14, &cannoli_memhook_write_r11_r14_end) },
                    unsafe { (&cannoli_memhook_write_r11_r15, &cannoli_memhook_write_r11_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r12_rax, &cannoli_memhook_write_r12_rax_end) },
                    unsafe { (&cannoli_memhook_write_r12_rcx, &cannoli_memhook_write_r12_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r12_rdx, &cannoli_memhook_write_r12_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r12_rbx, &cannoli_memhook_write_r12_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r12_rsp, &cannoli_memhook_write_r12_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r12_rbp, &cannoli_memhook_write_r12_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r12_rsi, &cannoli_memhook_write_r12_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r12_rdi, &cannoli_memhook_write_r12_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r12_r8, &cannoli_memhook_write_r12_r8_end) },
                    unsafe { (&cannoli_memhook_write_r12_r9, &cannoli_memhook_write_r12_r9_end) },
                    unsafe { (&cannoli_memhook_write_r12_r10, &cannoli_memhook_write_r12_r10_end) },
                    unsafe { (&cannoli_memhook_write_r12_r11, &cannoli_memhook_write_r12_r11_end) },
                    unsafe { (&cannoli_memhook_write_r12_r12, &cannoli_memhook_write_r12_r12_end) },
                    unsafe { (&cannoli_memhook_write_r12_r13, &cannoli_memhook_write_r12_r13_end) },
                    unsafe { (&cannoli_memhook_write_r12_r14, &cannoli_memhook_write_r12_r14_end) },
                    unsafe { (&cannoli_memhook_write_r12_r15, &cannoli_memhook_write_r12_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r13_rax, &cannoli_memhook_write_r13_rax_end) },
                    unsafe { (&cannoli_memhook_write_r13_rcx, &cannoli_memhook_write_r13_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r13_rdx, &cannoli_memhook_write_r13_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r13_rbx, &cannoli_memhook_write_r13_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r13_rsp, &cannoli_memhook_write_r13_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r13_rbp, &cannoli_memhook_write_r13_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r13_rsi, &cannoli_memhook_write_r13_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r13_rdi, &cannoli_memhook_write_r13_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r13_r8, &cannoli_memhook_write_r13_r8_end) },
                    unsafe { (&cannoli_memhook_write_r13_r9, &cannoli_memhook_write_r13_r9_end) },
                    unsafe { (&cannoli_memhook_write_r13_r10, &cannoli_memhook_write_r13_r10_end) },
                    unsafe { (&cannoli_memhook_write_r13_r11, &cannoli_memhook_write_r13_r11_end) },
                    unsafe { (&cannoli_memhook_write_r13_r12, &cannoli_memhook_write_r13_r12_end) },
                    unsafe { (&cannoli_memhook_write_r13_r13, &cannoli_memhook_write_r13_r13_end) },
                    unsafe { (&cannoli_memhook_write_r13_r14, &cannoli_memhook_write_r13_r14_end) },
                    unsafe { (&cannoli_memhook_write_r13_r15, &cannoli_memhook_write_r13_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r14_rax, &cannoli_memhook_write_r14_rax_end) },
                    unsafe { (&cannoli_memhook_write_r14_rcx, &cannoli_memhook_write_r14_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r14_rdx, &cannoli_memhook_write_r14_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r14_rbx, &cannoli_memhook_write_r14_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r14_rsp, &cannoli_memhook_write_r14_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r14_rbp, &cannoli_memhook_write_r14_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r14_rsi, &cannoli_memhook_write_r14_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r14_rdi, &cannoli_memhook_write_r14_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r14_r8, &cannoli_memhook_write_r14_r8_end) },
                    unsafe { (&cannoli_memhook_write_r14_r9, &cannoli_memhook_write_r14_r9_end) },
                    unsafe { (&cannoli_memhook_write_r14_r10, &cannoli_memhook_write_r14_r10_end) },
                    unsafe { (&cannoli_memhook_write_r14_r11, &cannoli_memhook_write_r14_r11_end) },
                    unsafe { (&cannoli_memhook_write_r14_r12, &cannoli_memhook_write_r14_r12_end) },
                    unsafe { (&cannoli_memhook_write_r14_r13, &cannoli_memhook_write_r14_r13_end) },
                    unsafe { (&cannoli_memhook_write_r14_r14, &cannoli_memhook_write_r14_r14_end) },
                    unsafe { (&cannoli_memhook_write_r14_r15, &cannoli_memhook_write_r14_r15_end) },
                ],
                [
                    unsafe { (&cannoli_memhook_write_r15_rax, &cannoli_memhook_write_r15_rax_end) },
                    unsafe { (&cannoli_memhook_write_r15_rcx, &cannoli_memhook_write_r15_rcx_end) },
                    unsafe { (&cannoli_memhook_write_r15_rdx, &cannoli_memhook_write_r15_rdx_end) },
                    unsafe { (&cannoli_memhook_write_r15_rbx, &cannoli_memhook_write_r15_rbx_end) },
                    unsafe { (&cannoli_memhook_write_r15_rsp, &cannoli_memhook_write_r15_rsp_end) },
                    unsafe { (&cannoli_memhook_write_r15_rbp, &cannoli_memhook_write_r15_rbp_end) },
                    unsafe { (&cannoli_memhook_write_r15_rsi, &cannoli_memhook_write_r15_rsi_end) },
                    unsafe { (&cannoli_memhook_write_r15_rdi, &cannoli_memhook_write_r15_rdi_end) },
                    unsafe { (&cannoli_memhook_write_r15_r8, &cannoli_memhook_write_r15_r8_end) },
                    unsafe { (&cannoli_memhook_write_r15_r9, &cannoli_memhook_write_r15_r9_end) },
                    unsafe { (&cannoli_memhook_write_r15_r10, &cannoli_memhook_write_r15_r10_end) },
                    unsafe { (&cannoli_memhook_write_r15_r11, &cannoli_memhook_write_r15_r11_end) },
                    unsafe { (&cannoli_memhook_write_r15_r12, &cannoli_memhook_write_r15_r12_end) },
                    unsafe { (&cannoli_memhook_write_r15_r13, &cannoli_memhook_write_r15_r13_end) },
                    unsafe { (&cannoli_memhook_write_r15_r14, &cannoli_memhook_write_r15_r14_end) },
                    unsafe { (&cannoli_memhook_write_r15_r15, &cannoli_memhook_write_r15_r15_end) },
                ],
            ],
         ],
    ],
];

