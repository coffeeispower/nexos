pub mod apic;
pub mod idt;
pub mod paging;
pub mod pic;
pub mod ports;
pub mod serial;
use core::hint::spin_loop;

use apic::LAPIC;
use limine::smp::Cpu;

use crate::{core::{current_core_id, local::CoreLocal}, global_allocator, limine::SMP};

pub fn init() {
    serial::init();
    global_allocator::init_heap();
    pic::disable();
    idt::IDT.load();
    x86_64::instructions::interrupts::enable();
    unsafe {
        LAPIC.write().enable();
    }
    let smp = SMP.get_response().unwrap();
    let cpus = smp.cpus();
    for cpu in cpus.iter().filter(|c| c.lapic_id != smp.bsp_lapic_id()) {
        cpu.goto_address.write(print_core_id);
    }

}

unsafe extern "C" fn print_core_id(_: &Cpu) -> ! {
    LAPIC.write().enable();
    println!("Starting up core {}", current_core_id());
    loop {
        spin_loop();
    }
}
