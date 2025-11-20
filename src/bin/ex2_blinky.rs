//! This example tests the RP Pico2 W on board LED
//!
//! NOTE: This targets a RP Pico2 W or PR Pico2 WH. It does not work with the RP Pico2 board (non-wifi).
//!
//! Why is it so complicated for a blinky? The led is physically connected to the wifi chip which is separate from the rp2350 mcu.
//! Therefore the wifi chip needs to be setup first and that is quite a procedure because we need to load its firmware and set the country locale martix.
//! We also need to setup the wifi task.

#![no_std]
#![no_main]

use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    peripherals::PIO0,
    pio::{self, Pio},
};
use embassy_time::{Duration, Timer};
use rp_pico2w_examples_wprobe::radio::setup_radio;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // wait for host to connect to usb serial port
    Timer::after(Duration::from_millis(1000)).await;
    info!("started");

    // setup spi bus for wifi modem
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );
    let (_net_device, mut control) = setup_radio(&spawner, pwr, spi).await;

    let delay = Duration::from_secs(1);
    loop {
        info!("led on!");
        control.gpio_set(0, true).await;
        Timer::after(delay).await;

        info!("led off!");
        control.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}
