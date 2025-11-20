//! Start here. This example tests the RP Pico2 W logging.
//! We are assuming that you have connected up a raspberry pi debug probe (D port) the following way to debug pins on the pico2: Orange (SWCLK), Black (GND), Yellow (SWDIO)
//! This demo starts up and logs to the connected debug probe using the rtt (real time transfer) peripheral which sends logging data to the SWDIO pin without requiring dedicated cpu time.
//!
//! How to Run:
//! cargo run --bin Ex1_logs --release

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

// the imports below allow the defmt (deferred format) logging library to handle logging and panicking
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());
    info!("started");

    let mut counter = 0;
    loop {
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
        info!("count: {}", counter);
    }
}
