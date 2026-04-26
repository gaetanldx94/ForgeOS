[ORG 0x8000]
[BITS 16]

_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    call detect_memory
    jmp SETUP_GDT_ADDR

detect_memory:
    mov di, 0x7004
    xor ebx, ebx
    xor bp, bp
    mov edx, 0x534D4150
    mov eax, 0xE820
    mov ecx, 20
    int 0x15
    jc .failed
    cmp eax, 0x534D4150
    jne .failed

.loop:
    jcxz .skip
    inc bp
    add di, 20
.skip:
    test ebx, ebx
    jz .done
    mov eax, 0xE820
    mov ecx, 20
    int 0x15
    jc .done
    jmp .loop

.done:
    mov [0x7000], bp
    ret

.failed:
    mov word [0x7000], 0
    ret