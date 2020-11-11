use nrf52840_hal::{
    gpio::{Input, Pin, PullUp},
    prelude::InputPin,
};

// Button struct contains the unit variant to keep record of button status
pub struct Button {
    pin: Pin<Input<PullUp>>,
    was_pressed: bool,
}

impl Button {
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            was_pressed: false,
        }
    }

    /// Button is pressed
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    /// what state is the button in?
    // a pressed button only counts as pressed if it is released and not held down
    //
    // This is a very simple state machine with two states.
    //
    // Note: This function should be called periodically
    pub fn check_rising_edge(&mut self) -> bool {
        let mut rising_edge = false;

        let is_pressed = self.is_pressed();
        // Only trigger on "rising edge" of the signal
        // Term: "Edge Triggering"
        if self.was_pressed && !is_pressed {
            // Was pressed, now isn't:
            rising_edge = true;
        }

        self.was_pressed = is_pressed;
        rising_edge
    }
}
