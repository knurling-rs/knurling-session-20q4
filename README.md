### User Survey

> We love feedback! Please give us < 5 minutes of your time so we can get to know our users better.
> 
> Help us shape knurling by filling out [this short survey](https://forms.office.com/r/aMfHG79N9K).


# Knurling Session 2020 Q4

This repo contains the source for the first knurling session as well as the example code.

## About
This course will cover building a CO2 measuring device. We will show how to start an embedded program from scratch, how to write a driver for the sensor, and how to read the sensor output in a terminal. We will build a visual indicator for air quality and an acoustic alarm. In the last part, we will add an ePaper display to the setup and talk bit about the embedded-graphics crate.

## Read it
You can access a rendered version at
> [**session20q4.ferrous-systems.com**](https://session20q4.ferrous-systems.com)

To build from source just run *(assuming you have `mdbook` [installed])*:
``` console
$ mdbook serve
```

## Example code
Example code can be found in [`code/`](./code/).

## Contributing
Please open an issue if you get stuck or find anything to improve this book!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as below, without any additional terms or conditions.

## Support

The knurling sessions is part of the [Knurling] project, [Ferrous Systems]' effort at
improving tooling used to develop for embedded systems.

If you think that our work is useful, consider sponsoring it via [GitHub
Sponsors].

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Knurling]: https://knurling.ferrous-systems.com/
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
[installed]: https://github.com/rust-lang/mdBook#installation
