#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

use nb::block;

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
    fn convert_temperature(&self, temperature: f32) -> f32 {
        match self {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                temperature
            }
            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                temperature
            }
            Unit::Celsius => temperature,
        }
    }
}

// Button struct contains the boolean struct field to keep record of button status
pub struct Button {
    pin: Pin<Input<PullUp>>,
    was_pressed: bool,
}

impl Button {
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            was_pressed: false,
        }
    }

    /// Returns true if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    /// what state is the button in?
    // a pressed button only counts as pressed if it is released and not held down
    //
    // This is a very simple state machine with two states.
    //
    // Note: This function should be called periodically
    pub fn check_rising_edge(&mut self) -> bool {
        let mut rising_edge = false;

        let is_pressed = self.is_pressed();
        // Only trigger on "rising edge" of the signal
        // Term: "Edge Triggering"
        if self.was_pressed && !is_pressed {
            // Was pressed, now isn't:
            rising_edge = true;
        }

        self.was_pressed = is_pressed;
        rising_edge
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();

    // Set up our "wall clock" timer, will be used to count milliseconds
    let mut periodic_timer = Timer::new(board.TIMER1).into_periodic();
    let mut millis: u64 = 0;

    // Initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    // Initialize access to pins for the button
    let pins = P0Parts::new(board.P0);
    let mut button_1 = Button::new(pins.p0_11.degrade());

    let mut current_unit = Unit::Celsius;
    // state of the button is read continuoulsly, and new button state is saved, so every input gets noticed,
    // but temp value is only printed if tick number is divisible
    loop {
        // Start by setting/resetting the timer for our next interval
        // Timer counts in microseconds/at 1MHz, we care about milliseconds.
        periodic_timer.start(1000u32);

        // Every 1000ms, print the current temperature reading
        if (millis % 1000) == 0 {
            defmt::info!("Tick (milliseconds): {:u32}", millis as u32);
            let temperature: f32 = temp.measure().to_num();
            let converted_temp = current_unit.convert_temperature(temperature);
            match current_unit {
                Unit::Fahrenheit => defmt::info!("{:f32} °F", converted_temp),
                Unit::Kelvin => defmt::info!("{:f32} K", converted_temp),
                Unit::Celsius => defmt::info!("{:f32} °C", converted_temp),
            };
        }

        // Every 5ms, check the current state of the button
        if (millis % 5) == 0 && button_1.check_rising_edge() {
            current_unit = match current_unit {
                Unit::Fahrenheit => Unit::Kelvin,
                Unit::Kelvin => Unit::Celsius,
                Unit::Celsius => Unit::Fahrenheit,
            };
        };

        // Now wait for the timer to complete
        block!(periodic_timer.wait()).unwrap();

        // Increment our millisecond count
        millis = millis.wrapping_add(1);
    }
}
