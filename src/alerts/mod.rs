use nrf52840_hal::{
    self as hal,
    prelude::*,
    pac::TIMER0,
    timer::OneShot,
    Timer,
};

use crate::rgb_led::LEDColor;
use crate::buzzer::Buzzer;

const UPPER_LIMIT: f32 = 2000.0;
const WARN_LIMIT: f32 = 1000.0;




pub fn check_levels(co2: &f32, buzzer: &mut Buzzer, led: &mut LEDColor, mut timer: &mut Timer<TIMER0, OneShot>){
    if *co2 < WARN_LIMIT {
        led.green();
    } else if *co2 > WARN_LIMIT && *co2 < UPPER_LIMIT {
        led.yellow();
    } else {
        led.red();
        buzzer.noise(&mut timer)
    } 
}