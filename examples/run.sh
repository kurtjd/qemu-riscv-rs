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

# CHANGEME: To wherever you built qemu-system-riscv32 using our branch:
# https://github.com/kurtjd/qemu/tree/ec
QEMU=~/repos/qemu/build/qemu-system-riscv32

# The keyboard example is special: UART0 carries real keystrokes (not defmt), so we hand it a
# dedicated PTY that a terminal (PuTTY/screen) can attach to. defmt still works because
# semihosting is routed to QEMU's own stdout (`target=native`) rather than a serial chardev, so
# UART0 and the log stream stay cleanly separated. The example also needs both the I2C target and
# GPIO sockets bridged (HID-over-I2C data + the keyboard->host interrupt line).
#
# With `-serial pty`, QEMU prints a single "char device redirected to /dev/pts/N (label serial0)"
# line on stdout before any semihosting data. We peel that first line off to stderr (so it stays
# visible on the terminal) and feed the remaining bytes to defmt-print. This mirrors the approach
# used by the `qemu-run` helper.
if [ "$NAME" = keyboard ]; then
    echo "Connect your terminal (e.g. PuTTY) to the PTY printed below, at 115200 8N1." >&2
    "$QEMU" \
        -machine ec -bios none -nographic -monitor none \
        -semihosting-config enable=on,target=native \
        -serial pty \
        -chardev socket,id=ec-i2c-target,path=$SOCK,server=on,wait=off \
        -chardev socket,id=ec-gpio0,path=$GPIO_SOCK,server=on,wait=off \
        -kernel "$ELF" | {
            # First stdout line is the PTY path; surface it on the terminal.
            IFS= read -r ptsline
            printf '%s\n' "$ptsline" >&2
            # Everything after is the semihosting defmt stream.
            defmt-print -e "$ELF"
        }
    exit $?
fi

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
