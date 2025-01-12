# NexOS

**NexOS** is a modern and innovative operating system, created to rethink user experience and the traditional architecture of Unix-based systems.  
Developed from scratch in [Rust](https://www.rust-lang.org/), NexOS combines the robustness and security of the language with a minimalist and efficient approach to create a powerful and user-friendly desktop operating system.

**NexOS** provides a modern and fast operating system experience without the unnecessary complexity of traditional Linux distributions. With a focus on simplicity, performance, and usability, NexOS aims to be "Linux as it would be if designed in 2024."

## Key Features

- **Integrated Wayland Compositor:** **NexOS** directly integrates a [Wayland](https://wayland.freedesktop.org/) compositor into the Kernel, offering a fast and responsive graphical environment without the need for additional layers or inter-process communication (IPC).
- **100% Compatible with Linux Binaries:** **NexOS** is fully compatible with Linux binaries, allowing any program that uses Wayland for the GUI to run flawlessly. Additionally, XWayland support ensures that older applications still based on Xorg can run without issues.
- **Innovative Tab System:** Inspired by [Essence OS](https://nakst.gitlab.io/essence), **NexOS** introduces an innovative way for users to organize their applications into tabs within user-created windows. Each created window can contain multiple applications opened in their respective tabs.  
    > "Applications are not opened in their own windows; instead, users create the windows and organize applications into tabs within these windows, similar to a browser."
- **Simplicity and Efficiency:** **NexOS** reduces boot time and simplifies the desktop experience by eliminating a traditional Init system, thus reducing the number of components and points of failure.
- **Developed in Rust:** Choosing the **[Rust](https://www.rust-lang.org/)** programming language ensures memory safety and high performance while making code maintenance and evolution easier.

## Current Project Status

**NexOS** is still under active development. Core functionalities such as process management, Linux program compatibility, and the integrated Wayland compositor are being implemented. This project is a work in progress, and constructive feedback is always welcome.

## Running in a VM

### Requirements

- Xorriso  
- Git  
- Build Essential (gcc, make, etc.)  
- QEMU (x86 or aarch64)  
- [rustup](https://rustup.rs) **(Recommended)** | Rust **nightly** compiler  
- A computer running **Linux** (If you’re using Windows, WSL will be necessary. Self-hosting within NexOS might be possible in the future.)

### Using Nix Dev Shell

You can also use [Nix](https://nixos.org) to automatically set up the development environment by running the command `nix develop`.

### Steps

1. Clone the project:

```bash
git clone https://github.com/coffee-is-power/nexos
```

2. Compile the Kernel and run QEMU:

```bash
cargo run
```

You can also specify a different target to compile for other architectures and run VMs on those architectures. The primary architecture is `x86_64`, but ARM support is also being developed as I learn more about it.
