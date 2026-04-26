#pragma once
#include <stdint.h>

#define VGA_BASE    ((volatile uint16_t *)0xB8000)
#define VGA_COLS    80
#define VGA_ROWS    25

#define BLACK   0x0
#define BLUE    0x1
#define GREEN   0x2
#define CYAN    0x3
#define RED     0x4
#define MAGENTA 0x5
#define BROWN   0x6
#define WHITE   0xF
#define YELLOW  0xE
#define LGRAY   0x7

#define COLOR(fg, bg) ((uint8_t)((bg << 4) | fg))

void vga_clear();
void vga_putchar(char c, uint8_t color);
void vga_print(const char *str, uint8_t color);
void vga_print_hex(uint64_t val, uint8_t color);
void vga_print_int(uint32_t val, uint8_t color);