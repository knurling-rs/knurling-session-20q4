# Panic!

There are two types of Errors, recoverable Errors and non-recoverable ones. Non-recoverable errors are the ones that bring the system into a state where it can't operate further. This is the one where we want to focus on today. 

For simulating an occurring error, we'll put an error in the code (and remove it later).


✅ Go to `src/scd30/mod.rs`. Change `DEFAULT_ADDRESS` from `0x61` to `0x62`.

```rust
pub const DEFAULT_ADDRESS: u8 = 0x62;
```

This change prevents communication between the sensor and the development board. 

The first time, the development board tries to communicate with the sensor is when it reads the sensor's firmware number. Let's look at this method. 

```rust
pub fn get_firmware_version(&mut self) -> Result<[u8; 2], Error> {
        let command: [u8; 2] = [0xd1, 0x00];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let major = u8::from_be(rd_buffer[0]);
        let minor = u8::from_be(rd_buffer[1]);

        Ok([major, minor])
    }
```

The method returns the `Result` type. Depending on the outcome of the operation, `Result` contains different values: If the operation was successful, it contains a `[u8; 2]` array, the firmware number of the sensor. If the operation was not successful, error from the `write()` or the `read()` method is propagated and returned. 

In the program we call the method and add `unwrap()`. 

```rust 
let firmware_version_result = sensor.get_firmware_version().unwrap();
```

Using unwrap returns the value in case of a success and panics the program in case of an error. Using unwrap is only useful, when the occurrence of an error is not to be expected or if the error is non-recoverable anyways. Even if both of these are true for our case, handling panics manually has some benefits. Let's look at some of them.

✅ Run your program. 

You should see something like this: 

```shell
0.000000 ERROR panicked at 'called `Result::unwrap()` on an `Err` value: AddressNack', src/bin/13_scd_30_error_handling.rs:61:28
└─ panic_probe::print_defmt::print @ /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:140
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
      <exception entry>
   1: __udf
   2: cortex_m::asm::udf
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.3/src/asm.rs:105
   3: rust_begin_unwind
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:75
   4: core::panicking::panic_fmt
        at /rustc/5c1f21c3b82297671ad3ae1e8c942d2ca92e84f2/src/libcore/panicking.rs:101
   5: core::option::expect_none_failed
        at /rustc/5c1f21c3b82297671ad3ae1e8c942d2ca92e84f2/src/libcore/option.rs:1272
   6: _13_scd_30_error_handling::__cortex_m_rt_main
   7: main
        at src/bin/13_scd_30_error_handling.rs:19
   8: ResetTrampoline
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
   9: Reset
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
The terminal process "/bin/bash '-c', 'cargo run --package knurling-session-20q4 --bin 13_scd_30_error_handling'" terminated with exit code: 134.

Terminal will be reused by tasks, press any key to close it.
```

In the first line, we are notified, that a panic occurred when `unwrap()` was called, and at what line it occurred. Reading through the rest of the message does not reveal more information, except at `7:` we can see that this happened inside `main()`.

✅ Substitute the following line

```rust
let firmware_version = sensor.get_firmware_version().unwrap();
```

with the following block of code:

```rust
let firmware_version_result = sensor.get_firmware_version();

let firmware_version = match firmware_version_result {
    Ok(version_number) => version_number,

    Err(error) => {
        panic!("Error getting firmware version: {:?}", error)
    }
};
```

Instead of calling `unwrap()`, we handle `Result` with `match`. In case of an error, now we get to decide what happens. We can still invoke a panic. 

✅ Run the program. 

You should see something like this:

```shell
0.000000 ERROR panicked at 'Error getting firmware version: AddressNack', src/bin/13_scd_30_error_handling.rs:67:13
└─ panic_probe::print_defmt::print @ /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:140
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
      <exception entry>
   1: __udf
   2: cortex_m::asm::udf
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.3/src/asm.rs:105
   3: rust_begin_unwind
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:75
   4: core::panicking::panic_fmt
        at /rustc/5c1f21c3b82297671ad3ae1e8c942d2ca92e84f2/src/libcore/panicking.rs:101
   5: _13_scd_30_error_handling::__cortex_m_rt_main
        at src/bin/13_scd_30_error_handling.rs:67
   6: main
        at src/bin/13_scd_30_error_handling.rs:19
   7: ResetTrampoline
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
   8: Reset
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
The terminal process "/bin/bash '-c', 'cargo run --package knurling-session-20q4 --bin 13_scd_30_error_handling'" terminated with exit code: 134.

Terminal will be reused by tasks, press any key to close it.
```

By adding our part to the error message, we let a future user know what failed additionally to why the program failed, without them having to check the code lines. While the custom error message is nice, the location of the the error occurrence is not quite right, as the place of panic invocation is not the same as the occurrence of the error, and currently the quoted code line is the one where panic! was invoked. But, we can do better than that!

✅ Substitute the above shown code block with the following lines:

```rust
let firmware_version = sensor.get_firmware_version()
    .unwrap_or_else(|error| {
        panic!("Error getting firmware version: {:?}", error)
    });
```

Instead of calling `unwrap()` we call `unwrap_or_else()`. Where unwrap() panics in case of an error, `unwrap_or_else()` can take a closure as argument, which allows you to provide the same additional functionality as handling `Result` with a match statement. 

Run your program. 

Your should see something like this:

```
0.000000 ERROR panicked at 'Error getting firmware version: AddressNack', src/bin/13_scd_30_error_handling.rs:63:5
└─ panic_probe::print_defmt::print @ /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:140
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
      <exception entry>
   1: __udf
   2: cortex_m::asm::udf
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.3/src/asm.rs:105
   3: rust_begin_unwind
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.1.0/src/lib.rs:75
   4: core::panicking::panic_fmt
        at /rustc/5c1f21c3b82297671ad3ae1e8c942d2ca92e84f2/src/libcore/panicking.rs:101
   5: _13_scd_30_error_handling::__cortex_m_rt_main::{{closure}}
        at src/bin/13_scd_30_error_handling.rs:63
   6: core::result::Result<T,E>::unwrap_or_else
   7: _13_scd_30_error_handling::__cortex_m_rt_main
        at src/bin/13_scd_30_error_handling.rs:61
   8: main
        at src/bin/13_scd_30_error_handling.rs:19
   9: ResetTrampoline
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
  10: Reset
        at /Users/tanks/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
The terminal process "/bin/bash '-c', 'cargo run --package knurling-session-20q4 --bin 13_scd_30_error_handling'" terminated with exit code: 134.

Terminal will be reused by tasks, press any key to close it.
```
Compared to the solution of handling `Result` with `match`, we can see at least in the stack backtrace exactly where the error occurred, and not just where the `panic!` was invoked. 


More elaborate error messages are nice, but we program on an embedded device that is supposed to be able to run without a host machine for logging. Writing your own panic handler allows you to provide "error messages" for this case, as logging messages to a host that is not there is not helpful. 

Go to `scr/rgb_led/mod.rs`. Add the following method:

```rust
pub fn error_blink_red(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
    for _i in 0 ..10 {
        self.red();
        timer.delay_ms(200_u32);
        self.off();
        timer.delay_ms(200_u32);
    }
}
```
When called, the LED will blink 10 times, relatively fast in red. 

✅ Call the method on the RGB LED, right before invoking `panic!`.

```rust
let firmware_version = sensor.get_firmware_version()
    .unwrap_or_else(|error| {
    led_indicator.error_blink_red(&mut timer);
        panic!("Error getting firmware version: {:?}", error)
    });
```

✅ Run your code. 

The RGB LED will now blink red a few times before the program panics. This alerts the user that something has gone wrong, and the device needs to be rebooted, or hooked up to a host for further diagnostics. 


✅ Go to `src/scd30/mod.rs`. Change `DEFAULT_ADDRESS` back to the original value: `0x61`.

```rust
pub const DEFAULT_ADDRESS: u8 = 0x61;
```
