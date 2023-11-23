use core::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use lazy_static::lazy_static;
use limine::MemmapRequest;
static PAGE_SIZE: usize = 0x1000;
static MEMMAP_REQ: MemmapRequest = MemmapRequest::new(0);
pub struct BitMap {
    bitmap: *mut u8,
    bitmap_size: usize,
}
impl BitMap {
    /// Creates a new bitmap
    /// # Safety
    /// The caller must ensure the bitmap is placed in a safe place in memory
    pub unsafe fn new(bitmap_pointer: *mut u8, size: usize, memset_0: bool) -> Self {
        if memset_0 {
            let slice = core::slice::from_raw_parts_mut(bitmap_pointer, size);
            for b in slice {
                *b = 0;
            }
        }
        Self {
            bitmap: bitmap_pointer,
            bitmap_size: size,
        }
    }
    /// Configures a bit from the bitmap to a the specified value, returns Some if the index is in bounds and None if not
    pub fn try_cfg(&self, index: usize, value: bool) -> Option<()> {
        if index > (self.bitmap_size * 8) - 1 {
            None
        } else {
            let div_index = index / 8;
            let offset = index % 8;
            let byte_addr = unsafe {
                self.bitmap.offset(
                    div_index
                        .try_into()
                        .expect("div_index doesn't fit into a isize"),
                )
            };
            let byte = unsafe { *byte_addr };
            unsafe {
                *byte_addr = if value {
                    byte | (0b10000000u8 >> offset)
                } else {
                    byte ^ (0b10000000u8 >> offset)
                };
            }
            Some(())
        }
    }
    /// Set a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_set(&self, index: usize) -> Option<()> {
        if index > (self.bitmap_size * 8) - 1 {
            None
        } else {
            let div_index = index / 8;
            let offset = index % 8;
            unsafe {
                *self.bitmap.byte_add(div_index) |= 0b10000000 >> offset;
            }
            Some(())
        }
    }
    /// Clears a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_clear(&self, index: usize) -> Option<()> {
        if index > (self.bitmap_size * 8) - 1 {
            None
        } else {
            let div_index = index / 8;
            let offset = index % 8;
            unsafe {
                *self.bitmap.byte_add(div_index) ^= 0b10000000 >> offset;
            }
            Some(())
        }
    }
    pub fn cfg(&self, index: usize, value: bool) {
        self.try_cfg(index, value)
            .expect("Tried to access bitmap out of bounds.")
    }
    pub fn set(&self, index: usize) {
        self.try_set(index)
            .expect("Tried to access bitmap out of bounds.")
    }
    pub fn clear(&self, index: usize) {
        self.try_clear(index)
            .expect("Tried to access bitmap out of bounds.")
    }
    /// Gets a bit from the bitmap
    /// # Panics
    /// This will panic if the index is out of bounds, use [`BitMap::try_get`] if you want to handle the error
    pub fn get(&self, index: usize) -> bool {
        self.try_get(index)
            .expect("Tried to access bitmap out of bounds.")
    }
    /// Gets a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_get(&self, index: usize) -> Option<bool> {
        if index > (self.bitmap_size * 8) - 1 {
            None
        } else {
            let div_index = index / 8;
            let offset = index % 8;
            let byte = unsafe { *self.bitmap.byte_add(div_index) };
            let masked_byte = byte & (0b10000000 >> offset);
            Some(masked_byte >= 1)
        }
    }
}
#[derive(Debug)]
pub enum RequestPageError {
    OutOfMemory,
}
pub struct BitmapAllocator {
    pub bitmap: BitMap,
    pub memory_region_start: *mut (),
    pub memory_region_size: usize,
    pub lock: AtomicBool,
}
impl BitmapAllocator {
    pub fn number_of_pages(&self) -> usize {
        (self.bitmap.bitmap_size) * 8
    }
    pub fn from_mmap() -> Self {
        println!("Creating new bitmap allocator from memory map");
        let memory_map = MEMMAP_REQ.get_response();
        let memory_map = memory_map.get().expect("memory map should be available");
        let entries = memory_map.memmap();
        let mut largest_mem_start: Option<*mut ()> = None;
        let mut largest_mem_size: Option<usize> = None;
        for entry in entries {
            if let limine::MemoryMapEntryType::Usable = entry.typ {
                if largest_mem_size.is_none() {
                    largest_mem_size = Some(entry.len as usize);
                    largest_mem_start = Some((entry.base as usize) as *mut ());
                } else if let Some(size) = largest_mem_size {
                    if size < entry.len as usize {
                        largest_mem_size = Some(entry.len as usize);
                        largest_mem_start = Some((entry.base as usize) as *mut ());
                    }
                }
            }
        }
        let (Some(largest_mem_start), Some(largest_mem_size)) =
            (largest_mem_start, largest_mem_size)
        else {
            panic!("Couldn't find a usable memory region")
        };
        let allocator = BitmapAllocator {
            bitmap: unsafe {
                BitMap::new(
                    largest_mem_start.cast(),
                    largest_mem_size.div_ceil(PAGE_SIZE * 8),
                    true,
                )
            },
            memory_region_size: largest_mem_size,
            memory_region_start: largest_mem_start,
            lock: AtomicBool::new(false),
        };
        println!("Locking bitmap pages");
        allocator.lock_pages(
            allocator.memory_region_start,
            allocator.number_of_pages() / 8,
        );
        println!("Bitmap allocator created successfully");
        allocator
    }
    fn lock_bitmap(&self) {
        while self.lock.swap(true, Ordering::SeqCst) {
            core::hint::spin_loop();
            print!("Waiting for bitmap to unlock\r");
        }
        println!("Locked bitmap");
    }
    fn unlock_bitmap(&self) {
        println!("Unlock bitmap");
        self.lock.store(false, Ordering::SeqCst);
    }
    pub fn lock_pages<T>(&self, addr: *mut T, size: usize) {
        self.lock_bitmap();
        let rel_addr = addr as usize - self.bitmap.bitmap as usize;
        let page = rel_addr.div_floor(PAGE_SIZE);
        let page_end = page + (size / PAGE_SIZE);
        for i in page..=page_end {
            if self.bitmap.get(i) {
                panic!("Double lock");
            }
            self.bitmap.set(i);
        }
        self.unlock_bitmap()
    }
    pub fn free_pages<T>(&self, addr: *mut T, size: usize) {
        self.lock_bitmap();
        let rel_addr = addr as usize - self.bitmap.bitmap as usize;
        let page = rel_addr.div_floor(PAGE_SIZE);
        let page_end = page + (size / PAGE_SIZE);
        for i in page..=page_end {
            if self.bitmap.get(i) {
                panic!("Double lock");
            }
            self.bitmap.clear(i);
        }
        self.unlock_bitmap()
    }
    pub fn request_page<T>(&self) -> Result<NonNull<T>, RequestPageError> {
        println!("Requested page");
        for i in 0..self.number_of_pages() {
            println!("Checking page {i}");
            self.lock_bitmap();
            if self.bitmap.get(i) {
                self.unlock_bitmap();
                continue;
            }

            println!("Page {i} is free!");
            let addr = unsafe { self.memory_region_start.byte_add(i * PAGE_SIZE) };
            self.unlock_bitmap();
            println!("Locking page {i}");
            self.lock_pages(addr, PAGE_SIZE);

            println!("Allocated address 0x{:X}", addr as usize);
            return Ok(
                NonNull::new(addr.cast()).expect("the requested page should not be a null ptr")
            );
        }
        Err(RequestPageError::OutOfMemory)
    }
    pub fn request_and_clear_page<T>(&self) -> Result<NonNull<T>, RequestPageError> {
        let page = self.request_page::<T>()?;
        // SAFETY: This will just clear the newly allocated page,
        // so we're not messing with other people's memory
        unsafe {
            let slice = core::slice::from_raw_parts_mut(page.as_ptr().cast::<u8>(), PAGE_SIZE);
            for b in slice {
                *b = 0;
            }
        }
        Ok(page)
    }
}
unsafe impl Send for BitmapAllocator {}
unsafe impl Sync for BitmapAllocator {}
lazy_static! {
    pub static ref GLOBAL_PAGE_ALLOCATOR: BitmapAllocator = BitmapAllocator::from_mmap();
}
#[cfg(test)]
mod tests {
    use super::*;
    // Basic tests for BitMap
    #[test]
    fn test_set_bit() {
        let mut bitmap_data = [0u8; 2];
        let bitmap = unsafe { BitMap::new(bitmap_data.as_mut_ptr(), 2, false) };

        bitmap.set(2);
        assert_eq!(bitmap_data, [0b00100000, 0]);
        bitmap.set(13);
        assert_eq!(bitmap_data, [0b00100000, 0b00000100]);
    }

    #[test]
    fn test_get_bit() {
        let mut bitmap_data = [0b00101000u8; 2];
        let bitmap = unsafe { BitMap::new(bitmap_data.as_mut_ptr(), 2, false) };
        assert_eq!(bitmap.try_get(4), Some(true));
        assert_eq!(bitmap.try_get(13), Some(false));
        assert_eq!(bitmap.try_get(2), Some(true));
    }

    #[test]
    fn test_out_of_bounds_set() {
        let mut bitmap_data = [0u8; 2];
        let bitmap = unsafe { BitMap::new(bitmap_data.as_mut_ptr(), 2, false) };
        // Index out of bounds, should return None
        assert_eq!(bitmap.try_set(20), None);
    }
    #[test]
    fn memset_0() {
        let mut bitmap_data = [0b00101000u8; 2];
        let _ = unsafe { BitMap::new(bitmap_data.as_mut_ptr(), 2, true) };
        assert_eq!(bitmap_data, [0, 0]);
    }
}
