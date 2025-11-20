//! This example tests the RP Pico2 W on board LED which it turns on if pin GP_15 is connected to GND
//!
//! NOTE: This targets a RP Pico2 W or PR Pico2 WH. It does not work with the RP Pico2 board (non-wifi).

#![no_std]
#![no_main]

use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIO0, USB},
    pio::{self, Pio},
    usb::{self},
};
use embassy_time::{Duration, Timer};
use rp_pico2w_examples_wprobe::radio::setup_radio;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
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

    // this is GP14 (not the physical chip pin number!)
    let mut button = Input::new(p.PIN_14, Pull::Up);

    loop {
        info!("waiting for button press");
        button.wait_for_low().await;

        info!("led on!");
        control.gpio_set(0, true).await;

        // debounce the button
        Timer::after(Duration::from_millis(250)).await;

        info!("waiting for button release");
        button.wait_for_high().await;

        info!("led off!");
        control.gpio_set(0, false).await;

        // debounce the button
        Timer::after(Duration::from_millis(250)).await;
    }
}
