#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::OutputPin,
};

// access to peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    // take returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0 pins
    let pins = P0Parts::new(board.P0);
    // some pins are pins on the board, some pins are on board LEDs
    // set pin p0_13 into push pull output, with the initial level LOW
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);

    loop {
        led_1.set_high().unwrap();
        time.delay_ms(1000_u32);
        led_1.set_low().unwrap();
        time.delay_ms(1000_32);
    }

    knurling_session_20q4::exit()
}
