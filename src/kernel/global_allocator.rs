use crate::kernel::KERNEL_MEMORY_MAP;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ops::DerefMut;
use lazy_static::lazy_static;
use spin::Mutex;

use super::heap::KernelHeap;
struct KernelHeapAllocator;

unsafe impl GlobalAlloc for KernelHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = GLOBAL_KERNEL_HEAP
            .lock()
            .allocate(layout, KERNEL_MEMORY_MAP.lock().deref_mut());
        assert!(p.is_aligned());
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        GLOBAL_KERNEL_HEAP.lock().deallocate(ptr)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: KernelHeapAllocator = KernelHeapAllocator;
const KERNEL_HEAP_START_ADDRESS: usize = 1024 * 1024 * 1024 * 10;
const KERNEL_HEAP_INITIAL_SIZE: usize = 1024 * 1024;
const KERNEL_HEAP_MAX_SIZE: usize = 1024 * 1024 * 1024 * 4;
lazy_static! {
    static ref GLOBAL_KERNEL_HEAP: Mutex<KernelHeap> = unsafe {
        Mutex::new(KernelHeap::init(KERNEL_HEAP_START_ADDRESS, KERNEL_HEAP_MAX_SIZE, KERNEL_HEAP_INITIAL_SIZE, KERNEL_MEMORY_MAP.lock().deref_mut())
                .expect("Failed to initialize heap"))
    };
}
#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    #[test(name = "Allocate 1 box containing an integer")]
    fn alloc_box() {
        let boxed_int = Box::new(42);
        assert_eq!(*boxed_int, 42);
    }
    #[test(name = "Try allocating a string")]
    fn string() {
        let test_string = String::from("Hello, kernel allocator!");
        assert_eq!(test_string, "Hello, kernel allocator!");
    }

    #[test(name = "Push 1000 integers into a vector")]
    fn push_one_thousand_elements_into_vector() {
        const SIZE: usize = 1_000;
        let mut test_vec: Vec<u64> = Vec::new();
        for i in 0..SIZE {
            test_vec.push(i as _);
        }
        assert_eq!(test_vec.len(), SIZE);
        for i in 0..SIZE {
            assert_eq!(test_vec[i], i as u64);
        }
    }
    #[test(name = "Allocate a big struct inside a Box")]
    fn box_with_big_struct() {
        // Try to allocate a big structure

        #[repr(align(16))]
        struct BigStruct {
            data: [u8; 1024],
        }

        let big_struct = Box::new(BigStruct { data: [0u8; 1024] });
        assert_eq!(big_struct.data[512], 0);
    }
    #[test(name = "Allocate a lot of small objects to test fragmentation")]
    fn allocate_a_lot_of_small_objects() {

        let mut small_boxes: Vec<Box<u8>> = Vec::new();
        for i in 0..1000 {
            println!("Allocating box with {i}");
            let b = Box::new(i as u8);
            println!("Pushing box");
            small_boxes.push(b);
        }
        for (i, small_box) in small_boxes.iter().enumerate() {
            println!("Checking box on index {i}");
            assert_eq!(**small_box, i as u8);
        }
    }
    #[test(name = "Allocate and deallocate vectores repeatedly")]
    fn allocate_and_deallocate_vectors() {
        // Allocate and deallocate multiple vectores

        let mut vecs: Vec<Vec<u8>> = Vec::new();
        for _ in 0..100 {
            let mut v = Vec::new();
            v.resize(512, 42);
            vecs.push(v);
        }
        for v in vecs.iter() {
            assert_eq!(v.len(), 512);
            assert_eq!(v[0], 42);
        }
    }
    #[test(name = "Try concatenating strings")]
    fn concat_strings() {
        // Testa concatenar strings

        let mut string1 = String::from("Kernel");
        let string2 = String::from("Allocator");
        string1.push_str(&string2);
        assert_eq!(string1, "KernelAllocator");
    }
    #[test(name = "Iterating though vectors")]
    fn iterators() {
        // Testa operações com iteradores

        let vec = vec![1, 2, 3, 4, 5];
        let sum: i32 = vec.iter().sum();
        assert_eq!(sum, 15);
    }
}
