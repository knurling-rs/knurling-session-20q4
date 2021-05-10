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
use nrf52840_hal::{self as hal, gpio::p0::Parts as P0Parts, prelude::*, Temp, Timer};

use core::ops::Range;

const FREEZING_TEMPERATURE: f32 = 19.99;
const CRISP_TEMPERATURES: Range<f32> = 20.00..21.99;
const PLEASANTLY_WARM_TEMPERATURES: Range<f32> = 22.0..23.99;
const A_BIT_TOO_STEAMY_TEMPERATURES: Range<f32> = 24.00..25.99;
const BOILING_TEMPERATURE: f32 = 26.00;

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
            defmt::info!("Tick (milliseconds): {=u32}", millis as u32);

            let temperature: f32 = temp.measure().to_num();

            if temperature < FREEZING_TEMPERATURE {
                led_indicator.blue();
            } else if CRISP_TEMPERATURES.contains(&temperature) {
                led_indicator.light_blue();
            } else if PLEASANTLY_WARM_TEMPERATURES.contains(&temperature) {
                led_indicator.green();
            } else if A_BIT_TOO_STEAMY_TEMPERATURES.contains(&temperature) {
                led_indicator.yellow();
            } else if temperature > BOILING_TEMPERATURE {
                led_indicator.red();
            }

            let converted_temp = current_unit.convert_temperature(&temperature);
            match current_unit {
                Unit::Fahrenheit => defmt::info!("{=f32} °F", converted_temp),
                Unit::Kelvin => defmt::info!("{=f32} K", converted_temp),
                Unit::Celsius => defmt::info!("{=f32} °C", converted_temp),
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
        millis = millis.wrapping_add(1);
    }
}
