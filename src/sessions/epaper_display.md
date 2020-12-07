# Hello, e-Paper display!

Before adding the e-Paper display to the sensor project, we'll try it out on it's own. Start with a new file based on `hello_extended.rs`.

# Wiring
Remove all jumper-wires from the breadboard. 

Connect the following wires to the respective pins.

|Name|Color|Pin|
|-------------|-----------------|--------------------|
|vcc|red|vdd|
|gnd|black|gnd|
|din|blue|p1.01|
|clk|yellow|p1.02|
|cs|orange|p1.03|
|dc|green|p1.04|
|rst|white|p1.05|
|busy|purple|p1.06|

# Code

## Cargo.toml

Add the following dependencies to the `cargo.toml`.

```rust
epd-waveshare = "0.4.0"
embedded-graphics = "0.6.2"
```

## Instantiate SPIM

Empty template

Bring the `spim` and the `p1` module into scope.

```rust
// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts, Level},
    prelude::*,
    spim::{self, Spim},
    Timer,
};
```

Configure the pins as follows:

```rust
let din = pins_1.p1_01.into_push_pull_output(Level::Low).degrade();
let clk = pins_1.p1_02.into_push_pull_output(Level::Low).degrade();
let cs = pins_1.p1_03.into_push_pull_output(Level::Low);
let dc = pins_1.p1_04.into_push_pull_output(Level::Low);
let rst = pins_1.p1_05.into_push_pull_output(Level::Low);
let busy = pins_1.p1_06.into_floating_input();
```

`din` is the data line, `clk` the clock. Both need to be floating. 
`cs` is short for chip select, `dc` for data/command control pin and `rst` for reset. They all have to be push-pull-outputs, with initial low level. 
`busy` is configured as input. This is the channel, where the display can communicate if it is busy or not. 

The SPI protocol works similar to I2C as in that it has a clock but data to and from the peripheral device use two different channels, `MISO` and `MOSI`. We only use the `MOSI` line, as data is only sent to the display, and not from the display. 

Configure the SPIM Pins and create a new instance of the SPIM peripheral. 

```rust
let spi_pins = spim::Pins {
        sck: clk,
        miso: None,
        mosi: Some(din),
    };
 

let mut spi = Spim::new(board.SPIM3, spi_pins, spim::Frequency::K500, spim::MODE_0, 0);
```

Run the program to make sure it builds. The display should not do anything at this point.


## Instantiate the ePaper Display

Bring the following modules into scope:

```rust
use epd_waveshare::{
    epd4in2::*,
    graphics::Display,
    prelude::*,
};
```

Add an instance of the timer. 
Create a new instance of the 4.2 inch E Paper display. 
Add a default display.

```rust
// instantiate ePaper
let mut delay = Timer::new(board.TIMER1);
let mut epd4in2 = EPD4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

let mut display = Display4in2::default();
```
Run your program, to make sure it builds. At this point the e-paper display switches to dark and back to light a few times. 

## Drawing on the ePaper Display

Bring the following modules into scope:

```rust
use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{ Circle, Triangle },
    style::PrimitiveStyle, 
};
```
The waveshare e-Paper display has a binary color system: Color is either `ON`, or `OFF`. `ON` means black and `OFF` means white. 

<!-- TODO [Image of display and drawing with coordinates.] -->

One way to draw on the display is using primitive shapes. The crate offers circle, triangle, rectangle and line. Each one is defined by significant points and distances, the circle for example is defined by it's center and it's radius. 

Each shape can be filled solid or just be depicted by a stroke around it's edges. Each time content for the display is defined, it needs to be added to the display buffer, this is done with the `draw` method.

Add the following definitions of shapes, two circles and a triangle, to your program.


```rust
let c1 = Circle::new(Point::new(171, 110), 30)
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    .draw(&mut display);

let c2 = Circle::new(Point::new(229, 110), 30)
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    .draw(&mut display);

let t1 = Triangle::new(Point::new(259, 120), Point::new(141, 120), Point::new(200, 200))
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    .draw(&mut display);
```

Using the `draw()` method is not enough to actually display something on the screen. For the shapes to show up on the display, the frame needs to be updated via the `spi` connection, and the frame needs to be displayed. 

Add the following lines to your code. 

```rust
    epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
    epd4in2.display_frame(&mut spi)
        .expect("display frame new graphics");
```

Run your code. You should see a symbol constructed of two circles and a triangle.
