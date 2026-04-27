use super::color::{Color, ColorCode};
use core::fmt;

const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
pub const VGA_COLS: usize = 80;
pub const VGA_ROWS: usize = 25;

pub struct VgaWriter {
    row: usize,
    col: usize,
    color: ColorCode,
}

impl VgaWriter {
    pub const fn new() -> Self {
        VgaWriter {
            row: 0,
            col: 0,
            color: ColorCode(0x0F),
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = ColorCode::new(fg, bg);
    }

    pub fn clear(&mut self) {
        for i in 0..VGA_COLS * VGA_ROWS {
            unsafe {
                *VGA_BUFFER.add(i) = (self.color.0 as u16) << 8 | b' ' as u16;
            }
        }
        self.row = 0;
        self.col = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col >= VGA_COLS {
                    self.newline();
                }
                unsafe {
                    *VGA_BUFFER.add(self.row * VGA_COLS + self.col) =
                        (self.color.0 as u16) << 8 | byte as u16;
                }
                self.col += 1;
            }
        }
    }

    pub fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b'?'),
            }
        }
    }

    fn newline(&mut self) {
        if self.row + 1 >= VGA_ROWS {
            self.scroll();
        } else {
            self.row += 1;
        }
        self.col = 0;
    }

    fn scroll(&mut self) {
        for row in 1..VGA_ROWS {
            for col in 0..VGA_COLS {
                unsafe {
                    let cell = *VGA_BUFFER.add(row * VGA_COLS + col);
                    *VGA_BUFFER.add((row - 1) * VGA_COLS + col) = cell;
                }
            }
        }
        for col in 0..VGA_COLS {
            unsafe {
                *VGA_BUFFER.add((VGA_ROWS - 1) * VGA_COLS + col) =
                    (self.color.0 as u16) << 8 | b' ' as u16;
            }
        }
        self.row = VGA_ROWS - 1;
    }

    pub fn print_usize(&mut self, val: usize) {
        if val == 0 {
            self.print("0");
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 20;
        let mut n = val;
        while n > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        for &b in &buf[i..] {
            self.write_byte(b);
        }
    }

    pub fn print_hex(&mut self, val: u64) {
        self.print("0x");
        for i in (0..16).rev() {
            let nibble = ((val >> (i * 4)) & 0xF) as u8;
            let c = if nibble < 10 {
                b'0' + nibble
            } else {
                b'a' + nibble - 10
            };
            self.write_byte(c);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}
