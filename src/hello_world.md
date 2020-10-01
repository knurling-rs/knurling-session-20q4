# Hello World

This is a step by step guide expand the `hello.rs` example in the [`knurling app template`](https://github.com/knurling-rs/app-template) from just logging `hello world` to blinking an onboard LED on the nrf52840-DK. This guide expects that you have generated the template according to [our guide](https://ferrous-systems.com/blog/defmt/).

<!-- 1. Set up with cargo generate according to defmt_setup. 
2. TODO1 : Insert your name if it was not done automatically.
3. TODO 2: Replace placeholder for chip so that the line looks like this:
```toml
    runner = "probe-run --chip nRF52840_xxAA --defmt" 
```
4. TODO3: Set target to target = `thumbv7em-none-eabihf`
5. TODO4: Add the following dependencies in `cargo.toml`:
```toml
nrf52840-hal = "0.11.0"
embedded-hal = "0.2.4"
```
6. TODO5: In `lib.rs` replace `some-hal` with `nrf52840-hal`. -->

## Getting Access to Resources

1. In your generated app-template folder, go to `src/bin/hello.rs`.
2. Bring the following modules into scope:

```rust
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    Timer,
};
```
The `nrf52840_hal` crate is a Hardware Abstraction Layer (HAL), which helps us access the board's resources, e.g. GPIO pins or timers. 
If you use a different microcontroller, you need to be able to gain access to a TIMER peripheral, pins for the onboard LEDs and Level of the pins.

3. Gain access to all the peripherals of the board by adding the following line in `fn main()`:

```rust
let board = hal::pac::Peripherals::take().unwrap();
```

If you use a different board, check the crate's docs on how to get access to all peripherals.

4. You need a timer to blink LEDs, as the LED is on and off for certain amounts of time. To access the timer peripheral add this line:

```rust
let mut timer = Timer::new(board.TIMER0);
```

5. If we want to use the onboard LEDs, we need to find out how to access them. Check the datasheet of your board to find out which GPIO pins they are connected to. For the nrf52840-DK you'll find the information [here](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fhw_buttons_leds.html).

The onboard LEDs are part of the P0 Pins. LED1 is p0.13. To gain access this group of pins add this line:

```rust
let pins = P0Parts::new(board.P0);
```

## Switching the Light on
1. Configure pin p0.13 into a push-pull-output with High Level:

```rust
let mut led_1 = pins.p0_13.into_push_pull_output(Level::High);
```
2. This HAL provides functionalities for the different resources, independent of board model. In our case we need to be able to set a pin high or low, or set delays for the Timer. Add the following dependency:

```rust
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::OutputPin,
};
```


3. Add a delay of 1000 miliseconds:

```rust
timer.delay_ms(1000u32);
```
4. Run the program!

LED1 on your microcontroller should light up for a second. Then the program ends. 

## Blinking the LED

1. Open a loop:

```rust
loop {

};
```
2. Inside the loop add the following lines:

```rust
    led_1.set_low().unwrap();
    timer.delay_ms(1000u32);
    led_1.set_high().unwrap();
    timer.delay_ms(1000u32);
```

3. Run the program.

LED1 should blink continuously. 
