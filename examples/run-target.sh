#!/bin/sh
# Run eSPI target example: connects to existing socket, pipes output through defmt-print
# defmt-semihosting writes to stderr; QEMU's own stdout is discarded
ELF="$1"
~/repos/qemu/build/qemu-system-riscv32 \
    -machine virt -bios none -nographic -semihosting \
    -chardev socket,id=espi0,path=/tmp/espi.sock \
    -kernel "$ELF" 2>&1 | defmt-print -e "$ELF"
