use core::{
    alloc::Layout,
    ptr::{null_mut, NonNull},
};

use crate::bitmap_allocator::{GLOBAL_PAGE_ALLOCATOR, PAGE_SIZE};
use super::memory_map::{MemoryFlags, MemoryMap};

struct Node {
    length: usize,
    last: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    in_use: bool,
}
impl Node {
    unsafe fn split(&mut self, first_half_length: usize) {
        let first_half_length = first_half_length.next_power_of_two().max(2usize.pow(4));
        assert!(!self.is_in_use(), "Tried to split node in use");
        assert!(
            first_half_length <= self.length - size_of::<Self>(),
            "Split point is beyond the current node"
        );
        let next_node = Node {
            length: self.length - first_half_length - size_of::<Self>(),
            in_use: false,
            last: NonNull::new(self),
            next: self.next,
        };
        let next_node_ptr = self.data_pointer::<Node>().byte_add(first_half_length);
        *next_node_ptr = next_node;
        if let Some(next) = &mut self.next {
            next.as_mut().last = NonNull::new(next_node_ptr);
        }
        self.next = NonNull::new(next_node_ptr);
        self.length = first_half_length;
    }
    unsafe fn combine_forward(&mut self) {
        assert!(!self.is_in_use(), "Tried to combine node in use");
        let next = self.next.expect("next node must not be null").as_mut();
        self.length += next.length + size_of::<Self>();
        self.next = next.next;
        let Some(new_next) = self.next.map(|mut n| n.as_mut()) else {
            return;
        };
        new_next.last = NonNull::new(self);
    }
    fn set_in_use(&mut self, in_use: bool) {
        self.in_use = in_use;
    }
    fn is_in_use(&self) -> bool {
        self.in_use
    }
    fn data_address(&mut self) -> usize {
        ((self as *mut Self) as usize) + size_of::<Self>()
    }
    fn data_pointer<T>(&mut self) -> *mut T {
        self.data_address() as *mut T
    }
    unsafe fn from_data_pointer(data_pointer: *mut u8) -> *mut Node {
        data_pointer.cast::<Node>().offset(-1)
    }
}
pub struct KernelHeap {
    start: usize,
    current_size: usize,
    max_size: usize,
    last_node: NonNull<Node>,
}
impl KernelHeap {
    pub unsafe fn init(
        start: usize,
        max_size: usize,
        initial_size: usize,
        mapper: &mut dyn MemoryMap,
    ) -> Option<Self> {
        assert!(
            initial_size <= max_size,
            "Initial heap size must not be bigger than the heap maximum size"
        );
        assert!(
            initial_size > size_of::<Node>(),
            "Initial heap size must be able to fit at least 1 node inside"
        );

        let initial_size_pages = initial_size.div_ceil(PAGE_SIZE);
        // The heap is required to have a contiguous memory region
        // The page allocator however does not guarantee that the allocated pages are one after the other in memory
        // This allocates the necessary number of pages and maps them to a contiguous virtual memory region
        for current_page in 0..initial_size_pages {
            let page = GLOBAL_PAGE_ALLOCATOR.lock().request_page()?.into();
            mapper.map_memory(start + (current_page * PAGE_SIZE), page, MemoryFlags::default());
        }
        (*(start as *mut Node)) = Node {
            in_use: false,
            last: None,
            next: None,
            length: initial_size - size_of::<Node>(),
        };
        Some(Self {
            start,
            current_size: initial_size_pages,
            max_size: max_size.div_floor(PAGE_SIZE),
            last_node: NonNull::new(start as *mut Node)?,
        })
    }

    pub fn expand_heap(&mut self, amount: usize, mapper: &mut dyn MemoryMap) -> bool {
        let amount = amount.next_power_of_two().max(2usize.pow(4));
        let new_size = self.current_size + amount.div_ceil(PAGE_SIZE);
        if new_size >= self.max_size {
            return false;
        }
        for current_page in self.current_size..new_size {
            let Some(physical_page_address) = GLOBAL_PAGE_ALLOCATOR.lock().request_page() else {
                return false;
            };
            let virtual_page_address = self.start + (current_page * PAGE_SIZE);
            unsafe { mapper.map_memory(virtual_page_address, physical_page_address.into(), MemoryFlags::default()) };
        }
        self.current_size = new_size;
        unsafe {
            let last_node = self.last_node.as_mut();
            if last_node.is_in_use() {
                let new_node_ptr = last_node.data_pointer::<Node>().byte_add(last_node.length);
                *new_node_ptr = Node {
                    length: amount - size_of::<Node>(),
                    in_use: false,
                    last: Some(NonNull::from(&mut *last_node)),
                    next: None,
                };
                last_node.next = NonNull::new(new_node_ptr);
            } else {
                last_node.length += amount;
            }
        }
        true
    }
    fn root_node(&mut self) -> *mut Node {
        self.start as *mut Node
    }
    pub fn allocate(&mut self, layout: Layout, mapper: &mut dyn MemoryMap) -> *mut u8 {
        let layout = layout.align_to(16).unwrap().pad_to_align();
        let layout = Layout::from_size_align(
            layout.size().next_power_of_two().max(2usize.pow(4)),
            layout.align(),
        )
        .unwrap();
        let Some(mut current_node) =
            NonNull::new(self.root_node()).map(|mut r| unsafe { r.as_mut() })
        else {
            return null_mut();
        };
        loop {
            if !current_node.is_in_use() {
                if current_node
                    .next
                    .is_some_and(|next| !unsafe { next.as_ref() }.is_in_use())
                {
                    unsafe { current_node.combine_forward() };
                    continue;
                }
                if current_node.length == layout.size() {
                    current_node.set_in_use(true);
                    break current_node.data_pointer();
                }
                if current_node.length > layout.size() + size_of::<Node>() {
                    if current_node.next.is_none() {
                        unsafe { current_node.split(layout.size()) };
                        self.last_node = current_node.next.unwrap();
                    } else {
                        unsafe { current_node.split(layout.size()) };
                    }
                    current_node.set_in_use(true);
                    break current_node.data_pointer();
                }
            } else {
            }
            if let Some(mut next) = current_node.next {
                current_node = unsafe { next.as_mut() };
            } else {
                if !self.expand_heap(layout.size(), mapper) {
                    break null_mut();
                }
                current_node = unsafe { self.root_node().as_mut().unwrap() };
            }
        }
    }
    pub unsafe fn deallocate(&mut self, address: *mut u8) {
        let mut node = Node::from_data_pointer(address);
        (*node).set_in_use(false);
        if let Some(mut last) = (*node).last.filter(|last| !last.as_ref().is_in_use()) {
            last.as_mut().combine_forward();
            node = last.as_mut();
        }
        if (*node).next.is_some_and(|n| !n.as_ref().is_in_use()) {
            (*node).combine_forward();
        }
        if (*node).next.is_none() {
            self.last_node = NonNull::new(node).unwrap();
        }
    }
}
impl !Sync for KernelHeap {}
unsafe impl Send for KernelHeap {}

#[cfg(test)]
mod tests {
    use core::ops::DerefMut;

    use super::super::KERNEL_MEMORY_MAP;
    use super::*;

    #[test(name = "Allocate 100 times using the heap and deallocating everything afterwards")]
    fn test_heap_allocation_and_deallocation() {
        let start_address = 1024 * 1024 * 1024 * 1024; // Um endereço de memória fictício para testes
        let max_size = 1024 * 1024; // 1 MB
        let initial_size = 1024 * 128; // 128 KB
        let mut mapper = KERNEL_MEMORY_MAP.lock();
        // Inicializa a heap
        let mut heap = unsafe {
            KernelHeap::init(start_address, max_size, initial_size, mapper.deref_mut())
                .expect("Failed to initialize heap")
        };
        let layout = Layout::from_size_align(256, 8).expect("Invalid layout");
        let mut allocations = [null_mut::<u8>(); 100];
        for (i, ptr) in allocations.iter_mut().enumerate() {
            // Testa alocação de memória
            *ptr = heap.allocate(layout, mapper.deref_mut());
            assert!(
                !ptr.is_null(),
                "Allocation failed after allocating {i} times"
            );
        }
        for ptr in allocations {
            // Testa liberação de memória
            unsafe { heap.deallocate(ptr) };
        }
        // Teste de expansão da heap
        let expansion_size = 1024 * 256; // 256 KB
        let expanded = heap.expand_heap(expansion_size, mapper.deref_mut());
        assert!(expanded, "Heap expansion failed");
    }
}
