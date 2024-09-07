use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::arch::x86_64::apic::LAPIC;

pub const APIC_TIMER_INTERRUPT_ID: u8 = 200;
pub const APIC_ERROR_INTERRUPT_ID: u8 = 201;
pub const APIC_SPURIOUS_INTERRUPT_ID: u8 = 202;
lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.general_protection_fault
            .set_handler_fn(gpf_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_interrupt_handler);
        idt[APIC_TIMER_INTERRUPT_ID].set_handler_fn(on_timer_pulse);
        idt[APIC_ERROR_INTERRUPT_ID].set_handler_fn(on_apic_error);
        idt[APIC_SPURIOUS_INTERRUPT_ID].set_handler_fn(on_apic_spurious_interrupt);
        idt
    };
}

extern "x86-interrupt" fn on_apic_spurious_interrupt(stack_frame: InterruptStackFrame) {
    println!("Apic Spurious Interrupt {stack_frame:#?}");
    unsafe { LAPIC.write().end_of_interrupt() }
}
extern "x86-interrupt" fn on_apic_error(stack_frame: InterruptStackFrame) {
    println!("Apic error {stack_frame:#?}");
    unsafe { LAPIC.write().end_of_interrupt() }
}
extern "x86-interrupt" fn on_timer_pulse(_stack_frame: InterruptStackFrame) {
    unsafe { LAPIC.write().end_of_interrupt() }
}

extern "x86-interrupt" fn page_fault_interrupt_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!(
        "Page Fault:
    Error Code: {error_code:#?}
    Stack Frame: {stack_frame:#?}"
    );
}

extern "x86-interrupt" fn gpf_interrupt_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "General Protection Fault:
    Error Code: {error_code:#?}
    Stack Frame: {stack_frame:#?}"
    );
}
