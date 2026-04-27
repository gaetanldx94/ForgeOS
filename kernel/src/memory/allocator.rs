use super::frame::Frame;
use super::map::{MemoryMap, MemoryType};

const MAX_FRAMES: usize = 32 * 1024;
const BITMAP_SIZE: usize = MAX_FRAMES / 64;

pub struct BitmapAllocator {
    bitmap:      [u64; BITMAP_SIZE],
    total:       usize,
    free:        usize,
    first_free:  usize,
}

impl BitmapAllocator {
    pub const fn new() -> Self {
        BitmapAllocator {
            bitmap:     [0xFFFFFFFFFFFFFFFF; BITMAP_SIZE],
            total:      0,
            free:       0,
            first_free: 0,
        }
    }

    pub fn init(&mut self, map: &MemoryMap) {
        for entry in map.iter() {
            if MemoryType::from(entry.kind) != MemoryType::Free {
                continue;
            }

            let start = entry.base as usize;
            let end   = (entry.base + entry.length) as usize;

            let first_frame = start / Frame::SIZE;
            let last_frame  = end   / Frame::SIZE;

            for frame in first_frame..last_frame {
                if frame < MAX_FRAMES {
                    self.set_free(frame);
                    self.total += 1;
                    self.free  += 1;
                }
            }
        }

        // Marquer les zones critiques comme réservées
        // 0x0000 - 0x9000  : BIOS, bootloader data
        // 0x8000 - 0x30000 : loader
        // 0x100000+        : kernel (déjà chargé)
        for frame in 0..(0x30000 / Frame::SIZE) {
            self.set_used(frame);
            self.free = self.free.saturating_sub(1);
        }

        for frame in (0x100000 / Frame::SIZE)..(0x200000 / Frame::SIZE) {
            self.set_used(frame);
            self.free = self.free.saturating_sub(1);
        }

        self.update_first_free();
    }

    pub fn alloc(&mut self) -> Option<Frame> {
        if self.free == 0 {
            return None;
        }

        let frame_num = self.find_free()?;
        self.set_used(frame_num);
        self.free -= 1;
        Some(Frame { number: frame_num })
    }

    pub fn free(&mut self, frame: Frame) {
        self.set_free(frame.number);
        self.free += 1;
        if frame.number < self.first_free {
            self.first_free = frame.number;
        }
    }

    pub fn total_frames(&self) -> usize { self.total }
    pub fn free_frames(&self)  -> usize { self.free  }
    pub fn used_frames(&self)  -> usize { self.total - self.free }

    fn set_free(&mut self, frame: usize) {
        if frame < MAX_FRAMES {
            self.bitmap[frame / 64] &= !(1u64 << (frame % 64));
        }
    }

    fn set_used(&mut self, frame: usize) {
        if frame < MAX_FRAMES {
            self.bitmap[frame / 64] |= 1u64 << (frame % 64);
        }
    }

    fn is_free(&self, frame: usize) -> bool {
        if frame >= MAX_FRAMES { return false; }
        self.bitmap[frame / 64] & (1u64 << (frame % 64)) == 0
    }

    fn find_free(&self) -> Option<usize> {
        for (i, &word) in self.bitmap[self.first_free / 64..].iter().enumerate() {
            if word != 0xFFFFFFFFFFFFFFFF {
                let bit = word.trailing_ones() as usize;
                let frame = (self.first_free / 64 + i) * 64 + bit;
                if frame < MAX_FRAMES {
                    return Some(frame);
                }
            }
        }
        None
    }

    fn update_first_free(&mut self) {
        for (i, &word) in self.bitmap.iter().enumerate() {
            if word != 0xFFFFFFFFFFFFFFFF {
                self.first_free = i * 64;
                return;
            }
        }
        self.first_free = MAX_FRAMES;
    }
}