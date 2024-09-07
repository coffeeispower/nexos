use crate::bitmap_allocator::GLOBAL_PAGE_ALLOCATOR;

use super::idt::{APIC_ERROR_INTERRUPT_ID, APIC_SPURIOUS_INTERRUPT_ID, APIC_TIMER_INTERRUPT_ID};
use lazy_static::lazy_static;
use spin::RwLock;

use core::ops::DerefMut;

use super::paging::active_page_table_mapper;
use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder, TimerDivide};
use x86_64::{
    structures::paging::{Mapper, Page, PageTableFlags, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};
// TODO: turn this into a thread_local as it is unsafe to share LAPIC between cores
lazy_static! {
    pub static ref LAPIC: RwLock<LocalApic> = {
        let apic_physical_address: u64 = unsafe { xapic_base() };
        let apic_virtual_address: u64 = 1024 * 1024 * 1024 * 32;
        unsafe {
            active_page_table_mapper()
                .map_to(
                    dbg!(Page::containing_address(VirtAddr::new(apic_virtual_address))),
                    dbg!(PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(apic_physical_address))),
                    PageTableFlags::WRITABLE
                            | PageTableFlags::PRESENT
                            | PageTableFlags::USER_ACCESSIBLE,
                    GLOBAL_PAGE_ALLOCATOR.lock().deref_mut(),
                )
                .unwrap()
                .flush();
        };
        RwLock::new(LocalApicBuilder::new()
            .timer_vector(APIC_TIMER_INTERRUPT_ID as usize)
            .error_vector(APIC_ERROR_INTERRUPT_ID as usize)
            .spurious_vector(APIC_SPURIOUS_INTERRUPT_ID as usize)
            .timer_mode(x2apic::lapic::TimerMode::Periodic)
            .timer_divide(TimerDivide::Div2)
            .timer_initial(10000)
            .set_xapic_base(apic_virtual_address)
            .build()
            .unwrap())
    };
}
