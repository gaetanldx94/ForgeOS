[BITS 32]
global enter_long_mode

extern boot_main

%include "boot/loader/vga.asm"

enter_long_mode:
    vga_print msg_lm_start, 320, 0x0E   ; ligne 2 — jaune

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    jmp 0x18:long_mode_entry

[BITS 64]
long_mode_entry:
    mov ax, 0x20
    mov ds, ax
    mov es, ax
    mov ss, ax
    xor ax, ax
    mov fs, ax
    mov gs, ax

    mov rsp, 0x90000

    mov rsi, msg_lm_ok
    mov rdi, 0xB8000 + 480
    mov ah, 0x0A
.log:
    lodsb
    or al, al
    jz .done
    mov [rdi], ax
    add rdi, 2
    jmp .log
.done:

    call boot_main

    cli
    hlt
.hang:
    jmp .hang

msg_lm_start db "[..] Entering long mode", 0
msg_lm_ok    db "[OK] Long mode", 0

section .note.GNU-stack noalloc noexec nowrite progbits