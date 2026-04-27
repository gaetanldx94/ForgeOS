use core::arch::asm;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GateType {
    Interrupt = 0x8E,
    Trap      = 0x8F,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low:  u16,
    selector:    u16,
    ist:         u8,
    attributes:  u8,
    offset_mid:  u16,
    offset_high: u32,
    _reserved:   u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        IdtEntry {
            offset_low:  0,
            selector:    0,
            ist:         0,
            attributes:  0,
            offset_mid:  0,
            offset_high: 0,
            _reserved:   0,
        }
    }

    pub fn new(handler: u64, gate: GateType) -> Self {
        IdtEntry {
            offset_low:  (handler & 0xFFFF) as u16,
            selector:    0x18,
            ist:         0,
            attributes:  gate as u8,
            offset_mid:  ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFFFFFF) as u32,
            _reserved:   0,
        }
    }
}

#[repr(C, packed)]
pub struct IdtPointer {
    limit: u16,
    base:  u64,
}

pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            entries: [IdtEntry::missing(); 256],
        }
    }

    pub fn set_handler(&mut self, vector: u8, handler: u64) {
        self.entries[vector as usize] = IdtEntry::new(handler, GateType::Interrupt);
    }

    pub fn set_trap(&mut self, vector: u8, handler: u64) {
        self.entries[vector as usize] = IdtEntry::new(handler, GateType::Trap);
    }

    pub fn load(&self) {
        let ptr = IdtPointer {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base:  self.entries.as_ptr() as u64,
        };
        unsafe {
            asm!("lidt [{}]", in(reg) &ptr, options(nostack));
        }
    }
}