mod global_allocator;
mod heap;
pub mod logger;
pub mod memory_map;

use alloc::boxed::Box;
use lazy_static::lazy_static;
use logger::Logger;
use spin::Mutex;

lazy_static! {
    pub static ref GLOBAL_LOGGER: Mutex<Box<dyn Logger>> =
        Mutex::new(Box::new(<dyn Logger>::new()));
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        lazy_static! {
            pub static ref KERNEL_MEMORY_MAP: Mutex<crate::arch::x86_64::paging::X86MemoryMap<x86_64::structures::paging::OffsetPageTable<'static>>> = Mutex::new(unsafe { crate::arch::x86_64::paging::X86MemoryMap::current_memory_map() });
        }
    } else {
        compile_error!("Kernel Memory Map for the current architecture is not implemented yet");
    }
}
