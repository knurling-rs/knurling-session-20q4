# More about Methods

In this section, you will write your own methods. 

We assume you used a common anode RGB LED. If you use a common cathode RGB LED, the settings `high` and `low` are the other way round. 

✅ Go back to your code for the external RGB LED. 

Instead of setting the Level for each of the channels individually, we can definde a type that contains all three channels and methods that define the behavior of the RGB LED.

✅ Bring the GPIOS as well as the pin configurations into scope. 

```rust
use nrf52840_hal::{
    self as hal,
    gpio::{
        p0::{Parts as P0Parts, P0_03, P0_04, P0_28},
        Level, Output, PushPull,
    },
    Timer,
};
```
✅ Above `fn main()`, define a struct with three fields, one for each channel. Each channel has it's own type!

```rust
struct LEDState {
    red: P0_03<Output<PushPull>>,
    blue: P0_04<Output<PushPull>>,
    green: P0_28<Output<PushPull>>,
}
```

✅ Under the `struct LEDState`, create an `impl` block for that `struct`.

```rust
impl LEDState {
    // empty
}
```

There are two types of methods: *static methods* and *instance methods*. Static methods are generally used as constructors of an instance. They are called with the :: syntax. Instance methods are called by an object, this is why they have a reference to that object as argument. They are called with the dot syntax.

✅ Inside the `impl` block create a static method that constructs the struct. The first part of the methods configures the pins, the second part creates the struct, which is then returned. 

```rust
fn init(pins: P0Parts) -> LEDState {
        let mut led_red = pins.p0_03.into_push_pull_output(Level::High);
        let mut led_blue = pins.p0_04.into_push_pull_output(Level::High);
        let mut led_green = pins.p0_28.into_push_pull_output(Level::High);

        LEDState {
            red: led_red,
            blue: led_blue,
            green: led_green,
        }
    }
```
✅ Inside `fn main()` substitute the 3 lines that configure the pins with calling this static method. 

```rust
- let mut led_red = pins.p0_03.into_push_pull_output(Level::High);
- let mut led_blue = pins.p0_04.into_push_pull_output(Level::High);
- let mut led_green = pins.p0_28.into_push_pull_output(Level::High);

+ let mut light = LEDState::init(pins);
```
We can now define all sorts of instance methods that control the behaviour of the LED. As an example we will refactor this piece of code that switches the led from red light to blue light with a 1000ms intervall:

```rust
loop {
        
        led_red.set_low().unwrap();
        led_blue.set_high().unwrap();

        timer.delay_ms(1000_u32);

        led_red.set_high().unwrap();
        led_blue.set_low().unwrap();

        timer.delay_ms(1000_u32);
    };
```
✅ Go back to the `impl` block. Define an instance method that sets the red channel low and the others high. 

```rust 
fn red(&mut self) {
        self.red.set_low().unwrap();
        self.green.set_high().unwrap();
        self.blue.set_high().unwrap();
    }
```
The methods takes a mutable reference of the instance of `LEDState` as argument. `&mut self` is short for `self: &mut Self`. The fields of the struct can be accessed with the . syntax.  

✅ Define a method that sets the blue channel high and the others low in the same way. 

✅ Go back to `fn main()` and substitute the lines with the corresponding function call. 
```rust
loop {
 -      led_red.set_low().unwrap();
 -      led_blue.set_high().unwrap();
 +      light.red();

        timer.delay_ms(1000_u32);

 -      led_red.set_high().unwrap();
 -      led_blue.set_low().unwrap();
 +      light.blue();

        timer.delay_ms(1000_u32);
    };
```

✅ Write a method that blinks the LED between the two colors. 
✅ Write a method that consumes the `LEDState` instance, so you can no longer use it. 

