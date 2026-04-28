#[derive(Debug, Clone, Copy)]
pub struct PageFlags(u64);

impl PageFlags {
    pub const PRESENT:      Self = Self(1 << 0);
    pub const WRITABLE:     Self = Self(1 << 1);
    pub const USER:         Self = Self(1 << 2);
    pub const WRITE_THROUGH:Self = Self(1 << 3);
    pub const NO_CACHE:     Self = Self(1 << 4);
    pub const ACCESSED:     Self = Self(1 << 5);
    pub const DIRTY:        Self = Self(1 << 6);
    pub const HUGE:         Self = Self(1 << 7);
    pub const GLOBAL:       Self = Self(1 << 8);
    pub const NO_EXECUTE:   Self = Self(1 << 63);

    pub const fn empty() -> Self { Self(0) }

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn set(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn clear(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

impl core::ops::BitOr for PageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageEntry(u64);

impl PageEntry {
    pub const fn empty() -> Self {
        PageEntry(0)
    }

    pub fn new(addr: u64, flags: PageFlags) -> Self {
        PageEntry((addr & 0x000FFFFF_FFFFF000) | flags.0)
    }

    pub fn addr(&self) -> u64 {
        self.0 & 0x000FFFFF_FFFFF000
    }

    pub fn flags(&self) -> PageFlags {
        PageFlags(self.0 & 0xFFF)
    }

    pub fn is_present(&self) -> bool {
        self.flags().contains(PageFlags::PRESENT)
    }

    pub fn is_huge(&self) -> bool {
        self.flags().contains(PageFlags::HUGE)
    }
}