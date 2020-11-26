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

    // high and low methods private, as they have no use outside of this block
    // In this case, we could definately set the pins high and low directly in `fn noise()`
    // But it is a quite simple example, of how abstractions work.

    fn high(&mut self) {
        self.0.set_high().unwrap();
    }

    fn low(&mut self) {
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

    // this function allows you to choose frequency and length of buzz
    pub fn noise_variable(
        &mut self,
        timer: &mut Timer<TIMER0, OneShot>,
        frequency_hz: u32,
        duration_ms: u32,
    ) {
        let delay_ms = frequency_to_delay(&frequency_hz);
        let max_range = duration_to_range(duration_ms, &frequency_hz);

        for _i in 0..max_range {
            self.high();
            timer.delay_ms(delay_ms);
            self.low();
            timer.delay_ms(delay_ms);
        }
    }
}

// helper functions
// they are private because they are only needed here.

fn frequency_to_delay(frequency_hz: &u32) -> u32 {
    let delay_ms = 1000 / frequency_hz;
    delay_ms
}

fn duration_to_range(duration_ms: u32, frequency_hz: &u32) -> i32 {
    let delay = frequency_to_delay(frequency_hz);
    let max_range = duration_ms / (delay * 2_u32);
    max_range as i32
}
