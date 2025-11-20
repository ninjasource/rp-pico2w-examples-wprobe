# Raspberry Pi Pico2 W examples with debug probe

This Rust repo was created to demonstrate a simple network interaction between two rp pico2w dev boards (RP2350) with an attached raspberry pi debug probe. 
This debug probe is used for programming the pico2 and viewing the logs. This should work with other SWD probes too.

## Setup

Install Rust on your machine.

Add the armv8 cross compiler to the rust toolchain
```bash
rustup target add thumbv8m.main-none-eabihf
```

Install the programmer tool
```bash
cargo install probe-rs-tools
```

## How to run the examples

See `src/bin/ex1_logs.rs` for instructions on how to run the example.

```bash
cargo run --bin ex1_logs --release
```

## Troubleshooting

> Error running: `Error: No connected probes were found.`

Reason: You have not plugged your raspberry pi debug probe into your computer

Solution: Plug your debug probe (not your pico2w) into a usb port on your computer


> Error running: `Error: Connecting to the chip was unsuccessful.`

Reason #1: Your pico2w is not plugger into your computer

Solution #1: Plug it in to a usb port

Reason #2: Your pico2w is not connected to your debug probe

Solution #2: Connect your raspberry pi debug probe (D port) the following way to debug pins on the pico2w: Orange (SWCLK), Black (GND), Yellow (SWDIO)

## What is the `memory.x` file? 

One of the last steps in compilation is linking which is the process of assigning physical memory addresses to variables and code.
On a computer with an operating system the OS uses virtual memory but embedded systems like the rp-pico don't have an OS 
and we need to create an executable with physical memory addresses in the correct locations that are expected by the pico. 
The `memory.x` file is the the developer facing linker script that tells the linker when RAM and FLASH physically start. 
If you look at `.cargo/config.toml` you will see a whole bunch of linker scripts referenced there. The `link.x` script references `memory.x`. 

## How can this be compiled on a PC and run on a pico?

Rust supports cross compilation and this is setup in the `.cargo/config.toml` file with the following config:

```toml
[build]
target = "thumbv8m.main-none-eabihf" 
```


## Send test data to the pico2w

In Linux (using netcat to fire and forget ipv4 udp packet - change the IP address accordingly):
```bash
echo -n "on" | nc -4u -w0 192.168.1.99 47900
echo -n "off" | nc -4u -w0 192.168.1.99 47900
```

## Ideas to work on when you finish all the examples

Morse code:

Get your pico to flash with morse code when sent text over USB serial.

TCP/IP:

Experiment with opening a TCP/IP connection rather than using UDP.

Internet access:

Instead of connecting to the sandbox wifi, connect to a wifi network that has access to the internet and attempt to make some http calls. 
The pico2w is an extremely low power device which can be left on all the time. This makes it ideal for checking RSS feeds or, say, looking up Transport-For-London bus times.

See:
https://tfl.gov.uk/info-for/open-data-users/api-documentation

For example:
https://api.tfl.gov.uk/StopPoint/490013767D/arrivals

