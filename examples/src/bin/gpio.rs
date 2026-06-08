#![no_std]
#![no_main]

use defmt::info;
use defmt_semihosting as _;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_qemu_riscv::bind_interrupts;
use embassy_qemu_riscv::gpio::{self, Flex, Level};
use embassy_time::Timer;
use semihosting as _;

bind_interrupts!(struct Irqs {
    GPIO => gpio::InterruptHandler;
});

/// How often we toggle our output to ping the peer.
const HEARTBEAT_MS: u64 = 700;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_qemu_riscv::init();

    // GPIO0 is the single pin bridged over the socket to the peer EC instance.
    // The wire is bidirectional: we drive our level out through OUT and observe the
    // peer's level through IN, exactly like two devices sharing one wire.
    //
    // The same example runs on both instances. One is launched as the socket master
    // (hosts the socket) and the other as the socket slave (connects to it); the guest
    // code is otherwise identical and fully symmetric.
    let mut pin = Flex::new_async(p.GPIO0, Irqs);
    pin.set_low();

    let mut out_level = Level::Low;
    let mut tx = 0u32;
    let mut rx = 0u32;

    info!("GPIO link up: driving output + watching peer input");

    loop {
        // Race our own heartbeat against the peer toggling its line: whichever
        // happens first wins, so we act as an output and an input concurrently.
        match select(Timer::after_millis(HEARTBEAT_MS), pin.wait_for_any_edge()).await {
            // Heartbeat fired: flip our output so the peer sees a fresh edge.
            Either::First(_) => {
                out_level = match out_level {
                    Level::Low => Level::High,
                    Level::High => Level::Low,
                };
                pin.set_level(out_level);
                tx += 1;
                info!("tx #{=u32}: drove output -> {}", tx, out_level);
            }
            // The peer toggled its line: report the new input level we observed.
            Either::Second(_) => {
                rx += 1;
                info!("rx #{=u32}: peer input is now {}", rx, pin.level());
            }
        }
    }
}
