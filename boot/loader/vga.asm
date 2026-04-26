%macro vga_clear 0
    mov edi, 0xB8000
    mov ecx, 2000
    mov ax, 0x0F00
%%clear_loop:
    mov [edi], ax
    add edi, 2
    dec ecx
    jnz %%clear_loop
%endmacro

%macro vga_print 3
    mov esi, %1
    mov edi, 0xB8000 + %2
    mov ah, %3
%%loop:
    lodsb
    or al, al
    jz %%done
    mov [edi], ax
    add edi, 2
    jmp %%loop
%%done:
%endmacro