use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Get the name of the package.
    let kernel_name = env::var("CARGO_PKG_NAME")?;
    let arch = env::var("CARGO_CFG_TARGET_ARCH")?;

    // Tell rustc to pass the linker script to the linker.
    match arch.as_str() {
        "x86_64" => {
            println!("cargo:rustc-link-arg-bin={kernel_name}=--script=conf/linker-x86_64.ld");
        }
        "aarch64" => {
            println!("cargo:rustc-link-arg-bin={kernel_name}=--script=conf/linker-aarch64.ld");
        }
        other_arch => todo!("{other_arch} is not implemented yet"),
    }
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    Ok(())
}
