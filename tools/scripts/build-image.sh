#!/usr/bin/env bash
# Build CodeOS QEMU ARM64 boot image (stub — v0.2)
set -euo pipefail

KERNEL_CONFIG="${1:-codeos-arm64-defconfig}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "CodeOS image builder (v0.1 stub)"
echo "  kernel config: kernel/configs/${KERNEL_CONFIG}"
echo "  device tree:   kernel/dts/codeos-qemu-virt.dts"
echo ""
echo "Full build pipeline lands in v0.2."
echo "  1. Build Linux kernel with CodeOS patches"
echo "  2. Build rootfs with system daemons"
echo "  3. Create bootable image for QEMU virt machine"

# Placeholder for v0.2:
# qemu-system-aarch64 -M virt -cpu cortex-a72 -m 2G \
#   -kernel arch/arm64/boot/Image \
#   -append "console=ttyAMA0 root=/dev/vda rw init=/sbin/codeos-init" \
#   -dtb codeos-qemu-virt.dtb
