#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::blocking::delay::DelayMs;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Pin, PullUp},
    prelude::*,
    Temp, Timer,
};

struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }
    /// Returns true if button is pressed
    fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);

    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    let pins = P0Parts::new(board.P0);
    let button_1 = Button::new(pins.p0_11.degrade());

    loop {
        let temperature: f32 = temp.measure().to_num();

        if button_1.is_pressed() {
            defmt::info!("{:f32} °C", temperature);
            timer.delay_ms(500_u32);
        }
    }
}
