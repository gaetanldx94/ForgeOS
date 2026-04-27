#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryType {
    Free     = 1,
    Reserved = 2,
    Acpi     = 3,
    Nvs      = 4,
    Bad      = 5,
    Unknown  = 0xFF,
}

impl From<u32> for MemoryType {
    fn from(val: u32) -> Self {
        match val {
            1 => MemoryType::Free,
            2 => MemoryType::Reserved,
            3 => MemoryType::Acpi,
            4 => MemoryType::Nvs,
            5 => MemoryType::Bad,
            _ => MemoryType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryEntry {
    pub base:   u64,
    pub length: u64,
    pub kind:   u32,
}

pub struct MemoryMap {
    entries: *const MemoryEntry,
    count:   usize,
}

impl MemoryMap {
    /// Lit la carte mémoire depuis l'adresse fixe 0x7000
    pub unsafe fn read() -> Self {
        let count = *(0x7000 as *const u32) as usize;
        let entries = 0x7004 as *const MemoryEntry;
        MemoryMap { entries, count }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn entry(&self, index: usize) -> Option<MemoryEntry> {
        if index < self.count {
            Some(unsafe { core::ptr::read_unaligned(self.entries.add(index)) })
        } else {
            None
        }
    }

    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter { map: self, index: 0 }
    }

    /// Mémoire totale libre en bytes
    pub fn total_free(&self) -> u64 {
        self.iter()
            .filter(|e| MemoryType::from(e.kind) == MemoryType::Free)
            .map(|e| e.length)
            .sum()
    }
}

pub struct MemoryMapIter<'a> {
    map:   &'a MemoryMap,
    index: usize,
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = MemoryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.map.entry(self.index);
        self.index += 1;
        entry
    }
}