use core::ops::DerefMut;

use crate::{
    bitmap_allocator::{BitmapAllocator, GLOBAL_PAGE_ALLOCATOR, PAGE_SIZE},
    heap::KernelHeapMapper,
};
use x86_64::{
    structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

unsafe impl<'a> FrameAllocator<Size4KiB> for BitmapAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        Some(PhysFrame::containing_address(PhysAddr::new(
            self.request_page()?.get() as u64,
        )))
    }
}
impl<'a> FrameDeallocator<Size4KiB> for BitmapAllocator<'a> {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.free_pages(frame.start_address().as_u64() as usize, PAGE_SIZE);
    }
}

unsafe impl<T: Mapper<Size4KiB>> KernelHeapMapper for T {
    unsafe fn map_memory(&mut self, from: usize, to: usize) -> bool {
        return self
            .map_to(
                Page::containing_address(VirtAddr::new(from as u64)),
                PhysFrame::containing_address(PhysAddr::new(to as u64)),
                PageTableFlags::WRITABLE
                    | PageTableFlags::PRESENT
                    | PageTableFlags::USER_ACCESSIBLE,
                GLOBAL_PAGE_ALLOCATOR.lock().deref_mut(),
            )
            .map(|f| f.flush())
            .is_ok();
    }
}
