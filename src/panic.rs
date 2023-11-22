use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel got a high five with a pan: {}", info);
    // TODO: Show backtrace on panic, we need to implement a heap allocator first
    /*
    if option_env!("RUST_BACKTRACE").is_some() {
        // SAFETY: this is executed on panic, which is sync for the most part
        unsafe {
            backtrace::trace_unsynchronized(|frame| {
                backtrace::resolve_frame_unsynchronized(frame, |symbol| {
                    println!(
                        "  {}:{}:{}",
                        symbol
                            .name()
                            .map(|m| m.as_str().unwrap_or("unknown"))
                            .unwrap_or("unknown"),
                        symbol.lineno().unwrap_or(0),
                        symbol.colno().unwrap_or(0)
                    );
                });
                true
            });
        }
    }
     */
    hcf();
}
pub fn hcf() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
