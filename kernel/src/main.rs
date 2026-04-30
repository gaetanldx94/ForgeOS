#![no_std]
#![no_main]

extern crate alloc;

pub mod arch;
pub mod drivers;
pub mod memory;
pub mod process;
pub mod utils;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use drivers::vga::{Color, VgaWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { core::arch::asm!("cli", options(nostack, nomem)); }

    let mut w = VgaWriter::new();
    w.clear();
    w.set_color(Color::Cyan, Color::Black);
    w.print("ForgeOS Kernel\n");
    w.print("==============\n");

    unsafe { memory::init(); }
    w.set_color(Color::Green, Color::Black);
    w.print("[OK] Memory initialized\n");

    unsafe { arch::x86_64::init(); }
    w.print("[OK] IDT initialized\n");

    process::init();
    let pid_a = process::spawn("process_a", process_a as u64);
    let pid_b = process::spawn("process_b", process_b as u64);

    w.print("[OK] Scheduler initialized\n");
    w.set_color(Color::White, Color::Black);
    w.print("     PID A: ");
    w.print_usize(pid_a as usize);
    w.print("\n     PID B: ");
    w.print_usize(pid_b as usize);
    w.print("\n");

    w.print("ForgeOS ready.\n");

    unsafe { core::arch::asm!("sti", options(nostack, nomem)); }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn process_a() {
    let mut w = VgaWriter::new();
    let mut i = 0usize;
    loop {
        w.set_color(Color::Yellow, Color::Black);
        unsafe {
            let vga = (0xB8000 + 20 * 80 * 2) as *mut u16;
            let msg = b"[Process A] tick: ";
            for (j, &b) in msg.iter().enumerate() {
                *vga.add(j) = 0x0E00 | b as u16;
            }
            *vga.add(msg.len()) = 0x0E00 | (b'0' + (i % 10) as u8) as u16;
        }
        i = i.wrapping_add(1);
        for _ in 0..1_000_000 { unsafe { core::arch::asm!("nop"); } }
    }
}

fn process_b() {
    let mut i = 0usize;
    loop {
        unsafe {
            let vga = (0xB8000 + 21 * 80 * 2) as *mut u16;
            let msg = b"[Process B] tick: ";
            for (j, &b) in msg.iter().enumerate() {
                *vga.add(j) = 0x0B00 | b as u16;
            }
            *vga.add(msg.len()) = 0x0B00 | (b'0' + (i % 10) as u8) as u16;
        }
        i = i.wrapping_add(1);
        for _ in 0..1_000_000 { unsafe { core::arch::asm!("nop"); } }
    }
}

#[unsafe(no_mangle)]
extern "C" fn __alloc_error_handler(_size: usize, _align: usize) -> ! {
    let mut w = VgaWriter::new();
    w.set_color(Color::Red, Color::Black);
    w.print("[ALLOC ERROR]\n");
    loop {}
}

#[unsafe(no_mangle)]
fn rust_oom(_: core::alloc::Layout) -> ! {
    let mut w = VgaWriter::new();
    w.set_color(Color::Red, Color::Black);
    w.print("[OOM]\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut w = VgaWriter::new();
    w.set_color(Color::Red, Color::Black);
    w.print("[PANIC]\n");
    unsafe {
        core::arch::asm!("2: jmp 2b", options(nostack, nomem, noreturn));
    }
}
