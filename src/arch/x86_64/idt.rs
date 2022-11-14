use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static!{
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault);
        idt.double_fault.set_handler_fn(double_fault);
        idt.divide_error.set_handler_fn(divide_by_0_error);
        idt.general_protection_fault.set_handler_fn(gpr);
        idt
    };
}
extern "x86-interrupt" fn divide_by_0_error(frame: InterruptStackFrame) {
    panic!("Divide By 0 Error: {frame:#?}");
}
extern "x86-interrupt" fn gpr(frame: InterruptStackFrame, code: u64) {
    panic!("GPR: {frame:#?}\nError Code: {code}");
}
extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("Page Fault: {frame:#?}\nError Code: {error_code:#?}")
}

extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Double Fault: {frame:#?}\nError Code: {error_code:#?}")
}