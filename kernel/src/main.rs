#![no_std]
#![no_main]

pub mod arch;
pub mod drivers;
pub mod memory;
pub mod process;
pub mod utils;

use core::panic::PanicInfo;
use drivers::vga::{Color, VgaWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { core::arch::asm!("cli", options(nostack, nomem)); }
    
    let mut w = VgaWriter::new();
    w.clear();
    w.set_color(Color::Cyan, Color::Black);
    w.print("================================\n");
    w.print("         ForgeOS Kernel         \n");
    w.print("================================\n");
    w.set_color(Color::Green, Color::Black);
    w.print("[OK] Kernel started\n");
    w.set_color(Color::White, Color::Black);
    w.print("Hello from Rust!\n");

    unsafe {
        core::arch::asm!(
            "2: jmp 2b",
            options(nostack, nomem, noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut w = VgaWriter::new();
    w.set_color(Color::Red, Color::Black);
    w.print("[PANIC]\n");
    loop {}
}