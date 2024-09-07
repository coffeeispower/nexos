use limine::request::*;
pub static MEMMAP_REQ: MemoryMapRequest = MemoryMapRequest::new();
pub static BOOTLOADER_INFO: BootloaderInfoRequest = BootloaderInfoRequest::new();
pub static HHDM: HhdmRequest = HhdmRequest::new();
pub static KERNEL_ADDRESS: KernelAddressRequest = KernelAddressRequest::new();
pub static KERNEL_FILE: KernelFileRequest = KernelFileRequest::new();
pub static SMP: SmpRequest = SmpRequest::new();
