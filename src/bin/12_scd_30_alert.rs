#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout
use knurling_session_20q4::{alerts, buzzer, rgb_led, scd30};

use embedded_hal::blocking::delay::DelayMs;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    twim::{self, Twim},
    Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);

    let pins = P0Parts::new(board.P0);
    // onboard led
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);

    // external led
    let led_channel_red = pins.p0_03.degrade();
    let led_channel_blue = pins.p0_04.degrade();
    let led_channel_green = pins.p0_28.degrade();

    let mut led_indicator =
        rgb_led::LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    // buzzer pin
    let buzzer_pin = pins.p0_29.degrade();
    let mut buzzer = buzzer::Buzzer::init(buzzer_pin);

    buzzer.noise(&mut timer);

    // instanciate I2C
    let scl = pins.p0_30.degrade();
    let sda = pins.p0_31.degrade();

    let pins = twim::Pins { scl, sda };
    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    // set ambient air pressure:
    let pressure = 1020_u16;

    let mut sensor = scd30::SCD30::init(i2c);

    let firmware_version = sensor.get_firmware_version().unwrap();
    defmt::info!(
        "Firmware Version: {:u8}.{:u8}",
        firmware_version[0],
        firmware_version[1]
    );

    sensor.start_continuous_measurement(pressure).unwrap();

    loop {
        if sensor.data_ready().unwrap() {
            defmt::info!("Data ready.");
            // green light for 2000ms to indicate data is ready
            led_indicator.green();
            timer.delay_ms(2000_u32);
            led_indicator.off();
            break;
        } else {
            // blinks red as long as data is not ready
           led_indicator.blink_red(&mut timer);
        }
    }

    loop {
        let result = sensor.read_measurement().unwrap();

        let co2 = result.co2;
        let temp = result.temperature;
        let humidity = result.humidity;

        alerts::check_levels(&co2, &mut buzzer, &mut led_indicator, &mut timer);

        defmt::info!("
            CO2 {:f32} ppm
            Temperature {:f32} °C
            Humidity {:f32} %
          ",
            co2,
            temp,
            humidity
        );

        // blink onboard LED with 2000ms delay as visual signal, that program is running
        // delay leads to new measurment every 4 sec. 
        // length of interval is arbitrary
        timer.delay_ms(2000_u32);
        led_1.set_high().unwrap();
        timer.delay_ms(2000_u32);
        led_1.set_low().unwrap();
    }
}
