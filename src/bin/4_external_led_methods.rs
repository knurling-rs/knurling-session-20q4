#![no_main]
#![no_std]

use hal::timer::OneShot;
use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{
        p0::{
            Parts as P0Parts, P0_03, P0_04, P0_28
        },
        Level, Output, PushPull,
    },
    pac::TIMER0,
    Timer,
};

struct LEDColor {
    red: P0_03<Output<PushPull>>,
    green: P0_04<Output<PushPull>>,
    blue: P0_28<Output<PushPull>>,
}

impl LEDColor {
    // static methods:
    // they don't need to be called by an instance.
    // they are used as constructors.
    // they don't have `self` as an argument.

    fn init(pins: P0Parts) -> LEDColor {
        let led_red = pins.p0_03.into_push_pull_output(Level::Low);
        let led_green = pins.p0_04.into_push_pull_output(Level::Low);
        let led_blue = pins.p0_28.into_push_pull_output(Level::Low);

        LEDColor {
            red: led_red,
            green: led_green,
            blue: led_blue,
        }
    }

    // instance methods:
    // they are called by an instance.
    // they have a reference `self` as an argument.

    fn off(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_low().unwrap();
        self.blue.set_low().unwrap();
    }

    fn blue(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_low().unwrap();
        self.blue.set_high().unwrap();
    }

    fn red(&mut self) {
        self.red.set_high().unwrap();
        self.green.set_low().unwrap();
        self.blue.set_low().unwrap();
    }

    fn green(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_low().unwrap();
    }

    fn yellow(&mut self) {
        self.red.set_high().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_low().unwrap();
    }

    fn pink(&mut self) {
        self.red.set_high().unwrap();
        self.green.set_low().unwrap();
        self.blue.set_high().unwrap();
    }

    fn light_blue(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_high().unwrap();
    }

    fn white(&mut self) {
        self.red.set_high().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_high().unwrap();
    }
    // blinks between two colors
    fn blinky(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        self.red.set_high().unwrap();
        self.blue.set_low().unwrap();
        timer.delay_ms(1000_u32);
        self.red.set_low().unwrap();
        self.blue.set_high().unwrap();
        timer.delay_ms(1000_u32);
    }
    // if the method takes ownership of self and allows it to go out of scope, the instance is dropped and can no longer be used.
    fn extinquish(self) {
        let LEDState = self;
        defmt::info!("Destroy LED instance");
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0 pins
    let pins = P0Parts::new(board.P0);

    let mut light = LEDColor::init(pins);
    light.blue();
    timer.delay_ms(1000_u32);

    light.extinquish();
    //light.red();
    timer.delay_ms(1000_u32);

    knurling_session_20q4::exit()
}
