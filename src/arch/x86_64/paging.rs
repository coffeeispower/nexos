//! # x86_64 paging implementation
//! Processes in the operating system should not be able to access the entirety of the memory of the operating system,
//! as they could read other processes memory, so in x86_64, we can use page maps to create virtual memory spaces for
//! our processes.
//!
//! We cannot map individual addresses using paging, because that would require a lot of memory just for a single page map, and a operating system will have several of them for each process.
//! Instead, we map blocks of 4KB of memory, called pages.
//!
//! Page maps in x86_64 normally have 4 levels of tables. Basically the page map we set on the CR3 register is a pointer to a level 4 page map, which is a array of page entries that point to level 3 page maps, and so on.
//!
//! On level 1 page maps we have the individual pages. The virtual address that will be mapped to the physical address stores inside the
//! entry is based on the index in the page map, so the very first page would map `0..0x1000` to `phys_addr..phys_addr + 0x1000`.
use core::ptr::NonNull;

use bitflags::bitflags;

use crate::bitmap_allocator::GLOBAL_PAGE_ALLOCATOR;

bitflags! {
    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    pub struct PageEntry: usize {
        /// Indicates if the CPU should map this page or not
        /// If the CPU tries to access this page witht this bit cleared, it will throw a Page Fault
        const PRESENT = 1 << 0;
        /// Allow this page to be written to
        const WRITABLE = 1 << 1;
        /// Only allow the kernel to access this page
        const KERNEL_ONLY = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        /// Disables CPU cache
        const DISABLE_CACHE = 1 << 4;
        /// This bit is set automatically by the CPU whenever this page gets accessed
        const ACCESSED = 1 << 5;
        const LARGER_PAGES = 1 << 7;
        const _ = !0;
    }
}
impl PageEntry {
    /// Returns the physical address this page is mapping to
    pub fn phys_addr(&self) -> Option<usize> {
        if !self.contains(Self::PRESENT) {
            None
        } else {
            Some((self.bits() & 0xFFFFFFFFFFFFF000) >> (4 * 3))
        }
    }
    /// Maps this page to a physical address
    pub fn set_phys_addr(&mut self, phys_addr: usize) {
        *self.0.bits_mut() = (phys_addr << (4 * 3)) | (self.bits() & 0xFFF)
    }
    /// Converts the address inside this entry to a reference to a `PageTable`
    /// This returns `None` if the page is not present or if the physical address is a null pointer
    /// # Safety
    /// The caller must guarantee that the address inside this page entry is actually pointing to a page table.
    /// Calling this function will not cause undefined behavior by itself, but dereferencing the pointer can.
    pub unsafe fn page_table(&self) -> Option<NonNull<PageTable>> {
        self.phys_addr()
            .and_then(|addr| NonNull::new(addr as *mut PageTable))
    }
    /// Allocates a new page table and sets the physical address of this entry to the address of the page table
    ///
    /// Returns `None` if the page is already present, or `Some` with the newly allocated page table
    pub fn allocate_page_table(&mut self) -> Option<NonNull<PageTable>> {
        if self.contains(Self::PRESENT) {
            return None;
        }
        let page_table = GLOBAL_PAGE_ALLOCATOR
            .request_and_clear_page()
            .expect("Out of Memory");
        self.set_phys_addr(page_table.as_ptr() as usize);
        self.insert(Self::PRESENT | Self::WRITABLE);
        Some(page_table)
    }
    /// Returns the physical address as a PageTable or allocates a new one if not present
    ///
    /// # Safety
    /// If there's a physical address in this entry, it is not guaranteed that it is valid, therefore, this function is unsafe.
    pub unsafe fn get_or_allocate_page_table(&mut self) -> NonNull<PageTable> {
        self.allocate_page_table()
            .or_else(|| self.page_table())
            .expect("page table must be present")
    }
    /// Clones this page entry including the page tables inside it
    ///
    /// # Safety
    /// Because this function assumes this is not a level 1 page map, and all present entries are pointing to page maps, this function is unsafe
    pub unsafe fn clone_page_tables(&self, level: usize) -> Option<PageEntry> {
        let page_table = self.page_table()?.as_mut();
        let mut new_page_table = GLOBAL_PAGE_ALLOCATOR
            .request_and_clear_page::<PageTable>()
            .expect("Out of Memory");
        for (i, new_entry) in page_table.0.iter().cloned().enumerate() {
            if level > 1 {
                new_page_table.as_mut().0[i] = new_entry.clone_page_tables(level-1)?;
            } else {
                new_page_table.as_mut().0[i] = new_entry;
            }
        }
        let mut new_page_entry = *self;
        new_page_entry.set_phys_addr(new_page_table.as_ptr() as usize);
        Some(new_page_entry)
    }
}
#[repr(C, align(0x1000))]
pub struct PageTable(pub [PageEntry; 512]);
impl Default for PageTable {
    fn default() -> Self {
        Self([PageEntry::default(); 512])
    }
}
impl PageTable {
    /// Clones this page table recursively
    /// # Safety
    /// The caller must ensure that:
    /// - All the page entries are pointing to valid page tables
    /// - The level is correct; if the level is greater than the actual level of this table,
    ///   it will start cloning garbage
    pub unsafe fn clone_recursive(&self, level: usize) -> NonNull<PageTable> {
        let mut new_page_table = GLOBAL_PAGE_ALLOCATOR
            .request_and_clear_page::<PageTable>()
            .expect("Out of Memory");
        unsafe {
            for (i, &entry) in self.0.iter().enumerate() {
                let Some(new_entry) = entry.clone_page_tables(level) else {
                    new_page_table.as_mut().0[i] = entry;
                    continue;
                };
                new_page_table.as_mut().0[i] = new_entry;
            }
        }

        new_page_table
    }
    /// Maps the page at the specified virtual address to a physical address
    pub fn map_memory(&mut self, virtual_address: usize, physical_address: usize) {
        let indices = get_page_index(virtual_address);
        // SAFETY: All page maps > level 1 always point to page maps
        let l4 = unsafe { self.0[indices[3]].get_or_allocate_page_table().as_mut() };
        let l3 = unsafe { l4.0[indices[2]].get_or_allocate_page_table().as_mut() };
        let l2 = unsafe { l3.0[indices[1]].get_or_allocate_page_table().as_mut() };
        let l1_page = &mut l2.0[indices[0]];
        l1_page.set_phys_addr(physical_address);
        l1_page.insert(PageEntry::PRESENT | PageEntry::WRITABLE);
    }

    pub fn get_page(&mut self, indices: [usize; 4]) -> Option<&mut PageEntry> {
        // SAFETY: All page maps > level 1 always point to page maps
        let l4 = unsafe { self.0[indices[3]].page_table()?.as_mut() };
        let l3 = unsafe { l4.0[indices[2]].page_table()?.as_mut() };
        let l2 = unsafe { l3.0[indices[1]].page_table()?.as_mut() };
        Some(&mut l2.0[indices[0]])
    }
}
/// Calculates the correct indices in the page table for a given virtual memory address
/// For example if you want to map 0x1300-0x2300 to 0x4000-5000, you would need to put the address 0x1300 into the following indices: 3 - 0 - 0 - 0
/// So this function calculates those indices in the page table
pub fn get_page_index(virtual_address: usize) -> [usize; 4] {
    [
        (virtual_address >> 12) & 0x1ff,
        (virtual_address >> (12 + 9)) & 0x1ff,
        (virtual_address >> (12 + 9 + 9)) & 0x1ff,
        (virtual_address >> (12 + 9 + 9 + 9)) & 0x1ff,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_index_test() {
        assert_eq!(get_page_index(0x4000), [4, 0, 0, 0]);
        assert_eq!(get_page_index(0x1000 * 52 + 0x50000 * 7), [100, 1, 0, 0]);
    }
    #[test]
    fn map_memory_should_set_the_correct_page() {
        let mut page_map = PageTable::default();
        page_map.map_memory(0x1000, 0x5000);
        page_map.map_memory(0x4000, 0x4040);
        page_map.map_memory((0x1000 * 512) + 0x4000, 0x70000);
        assert_eq!(
            page_map.get_page([1, 0, 0, 0]).unwrap().phys_addr(),
            Some(0x5000)
        );
        assert_eq!(
            page_map.get_page([4, 0, 0, 0]).unwrap().phys_addr(),
            Some(0x4040)
        );
        assert_eq!(
            page_map.get_page([4, 1, 0, 0]).unwrap().phys_addr(),
            Some(0x70000)
        );
    }
    #[test]
    fn clone_page_table_recursively() {
        let mut page_map = PageTable::default();
        page_map.map_memory(0x1000, 0x5000);
        page_map.map_memory(0x4000, 0x4040);
        page_map.map_memory((0x1000 * 512) + 0x4000, 0x70000);
        let page_map = unsafe { page_map.clone_recursive(4).as_mut() };
        assert_eq!(
            page_map.get_page([1, 0, 0, 0]).unwrap().phys_addr(),
            Some(0x5000)
        );
        assert_eq!(
            page_map.get_page([4, 0, 0, 0]).unwrap().phys_addr(),
            Some(0x4040)
        );
        assert_eq!(
            page_map.get_page([4, 1, 0, 0]).unwrap().phys_addr(),
            Some(0x70000)
        );
    }
}
