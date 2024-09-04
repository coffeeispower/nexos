use core::{arch::asm, panic::PanicInfo, sync::atomic::{AtomicBool, Ordering}};

static PANIC: AtomicBool = AtomicBool::new(false);
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if PANIC.swap(true, Ordering::Relaxed) {
        println!("DOUBLE PANIC!!!!!!!!!!!!!!!!!");
        println!("{info}");
        hcf();
    }
    #[cfg(test)]
    {
        println!("TEST \x1b[1;31mPANIC\x1b[1;0m");
        println!("---------- Test Error Message ----------");
        println!("{info}");
        unsafe {
            print_stack_trace();
        }
        println!("----------------------------------------");
        #[cfg(all(target_arch = "x86_64", feature = "qemu-exit"))]
        unsafe {
            use crate::arch::x86_64::ports::write;
            write(0xf4, 0x11);
        }
    }
    #[cfg(not(test))]
    {
        println!("Kernel got a high five with a pan: {}", info);
        unsafe {
            print_stack_trace();
        }

    }
    hcf();
}
pub fn hcf() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

unsafe fn print_stack_trace() {
    let mut base_pointer: *const usize;
    // let offset = KERNEL_ADDRESS.get_response().unwrap().virtual_base() as usize;
    unsafe {
        asm!("mov rax, rbp", out("rax") base_pointer);
    }
    while !base_pointer.is_null() {
        let return_address = unsafe { *(base_pointer.add(1)) } as usize;
        if return_address == 0 {
            break;
        }
        println!(" - <0x{:X}>", return_address);
        // resolve_address(return_address as u64);

        // Atualiza o base_pointer para o próximo frame da pilha
        base_pointer = unsafe { *base_pointer as *const usize };
    }
}
// use addr2line::Context;
// use gimli::{EndianSlice, RunTimeEndian};
// use object::{Object, ObjectSection};

// fn resolve_address(address: u64) {
//     let kernel_file = KERNEL_FILE.get_response().unwrap().file();

//     let elf_data =
//         unsafe { core::slice::from_raw_parts(kernel_file.addr(), kernel_file.size() as usize) };

//     // Parse o arquivo ELF
//     let object = object::File::parse(elf_data).expect("Failed to parse ELF file");

//     // Configurar a leitura DWARF
//     let endian = if cfg!(target_endian = "little") {
//         RunTimeEndian::Little
//     } else {
//         RunTimeEndian::Big
//     };

//     let dwarf = gimli::Dwarf::load(|id| {
//         Ok::<_, ()>(EndianSlice::new(
//             object
//                 .section_by_name(id.name())
//                 .and_then(|section| section.data().ok()).unwrap_or(&[]),
//             endian,
//         ))
//     })
//     .unwrap();

//     let context = Context::from_dwarf(dwarf).unwrap();
//     // Resolver o endereço
//     if let Ok(Some(location)) = context.find_location(address) {
//         println!(
//             "  at {}:{}:{}",
//             location.file.unwrap_or("unknown"),
//             location.line.unwrap_or(0),
//             location.column.unwrap_or(0)
//         );
//     }
// }
