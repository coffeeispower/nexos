pub mod apic;
pub mod idt;
pub mod paging;
pub mod pic;
pub mod ports;
pub mod serial;
use apic::LAPIC;

use crate::global_allocator;

pub fn init() {
    serial::init();
    global_allocator::init_heap();
    pic::disable();
    idt::IDT.load();
    x86_64::instructions::interrupts::enable();

    unsafe {
        LAPIC.write().enable();
    }
}
