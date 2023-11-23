#![no_std]
#![no_main]
#![feature(
    naked_functions,
    ptr_sub_ptr,
    alloc_error_handler,
    abi_x86_interrupt,
    custom_test_frameworks,
    int_roundings
)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![test_runner(crate::test_runner::run_tests)]
#![reexport_test_harness_main = "test_main"]
#[macro_use]
pub mod io;
#[macro_use]
pub mod panic;
pub mod arch;
pub mod bitmap_allocator;
pub mod interrupts;
pub mod test_runner;

use limine::BootInfoRequest;

use crate::bitmap_allocator::GLOBAL_PAGE_ALLOCATOR;
static BOOTLOADER_INFO: BootInfoRequest = BootInfoRequest::new(0);
/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::serial::init();
    }
    #[cfg(target_arch = "aarch64")]
    {
        // crate::arch::aarch64::serial::init();
    }
    println!("hello, world!");

    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        println!(
            "booted by {} v{}",
            bootinfo.name.to_str().unwrap().to_str().unwrap(),
            bootinfo.version.to_str().unwrap().to_str().unwrap(),
        );
    }
    interrupts::load_interrupts();
    println!("{:#?}", GLOBAL_PAGE_ALLOCATOR.number_of_pages());

    #[cfg(test)]
    test_main();
    panic!("Reached end of main function")
}
#[cfg(test)]
mod tests {
    #[test_case]
    fn some_shitty_test() {
        assert_eq!(1, 0);
    }
}
