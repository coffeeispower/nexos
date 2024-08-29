# NexOS
Linux

# Running a VM
## Requirements:
  - Xorriso
  - Git
  - Build Essential (gcc, make, etc...)
  - qemu (x86 or aarch64)
  - [rustup](https://rustup.rs) **(Recommended)** | **nightly** rust compiler
  - And have a **linux machine** (If you have windows you'll need WSL)
## Steps
1. Clone the project:
```bash
git clone https://github.com/coffee-is-power/nexos
```
2. Build kernel and run qemu
```bash
cargo run
```
You're done!

You can also specify a different target to compile to other architectures and run vms on other architectures. The main arch is `x86_64`, but `aarch64` is also being worked on as I learn about it.
