
pub mod local;
/// Gets the ID of whatever core calls this function
pub fn current_core_id() -> usize {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use crate::arch::x86_64::apic::LAPIC;
            unsafe { LAPIC.read().id() }.try_into().expect("LAPIC ID must fit into a usize")
        } else {
            todo!()
        }
    }
}

/// Gets the total number of available cores on the running machine
pub fn number_of_cores() -> usize {
    crate::limine::SMP.get_response().unwrap().cpus().len()
}
