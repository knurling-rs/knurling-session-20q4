use nrf52840_hal::{pac::TIMER0, timer::OneShot, Timer};

use crate::buzzer::Buzzer;
use crate::rgb_led::LEDColor;

const UPPER_LIMIT: f32 = 2000.0;
const WARN_LIMIT: f32 = 1000.0;

pub fn check_levels(
    co2: &f32,
    buzzer: &mut Buzzer,
    led: &mut LEDColor,
    mut timer: &mut Timer<TIMER0, OneShot>,
) {
    if *co2 < WARN_LIMIT {
        led.green();
    } else if *co2 > WARN_LIMIT && *co2 < UPPER_LIMIT {
        led.yellow();
    } else {
        led.red();
        buzzer.noise(&mut timer)
    }
}
