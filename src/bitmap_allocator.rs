use core::num::NonZeroUsize;

use lazy_static::lazy_static;
use limine::memory_map::EntryType;
use spin::Mutex;

use crate::limine::MEMMAP_REQ;
pub const PAGE_SIZE: usize = 0x1000;
pub struct BitMap<'a> {
    bitmap: &'a mut [u8],
}
impl<'a> BitMap<'a> {
    /// Creates a new bitmap
    pub fn new(bitmap: &'a mut [u8]) -> Self {
        Self { bitmap }
    }
    /// Configures a bit from the bitmap to a the specified value, returns Some if the index is in bounds and None if not
    pub fn try_cfg(&mut self, index: usize, value: bool) -> bool {
        let div_index = index / 8;
        if self.bitmap.len() <= div_index {
            return false;
        }
        let offset = index % 8;

        self.bitmap[div_index] = if value {
            self.bitmap[div_index] | (0b10000000u8 >> offset)
        } else {
            self.bitmap[div_index] ^ (0b10000000u8 >> offset)
        };
        true
    }
    /// Set a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_set(&mut self, index: usize) -> bool {
        let div_index = index / 8;
        if self.bitmap.len() <= div_index {
            return false;
        }
        let offset = index % 8;
        self.bitmap[div_index] |= 0b10000000 >> offset;
        true
    }
    /// Clears a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_clear(&mut self, index: usize) -> bool {
        let div_index = index / 8;
        if self.bitmap.len() <= div_index {
            return false;
        }
        let offset = index % 8;
        self.bitmap[div_index] ^= 0b10000000 >> offset;
        true
    }
    pub fn cfg(&mut self, index: usize, value: bool) {
        assert!(
            self.try_cfg(index, value),
            "Tried to access bitmap out of bounds: index {index} vs max {}",
            self.bitmap.len() * 8
        )
    }
    pub fn set(&mut self, index: usize) {
        assert!(
            self.try_set(index),
            "Tried to access bitmap out of bounds: index {index} vs max {}",
            self.bitmap.len() * 8
        );
    }
    pub fn clear(&mut self, index: usize) {
        assert!(
            self.try_clear(index),
            "Tried to access bitmap out of bounds: index {index} vs max {}",
            self.bitmap.len() * 8
        );
    }
    /// Gets a bit from the bitmap
    /// # Panics
    /// This will panic if the index is out of bounds, use [`BitMap::try_get`] if you want to handle the error
    pub fn get(&self, index: usize) -> bool {
        self.try_get(index).unwrap_or_else(|| {
            panic!(
                "Tried to access bitmap out of bounds: index {index} vs max {}",
                self.bitmap.len() * 8
            )
        })
    }
    /// Gets a bit from the bitmap, returns Some if the index is in bounds and None if not
    pub fn try_get(&self, index: usize) -> Option<bool> {
        let div_index = index / 8;
        let offset = index % 8;
        let byte = self.bitmap.get(div_index)?;
        let masked_byte = byte & (0b10000000 >> offset);
        Some(masked_byte >= 1)
    }
}
pub struct BitmapAllocator<'a> {
    bitmap: BitMap<'a>,
    memory_region_start: usize,
    memory_region_size: usize,
    last_allocated_page_index: usize,
}
impl<'a> BitmapAllocator<'a> {
    pub fn number_of_pages(&self) -> usize {
        self.memory_region_size.div_floor(PAGE_SIZE)
    }
    pub fn from_mmap() -> BitmapAllocator<'static> {
        let memory_map = MEMMAP_REQ
            .get_response()
            .expect("memory map should be available");
        // dbg!(memory_map.entries().iter().filter(|e| e.entry_type == EntryType::USABLE).map(|e| dbg!(e.length)).sum::<u64>());
        let Some(&entry) = memory_map
            .entries()
            .iter()
            .filter(|e| e.entry_type == EntryType::USABLE)
            .max_by_key(|entry| entry.length)
        else {
            panic!("Couldn't find a usable memory region")
        };
        let mut allocator = BitmapAllocator {
            bitmap: unsafe {
                BitMap::new({
                    let bitmap_slice = core::slice::from_raw_parts_mut(
                        entry.base as usize as *mut u8,
                        (entry.length as usize).div_ceil(PAGE_SIZE * 8),
                    );
                    bitmap_slice.fill(0);
                    bitmap_slice
                })
            },
            memory_region_size: entry.length as usize,
            memory_region_start: entry.base as usize,
            last_allocated_page_index: 0,
        };
        println!(
            "memory_region_start: {}, memory_region_size: {}",
            allocator.memory_region_start, allocator.memory_region_size
        );
        allocator.lock_pages(
            allocator.memory_region_start,
            allocator.number_of_pages() / 8,
        );
        allocator
    }
    pub fn lock_pages(&mut self, addr: usize, size: usize) {
        let rel_addr = addr as usize - self.memory_region_start;
        let page = rel_addr.div_floor(PAGE_SIZE);
        let page_end = page + (size.div_ceil(PAGE_SIZE));
        for i in page..=page_end {
            self.bitmap.try_set(i);
        }
        self.last_allocated_page_index = page;
    }
    pub fn free_pages(&mut self, addr: usize, size: usize) {
        let rel_addr = addr - self.memory_region_start;
        let page = rel_addr.div_floor(PAGE_SIZE);
        let page_end = page + (size.div_ceil(PAGE_SIZE));
        for i in page..=page_end {
            self.bitmap.try_clear(i);
        }

        self.last_allocated_page_index = page;
    }
    pub fn request_page(&mut self) -> Option<NonZeroUsize> {
        for i in (self.last_allocated_page_index..self.number_of_pages())
            .chain(0..self.last_allocated_page_index)
        {
            if self.bitmap.get(i) {
                continue;
            }
            let addr = self.memory_region_start + (i * PAGE_SIZE);
            assert!(
                addr - self.memory_region_start <= self.memory_region_size,
                "addr: 0x{addr:X} | self.memory_region_start: 0x{:X} | self.memory_region_size: 0x{:X}",
                self.memory_region_start,
                self.memory_region_size
            );
            self.lock_pages(addr, PAGE_SIZE);
            self.last_allocated_page_index = i;
            return Some(NonZeroUsize::new(addr).unwrap());
        }
        None
    }
    pub fn request_and_clear_page(&mut self) -> Option<NonZeroUsize> {
        let page = self.request_page()?;
        // SAFETY: This will just clear the newly allocated page,
        unsafe {
            let slice = core::slice::from_raw_parts_mut(page.get() as *mut u8, PAGE_SIZE);
            for b in slice {
                *b = 0;
            }
        }
        Some(page)
    }
}
unsafe impl<'a> Send for BitmapAllocator<'a> {}

lazy_static! {
    pub static ref GLOBAL_PAGE_ALLOCATOR: Mutex<BitmapAllocator<'static>> =
        Mutex::new(BitmapAllocator::from_mmap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test(name = "Try setting a bit in a BitMap")]
    fn test_set_bit() {
        let mut bitmap_data = [0u8; 2];
        BitMap::new(&mut bitmap_data).set(2);
        assert_eq!(bitmap_data, [0b00100000, 0]);
        BitMap::new(&mut bitmap_data).set(13);
        assert_eq!(bitmap_data, [0b00100000, 0b00000100]);
    }

    #[test(name = "Try fetching a bit in a BitMap")]
    fn test_get_bit() {
        let mut bitmap_data = [0b00101000u8; 2];
        let bitmap = BitMap::new(&mut bitmap_data);
        assert_eq!(bitmap.try_get(4), Some(true));
        assert_eq!(bitmap.try_get(13), Some(false));
        assert_eq!(bitmap.try_get(2), Some(true));
    }

    #[test(name = "Try accessing a BitMap out of bounds")]
    fn test_out_of_bounds_set() {
        let mut bitmap_data = [0u8; 2];
        let mut bitmap = BitMap::new(&mut bitmap_data);
        // Index out of bounds, should return None
        assert_eq!(bitmap.try_set(20), false);
    }
}
