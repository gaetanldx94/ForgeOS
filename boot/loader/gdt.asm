[BITS 16]
global setup_gdt

extern enter_protected_mode

gdt_start:
    dq 0

    ; Code 32 bits (0x08)
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10011010b
    db 11001111b
    db 0x00

    ; Data 32 bits (0x10)
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 11001111b
    db 0x00

    ; Code 64 bits (0x18)
    dw 0x0000
    dw 0x0000
    db 0x00
    db 10011010b
    db 10101111b
    db 0x00

    ; Data 64 bits (0x20)
    dw 0x0000
    dw 0x0000
    db 0x00
    db 10010010b
    db 10101111b
    db 0x00

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

setup_gdt:
    lgdt [gdt_descriptor]
    call enter_protected_mode
    ret