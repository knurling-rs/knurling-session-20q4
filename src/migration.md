# Migrating from git defmt to stable defmt

People that started working on this book before 2020-11-11 are using the unstable git version of the `defmt` logging framework.
On 2020-11-11, a stable version of `defmt` became available on crates.io.
If you are still using the git version you are encouraged to migrate to the crates.io version!
Here's how:

1. In your `app-template` project, change the root `Cargo.toml` as shown below:

``` diff
 [workspace]
 members = ["testsuite"]

-[dependencies.defmt]
-git = "https://github.com/knurling-rs/defmt"
-branch = "main"
-
-[dependencies.defmt-rtt]
-git = "https://github.com/knurling-rs/defmt"
-branch = "main"
-
-[dependencies.panic-probe]
-git = "https://github.com/knurling-rs/probe-run"
-branch = "main"
-
 [dependencies]
+defmt = "0.1.0"
+defmt-rtt = "0.1.0"
+panic-probe = { version = "0.1.0", features = ["print-defmt"] }
 cortex-m = "0.6.4"
 cortex-m-rt = "0.6.13"
```

2. In your `app-template` project, also change the `testsuite/Cargo.toml` as shown below:

``` diff
 name = "test"
 harness = false

-[dependencies.defmt]
-git = "https://github.com/knurling-rs/defmt"
-branch = "main"
-
-[dependencies.defmt-rtt]
-git = "https://github.com/knurling-rs/defmt"
-branch = "main"
-
-[dependencies.panic-probe]
-git = "https://github.com/knurling-rs/probe-run"
-branch = "main"
-# enable the `print-defmt` feature for more complete test output
-features = ["print-defmt"]
-
 [dependencies]
+defmt = "0.1.0"
+defmt-rtt = "0.1.0"
+panic-probe = { version = "0.1.0", features = ["print-defmt"] }
 cortex-m = "0.6.3"
 cortex-m-rt = "0.6.12"
```

3. Finally, install `probe-run` version v0.1.4 (or newer)

``` console
$ cargo install probe-run -f
```

Now you can resume working on your project!
