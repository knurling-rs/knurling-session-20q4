# Internal Temperature Sensor

An example of this implementation can be found here: [3_temperature.rs](https://github.com/knurling-rs/knurling-session-20q4/blob/main/code/src/bin/3_temperature.rs).

✅ Preparations: have the board and the timer peripheral initialized in your code.

Before we start to work with an external sensor, where we would have to write a driver, we will access the board's internal temperature sensor first. We'll take a look at the HAL to learn more about how accessing peripherals works in detail and how methods work in Rust.

✅  Open the [nrf-hal-common 0.11.1](https://github.com/nrf-rs/nrf-hal/tree/v0.11.1/nrf-hal-common)

✅  Open `/src/temp.rs`, the place where the communication with the boards temperature sensor is implemented. 

The integrated temperature is a struct: `pub struct Temp(TEMP)`. It needs to be public, so it can be called from the outside. `TEMP` is a type defined in the peripheral access crate (pac), it accesses the temperature sensor's register block.  In the `impl` block are all the methods that are defined for `Temp`.

Methods are different from functions in that they are attached to objects. Let's look at them in detail:

`pub fn new()` takes `TEMP` as argument and returns `Temp`. The method takes ownership of the temperature sensor's register block.  


✅  In order to be able to use `Temp` in your code, you have to bring it into scope first. Add the following lines to your code:

```rust
use nrf52840_hal::{
    self as hal,
    Temp,
    Timer,
};
```

✅  Take ownership of the temperature sensor's register block by calling the new method, using `board.TEMP` as argument. The variable needs to be mutable. 

 ```rust
 let mut temp = Temp::new(board.TEMP);
 ```

Now that we have an instance of the temperature sensor, we can take a measurement. 

✅ Go back to [temp.rs](https://github.com/nrf-rs/nrf-hal/blob/v0.11.1/nrf-hal-common/src/temp.rs) in the HAL code. 

`fn measure()` takes a mutable reference to `self` as an argument. `self` is the instance of the temperature sensor that was created with `fn new()`. The method will stop a measurement, if one has already been started, starts a new measurement and block the program until it has completed the measurement and then returns a fixed point number `I30F2`. The second option is starting a measurement with `fn start_measurement()` and reading the measurement with `fn read()` which works in a non-blocking way. A measurement is started or stopped by writing to the register. 

We'll stick with the blocking method `fn measure()` for now. 

✅  In your code, add a line that takes a measurement, and one that logs the temperature value. 

```rust
let temperature = temp.measure();
defmt::info!("{:?}", temperature);
```
The syntax reflects that methods are attached to objects: The argument `&mut self` refers to the object in front of the dot, and the parenthesis remain empty. 

If you run the code now, you'll run into a compiler error, because `the trait defmt::Format is not implemented for I30F2`, the return type of `fn measure()`. 

✅ Add another method `to_num()` behind `fn measure()`. This method casts the fix point number into an `f32`. In order to be displayable, the type needs to be indicated in the format string. 

```rust
let temperature: f32 = temp.measure().to_num();
defmt::info!("{:f32} °C", temperature);
```

✅ Initialize a loop that measures and displays the temperature every 60 seconds. 