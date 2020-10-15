# Internal Temperature Sensor

✅ Preparations: have the board and the timer peripheral initialized in your code.

Before we start to work with an external sensor, where we would have to write a driver, we will access the board's internal temperature sensor first. We'll take a look at the HAL to learn more about how accessing peripherals works in detail and how methods work in Rust.

✅  Open the [nrf-hal-common](https://github.com/nrf-rs/nrf-hal/blob/master/nrf-hal-common/)

✅  Open `/src/temp.rs`, the place where the communication with the boards temperature sensor is implemented. 

The integrated temperature is a struct: `pub struct Temp(TEMP)`. It needs to be public, so it can be called from the outside. `TEMP` is a type definded in the peripheral access crate (pac), it accesses the temperature sensor's register block.  In the `impl` block are all the methods that are defined for `Temp`.

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

✅ Go back to the HAL code. 

`fn measurement()` takes a mutable reference to `self` as an argument. `self` is the instance of the temperature sensor that was created with `fn new()`. The method will stop a measurement, if one has already been started, starts a new measurment and block the program until it has completed the measurement and then returns a fixed point number `I30F2`. The second option is starting a measurement with `fn start_measurement()` and reading the measurement with `fn read()` which works in a non-blocking way. A measurement is started or stopped by writing to the register. 

We'll stick with the blocking method `fn measurement()` for now. 

✅  In your code, add a line that takes a measurement, and one that logs the temperature value. 

```rust
let temperature = temp.measurement();
defmt::info!("{}", temperature);
```
The syntax reflects that methods are attached to objects: The argument `self` refers to the object in front of the dot, and the parenthesis remain empty. 

✅ Run your code.

You notice that the return type of `fn measurement()`, `I30F2` is not in scope, as it it not part of the `core` library. 

✅ Add another method `to_num()` in the measurement line. This method casts the fix point number into an `f32`. In order to be displayable, the type needs to be indicated in the format string. 

```rust
let temperature = temp.measurement().to_num();
defmt::info!("{:f32}", temperature);
```


✅ Initialize a loop that measures and displays the temperature every 60 seconds. 