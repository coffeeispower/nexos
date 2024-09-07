pub fn run_tests(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("\x1B[1;32mAll tests run successfully!\x1B[1;0m");
    #[cfg(all(target_arch = "x86_64", feature = "qemu-exit"))]
    unsafe {
        use crate::arch::x86_64::ports::write;
        write(0xf4, 0x10);
    }
    loop {
        core::hint::spin_loop();
    }
}
