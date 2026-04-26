#include "vga.h"

static int vga_row = 5;
static int vga_col = 0;

void vga_clear() {
    for (int i = 0; i < VGA_COLS * VGA_ROWS; i++) {
        VGA_BASE[i] = ((uint16_t)COLOR(WHITE, BLACK) << 8) | ' ';
    }
    vga_row = 0;
    vga_col = 0;
}

void vga_putchar(char c, uint8_t color) {
    if (c == '\n' || vga_col >= VGA_COLS) {
        vga_row++;
        vga_col = 0;
        if (c == '\n') return;
    }
    if (vga_row >= VGA_ROWS) {
        for (int i = 0; i < (VGA_ROWS - 1) * VGA_COLS; i++) {
            VGA_BASE[i] = VGA_BASE[i + VGA_COLS];
        }
        for (int i = (VGA_ROWS - 1) * VGA_COLS; i < VGA_ROWS * VGA_COLS; i++) {
            VGA_BASE[i] = ((uint16_t)COLOR(WHITE, BLACK) << 8) | ' ';
        }
        vga_row = VGA_ROWS - 1;
    }
    VGA_BASE[vga_row * VGA_COLS + vga_col] = ((uint16_t)color << 8) | (uint8_t)c;
    vga_col++;
}

void vga_print(const char *str, uint8_t color) {
    while (*str) {
        vga_putchar(*str++, color);
    }
    vga_row++;
    vga_col = 0;
}

void vga_print_hex(uint64_t val, uint8_t color) {
    char buf[19];
    buf[0] = '0';
    buf[1] = 'x';
    for (int i = 0; i < 16; i++) {
        uint8_t nibble = (val >> (60 - i * 4)) & 0xF;
        buf[2 + i] = nibble < 10 ? '0' + nibble : 'A' + nibble - 10;
    }
    buf[18] = 0;
    vga_print(buf, color);
}

void vga_print_int(uint32_t val, uint8_t color) {
    if (val == 0) {
        vga_print("0", color);
        return;
    }
    char buf[11];
    int i = 10;
    buf[i] = 0;
    while (val > 0) {
        buf[--i] = '0' + (val % 10);
        val /= 10;
    }
    vga_print(&buf[i], color);
}

void vga_println(const char *label, uint8_t lcolor,
                 const char *val,   uint8_t vcolor) {
    while (*label) vga_putchar(*label++, lcolor);
    while (*val)   vga_putchar(*val++,   vcolor);
    vga_row++;
    vga_col = 0;
}