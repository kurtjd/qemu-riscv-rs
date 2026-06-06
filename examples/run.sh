#!/bin/sh
# Run example: pipes output through defmt-print to decode defmt semihosting frames.
#
# For the I2C examples we wire up a socket chardev so a controller (i2c-controller)
# and a target (i2c-target) running in two separate QEMU instances can talk to each
# other: the target hosts the socket and the controller connects to it
#
# Start the target first (`cargo run-target`), then the controller
# (`cargo run-controller`).
ELF="$1"
NAME=$(basename "$ELF")
SOCK="${EC_I2C_SOCK:-/tmp/qemu-ec-i2c.sock}"
QEMU=~/repos/qemu/build/qemu-system-riscv32

CHARDEV=""
case "$NAME" in
    i2c-controller)
        # I2C controller: connect to the target's socket as a client.
        CHARDEV="-chardev socket,id=ec-i2c-controller,path=$SOCK,server=off"
        ;;
    i2c-target)
        # I2C target: host the socket so the controller can connect.
        CHARDEV="-chardev socket,id=ec-i2c-target,path=$SOCK,server=on,wait=off"
        ;;
esac

# shellcheck disable=SC2086 # CHARDEV is intentionally word-split into args
"$QEMU" \
    -machine ec -bios none -nographic -semihosting \
    $CHARDEV \
    -kernel "$ELF" 2>&1 | defmt-print -e "$ELF"
