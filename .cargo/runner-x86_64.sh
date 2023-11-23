#!/usr/bin/env bash
#
# This script will be executed by `cargo run`.
set -xe
KERNEL=$1
ISO_IMAGE_PATH=$KERNEL.iso
$(dirname $0)/create-image.sh $@
# Run the created image with QEMU.
qemu-system-x86_64 \
    -bios ${OVMF_PATH:-"/usr/share/OVMF/OVMF_CODE.fd"} \
    $QEMU_FLAGS \
    -enable-kvm \
    -machine q35 -cpu qemu64 -M smm=off \
    -D target/x86_64-log.txt -d int,guest_errors \
    -serial stdio \
    $ISO_IMAGE_PATH
