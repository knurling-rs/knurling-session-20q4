#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{
    blocking::delay::DelayMs
};

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Pin, PullUp},
    pac::TIMER0,
    prelude::*,
    Temp, 
    timer::OneShot,
    Timer,
};

enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }

    /// Button is pressed
    pub fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
        
    }

    /// Button is released
    pub fn is_released(&self) -> bool {
        self.0.is_high().unwrap()
    }

    /// what state is the button in? 
    pub fn button_state(&self, timer: &mut Timer<TIMER0, OneShot>, state: Unit) -> Unit {

        if self.is_pressed() == true {
            timer.delay_ms(10_u32);
            if self.is_released() == true {
                defmt::info!("Unit changed");
                match state {
                    Unit::Fahrenheit => {
                        let state = Unit::Kelvin;
                        state
                    },
    
                    Unit::Kelvin => {
                        let state = Unit::Celsius; 
                        state  
                    },
                
                    Unit::Celsius => {
                        let state = Unit::Fahrenheit;
                        state  
                    }
                }
            }
        } 
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
   
    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    let pins = P0Parts::new(board.P0);
    let button_1 = Button::new(pins.p0_11.degrade());

    let mut state = Unit::Celsius;

    loop {

        let temperature: f32 = temp.measure().to_num();
        let state = button_1.button_state(&mut timer, state);
        match state {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                defmt::info!("{:?} °F", temperature);
                timer.delay_ms(100_u32); 
            },

            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                defmt::info!("{:?} K", temperature);
                timer.delay_ms(100_u32);     
            },
        
            Unit::Celsius => {
                defmt::info!("{:?} °C", temperature);
                timer.delay_ms(100_u32);    

            }
        }
    };

    // knurling_session_20q4::exit()
}