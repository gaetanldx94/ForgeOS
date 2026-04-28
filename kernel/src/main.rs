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
    w.print("ForgeOS Kernel\n");
    w.print("==============\n");

    unsafe { memory::init(); }
    w.set_color(Color::Green, Color::Black);
    w.print("[OK] Memory initialized\n");

    unsafe { arch::x86_64::init(); }
    w.print("[OK] IDT initialized\n");

    unsafe {
        let mut mapper = arch::x86_64::paging::Mapper::from_cr3();

        let vga_phys = mapper.translate(0xB8000);
        w.set_color(Color::White, Color::Black);
        w.print("     VGA 0xB8000 -> 0x");
        if let Some(p) = vga_phys {
            w.print_hex(p);
        } else {
            w.print("NOT MAPPED");
        }
        w.print("\n");

        let frame = memory::alloc_frame().unwrap();
        let test_vaddr = 0x400000u64;
        mapper.map(
            test_vaddr,
            frame,
            arch::x86_64::paging::PageFlags::PRESENT |
            arch::x86_64::paging::PageFlags::WRITABLE,
        );

        let ptr = test_vaddr as *mut u64;
        *ptr = 0xDEADBEEF;
        let val = *ptr;

        w.set_color(Color::Green, Color::Black);
        w.print("[OK] Paging initialized\n");
        w.set_color(Color::White, Color::Black);
        w.print("     Test write/read : 0x");
        w.print_hex(val);
        w.print("\n");
    }

    unsafe { core::arch::asm!("sti", options(nostack, nomem)); }
    w.set_color(Color::Green, Color::Black);
    w.print("[OK] Interrupts enabled\n");

    w.set_color(Color::White, Color::Black);
    w.print("ForgeOS ready.\n");

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
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
