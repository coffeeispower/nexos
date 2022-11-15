#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#[macro_use]
pub mod io;
#[macro_use]
pub mod panic;
pub mod arch;
pub mod bitmap_allocator;
pub mod interrupts;

use limine::LimineBootInfoRequest;
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello, world!");

    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        println!(
            "booted by {} v{}",
            bootinfo.name.to_str().unwrap().to_str().unwrap(),
            bootinfo.version.to_str().unwrap().to_str().unwrap(),
        );
    }
    interrupts::load_interrupts();
    let allocator = bitmap_allocator::BitmapAllocator::from_mmap();
    println!("{:#?}", allocator.number_of_pages());
    panic!("Reached end of main function")
}
