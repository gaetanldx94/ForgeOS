[BITS 16]
global enter_protected_mode

extern setup_paging
extern enter_long_mode

%include "boot/loader/vga.asm"

enter_protected_mode:
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    jmp 0x08:pmode_entry

[BITS 32]
pmode_entry:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax
    mov esp, 0x90000

    vga_clear
    vga_print msg_pmode, 0, 0x0A

    call setup_paging
    call enter_long_mode

msg_pmode db "[OK] Protected mode", 0
section .note.GNU-stack noalloc noexec nowrite progbits