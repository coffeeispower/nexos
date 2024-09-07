use crate::{heap::KernelHeap, limine::HHDM};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ops::DerefMut;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::OffsetPageTable;
struct KernelHeapAllocator;

unsafe impl GlobalAlloc for KernelHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = GLOBAL_KERNEL_HEAP
            .lock()
            .allocate(layout, MAPPER.lock().deref_mut());
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
    #[cfg(target_arch = "x86_64")]
    static ref MAPPER: Mutex<OffsetPageTable<'static>> = unsafe {
        use crate::arch::x86_64::paging::active_level_4_table;
        use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

        let offset = VirtAddr::new(HHDM.get_response().unwrap().offset());
        let active_table = active_level_4_table(offset);
        OffsetPageTable::new(active_table, offset).into()
    };
    static ref GLOBAL_KERNEL_HEAP: Mutex<KernelHeap> = unsafe {
        Mutex::new(KernelHeap::init(KERNEL_HEAP_START_ADDRESS, KERNEL_HEAP_MAX_SIZE, KERNEL_HEAP_INITIAL_SIZE, MAPPER.lock().deref_mut())
                .expect("Failed to initialize heap"))
    };
}
pub fn init_heap() {
    lazy_static::initialize(&GLOBAL_KERNEL_HEAP);
}
#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    #[test]
    fn alloc_box() {
        let boxed_int = Box::new(42);
        assert_eq!(*boxed_int, 42);
    }
    #[test]
    fn string() {
        let test_string = String::from("Hello, kernel allocator!");
        assert_eq!(test_string, "Hello, kernel allocator!");
    }

    #[test]
    fn push_one_thousand_elements_into_vector() {
        // Verifica alocação de um vetor
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
    #[test]
    fn box_with_big_struct() {
        // Verifica alocação de uma struct grande

        #[repr(align(16))]
        struct BigStruct {
            data: [u8; 1024],
        }

        let big_struct = Box::new(BigStruct { data: [0u8; 1024] });
        assert_eq!(big_struct.data[512], 0);
    }
    #[test]
    fn allocate_a_lot_of_small_objects() {
        // Verifica alocação de múltiplos objetos pequenos

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
    #[test]
    fn allocate_and_deallocate_vectors() {
        // Verifica comportamento após múltiplas alocações e desalocações

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
    #[test]
    fn concat_strings() {
        // Testa concatenar strings

        let mut string1 = String::from("Kernel");
        let string2 = String::from("Allocator");
        string1.push_str(&string2);
        assert_eq!(string1, "KernelAllocator");
    }
    #[test]
    fn iterators() {
        // Testa operações com iteradores

        let vec = vec![1, 2, 3, 4, 5];
        let sum: i32 = vec.iter().sum();
        assert_eq!(sum, 15);
    }
}
