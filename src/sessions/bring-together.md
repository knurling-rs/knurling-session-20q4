# Bringing it all Together
## Using the LED as Temperature Indicator

You have learned the following:
* Lighting and wiring RGB LEDs.
* Using a temperature sensor.
* Implementing User Input

Build a program that indicates temperatures around your personal comfort temperature with different light colors. 

What is the temperature where you feel most comfortable?
Define a spectrum around that temperature spanning 3 Degrees (°C). Temperatures below that interval are too cold, temperatures above are too hot. The middle degree is your comfort zone, around it are acceptable values. Assign a signal color for each zone. Feel free to adapt the ranges. 

✅ Integrate this behavior of the LED into the last program. 

You have written a lot of code in one file. This makes everything overwhelming and hard to reuse code. Let's refactor, by putting code we're likely to reuse in modules.

✅ Inside `scr/` create a new folder with the name `dk_button`.

✅ Inside `dk_button' create a file with the name `mod.rs`.

✅ Move the `struct Button` definition and its `impl` block from `src/bin/thermometer` to `dk_button/mod.rs`.

✅ Bring all necessary modules into scope.

✅ Add `pub` in front of every method and every `struct` or `enum` definition, to make them accessible from other files. 

✅ In `scr/lib.rs` add the following line, to export this module:

```rust
pub mod dk_button;
```

✅ In `src/bin/thermometer`, bring the `dk_button` module into scope:

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
