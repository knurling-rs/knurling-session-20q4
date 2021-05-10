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

enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl Unit {
    fn convert_temperature(&self, temperature: &f32) -> f32 {
        match self {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                temperature
            }
            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                temperature
            }
            Unit::Celsius => *temperature,
        }
    }
}

pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }

    /// Returns true if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut periodic_timer = Timer::periodic(board.TIMER0);

    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    let pins = P0Parts::new(board.P0);
    let button_1 = Button::new(pins.p0_11.degrade());

    let mut current_unit = Unit::Celsius;

    loop {
        let temperature: f32 = temp.measure().to_num();
        let converted_temp = current_unit.convert_temperature(&temperature);
        match current_unit {
            Unit::Fahrenheit => defmt::info!("{=f32} °F", converted_temp),
            Unit::Kelvin => defmt::info!("{=f32} K", converted_temp),
            Unit::Celsius => defmt::info!("{=f32} °C", converted_temp),
        }

        if button_1.is_pressed() {
            current_unit = match current_unit {
                Unit::Fahrenheit => Unit::Kelvin,
                Unit::Kelvin => Unit::Celsius,
                Unit::Celsius => Unit::Fahrenheit,
            };
            defmt::info!("Unit changed");
        };

        periodic_timer.delay_ms(100_u32);
    }
}
