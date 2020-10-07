# Installation Instructions

## Rust and tooling

### Base Rust installation

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

**Windows**: *Do* install the optional components of the [C++ build tools package](https://visualstudio.microsoft.com/visual-cpp-build-tools/). The installation size may take up to 2 GB of disk space.

### `probe-run`

`probe-run` is [a custom Cargo runner that lets you run embedded apps as if they were native apps][probe-run-post]. Install the **git** version of it with the `defmt` Cargo feature enabled:

[`probe-run`]: https://crates.io/crates/probe-run
[probe-run-post]: https://ferrous-systems.com/blog/probe-run/

~~~ console
$ cargo install \
    --git https://github.com/knurling-rs/probe-run \
    --branch main \
    --features defmt
~~~

### `cargo-generate`

`cargo-generate` generates a new Rust project from a predefined template of choice for you. Install it like so:

```console
$ cargo install cargo-generate
```

### `flip-link`

install `flip-link`

```console
cargo install \
    --git https://github.com/knurling-rs/flip-link \
    --branch main
```

### Rust Analyzer

If you use Visual Studio Code, we recommend you install [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) to help you during development.

**Windows**: It's OK to ignore the message about `git` not being installed, if you get one!

## OS specific dependencies

### Linux only: Access USB Devices as non-root User 

Some of our tools depend on `pkg-config` and `libudev.pc`. Ensure you have the proper packages installed; on Debian based distributions you can use:

``` console
$ sudo apt-get install libudev-dev libusb-1.0-0-dev
```

To access the USB devices as a non-root user, follow these steps:

1. Create the following file with the displayed contents. You'll need root permissions to create the file.

``` console
$ cat /etc/udev/rules.d/50-knurling.rules
# udev rules to allow access to USB devices as a non-root user

# nRF52840 Development Kit
ATTRS{idVendor}=="1366", ATTRS{idProduct}=="1015", TAG+="uaccess"
```

2. Run the following command to make the new udev rules effective

``` console
$ sudo udevadm control --reload-rules
```


### Windows only: Zadig JLink driver

On Windows you'll need to associate the nRF52840 Development Kit's USB device to the WinUSB driver.

To do that connect the nRF52840 DK to your PC using micro-USB port J2 (as done before) then download and run the [Zadig] tool.

[Zadig]: https://zadig.akeo.ie/

In Zadig's graphical user interface,

1. Select the 'List all devices' option from the Options menu at the top.

2. From the device (top) drop down menu select "BULK interface (Interface 2)"

3. Once that device is selected, `1366 1015` should be displayed in the USB ID field. That's the Vendor ID - Product ID pair.

4. Select 'WinUSB' as the target driver (right side)

5. Click "Install WinUSB driver". The process may take a few minutes to complete.
