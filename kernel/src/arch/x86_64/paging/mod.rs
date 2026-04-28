pub mod entry;
pub mod mapper;
pub mod table;

pub use entry::{PageEntry, PageFlags};
pub use mapper::{Mapper, VirtAddr};
pub use table::PageTable;

use core::arch::asm;

pub unsafe fn flush_tlb() {
    let cr3: u64;
    asm!("mov {}, cr3", out(reg) cr3, options(nostack, nomem));
    asm!("mov cr3, {}", in(reg) cr3, options(nostack, nomem));
}

pub unsafe fn current_pml4() -> u64 {
    let cr3: u64;
    asm!("mov {}, cr3", out(reg) cr3, options(nostack, nomem));
    cr3
}