use x86_64::{structures::paging::{FrameAllocator, FrameDeallocator, PageTable, PhysFrame, Size4KiB}, PhysAddr};
use crate::bitmap_allocator::BitmapAllocator;

unsafe impl FrameAllocator<Size4KiB> for BitmapAllocator {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        Some(unsafe { PhysFrame::from_start_address_unchecked(PhysAddr::new_unsafe(self.request_and_clear_page::<PageTable>()?.as_ptr() as usize as u64))})
    }
}
impl FrameDeallocator<Size4KiB> for BitmapAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.free_pages(frame.start_address().as_u64() as *mut PageTable, 1);
    }
}
