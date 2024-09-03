#!/usr/bin/env bash
# This script will be executed by `cargo run`.
KERNEL=$1

$(dirname $0)/create-image.sh $@
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
