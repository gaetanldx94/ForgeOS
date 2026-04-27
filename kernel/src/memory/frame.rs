#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}

impl Frame {
    pub const SIZE: usize = 4096;

    pub fn from_addr(addr: usize) -> Self {
        Frame { number: addr / Self::SIZE }
    }

    pub fn start_addr(&self) -> usize {
        self.number * Self::SIZE
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + Self::SIZE
    }
}