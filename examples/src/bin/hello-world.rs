#![no_std]
#![no_main]

use defmt::info;
use defmt_semihosting as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use semihosting as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_qemu_riscv::init();

    loop {
        info!("Hello world");
        Timer::after_secs(1).await;
    }
}
