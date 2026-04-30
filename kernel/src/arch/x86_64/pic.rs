const PIC1_CMD:  u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD:  u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 40;

unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nostack, nomem)
    );
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") val,
        in("dx") port,
        options(nostack, nomem)
    );
    val
}

unsafe fn io_wait() {
    outb(0x80, 0);
}

pub unsafe fn remap() {
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    outb(PIC1_CMD,  ICW1_INIT); io_wait();
    outb(PIC2_CMD,  ICW1_INIT); io_wait();

    outb(PIC1_DATA, PIC1_OFFSET); io_wait();
    outb(PIC2_DATA, PIC2_OFFSET); io_wait();

    outb(PIC1_DATA, 4); io_wait();
    outb(PIC2_DATA, 2); io_wait();

    outb(PIC1_DATA, ICW4_8086); io_wait();
    outb(PIC2_DATA, ICW4_8086); io_wait();

    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub unsafe fn disable() {
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}

pub unsafe fn eoi(irq: u8) {
    if irq >= 8 {
        outb(PIC2_CMD, 0x20);
    }
    outb(PIC1_CMD, 0x20);
}

/// frequency = 1193182 / divisor Hz
pub unsafe fn init_pit(divisor: u16) {
    outb(0x43, 0x36);
    outb(0x40, (divisor & 0xFF) as u8);
    outb(0x40, (divisor >> 8) as u8);
}

pub unsafe fn enable_timer() {
    let mask = inb(PIC1_DATA);
    outb(PIC1_DATA, mask & !1);
}