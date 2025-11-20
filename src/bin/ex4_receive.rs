//! This example receives incomming udp packets and turns an led on or off depending on the payload
//! In order to connect to the wifi network please create the following two files in the `src` folder:
//! WIFI_SSID.txt and WIFI_PASSWORD.txt
//! The files above should contain the exact ssid and password to connect to the wifi network. No newline characters or quotes.
//!
//! NOTE: This targets a RP Pico2 W or PR Pico2 WH. It does not work with the RP Pico2 board (non-wifi).

#![no_std]
#![no_main]

use core::str::from_utf8;

use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::{error, info, warn};
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    peripherals::{PIO0, USB},
    pio::{self, Pio},
    usb::{self},
};
use embassy_time::{Duration, Timer};
use rp_pico2w_examples_wprobe::{network::setup_network, radio::setup_radio};
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
    let (net_device, mut control) = setup_radio(&spawner, pwr, spi).await;

    const LOCAL_PORT: u16 = 47900;
    let socket = setup_network(&spawner, net_device, &mut control, None, LOCAL_PORT).await;
    info!("waiting for udp packets on port {}", LOCAL_PORT);

    let mut buf: [u8; 32] = [0; 32];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, meta)) => match from_utf8(&buf[..len]) {
                Ok(s) => {
                    info!("received '{}' from {:?}", s, meta);
                    match s {
                        "on" => control.gpio_set(0, true).await,
                        "off" => control.gpio_set(0, false).await,
                        _ => warn!("unknown command received"),
                    }
                }
                Err(_e) => warn!("received {} bytes from {:?} with invalid utf8", len, meta),
            },
            Err(e) => error!("error receiving packet: {:?}", e),
        }
    }
}
