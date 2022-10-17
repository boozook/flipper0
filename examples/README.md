# Examples

Examples in order from low level with handy ctrl for all to high level with less ctrl but with some neat "single-button" solutions.

After build all examples you usually need to build an applications package ("fap") with official build tool `fbt`,
so need to clone [flipper-firmware repo][] to somewhere and save its path to the ENV VAR `FLIPPER_FW_SRC_PATH`.

For example run this:
```bash
git clone --recursive https://github.com/flipperdevices/flipperzero-firmware.git
git checkout release --recurse-submodules

export FLIPPER_FW_SRC_PATH="$(PWD)/flipperzero-firmware"
```

Also you need to call `fbt` to ARM toolchain if you don't have one already.
```bash
cd $FLIPPER_FW_SRC_PATH
# install dependencies for fbt
pip3 install -r scripts/requirements.txt
# run
./fbt
```
This steps can fails if something changed in official tooling. Better use [official documentation][fbt-doc].

[fbt-doc]: https://github.com/flipperdevices/flipperzero-firmware/blob/release/documentation/fbt.md


[flipper-firmware repo]: https://github.com/flipperdevices/flipperzero-firmware/tree/release


- - -


## [`hello-fap`][]

Defined as the real example in the main crate manifest [there][hello-fap-def].

Just two files:
- hello-fap.rs
- hello-fap.fam - manifest

### Build

```bash
cargo +nightly build --example=hello-fap --target=thumbv7em-none-eabihf

# Copy manifest to build directory, so we have a package there:
cp ./examples/hello-fap.fam ./target/thumbv7em-none-eabihf/debug/examples/application.fam
```

Now there are two files:
- libhello_fap.a
- application.fam

That's often to build Flipper Application Package (fap, yeah) with official build tool `fbt`.

```bash
# Remember full path to our application build directory:
FAP_DIR="$(PWD)/target/thumbv7em-none-eabihf/debug/examples"

# Go to the Flipper FW repo with sources and FBT:
pushd "${FLIPPER_FW_SRC_PATH}"

# Create folder for external applications if doesn't exist yet:
mkdir -p ./applications_user

# Create link to our application build directory:
ln -s $FAP_DIR ./applications_user/hello_fap

# Run fbt:
./fbt fap_hello_fap COMPACT=1 DEBUG=0
```

Now if no errors we can get package at `build/f7-firmware-C/.extapps/hello_fap.fap` then copy to Flipper and launch.


[hello-fap-def]: https://github.com/boozook/flipper0/blob/master/Cargo.toml#L50-L53


- - -


## [`app-manifest-meta`][]

This example uses build scripts to generate Flipper Application Manifest ("fam") using crate metadata defined in the crate manifest file `Cargo.toml` in the section `[package.metadata.fap]`.

Manifest will saved in the `OUT_DIR` provided by `cargo`.


### Build

```bash
cargo +nightly build -p=fap-manifest-metadata-example --target=thumbv7em-none-eabihf
```

Then just link or copy `OUT_DIR` to `$FLIPPER_FW_SRC_PATH/applications_user/[crate-name]` and run `fbt` as in first example.


- - -


## [`app-manifest-toml`][]

Almost the same as in previous example `app-manifest-meta` but uses `Flipper.toml` as source for the manifest (fam).

Manifest will saved in the `OUT_DIR` provided by `cargo`.


### Build

```bash
cargo +nightly build -p=fap-manifest-toml-example --target=thumbv7em-none-eabihf
```

Then do absolutely same as for previous example.


- - -


## [`app-build`][]

This example uses build scripts to do almost all things automatically that we did in previous examples by hands.
So it build manifest and link `$FLIPPER_FW_SRC_PATH/applications_user/[crate-name]` to the build directory with just generated manifest.

Metadata for Flipper Application Manifest ("fam") described in the crate manifest `Cargo.toml` in the section `[package.metadata.fap]`.


### Build

__Requires__ `FLIPPER_FW_SRC_PATH` env variable.

```bash
cargo +nightly build -p=fap-build-example --target=thumbv7em-none-eabihf

# Then build with fbt:
pushd "${FLIPPER_FW_SRC_PATH}"
./fbt fap_fap-build-example
```


- - -


## [`main` macro][main-macro]

Demonstration of `#[main]` macro that needed to set entry point of you application to the manifest and makes signature of that entry-point function more comfortable.

Example:
```rust
#[macro_use]
extern crate flipper0;

#[main] // wraps the function and sets its name to the manifest.
pub fn my_entry_point() -> Result { Ok(()) }
```

For more information about this macro check out [documentation]...

Two crates shares (exports) this macro:
- [flipper0-sys][]
- [flipper0][]

[flipper0]: https://crates.io/crates/flipper0
[flipper0-sys]: https://crates.io/crates/flipper0-sys


### Build

__Requires__ `FLIPPER_FW_SRC_PATH` env variable.

```bash
cargo +nightly build -p=main-macro-example --target=thumbv7em-none-eabihf
```

Build process is same as example above.


- - -


## [`File System`][fs]

Just a little example demonstrates file system API with rusty wrapper implemented in [flipper0][] crate.

API wants to looks like `fs` module in the std-lib.

### Build

__Requires__ `FLIPPER_FW_SRC_PATH` env variable.

```bash
cargo +nightly build -p=fs-example --target=thumbv7em-none-eabihf
```

Build process is same as examples above.




[`hello-fap`]: https://github.com/boozook/flipper0/blob/master/examples/hello-fap.rs
[`app-manifest-meta`]: https://github.com/boozook/flipper0/blob/master/examples/app-manifest-meta
[`app-manifest-toml`]: https://github.com/boozook/flipper0/blob/master/examples/app-manifest-toml
[main-macro]: https://github.com/boozook/flipper0/blob/master/examples/macro
[`app-build`]: https://github.com/boozook/flipper0/blob/master/examples/app-build
[fs]: https://github.com/boozook/flipper0/blob/master/examples/fs
