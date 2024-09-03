#!/usr/bin/env bash
# This script will be executed by `cargo run`.
KERNEL=$1
ISO_IMAGE_PATH=$KERNEL.iso
MEMORY=${MEMORY:-500M}
$(dirname $0)/create-image.sh $@
# Run the created image with QEMU.
qemu-system-x86_64 \
    -bios ${OVMF_PATH:-"/usr/share/OVMF/OVMF_CODE.fd"} \
    $QEMU_FLAGS \
    -enable-kvm \
    -machine q35 -cpu qemu64 -M smm=off \
    -D target/x86_64-log.txt -d int,guest_errors -no-reboot \
    -serial stdio \
    -m $MEMORY \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    $ISO_IMAGE_PATH
EXITCODE=$?
if [ $EXITCODE -ne 33 ]; then
    exit 1
fi
