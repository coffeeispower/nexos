use lazy_static::lazy_static;
use limine::{LimineMemmapRequest, NonNullPtr};
use spin::{Mutex, MutexGuard};
static PAGE_SIZE: usize = 0x1000;
static MEMMAP_REQ: LimineMemmapRequest = LimineMemmapRequest::new(0);
pub struct BitMap {
    bitmap: *mut u8,
    bitmap_size: usize
}
impl BitMap {
    /// Creates a new bitmap
    /// # Safety
    /// The bitmap address and size are arbitrary, so safety cannot be guaranteed.
    pub unsafe fn new(bitmap_pointer: *mut u8, size: usize) -> Self{
        Self {
            bitmap: bitmap_pointer,
            bitmap_size: size
        }
    }
    /// Configures a bit from the bitmap to a the specified value, returns Some if the index is in bounds and None if not
    pub fn try_cfg(&self, index: usize, value: bool) -> Option<()> {
        if index > (self.bitmap_size*8) - 1 {
            None
        } else {
            let div_index = index/8;
            let offset = index % 8;
            let byte = unsafe {
                 *self.bitmap.add(div_index)
            };
            unsafe {
                *self.bitmap.add(div_index) = if value {
                    byte | (128 >> offset)
                } else {
                    byte ^ (128 >> offset)
                };
            }
            Some(())
        }
    }
    /// Set a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_set(&self, index: usize) -> Option<()> {
        if index > (self.bitmap_size*8) - 1 {
            None
        } else {
            let div_index = index/8;
            let offset = index % 8;
            unsafe {
                *self.bitmap.add(div_index) |= 128 >> offset;
            }
            Some(())
        }
    }
    /// Clears a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_clear(&self, index: usize) -> Option<()>{
        if index > (self.bitmap_size*8) - 1 {
            None
        } else {
            let div_index = index/8;
            let offset = index % 8;
            unsafe {
                *self.bitmap.add(div_index) ^= 128 >> offset;
            }
            Some(())
        }
    }
    pub fn cfg(&self, index: usize, value: bool) {
        self.try_cfg(index, value).expect("Tried to access bitmap out of bounds.")

    }
    pub fn set(&self, index: usize) {
        self.try_set(index).expect("Tried to access bitmap out of bounds.")

    }
    pub fn clear(&self, index: usize) {
        self.try_clear(index).expect("Tried to access bitmap out of bounds.")

    }
    /// Gets a bit from the bitmap
    /// # Panics
    /// This will panic if the index is out of bounds, use [`BitMap::try_get`] if you want to handle the error
    pub fn get(&self, index: usize) -> bool{
        self.try_get(index).expect("Tried to access bitmap out of bounds.")
    }
    /// Gets a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_get(&self, index: usize) -> Option<bool> {
        if index > (self.bitmap_size*8) - 1 {
            None
        } else {
            let div_index = index/8;
            let offset = index % 8;
            let byte = unsafe {
                 *self.bitmap.add(div_index)
            };
            let masked_byte = byte & (128 >> offset);
            Some(masked_byte > 0)
        }
    }
}
#[derive(Debug)]
pub enum RequestPageError {
    OutOfMemory,
}
pub struct BitmapAllocator {
    pub bitmap: BitMap,
    pub memory_region_start: *const u8,
    pub memory_region_size: usize
}
impl BitmapAllocator {
    pub fn number_of_pages(&self) -> usize{
        (self.bitmap.bitmap_size) * 8
    }
    pub fn from_mmap() -> Self {
        let memory_map = MEMMAP_REQ.get_response();
        let Some(memory_map) = memory_map.get() else {
            panic!("Couldn't get memory map.");
        };
        let entries = memory_map.memmap();
        let mut largest_mem_start: Option<*mut u8> = None;
        let mut largest_mem_size: Option<u64> = None;
        for entry in entries {
            if let limine::LimineMemoryMapEntryType::Usable = entry.typ {
                if largest_mem_size.is_none() {
                    largest_mem_size = Some(entry.len);
                    largest_mem_start = Some((entry.base as usize) as *mut u8);
                } else if let Some(size) = largest_mem_size {
                    if size < entry.len {
                        largest_mem_size = Some(entry.len);
                        largest_mem_start = Some((entry.base as usize) as *mut u8);
                    }
                }
            }
        }
        let Some(largest_mem_start) = largest_mem_start else {
            panic!("Couldn't find usable memory.")
        };
        let Some(largest_mem_size) = largest_mem_size else {
            panic!("Couldn't find usable memory.")
        };
        let allocator = BitmapAllocator {
            bitmap: BitMap { bitmap: largest_mem_start, bitmap_size: ((((largest_mem_size as usize)/PAGE_SIZE)+1)/8) },
            memory_region_size: largest_mem_size as usize,
            memory_region_start: largest_mem_start
        };
        allocator.initialize_bitmap();
        allocator
    }

    fn initialize_bitmap(&self) {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(self.bitmap.bitmap, self.bitmap.bitmap_size)
        };
        for b in slice {
            *b = 0;
        }
        self.lock_pages(self.memory_region_start, self.number_of_pages()/8);
    }
    pub fn lock_pages<T>(&self, addr: *const T, size: usize) {
        let rel_addr = (addr as usize - self.bitmap.bitmap as usize) as *const T;
        let page = (rel_addr as usize) / PAGE_SIZE;

        let page_end = page+(size/8);
        for i in page..=page_end {
            if self.bitmap.get(i) {
                panic!("Double lock");
            }
            self.bitmap.set(i);
        }
    }
    pub fn free_pages<T>(&self, addr: *const T, size: usize) {
        let rel_addr = (self.bitmap.bitmap as usize - addr as usize) as *const T;
        let page = (rel_addr as usize) / PAGE_SIZE;

        let page_end = page+(size/8);
        for i in page..page_end {
            if !self.bitmap.get(i) {
                panic!("Double free");
            }
            self.bitmap.clear(i);
        }
    }
    pub fn request_page<T>(&self) -> Result<*mut T, RequestPageError> {
        for i in 0..self.number_of_pages() {
            if self.bitmap.get(i) {
                continue;
            }
            let addr = ((i * PAGE_SIZE)+self.memory_region_start as usize) as *mut T;
            self.lock_pages(addr, PAGE_SIZE);
            return Ok(addr)
        }
        Err(RequestPageError::OutOfMemory)
    }
}
