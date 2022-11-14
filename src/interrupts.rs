use crate::arch;

pub fn load_interrupts() {
    #[cfg(target_arch = "x86_64")]
    {
        arch::x86_64::idt::IDT.load();
    }
    #[cfg(target_arch = "aarch64")]
    {
        arch::aarch64::interrupts::load_interrupts();
    }
}