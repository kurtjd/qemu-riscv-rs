#![no_std]
#![no_main]

use defmt::info;
use defmt_semihosting as _;
use embassy_executor::Spawner;
use embassy_qemu_riscv::i2c::target::{self, I2c};
use embassy_qemu_riscv::{bind_interrupts, peripherals};
use embedded_mcu_hal::i2c::target::Request;
use semihosting as _;

bind_interrupts!(struct Irqs {
    I2C_TARGET => target::InterruptHandler<peripherals::I2C_TARGET>;
});

/// 7-bit address this target answers to.
const TARGET_ADDR: u8 = 0x50;

/// The single register we serve.
const MAGIC_REG: u8 = 0x42;

/// Value returned when reading [`MAGIC_REG`] (big-endian on the wire).
const MAGIC: u32 = 0xCAFE_BABE;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_qemu_riscv::init();

    let mut i2c = I2c::new_async(p.I2C_TARGET, Irqs, TARGET_ADDR);

    // The most recently selected register.
    let mut selected = 0u8;

    info!("I2C target listening at {=u8:#x}", TARGET_ADDR);

    loop {
        match i2c.listen().await {
            Request::Write(_addr) => {
                // The controller selects a register with a single byte.
                let mut buf = [0u8; 1];
                i2c.respond_to_write(&mut buf).await;
                selected = buf[0];
                info!("Selected register {=u8:#x}", selected);
            }
            Request::Read(_addr) => {
                // Serve the magic value for our register, otherwise all zeros.
                let payload = if selected == MAGIC_REG {
                    MAGIC.to_be_bytes()
                } else {
                    [0u8; 4]
                };
                i2c.respond_to_read(&payload).await;
                info!("Served {=[u8]:#x}", payload);
            }
            _ => {}
        }
    }
}
