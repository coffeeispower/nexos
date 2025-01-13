use core::ops::DerefMut;

use crate::{
    bitmap_allocator::{BitmapAllocator, GLOBAL_PAGE_ALLOCATOR, PAGE_SIZE}, kernel::memory_map::{MemoryFlags, MemoryMap}, limine::HHDM
};
use x86_64::{
    registers::control::{Cr3, Cr3Flags}, structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    }, PhysAddr, VirtAddr
};

unsafe impl<'a> FrameAllocator<Size4KiB> for BitmapAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        Some(PhysFrame::containing_address(PhysAddr::new(
            self.request_and_clear_page()?.get() as u64,
        )))
    }
}
impl<'a> FrameDeallocator<Size4KiB> for BitmapAllocator<'a> {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.free_pages(frame.start_address().as_u64() as usize, PAGE_SIZE);
    }
}
pub struct X86MemoryMap<M: Mapper<Size4KiB> + Send>(M, PhysFrame<Size4KiB>);
impl<M: Mapper<Size4KiB> + Send> X86MemoryMap<M> {
    pub unsafe fn new(mapper: M, addr: PhysFrame<Size4KiB>) -> Self {
        Self(mapper, addr)
    }
}
impl<'a> X86MemoryMap<OffsetPageTable<'a>> {
    pub unsafe fn current_memory_map() -> Self {
        unsafe {
            let offset = VirtAddr::new(HHDM.get_response().unwrap().offset());
            let active_table = active_level_4_table(offset);
            Self::new(OffsetPageTable::new(active_table, offset), Cr3::read().0)
        }
    }
}
impl From<MemoryFlags> for PageTableFlags {
    fn from(value: MemoryFlags) -> Self {
        let mut x86_flags = PageTableFlags::PRESENT;
        if value.contains(MemoryFlags::WRITABLE) {
            x86_flags |= PageTableFlags::WRITABLE;
        }
        if value.contains(MemoryFlags::USER_ACCESSIBLE) {
            x86_flags |= PageTableFlags::USER_ACCESSIBLE;
        }
        if value.contains(MemoryFlags::WRITE_THROUGH) {
            x86_flags |= PageTableFlags::WRITE_THROUGH;
        }
        if value.contains(MemoryFlags::NO_CACHE) {
            x86_flags |= PageTableFlags::NO_CACHE;
        }
        if value.contains(MemoryFlags::NO_EXECUTE) {
            x86_flags |= PageTableFlags::NO_EXECUTE;
        }
        x86_flags
    }
}
unsafe impl<M: Mapper<Size4KiB> + Send> MemoryMap for X86MemoryMap<M> {
    unsafe fn map_memory(&mut self, from: usize, to: usize, flags: MemoryFlags) -> bool {

        return self.0.map_to(
                Page::containing_address(VirtAddr::new(from as u64)),
                PhysFrame::containing_address(PhysAddr::new(to as u64)),
                flags.into(),
                GLOBAL_PAGE_ALLOCATOR.lock().deref_mut(),
            )
            .map(|f| f.flush())
            .is_ok();
    }

    unsafe fn unmap_memory(&mut self, from: usize) -> bool {
        return self.0.unmap(Page::containing_address(VirtAddr::new(from as u64))).map(|f| f.1.flush()).is_ok();
    }

    unsafe fn load_memory_map(&self) {
        Cr3::write(self.1, Cr3Flags::empty());
    }
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub unsafe fn active_page_table_mapper() -> OffsetPageTable<'static> {
    let offset = VirtAddr::new(HHDM.get_response().unwrap().offset());
    let active_table = unsafe { active_level_4_table(offset) };
    OffsetPageTable::new(active_table, offset)
}

#[cfg(test)]
mod tests {
    use core::ops::DerefMut;

    use crate::{
        bitmap_allocator::{GLOBAL_PAGE_ALLOCATOR, PAGE_SIZE},
        limine::HHDM,
    };

    use super::*;
    use x86_64::structures::paging::{
        mapper::CleanUp, FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags,
        PageTableIndex,
    };
    #[test(name = "Map a page and then trying to access that memory through the virtual memory")]
    fn accessing_current_page_table_and_map_memory() {
        let offset = VirtAddr::new(HHDM.get_response().unwrap().offset());
        let active_table = unsafe { active_level_4_table(offset) };

        for (i, entry) in active_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("L4 Entry {}: {:?}", i, entry);
            }
        }
        unsafe {
            let mut mapper = OffsetPageTable::new(active_table, offset);
            const TEST_VALUE: u64 = 0xe55fefabcdef;
            let mut allocator = GLOBAL_PAGE_ALLOCATOR.lock();
            let real_addr = allocator.allocate_frame().unwrap();
            use PageTableIndex as PTI;
            let page =
                Page::from_page_table_indices(PTI::new(10), PTI::new(0), PTI::new(0), PTI::new(1));
            mapper
                .map_to(
                    page,
                    real_addr,
                    PageTableFlags::WRITABLE
                        | PageTableFlags::PRESENT
                        | PageTableFlags::USER_ACCESSIBLE,
                    &mut *allocator,
                )
                .unwrap()
                .flush();
            page.start_address()
                .as_mut_ptr::<u64>()
                .write_volatile(TEST_VALUE);
            println!("Virtual address 0x{:X}", page.start_address().as_u64());
            assert_eq!(
                (real_addr.start_address().as_u64() as *mut u64).read_volatile(),
                TEST_VALUE
            );
            mapper.unmap(page).unwrap().1.flush();
            allocator.free_pages(real_addr.start_address().as_u64() as usize, PAGE_SIZE);
            mapper.clean_up_addr_range(Page::range_inclusive(page, page), allocator.deref_mut());
        }
    }
}
