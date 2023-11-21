#!/usr/bin/env bash
#
# This script will be executed by `cargo run`.

set -xe

LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1

# Clone the `limine` repository if we don't have it yet.
if [ ! -d target/limine ]; then
    git clone $LIMINE_GIT_URL --depth=1 --branch v5.x-branch-binary target/limine
fi

# Make sure we have an up-to-date version of the bootloader.
cd target/limine
git fetch
make
cd -

# Copy the needed files into an ISO image.
mkdir -p target/iso_root
cp $KERNEL conf/limine.cfg target/limine/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} target/iso_root

xorriso -as mkisofs                                             \
    -b limine-bios-cd.bin                                       \
    -no-emul-boot -boot-load-size 4 -boot-info-table            \
    --efi-boot limine-uefi-cd.bin                               \
    -efi-boot-part --efi-boot-image --protective-msdos-label    \
    target/iso_root -o $KERNEL.iso
ti
# For the image to be bootable on BIOS systems, we must run `limine-deploy` on it.
target/limine/limine-deploy $KERNEL.iso

# Run the created image with QEMU.
qemu-system-aarch64 \
    -bios ${AVMF_PATH:-"/usr/share/AAVMF/AAVMF_CODE.fd"} \
    $QEMU_FLAGS \
    -cpu cortex-a57 -M virt \
    -D target/aarch64-log.txt -d int,guest_errors -no-reboot -no-shutdown \
    -m 1GB \
    -serial stdio \
    -device ramfb \
    $KERNEL.iso
