use crate::arch::x86_64::apic::LAPIC;

pub mod local;
/// Gets the ID of whatever core calls this function
pub fn current_core_id() -> usize {
    #[cfg(target_arch = "x86_64")]
    unsafe { LAPIC.read().id() }.try_into().expect("LAPIC ID must fit into a usize")
}

/// Gets the total number of available cores on the running machine
pub fn number_of_cores() -> usize {
    #[cfg(target_arch = "x86_64")]
    crate::limine::SMP.get_response().unwrap().cpus().len()
}
