
# defmt Setup

We have created a [Cargo project template] to make the setup easier.
Here are the setup steps.

[Cargo project template]: https://github.com/knurling-rs/app-template

0. Install version v0.1.4 (or newer) of `probe-run`, [a custom Cargo runner that lets you run embedded apps if they were native apps][probe-run-post].

[`probe-run`]: https://crates.io/crates/probe-run
[probe-run-post]: https://ferrous-systems.com/blog/probe-run/

~~~ console
$ cargo install probe-run
~~~

1. Either initialize the project template with [`cargo-generate`] or [fetch a copy] and initialize the `Cargo.toml` manually.

[`cargo-generate`]: https://crates.io/crates/cargo-generate
[fetch a copy]: https://github.com/knurling-rs/app-template/archive/master.zip

~~~ console
$ cargo generate \
    --git https://github.com/knurling-rs/app-template \
    --branch main \
    --name my-app
~~~

If you are not using `cargo-generate` you'll need to manually enter the `author` and `name` fields in `Cargo.toml`.

~~~ diff
 # Cargo.toml
 [package]
-# authors = ["{{authors}}"]
-# name = "{{project-name}}"
+name = "my-app"
~~~

After that there are a few TODOs in the template.
You can [`ripgrep`] for the word TODO (`rg TODO .cargo .`) to find them but we'll walk you through all of them in this blog post.

[`ripgrep`]: https://github.com/BurntSushi/ripgrep

2. Pick a chip from `probe-run --list-chips` and enter it into `.cargo/config.toml`.

If, for example, you have a nRF52840 Development Kit from one of [our workshops] then replace `{{chip}}` with `nRF52840_xxAA`.

[our workshops]: https://github.com/ferrous-systems/embedded-trainings-2020

~~~ diff
 # .cargo/config.toml
 [target.'cfg(all(target_arch = "arm", target_os = "none"))']
-runner = "probe-run --chip {{chip}} --defmt"
+runner = "probe-run --chip nRF52840_xxAA --defmt"
~~~

3. Adjust the compilation target in `.cargo/config.toml`.

For the nRF52840 chip you'll want to use the `thumbv7em-none-eabihf` target.

~~~ diff
 # .cargo/config.toml
 [build]
-target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
~~~

If you haven't done so already, install the `rust-std` component for the target above:
~~~ console
$ rustup target add thumbv7em-none-eabihf
~~~

4. Add a HAL as a dependency.

For the nRF52840 you'll want to use the [`nrf52840-hal`].

[`nrf52840-hal`]: https://crates.io/crates/nrf52840-hal

~~~ diff
 # Cargo.toml
 [dependencies]
-# some-hal = "1.2.3"
+nrf52840-hal = "0.11.0"
~~~

5. Now that you have selected a HAL fix the HAL import in `src/lib.rs`

~~~ diff
 // my-app/src/lib.rs
-// use some_hal as _; // memory layout
+use nrf52840_hal as _; // memory layout
~~~

# Hello `defmt`

You are now all set to `cargo-run` your first `defmt`-powered application!
There are some examples in the `src/bin` directory.

~~~ rust
// my-app/src/bin/hello.rs

#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    my_app::exit()
}
~~~

`cargo run`-ning this program produces the classic "Hello, world!" output.

~~~ console
$ # `rb` is an alias for `run --bin`
$ cargo rb hello
    Finished dev [optimized + debuginfo] target(s) in 0.03s
flashing program ..
DONE
resetting device
0.000000 INFO Hello, world!
(..)

$ echo $?
0
~~~

Or if you are using VS code + Rust-Analyzer, instead, you can open the `src/bin/hello.rs` file and [click the "Run" button as we demonstrated in a previous blog post][click-run-post].

[click-run-post]: https://ferrous-systems.com/blog/run-rust-on-your-embedded-device-from-vscode/


For more details check out [the `defmt` book][book].

[book]: https://defmt.ferrous-systems.com/
