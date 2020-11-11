#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout
use knurling_session_20q4::{
    dk_button,
    number_representation::{self, Unit},
    rgb_led,
};

use nb::block;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts},
    prelude::*,
    Temp, Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();

    // one for continuous counting
    let mut periodic_timer = Timer::periodic(board.TIMER0);
    let mut millis: u64 = 0;

    // Initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    // Initialize access to pins for the button and led
    let pins = P0Parts::new(board.P0);
    let mut button_1 = dk_button::Button::new(pins.p0_11.degrade());

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut led_indicator =
        rgb_led::LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    // state of the button is read and updated continuoulsly
    // but temp value is only printed if tick number is divisible

    let mut current_unit = number_representation::Unit::Celsius;

    loop {
        // Start by setting/resetting the timer for next interval
        // Timer counts in microseconds/at 1MHz, we care about milliseconds.
        periodic_timer.start(1000u32);

        // Every 1000ms:
        // read temperature
        // light led in appropriate color
        // print the current temperature reading

        if (millis % 1000) == 0 {
            defmt::info!("Tick (milliseconds): {:u32}", millis as u32);

            let temperature: f32 = temp.measure().to_num();

            if temperature < 23.4_f32 {
                led_indicator.blue();
            } else if temperature > 23.04_f32 && temperature < 24.4_f32 {
                led_indicator.light_blue();
            } else if temperature > 24.04_f32 && temperature < 25.5_f32 {
                led_indicator.green();
            } else if temperature > 25.05_f32 && temperature < 26.5_f32 {
                led_indicator.yellow();
            } else if temperature > 26.5_f32 {
                led_indicator.red();
            }

            let converted_temp = current_unit.convert_temperature(&temperature);
            match current_unit {
                Unit::Fahrenheit => defmt::info!("{:f32} °F", converted_temp),
                Unit::Kelvin => defmt::info!("{:f32} K", converted_temp),
                Unit::Celsius => defmt::info!("{:f32} °C", converted_temp),
            };
        };

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
        millis = millis.saturating_add(1);
    }
}
