#!/bin/sh
# Run example: pipes output through defmt-print to decode defmt semihosting frames.
#
# For the I2C examples we wire up a socket chardev so a controller (i2c-controller)
# and a target (i2c-target) running in two separate QEMU instances can talk to each
# other: the target hosts the socket and the controller connects to it
#
# Start the target first (`cargo run-target`), then the controller
# (`cargo run-controller`).
#
# The GPIO example runs the same binary on two instances bridged over GPIO pin 0.
# A runner argument selects the socket role: `master` hosts the socket, `slave`
# connects to it. Start `cargo run-gpio-master` first, then `cargo run-gpio-slave`.
ELF="$1"
ROLE="$2"
NAME=$(basename "$ELF")
SOCK="${EC_I2C_SOCK:-/tmp/qemu-ec-i2c.sock}"
GPIO_SOCK="${EC_GPIO_SOCK:-/tmp/qemu-ec-gpio.sock}"
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
    gpio)
        case "$ROLE" in
            master)
                # GPIO master: host the socket so the slave can connect.
                CHARDEV="-chardev socket,id=ec-gpio0,path=$GPIO_SOCK,server=on,wait=off"
                ;;
            slave)
                # GPIO slave: connect to the master's socket as a client.
                CHARDEV="-chardev socket,id=ec-gpio0,path=$GPIO_SOCK,server=off"
                ;;
        esac
        ;;
esac

# shellcheck disable=SC2086 # CHARDEV is intentionally word-split into args
"$QEMU" \
    -machine ec -bios none -nographic -semihosting \
    $CHARDEV \
    -kernel "$ELF" 2>&1 | defmt-print -e "$ELF"
