use super::entry::{PageEntry, PageFlags};
use super::table::PageTable;
use crate::memory;
use crate::memory::frame::Frame;

pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(addr: u64) -> Self {
        VirtAddr(addr)
    }

    pub fn pml4_index(&self) -> usize {
        ((self.0 >> 39) & 0x1FF) as usize
    }
    pub fn pdpt_index(&self) -> usize {
        ((self.0 >> 30) & 0x1FF) as usize
    }
    pub fn pdt_index(&self) -> usize {
        ((self.0 >> 21) & 0x1FF) as usize
    }
    pub fn pt_index(&self) -> usize {
        ((self.0 >> 12) & 0x1FF) as usize
    }
    pub fn page_offset(&self) -> u64 {
        self.0 & 0xFFF
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

pub struct Mapper {
    pml4: &'static mut PageTable,
}

impl Mapper {
    pub unsafe fn from_cr3() -> Self {
        let cr3: u64;
        core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nostack, nomem));
        let pml4 = &mut *(cr3 as *mut PageTable);
        Mapper { pml4 }
    }

    pub fn translate(&self, vaddr: u64) -> Option<u64> {
        let v = VirtAddr::new(vaddr);
        unsafe {
            let pdpt = self.pml4.next_table(v.pml4_index())?;
            let pdt = pdpt.next_table(v.pdpt_index())?;

            let pdt_entry = pdt.get(v.pdt_index());
            if pdt_entry.is_huge() {
                let base = pdt_entry.addr();
                let offset = vaddr & 0x1FFFFF;
                return Some(base + offset);
            }

            let pt = pdt.next_table(v.pdt_index())?;
            let pt_entry = pt.get(v.pt_index());
            if !pt_entry.is_present() {
                return None;
            }
            Some(pt_entry.addr() + v.page_offset())
        }
    }

    pub unsafe fn map(&mut self, vaddr: u64, frame: Frame, flags: PageFlags) {
        let v = VirtAddr::new(vaddr);
        let phys = frame.start_addr() as u64;

        let pml4 = self.pml4 as *mut PageTable;

        let pdpt = self.get_or_create(pml4, v.pml4_index());
        let pdt = self.get_or_create(pdpt, v.pdpt_index());
        let pt = self.get_or_create(pdt, v.pdt_index());

        let entry = (*pt).get_mut(v.pt_index());
        *entry = PageEntry::new(phys, flags | PageFlags::PRESENT);

        core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, nomem));
    }

    pub unsafe fn unmap(&mut self, vaddr: u64) {
        let v = VirtAddr::new(vaddr);

        if let Some(pdpt) = self.pml4.next_table_mut(v.pml4_index()) {
            if let Some(pdt) = pdpt.next_table_mut(v.pdpt_index()) {
                if let Some(pt) = pdt.next_table_mut(v.pdt_index()) {
                    *pt.get_mut(v.pt_index()) = PageEntry::empty();
                    core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, nomem));
                }
            }
        }
    }

    unsafe fn get_or_create(&mut self, table: *mut PageTable, index: usize) -> *mut PageTable {
        let entry = (*table).get_mut(index);

        if entry.is_present() {
            return entry.addr() as *mut PageTable;
        }

        let frame = memory::alloc_frame().expect("out of memory: cannot allocate page table");

        let new_table = frame.start_addr() as *mut PageTable;
        (*new_table).zero();

        *entry = PageEntry::new(
            frame.start_addr() as u64,
            PageFlags::PRESENT | PageFlags::WRITABLE,
        );

        new_table
    }
}
