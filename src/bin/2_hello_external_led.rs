#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{
        p0::{Parts as P0Parts},
        Level,
    },
    Timer,
};


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0 pins
    let pins = P0Parts::new(board.P0);

    // set 3 gpios into push pull output, with the initial level HIGH
    let mut led_red = pins.p0_03.into_push_pull_output(Level::High);
    let mut led_blue = pins.p0_04.into_push_pull_output(Level::High);
    let mut led_green = pins.p0_28.into_push_pull_output(Level::High);


    loop {
       
        led_red.set_high().unwrap();
        led_blue.set_low().unwrap();
        led_green.set_high().unwrap();

        timer.delay_ms(1000_u32);

        led_red.set_low().unwrap();
        led_blue.set_high().unwrap();
        led_green.set_high().unwrap();

        timer.delay_ms(1000_u32);
    };

    knurling_session_20q4::exit()
}



