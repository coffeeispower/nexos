use core::fmt;
use spin::Mutex;

struct Writer {}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(target_arch = "x86_64")]
        {
            crate::arch::x86_64::serial::print_str(s);
        }
        #[cfg(target_arch = "aarch64")]
        {
            // crate::arch::aarch64::serial::print_str(s);
        }
        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer {});

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::io::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)); };
}
