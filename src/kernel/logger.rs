use cfg_if::cfg_if;


pub trait Logger: Send {
    fn print_str(&self, message: &str);
}
impl dyn Logger {
    pub fn new() -> impl Logger {
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                crate::arch::x86_64::LoggerX86Impl::new()
            } else {
                todo!()
            }
        }
    }
}
