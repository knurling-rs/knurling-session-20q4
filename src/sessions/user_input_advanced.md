# Adding User Input - Advanced

## Convert Temperature Unit by pushing a Button

The user experience is pretty straight forward: the program does one thing while the button is pressed, and another thing when the button is not pressed. This gets more complicated when pressing a button should only trigger a one-time event like switching the way temperature is displayed. 

An example of this implementation can be found here: [8_temp_unit_convert_buttons.rs](https://github.com/knurling-rs/knurling-sessions-20q4/blob/main/src/bin/8_temp_unit_convert_buttons.rs).

✅  Start with the file from the last chapter. 

✅  Bring the following ressources into scope:

```rust 
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Pin, PullUp},
    prelude::*,
    Temp, 
    Timer,
};
```

We want to be able to switch the unit in which the temperature is displayed, while the temperature is updated regularly. Since some of the programs behavior depends on the current choice of unit, that unit needs to be kept track of.

There are three common ways of displaying Temperature: Celsius, Kelvin and Fahrenheit. They are three variants of the same concept, this calls for the use of an `enum` for this type. 

✅  Add the following `enum` before the `struct Button`.

```rust
enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}
```

The sensor gives out the temperature in degrees Celsius. 

✅  Go to `fn main()`. Before the loop, add a variable that sets the current display unit.

```rust
let mut current_unit = Unit::Celsius;

loop {
    // ...
}

```

We can define methods for an `enum` in the same way we can do that for a `struct`.

✅  Add a method to the `enum Unit`, that contains a match statement. In each of the `match` arms, implement the conversion of the temperature value to the corresponding unit.

```rust
impl Unit {
    fn convert_temperature(&self, temperature: f32) -> f32 {
        match self {
            Unit::Fahrenheit => {
                // convert and return temperature
            },

            Unit::Kelvin => {
                // convert and return temperature
            },

            Unit::Celsius => {
                // return temperature as it is
            },
        };
    }
}
```
Now we need to implement the change of the unit on pressing a button.

✅  Go to fn `main()`. Inside the loop, use a match statement that, depending on the current unit, switches to different one if the button is pressed. Add a log statement, that indicates, that the unit was changed. 

```rust
if button_1.is_pressed() {
    current_unit = match current_unit {
        Unit::Fahrenheit => Unit::Kelvin,
        Unit::Kelvin => Unit::Celsius,
        Unit::Celsius => Unit::Fahrenheit,
    };
    defmt::info!("Unit changed");
};
```

✅  Run the program. Upon pressing the button, you should see continuous log output.

✅  Implement a periodic timer instance. Use this timer instead of the regular one.

```rust
let mut periodic_timer= Timer::periodic(board.TIMER0);
```

✅ Inside the loop, after the temperature is read from the sensor, call the `convert_temperature` method on the `current_unit` and bind to a new variable. This is followed by a match statement, that prints the temperature value with the right unit displayed to the log. 

```rust
loop {
    let temperature: f32 = temp.measure().to_num();
    let converted_temp = current_unit.convert_temperature(temperature);

    match current_unit {
        Unit::Fahrenheit => defmt::info!("{:f32} °F", converted_temp),
        Unit::Kelvin => defmt::info!("{:f32} K", converted_temp),
        Unit::Celsius => defmt::info!("{:f32} °C", converted_temp),
    };
    if button_1.is_pressed() {
        // ...
    };       
}
```

✅  Run the program.

This should lead to many log outputs displaying the temperature in the current unit. Pushing the button once, changes the unit a number of times, so changing it intentionally to a certain unit is impossible.

✅  Add a delay of 100 ms to the end of the loop.

```rust
loop {
    // ... 

    if button_1.is_pressed() {
        // ...
    };
    periodic_timer.delay_ms(100_u32);        
}
```
✅  Run the program.

While the program kind of does what we want, the user experience is quite horrible. Let's improve that. 

An example of this implementation to this point can be found here: [7_temp_convert_button_noisy.rs](https://github.com/knurling-rs/knurling-sessions-20q4/blob/main/src/bin/7_temp_convert_button_noisy.rs).

A first step is to define the behaviour we want to see a bit more detailed. Let's look at three components.

## State of the button out of human perspective
A button can be in four states:

1. It can be pressed
2. It can be not pressed
3. It can be in transition from pressed to not pressed
4. It can be in transition from not pressed to pressed

To define these states a bit more binary, we can look at these states by asking in what position the button was last, and in what position it is now. 

||was|is|
|---|---|---|
|1.|pressed|pressed|
|2.|not pressed|not pressed|
|3.|pressed|not pressed|
|4.|not pressed|pressed|

## State of the button out of machine perspective

While the human perspective seems pretty straight forward, determining what the button states mean in hardware is a bit more complicated. In theory pushing a button causes a signal change, but this change is often not so clean and rather noisy, especially when the button gets older. Compensating for this behavior is called *debouncing* a button. In software, this can be done by having a state machine that keeps track of the 4 states of the button, and by defining that a pushed button counts as a pushed button if it is pushed for a certain amount of time and not because of a sudden signal spike, because a conductive dust spec got in the way. 

## Persistance of system change

We implement buttons, because we want people to be able to interact with a system and change the systems behavior by pushing a button. This change can either be only there while the button is pressed and ended by it's release, or started by pressing a button and persisting despite the button is released.

## What should the program behavior be like?

We want to change the unit in which the temperature is displayed by pressing a button. The change should persist once the button is released. We use one of the button's transition from "being pressed" to "not being pressed" as the triggering event for the unit conversion. To detect the button's transitions, the program keeps track of the past state of the button. The temperature should be displayed every 1000ms.

## Improve Button behaviour

✅  Add another field to the button struct, that keeps track of the button's past state with a `bool`. The initial state is `false`.

Note that the former anonymous struct now has fields. This change needs to be reflected in the methods that are implemented for this struct. 

```rust 
struct Button {
    pin: Pin<Input<PullUp>>,
    was_pressed: bool,
}
```
✅  Add a method to the `impl Button` block that detects a rising edge in the signal by
    * reading the current state of the button
    * comparing the current state with the past state, which is saved in the button struct. 
    * returns `true`, if button was pressed, but currently is not pressed. 
    * updating the past state of the button. 

```rust
fn check_rising_edge(&mut self) -> bool {

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
```

✅  Go to `fn main()`. Declare the button's pin as mutable. Substitute the `is_presses` method with `check_rising_edge()`.

```rust
let mut button_1 = Button::new(pins.p0_11.degrade()); 

loop {
    // ...
    if button_1.check_rising_edge() {
        // ...
    }
    // ...
}
```

✅  Run the program. 

No matter how long you push the button, the unit only changes once. If you don't push the button more than once within 100 ms, every interaction is registered. But our log output is still 10 times more than planned and button timing is not ideal. 


## Timing

In order to detect all human button interactions and register the button's state, the button state needs to be read quite often. To filter out noise from the hardware, reading the button about every 5 ms is enough. We're looking to detect a rising edge, that is long enough to be intentional. Reacting on the rising edge of the button release, after a falling edge of a button press gives even more assurance, that the signal is intentional. 

On a high level the implementation looks like this: A timer counts up until 1000 microseconds. Every time 1000 µs have passed, a counter that keeps track of passed miliseconds is updated. If the number of passed milliseconds is divisible by 5 and a rising edge is detected, the unit is changed. Every time the number of passed milliseconds is divisible by 1000 (one second) the temperature is logged. 

Here, it is relevant, which type of unsigned integer the counter has. If the maximum value of the type is reached, we have a problem. For reference: A counter with u32 would run out after 49.7 days, a counter with u64 would run out after 267844497 years.

✅  After timer instance, add variable that will keep track of passed miliseconds. 

```rust
let mut periodic_timer= Timer::periodic(board.TIMER0);
let mut millis: u64 = 0;
```

✅  Inside the loop, start the timer with a maximum value of 1000 µs. Implement the controll flow for updating the button and logging the temperature. Then add a line, where after each iteration of the loop 1 is added to the counter for passed microseconds. 

```rust
loop {
    periodic_timer.start(1000u32);

    if (millis % 1000) == 0 {
        defmt::info!("Tick (milliseconds): {:u64}", millis);
        // measure temperature
        // display temperature
    };
    if (millis % 5) == 0 {
        // read and update button status
    };

    millis = millis.saturating_add(1);
}
```

✅  Run the code. 

The temperature is still logged way more often then every 1000 ms, because the entire execution of the loop takes under 1000 µs. So the number of passed microseconds is increased before that time has actually passed. In order for the program to have the correct timing, we need to block the execution of the loop until the 1000 µs have passed before increasing the number. 

✅  Go to the `cargo.toml` file. 

✅  Import the crate `nb = "1.0.0"`. 

✅  Go back to your program file and bring that crate and it's block `module` into scope. 

```rust
use nb::block;
```

✅  Before incrementing the number of milliseconds add the following line that will turn the nonblocking counter into a blocking one, until it has counted up to 1000 µs.

```rust
block!(periodic_timer.wait()).unwrap();
```

✅  Run the program. Enjoy pushing buttons!
