#!/usr/bin/env bash
#
set -xe

LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"
# Cargo passes the path to the built executable as the first argument.
KERNEL=$1
TARGET_BASEDIR=$(dirname $KERNEL)
ISO_ROOT=$TARGET_BASEDIR/iso_root
LIMINE_BOOTLOADER_REPO=$TARGET_BASEDIR/limine
ISO_IMAGE_PATH=$KERNEL.iso
# Clone the `limine` repository if we don't have it yet.
if [ ! -d $LIMINE_BOOTLOADER_REPO ]; then
    git clone $LIMINE_GIT_URL --depth=1 --branch v5.x-branch-binary $LIMINE_BOOTLOADER_REPO
    # Make sure we have an up-to-date version of the bootloader.
    cd $LIMINE_BOOTLOADER_REPO
    make
    cd -
fi

# Copy the needed files into an ISO image.
mkdir -p $ISO_ROOT
cp conf/limine.cfg $LIMINE_BOOTLOADER_REPO/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} $ISO_ROOT
cp $KERNEL $ISO_ROOT/tinyx
mkdir -p $ISO_ROOT/EFI/BOOT
cp -v $LIMINE_BOOTLOADER_REPO/BOOTX64.EFI $ISO_ROOT/EFI/BOOT/
cp -v $LIMINE_BOOTLOADER_REPO/BOOTIA32.EFI $ISO_ROOT/EFI/BOOT/
xorriso -as mkisofs -b limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    $ISO_ROOT -o $ISO_IMAGE_PATH
rm $ISO_ROOT -rf
# For the image to be bootable on BIOS systems, we must run `limine bios-install` on it.
$LIMINE_BOOTLOADER_REPO/limine bios-install $ISO_IMAGE_PATH
