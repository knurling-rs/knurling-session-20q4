use embedded_hal::blocking::delay::DelayMs;

use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};

// This module is written for common anode rgb leds. For common cathode rgb leds, switch high and low.
pub struct LEDColor {
    r: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
}

impl LEDColor {
    pub fn init<Mode>(led_red: Pin<Mode>, led_blue: Pin<Mode>, led_green: Pin<Mode>) -> Self {
        LEDColor {
            r: led_red.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
        }
    }

    pub fn off(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_high().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_low().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn red(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_high().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn green(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_high().unwrap();
        self.g.set_low().unwrap();
    }

    pub fn yellow(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_high().unwrap();
        self.g.set_low().unwrap();
    }

    pub fn pink(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_low().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn light_blue(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_low().unwrap();
        self.g.set_low().unwrap();
    }

    pub fn white(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_low().unwrap();
        self.g.set_low().unwrap();
    }
    // blinks between two colors
    pub fn blinky(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        self.red();
        timer.delay_ms(1000_u32);
        self.blue();
        timer.delay_ms(1000_u32);
    }
}
