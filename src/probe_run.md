## Probe-run 

The [rust-analyzer](https://rust-analyzer.github.io) plugin for VSCode provides you with a helpful little  `▶ Run` button above every test or `main()` function that lets you execute your code right out of the editor.
Unfortunately though, this won't work out of the box in embedded projects, since rust-analyzer will invoke `cargo run` when you click on `▶ Run`, and cargo itself does not know how to flash and run applications on embedded targets.

However, Rust-Analyzer *does* seamlessly integrate with [probe-run, a custom cargo runner for embedded development](https://ferrous-systems.com/blog/probe-run/):
Since `probe-run` is a cargo runner and not a cargo subcommand, all we have to do is modify our settings to use `probe-run` instead whenever `cargo run` is called. After that, we can use the `▶ Run` button like we would in a native project.

To show how to set this up, let's configure a project based on `cortex-m-quickstart`, namely our [beginner embedded training](https://github.com/ferrous-systems/embedded-trainings-2020/tree/main/beginner/apps) code examples.

First, make sure you've installed [Rust-Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) and `probe-run`:

~~~ console
$ cargo install probe-run
~~~

Then, we need to find out if our chip is supported. This also tells us which variant name to use in our configuration later.

~~~ console
$ probe-run --list-chips
(..)
        STM32F107VB
        STM32F107VC
nrf52 series
    Variants:
        nRF52810_xxAA
        nRF52811_xxAA
        nRF52832_xxAA
        nRF52832_xxAB
        nRF52840_xxAA
nrf51 series
(..)
~~~

Since we want to build our examples for the nRF52840 Development Kit, our variant of choice is `nRF52840_xxAA`.

Now, in the `.cargo/config` or `.cargo/config.toml` file of our project, we can set `probe-run` as the default `runner` to be used when running executables built for our board.

~~~ toml
[target.thumbv7em-none-eabi]
runner = "probe-run --chip nRF52840_xxAA"
#         ^^^^^^^^^        ^^^^^^^^^^^^^

[build]
target = "thumbv7em-none-eabi" # = ARM Cortex-M4
~~~

And that's it! Now you can use the Run button as usual.



