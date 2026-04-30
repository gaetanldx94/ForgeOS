use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

pub const HEAP_START: usize = 0x0060_0000;
pub const HEAP_SIZE:  usize = 256 * 1024;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_free_region(start, size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert!(align_up(addr, core::mem::align_of::<ListNode>()) == addr);
        assert!(size >= core::mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    fn find_region(&mut self, size: usize, align: usize)
        -> Option<(&'static mut ListNode, usize)>
    {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        None
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize)
        -> Result<usize, ()>
    {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end   = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess = region.end_addr() - alloc_end;
        if excess > 0 && excess < core::mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(core::mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(core::mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for LinkedListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = Self::size_align(layout);

        let this = self as *const Self as *mut Self;

        if let Some((region, alloc_start)) = (*this).find_region(size, align) {
            let alloc_end   = alloc_start + size;
            let excess_size = region.end_addr() - alloc_end;

            if excess_size > 0 {
                (*this).add_free_region(alloc_end, excess_size);
            }

            alloc_start as *mut u8
        } else {
            core::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = Self::size_align(layout);
        let this = self as *const Self as *mut Self;
        (*this).add_free_region(ptr as usize, size);
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub struct LockedAllocator(LinkedListAllocator);

impl LockedAllocator {
    pub const fn new() -> Self {
        LockedAllocator(LinkedListAllocator::new())
    }

    pub unsafe fn init(&self, start: usize, size: usize) {
        let this = &self.0 as *const LinkedListAllocator as *mut LinkedListAllocator;
        (*this).init(start, size);
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }
}