#![no_std]
#![no_main]
#![feature(
    naked_functions,
    ptr_sub_ptr,
    alloc_error_handler,
    abi_x86_interrupt,
    custom_test_frameworks,
    int_roundings,
    negative_impls
)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![test_runner(crate::test_runner::run_tests)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
#[macro_use(test)]
extern crate test_macros;
#[macro_use]
extern crate alloc;
#[macro_use]
pub mod print;
#[macro_use]
pub mod panic;
pub mod arch;
pub mod bitmap_allocator;
pub mod limine;
pub mod multicore;
pub mod kernel;
#[cfg(test)]
pub mod test_runner;

use limine::BOOTLOADER_INFO;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Booting NexOS v{}", env!("CARGO_PKG_VERSION"));

    if let Some(bootinfo) = BOOTLOADER_INFO.get_response() {
        print!(" with {} v{}", bootinfo.name(), bootinfo.version(),);
    }
    println!();
    #[cfg(test)]
    test_main();
    panic!("Reached end of main function")
}
