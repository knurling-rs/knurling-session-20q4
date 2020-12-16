# Bringing it all Together

## Using the LED as Comfort Temperature Indicator

You have learned the following:
* Lighting and wiring RGB LEDs.
* Using a temperature sensor.
* Implementing User Input

Build a program that indicates temperatures around your personal comfort temperature with different light colors. 

An example of this implementation can be found here: [9_comfy_temp_indicator.rs](https://github.com/knurling-rs/knurling-sessions-20q4/blob/main/src/bin/9_comfy_temp_indicator.rs).

What is the temperature where you feel most comfortable?
Define a spectrum spanning 2 Degrees (°C) that you feel most comfortable at. Temperatures up to two degrees above and below that interval are acceptable, temperatures outside this range of six degrees are too hot or too cold. Assign a signal color for each zone. Feel free to adapt the ranges. 

✅ Integrate this behavior of the LED into the last program. 

You have written a lot of code in one file. This makes everything overwhelming and hard to reuse code. Let's refactor by putting code we're likely to reuse in modules.

✅ Inside `src/` create a new folder with the name `dk_button`.

✅ Inside `dk_button` create a file with the name `mod.rs`.

✅ Move the `struct Button` definition and its `impl` block from `src/bin/thermometer` to `dk_button/mod.rs`.

✅ Bring all necessary modules into scope.

✅ Add `pub` in front of every method and every `struct` or `enum` definition, to make them accessible from other files. 

✅ In `scr/lib.rs` add the following line, to export this module:

```rust
pub mod dk_button;
```

✅ In `src/bin/comfy_temp_indicator`, bring the `dk_button` module into scope:

```rust
use knurling_session_20q4::{
    dk_button, 
};
```

✅ Change the line, where the static method for instantiating the button is called, so that the method is called from the `dk_button` module:

```rust
let mut button_1 = dk_button::Button::new(pins.p0_11.degrade());
```

✅ Create a module `rgb_led` for the LED related code and a module `number_representations` for the unit conversions in the same way. It makes sense, that the method for unit conversion is changed to only taking a reference to `temperature`, because ownership is not needed. 
