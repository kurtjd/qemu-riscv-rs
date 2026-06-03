#![no_std]
#![no_main]

use defmt::info;
use defmt_semihosting as _;
use embassy_executor::Spawner;
use embassy_qemu_riscv::espi::{self, Espi};
use embassy_qemu_riscv::{bind_interrupts, peripherals};
use semihosting as _;

bind_interrupts!(struct Irqs {
    ESPI0 => espi::InterruptHandler<peripherals::ESPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_qemu_riscv::init();
    let mut espi = Espi::new_async(p.ESPI0, Irqs);

    // Write 0xDEADBEEF to shared memory, which also lets the host/controller QEMU instance know we've started
    let magic = 0xDEAD_BEEFu32.to_le_bytes();
    espi.write_shmem(0, &magic).unwrap();
    info!("Target: Wrote 0xDEADBEEF at offset 0");

    // Wait for the controller to write to shared memory
    info!("Target: Waiting for periph data from controller...");
    let offset = espi.wait_periph().await;
    info!("Target: Periph interrupt! Offset: {:#X}", offset);

    // Read shared memory at that offset and print it
    let mut buf = [0u8; 4];
    espi.read_shmem(offset as usize, &mut buf).unwrap();
    let val = u32::from_le_bytes(buf);
    info!("Target: Data at offset {:#X}: {:#010X}", offset, val);

    // Send a vwire status to the controller
    let status: u32 = 0x42;
    espi.write_vwire(status).await;
    info!("Target: Sent vwire status {:#010X}", status);

    info!("Target: Done!");
    loop {}
}
