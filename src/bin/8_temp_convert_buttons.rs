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

impl Unit {
    fn convert_unit_and_display (&self, temperature: f32) {
        match self {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                defmt::info!("{:?} °F", temperature);
            },

            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                defmt::info!("{:?} K", temperature);  
            },
        
            Unit::Celsius => {
                defmt::info!("{:?} °C", temperature);
            }
        }
    }
}

// Button struct contains the unit variant to keep record of button status
pub struct Button{
    pin: Pin<Input<PullUp>>,
    unit: Unit,
}

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            unit: Unit::Celsius,
        }
    }

    /// Button is pressed
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
        
    }

    /// Button is released
    pub fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }

    /// what state is the button in? 
    // a pressed button only counts as pressed if it is released and not held down
    pub fn change_unit_on_interaction(&mut self, timer: &mut Timer<TIMER0, OneShot>) {

        if self.is_pressed() == true {
            // defmt::info!("pressed");
            timer.delay_ms(5_u32);
            if self.is_released() == true {
                defmt::info!("Unit changed");
                match self.unit {
                    Unit::Fahrenheit => {
                        self.unit = Unit::Kelvin
                    },
    
                    Unit::Kelvin => {
                        self.unit  = Unit::Celsius   
                    },
                
                    Unit::Celsius => {
                        self.unit  = Unit::Fahrenheit   
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
    
    // Initialize two timer resources:
    // one for delay
    let mut delay_timer = Timer::new(board.TIMER0);

     // one for continuous counting
    let mut periodic_timer= Timer::periodic(board.TIMER1);
    periodic_timer.start(10000000_u32);
   
    // Initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    // Initialize access to pins for the button
    let pins = P0Parts::new(board.P0);
    let mut button_1 = Button::new(pins.p0_11.degrade());

    // state of the button is read continuoulsly, and new button state is saved, so every input gets noticed, 
    // but temp value is only printed if tick number is divisible
    loop {

        let tick = periodic_timer.read();
        
        let temperature: f32 = temp.measure().to_num();
        button_1.change_unit_on_interaction(&mut delay_timer);
        if tick % 100000_u32 == 0 {
            defmt::info!("Tick: {:u32}", tick);
            button_1.unit.convert_unit_and_display(temperature);
        }
    };

    // knurling_session_20q4::exit()
}