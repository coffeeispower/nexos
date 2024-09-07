use bitflags::bitflags;
use x86_64::instructions::interrupts::without_interrupts;

use super::ports::write;

pub const DATA_PORT: u16 = 0x3F8u16;
pub const INTERRUPT_ENABLE_PORT: u16 = DATA_PORT + 1;
pub const FIFO_CONTROL_PORT: u16 = DATA_PORT + 2;
pub const LINE_CONTROL_PORT: u16 = DATA_PORT + 3;
pub const MODEM_CONTROL_PORT: u16 = DATA_PORT + 4;
pub const LINE_STATUS_PORT: u16 = DATA_PORT + 5;

bitflags! {
    /// Interrupt enable flags
    struct IntEnFlags: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}
pub fn init() {
    without_interrupts(|| {
        unsafe {
            // Enable DLAB
            write(LINE_CONTROL_PORT, 0x80);

            // Set maximum speed to 38400 bps by configuring DLL and DLM
            write(DATA_PORT, 0x03);
            write(INTERRUPT_ENABLE_PORT, 0x00);

            // Disable DLAB and set data word length to 8 bits
            write(LINE_CONTROL_PORT, 0x03);

            // Enable FIFO, clear TX/RX queues and
            // set interrupt watermark at 14 bytes
            write(FIFO_CONTROL_PORT, 0xc7);

            // Mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            write(MODEM_CONTROL_PORT, 0x0b);
        }
    })
}
fn line_sts() -> LineStsFlags {
    unsafe { LineStsFlags::from_bits_truncate(super::ports::read(LINE_STATUS_PORT)) }
}
fn io_wait() {
    while !line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
}
pub fn print_byte(ch: u8) {
    unsafe {
        match ch {
            8 | 0xf7 => {
                io_wait();
                write(DATA_PORT, 8);
                io_wait();
                write(DATA_PORT, b' ');
                io_wait();
                write(DATA_PORT, 8);
            }
            _ => {
                io_wait();
                write(DATA_PORT, ch);
            }
        }
    }
}
pub fn print_str(text: &str) {
    for ch in text.bytes() {
        print_byte(ch);
    }
}
