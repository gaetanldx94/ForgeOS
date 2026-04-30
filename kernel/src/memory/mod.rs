pub mod allocator;
pub mod frame;
pub mod heap;
pub mod map;

use allocator::BitmapAllocator;
use heap::{LockedAllocator, HEAP_SIZE, HEAP_START};
use map::MemoryMap;

pub static mut FRAME_ALLOCATOR: BitmapAllocator = BitmapAllocator::new();

#[global_allocator]
static HEAP_ALLOCATOR: LockedAllocator = LockedAllocator::new();

pub unsafe fn init() {
    let map = MemoryMap::read();
    FRAME_ALLOCATOR.init(&map);

    init_heap();
}

unsafe fn init_heap() {
    use crate::arch::x86_64::paging::{Mapper, PageFlags};

    let mut mapper = Mapper::from_cr3();
    let pages = HEAP_SIZE / frame::Frame::SIZE;

    for i in 0..pages {
        let vaddr = HEAP_START + i * frame::Frame::SIZE;
        let frame = FRAME_ALLOCATOR
            .alloc()
            .expect("out of memory: cannot map heap");

        mapper.map(
            vaddr as u64,
            frame,
            PageFlags::PRESENT | PageFlags::WRITABLE,
        );
    }

    HEAP_ALLOCATOR.init(HEAP_START, HEAP_SIZE);
}

pub fn alloc_frame() -> Option<frame::Frame> {
    unsafe { FRAME_ALLOCATOR.alloc() }
}

pub fn free_frame(frame: frame::Frame) {
    unsafe { FRAME_ALLOCATOR.free(frame) }
}