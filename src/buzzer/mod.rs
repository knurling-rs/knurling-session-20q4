use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level, Pin, Output, PushPull},
    prelude::*,
    pac::TIMER0,
    timer::OneShot,
    Timer,
};

use embedded_hal::blocking::delay::DelayMs;
pub struct Buzzer(Pin<Output<PushPull>>);


impl Buzzer {
    pub fn init<Mode>(buzzer_pin: Pin<Mode>) -> Self {
        Buzzer (buzzer_pin.into_push_pull_output(Level::Low))
    }

    pub fn on(&mut self) {
        self.0.set_high().unwrap();
    }

    pub fn off(&mut self) {
        self.0.set_low().unwrap();
    }

    pub fn noise(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        
        for _i in 0..250 {
            self.on();
            timer.delay_ms(1_u32);
            self.off();
            timer.delay_ms(1_u32);
        }    
    }
}