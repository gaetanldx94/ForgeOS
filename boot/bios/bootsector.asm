[ORG 0x7C00]
[BITS 16]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    mov [boot_drive], dl
    sti

    mov ah, 0x00
    mov al, 0x03
    int 0x10

    mov ah, 0x02
    mov bh, 0
    mov dh, 0
    mov dl, 0
    int 0x10

    mov si, msg_boot
    call print

    mov si, msg_loader
    call print

    mov ax, 0x0000
    mov es, ax
    mov bx, 0x8000
    mov ah, 0x02
    mov al, 20
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drive]
    int 0x13
    jc disk_error

    mov si, msg_loader_ok
    call print

    mov si, msg_kernel
    call print

    mov word [dap_size],     0x0010
    mov word [dap_count],    100
    mov word [dap_offset],   0x0000
    mov word [dap_segment],  0x2000
    mov dword [dap_lba_low], 22
    mov dword [dap_lba_high],0

    mov ah, 0x42
    mov dl, [boot_drive]
    mov si, dap
    int 0x13
    jc disk_error

    call copy_to_high

    mov si, msg_kernel_ok
    call print

    jmp 0x0000:0x8000

copy_to_high:
    cli
    push es

    lgdt [gdt_desc]

    mov eax, cr0
    or eax, 1
    mov cr0, eax

    mov ax, 0x08
    mov es, ax

    mov eax, cr0
    and eax, 0xFFFFFFFE
    mov cr0, eax

    xor ax, ax
    mov ds, ax

    mov esi, 0x20000
    mov edi, dword [high_dest]
    mov ecx, 512 * 25
    a32 rep movsd

    pop es
    sti
    ret

disk_error:
    mov si, msg_err
    call print
    cli
    hlt
    jmp disk_error

print:
    mov ah, 0x0e
    mov bh, 0
.next:
    lodsb
    or al, al
    jz .done
    int 0x10
    jmp .next
.done:
    ret

gdt_start:
    dq 0
    dq 0x00CF92000000FFFF
gdt_end:

gdt_desc:
    dw gdt_end - gdt_start - 1
    dd gdt_start

dap:
dap_size        dw 0x0010
dap_count       dw 0
dap_offset      dw 0
dap_segment     dw 0
dap_lba_low     dd 0
dap_lba_high    dd 0

boot_drive      db 0
high_dest       dd 0x100000

msg_boot        db "[OK] Bootsector", 0x0D, 0x0A, 0
msg_loader      db "[..] Loading loader...", 0x0D, 0x0A, 0
msg_loader_ok   db "[OK] Loader loaded", 0x0D, 0x0A, 0
msg_kernel      db "[..] Loading kernel...", 0x0D, 0x0A, 0
msg_kernel_ok   db "[OK] Kernel loaded", 0x0D, 0x0A, 0
msg_err         db "[ERR] Disk read failed", 0x0D, 0x0A, 0

times 510-($-$$) db 0
dw 0xAA55