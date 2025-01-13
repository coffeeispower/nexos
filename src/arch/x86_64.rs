pub mod apic;
pub mod idt;
pub mod paging;
pub mod pic;
pub mod ports;
pub mod serial;
use crate::kernel::logger::Logger;
pub struct LoggerX86Impl(());
impl LoggerX86Impl {
    pub fn new() -> Self {
        serial::init();
        LoggerX86Impl(())
    }
}
impl Logger for LoggerX86Impl {
    fn print_str(&self, message: &str) {
        serial::print_str(message);
    }
}
