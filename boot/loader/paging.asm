[BITS 32]
global setup_paging

%include "boot/loader/vga.asm"

setup_paging:
    mov edi, pml4
    mov ecx, 4096 * 3
    xor eax, eax
    rep stosb

    mov eax, pdpt
    or eax, 0x3
    mov [pml4], eax

    mov eax, pdt
    or eax, 0x3
    mov [pdpt], eax

    mov ecx, 0
    mov eax, 0x83
.map_loop:
    mov [pdt + ecx * 8], eax
    add eax, 0x200000
    inc ecx
    cmp ecx, 512
    jl .map_loop

    mov eax, pml4
    mov cr3, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    vga_print msg_paging, 160, 0x0A

    ret

msg_paging db "[OK] Paging", 0

section .bss
align 4096
pml4:   resb 4096
pdpt:   resb 4096
pdt:    resb 4096

section .note.GNU-stack noalloc noexec nowrite progbits