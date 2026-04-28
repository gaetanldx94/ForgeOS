use super::entry::PageEntry;

pub const ENTRY_COUNT: usize = 512;

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageEntry; ENTRY_COUNT],
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable {
            entries: [PageEntry::empty(); ENTRY_COUNT],
        }
    }

    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageEntry::empty();
        }
    }

    pub fn get(&self, index: usize) -> &PageEntry {
        &self.entries[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut PageEntry {
        &mut self.entries[index]
    }

    pub unsafe fn next_table(&self, index: usize) -> Option<&PageTable> {
        let entry = &self.entries[index];
        if !entry.is_present() || entry.is_huge() {
            return None;
        }
        let addr = entry.addr() as usize;
        Some(&*(addr as *const PageTable))
    }

    pub unsafe fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable> {
        let entry = &self.entries[index];
        if !entry.is_present() || entry.is_huge() {
            return None;
        }
        let addr = entry.addr() as usize;
        Some(&mut *(addr as *mut PageTable))
    }
}