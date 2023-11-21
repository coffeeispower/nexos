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
set +e
git fetch
set -e
make
cd -

# Copy the needed files into an ISO image.
mkdir -p target/iso_root
cp $KERNEL conf/limine.cfg target/limine/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} target/iso_root

mkdir -p target/iso_root/EFI/BOOT
cp -v target/limine/BOOTX64.EFI target/iso_root/EFI/BOOT/
cp -v target/limine/BOOTIA32.EFI target/iso_root/EFI/BOOT/
xorriso -as mkisofs -b limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    target/iso_root -o $KERNEL.iso
# For the image to be bootable on BIOS systems, we must run `limine bios-install` on it.
target/limine/limine bios-install $KERNEL.iso

# Run the created image with QEMU.
qemu-system-x86_64 \
    -bios ${OVMF_PATH:-"/usr/share/OVMF/OVMF_CODE.fd"} \
    $QEMU_FLAGS \
    -enable-kvm \
    -machine q35 -cpu qemu64 -M smm=off \
    -D target/x86_64-log.txt -d int,guest_errors \
    -serial stdio \
    $KERNEL.iso
