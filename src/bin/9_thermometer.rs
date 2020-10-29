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
    gpio::{
        p0::{
            Parts as P0Parts
        }, 
        Input, Level, Output, PushPull, Pin, PullUp
    },
    pac::TIMER0,
    prelude::*,
    Temp, 
    timer::OneShot,
    Timer,
};

struct LEDColor {
    red: Pin<Output<PushPull>>,
    green: Pin<Output<PushPull>>,
    blue: Pin<Output<PushPull>>,
}

impl LEDColor {

    fn init<Mode>(led_red: Pin<Mode>, led_blue: Pin<Mode>, led_green: Pin<Mode>) -> LEDColor {

        LEDColor {
            red: led_red.into_push_pull_output(Level::Low),
            green: led_green.into_push_pull_output(Level::Low),
            blue: led_blue.into_push_pull_output(Level::Low),
        }
    }

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

    fn light_blue(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_high().unwrap();
    }
}

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

// Button struct contains the unit variant
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

    // a pressed button only counts as pressed if it is released and not held down
    pub fn change_unit_on_interaction(&mut self, timer: &mut Timer<TIMER0, OneShot>) {

        if self.is_pressed() == true {
            // defmt::info!("pressed");
            timer.delay_ms(5_u32);
            if self.is_released() == true {
                defmt::info!("Unit changed");
                match self.unit {
                    Unit::Fahrenheit => self.unit = Unit::Kelvin,
                    Unit::Kelvin => self.unit  = Unit::Celsius,
                    Unit::Celsius => self.unit  = Unit::Fahrenheit   
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

    // Initialize access to pins for the button and led
    let pins = P0Parts::new(board.P0);
    let mut button_1 = Button::new(pins.p0_11.degrade());

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut led_indicator = LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    // state of the button is read and updated continuoulsly
    // but temp value is only printed if tick number is divisible
    loop {

        let tick = periodic_timer.read();
        
        let temperature: f32 = temp.measure().to_num();

        if temperature < 23.4_f32 {
            led_indicator.blue();

        } else if temperature > 23.04_f32 && temperature < 24.4_f32 {
            led_indicator.light_blue();

        } else if temperature > 24.04_f32 && temperature < 25.5_f32 {
            led_indicator.green();

        } else if temperature > 25.05_f32 && temperature < 26.5_f32 {
            led_indicator.yellow();

        } else if temperature > 26.5_f32 { 
            led_indicator.red();
        }

        button_1.change_unit_on_interaction(&mut delay_timer);
        if tick % 100000_u32 == 0 {
            defmt::info!("Tick: {:u32}", tick);
            button_1.unit.convert_unit_and_display(temperature);
        }
    };

    // knurling_session_20q4::exit()
}