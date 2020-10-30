# Adding User - Input

This section focuses on getting the buttons to work, so you can interact with the hardware!

The buttons on the board are numbered pins, just like the on-board leds. Their pins are `p0.11`, `p0.12`, `p0.24` and `p0.25`. 

✅ Bring the `gpio` module with `p0` parts into scope and add a line to `fn main()` that gives you access to the `p0` pins. 

✅ Build a type and a static method for the buttons. This static method will take pins of any configuration and turn them into a [pull-up input](../glossary.html#pull-up-input). 

```rust
pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }
}
```

✅ Create an instance of a button:

```rust 
let button_1 = Button::new(pins.p0_11.degrade());
```
In order to have an effect, we first need to know the status of the button. Is the button pushed or not? Next, we have to connect an event with the button state. 

✅ Inside the `impl Button` block, implement two instance methods, one that returns true, if the button is pressed:

```rust
pub fn is_pressed(&self) -> bool {
    self.0.is_low().unwrap()
}
```
Note, that `struct Button` does not have any named fields. To access the associated type, index with `0`. 

✅ Inside `fn main()`, implement one of the onboard LEDs.

✅ Continue to write the program, so that the LED is on, when the button is pushed and off, when the button is not pushed. 





