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

    info!("Controller: Waiting for target to write 0xDEADBEEF...");

    // Wait for the target to write 0xDEADBEEF to shared memory
    // Note: In reality the host/controller will likely not care about a PERIPH_PENDING interrupt,
    // this is just a quick hack to wait until the EC/target qemu instance has started.
    loop {
        let offset = espi.wait_periph().await;
        let mut buf = [0u8; 4];
        espi.read_shmem(offset as usize, &mut buf).unwrap();
        let val = u32::from_le_bytes(buf);
        if val == 0xDEAD_BEEF {
            info!("Controller: Got 0xDEADBEEF at offset {}", offset);
            break;
        }
    }

    // Write data to shared memory
    let data: [u8; 4] = 0xCAFE_BABEu32.to_le_bytes();
    let write_offset: usize = 0x100;
    espi.write_shmem(write_offset, &data).unwrap();
    info!("Controller: Wrote 0xCAFEBABE at offset {:#X}", write_offset);

    // Read it back and verify
    let mut readback = [0u8; 4];
    espi.read_shmem(write_offset, &mut readback).unwrap();
    assert_eq!(data, readback);
    let readback_val = u32::from_le_bytes(readback);
    info!("Controller: Readback matches! {:#010X}", readback_val);

    // Wait for a vwire from the target
    info!("Controller: Waiting for vwire from target...");
    let vwire = espi.read_vwire().await;
    info!("Controller: Got vwire data: {:#010X}", vwire);

    info!("Controller: Done!");
    loop {}
}
