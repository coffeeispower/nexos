use core::arch::asm;

/// Writes a byte to a port
///
/// # Safety
/// This is unsafe because the caller must ensure the port is correct, otherwise it may cause undefined behavior
pub unsafe fn write(port: u16, data: u8) {
    asm!("out dx, al", in("dx") port, in("al") data, options(nomem, nostack, preserves_flags));
}

/// Read a byte from a port
///
/// # Safety
/// This is unsafe because the caller must ensure the port is correct, otherwise it may cause undefined behavior
pub unsafe fn read(port: u16) -> u8 {
    let mut data;
    asm!("in al, dx", out("al") data, in("dx") port, options(nomem, nostack, preserves_flags));
    data
}
