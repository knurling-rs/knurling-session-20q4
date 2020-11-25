use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};

use embedded_hal::blocking::delay::DelayMs;
pub struct Buzzer(Pin<Output<PushPull>>);

impl Buzzer {
    pub fn init<Mode>(buzzer_pin: Pin<Mode>) -> Self {
        Buzzer(buzzer_pin.into_push_pull_output(Level::Low))
    }

    pub fn high(&mut self) {
        self.0.set_high().unwrap();
    }

    pub fn low(&mut self) {
        self.0.set_low().unwrap();
    }

    pub fn noise(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        for _i in 0..250 {
            self.high();
            timer.delay_ms(10_u32);
            self.low();
            timer.delay_ms(10_u32);
        }
    }
}
