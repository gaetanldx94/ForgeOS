pub mod allocator;
pub mod frame;
pub mod map;

use allocator::BitmapAllocator;
use map::MemoryMap;

pub static mut FRAME_ALLOCATOR: BitmapAllocator = BitmapAllocator::new();

pub unsafe fn init() {
    let map = MemoryMap::read();
    FRAME_ALLOCATOR.init(&map);
}

pub fn alloc_frame() -> Option<frame::Frame> {
    unsafe { FRAME_ALLOCATOR.alloc() }
}

pub fn free_frame(frame: frame::Frame) {
    unsafe { FRAME_ALLOCATOR.free(frame) }
}