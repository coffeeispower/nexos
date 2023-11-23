pub fn run_tests(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("\x1B[1;32mAll tests run successfully!\x1B[1;0m");
    loop {
        core::hint::spin_loop();
    }
}