#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

use embedded_hal::digital::v2::InputPin;
// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level, Input, Pin, PullUp},
    prelude::*,
};

struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }
    /// Button is pressed
    fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);
    let button_1 = Button::new(pins.p0_11.degrade());
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::High);

    loop {
        if button_1.is_pressed() == true {
            led_1.set_low().unwrap();
        } else {
            led_1.set_high().unwrap();
        }
    };
}