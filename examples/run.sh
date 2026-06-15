#!/bin/sh
# Run the keyboard example: a UART-driven HID-over-I2C keyboard service.
#
# UART0 carries real keystrokes (not defmt), so we hand it a dedicated PTY that a
# terminal (picocom) can attach to. defmt still works because semihosting is
# routed to QEMU's own stdout (`target=native`) rather than a serial chardev, so
# UART0 and the log stream stay cleanly separated. The example also needs both the
# I2C target and GPIO sockets bridged (HID-over-I2C data + the keyboard->host
# interrupt line).
#
# With `-serial pty`, QEMU prints a single "char device redirected to /dev/pts/N
# (label serial0)" line on stdout before any semihosting data. We peel that first
# line off to stderr (so it stays visible on the terminal) and feed the remaining
# bytes to defmt-print. This mirrors the approach used by the `qemu-run` helper.
ELF="$1"
SOCK="${EC_I2C_SOCK:-/tmp/qemu-ec-i2c.sock}"
GPIO_SOCK="${EC_GPIO_SOCK:-/tmp/qemu-ec-gpio.sock}"

# CHANGEME: To wherever you built qemu-system-riscv32 using our branch:
# https://github.com/kurtjd/qemu/tree/odp
QEMU=~/qemu/build/qemu-system-riscv32

echo "Connect your terminal (e.g. picocom) to the PTY printed below, at 115200 8N1." >&2
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
