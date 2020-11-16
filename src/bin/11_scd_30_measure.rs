#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout
use knurling_session_20q4::{
    scd30,
};

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

    sensor.start_measuring(pressure).unwrap();

    'ready: loop {
        if sensor.data_ready().unwrap() {
            defmt::info!("Data ready.");
            break 'ready
    }

    loop {

        let result = sensor.get_measurement().unwrap();

        let co2 = result.co2;
        let temp = result.temperature;
        let humidity = result.humidity;

        defmt::info!("CO2 {:?} ppm \r\nTemperature {:?} C \r\nHumidity {:?} % \r\n\r\n",
            co2, temp, humidity
        );

        timer.delay_ms(2000_u32);
        led_1.set_high().unwrap();
        timer.delay_ms(2000_u32);
        led_1.set_low().unwrap();
    }
}
