#![no_std]
#![no_main]

use defmt::info;
use defmt_semihosting as _;
use embassy_executor::Spawner;
use embassy_qemu_riscv::i2c::controller::{self, I2c};
use embassy_qemu_riscv::{bind_interrupts, peripherals};
use semihosting as _;

bind_interrupts!(struct Irqs {
    I2C0 => controller::InterruptHandler<peripherals::I2C0>;
});

/// 7-bit address of the target device on the bus.
const TARGET_ADDR: u8 = 0x50;

/// Register within the target device to read from.
const REGISTER: u8 = 0x42;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_qemu_riscv::init();

    let mut i2c = I2c::new_async(p.I2C0, Irqs);

    // Write the register address, then read 4 bytes back after a repeated START.
    let mut buf = [0u8; 4];
    match i2c.write_read(TARGET_ADDR, &[REGISTER], &mut buf).await {
        Ok(()) => info!("Read from register {=u8:#x}: {=[u8]:#x}", REGISTER, buf),
        Err(e) => info!("I2C error: {}", e),
    }

    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
