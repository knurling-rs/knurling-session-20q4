#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

use nb::block;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Pin, PullUp},
    prelude::*,
    Temp,
    Timer,
};

enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl Unit {
    fn convert_unit_and_display (&self, temperature: f32) {
        match self {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                defmt::info!("{:?} °F", temperature);
            },

            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                defmt::info!("{:?} K", temperature);
            },

            Unit::Celsius => {
                defmt::info!("{:?} °C", temperature);
            }
        }
    }
}

// Button struct contains the unit variant to keep record of button status
pub struct Button{
    pin: Pin<Input<PullUp>>,
    unit: Unit,
    was_pressed: bool,
}

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            unit: Unit::Celsius,
            was_pressed: false,
        }
    }

    /// Button is pressed
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()

    }

    /// Button is released
    pub fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }

    /// what state is the button in?
    // a pressed button only counts as pressed if it is released and not held down
    //
    // This is a very simple state machine with two states.
    //
    // Note: This function should be called periodically
    pub fn check_falling_edge(&mut self) {

        let is_pressed = self.is_pressed();

        // Only trigger on "falling edge" of the signal
        // Term: "Edge Triggering"
        if self.was_pressed && !is_pressed {
            // Was pressed, now isn't:
            defmt::info!("Unit changed");
            match self.unit {
                Unit::Fahrenheit => {
                    self.unit = Unit::Kelvin
                },

                Unit::Kelvin => {
                    self.unit  = Unit::Celsius
                },

                Unit::Celsius => {
                    self.unit  = Unit::Fahrenheit
                }
            }
        }

        // NOTE FOR TANKS, REMOVE BEFORE MERGE
        // match (self.was_pressed, is_pressed) {
        //     (false, false) => {
        //         // Wasn't pressed, still isn't:
        //         // Nothing to do.
        //     }
        //     (false, true) => {
        //         // Wasn't pressed, but now is:
        //         // Take note of that, but no further action
        //     }
        //     (true, false) => {
        //         // Was pressed, now isn't:
        //         defmt::info!("Unit changed");
        //         match self.unit {
        //             Unit::Fahrenheit => {
        //                 self.unit = Unit::Kelvin
        //             },

        //             Unit::Kelvin => {
        //                 self.unit  = Unit::Celsius
        //             },

        //             Unit::Celsius => {
        //                 self.unit  = Unit::Fahrenheit
        //             }
        //         }
        //     }
        //     (true, true) => {
        //         // Was pressed, still is:
        //         // Nothing to do.
        //     }
        // }

        self.was_pressed = is_pressed;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();

    // Set up our "wall clock" timer, will be used to count milliseconds
    let mut periodic_timer = Timer::new(board.TIMER1);
    let mut millis: u64 = 0;

    // Initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    // Initialize access to pins for the button
    let pins = P0Parts::new(board.P0);
    let mut button_1 = Button::new(pins.p0_11.degrade());

    // state of the button is read continuoulsly, and new button state is saved, so every input gets noticed,
    // but temp value is only printed if tick number is divisible
    loop {

        // Start by setting/resetting the timer for our next interval
        // Timer counts in microseconds/at 1MHz, we care about milliseconds.
        periodic_timer.start(1000u32);

        // Every 250ms, print the current temperature reading
        if (millis % 250) == 0 {
            defmt::info!("Tick (milliseconds): {:u32}", millis as u32);
            let temperature: f32 = temp.measure().to_num();
            button_1.unit.convert_unit_and_display(temperature);
        }

        // Every 5ms, check the current state of the button
        if (millis % 5) == 0 {
            button_1.check_falling_edge();
        }

        // Now wait for the timer to complete
        block!(periodic_timer.wait()).unwrap();

        // Increment our millisecond count
        millis = millis.saturating_add(1);
    }
}
