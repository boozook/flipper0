# Rusty API for Flipper Zero

🦀 ❤️‍🔥 🐬


## Project Structure

This project is composed of the following crates:

* [`flipper0-sys`](#bindings-for-flipper-zero): CFFI bindings.

* [`flipper0`](//github.com/boozook/flipper0/tree/master/flipper#readme): High-level idiomatic bindings and wrappers that almost looks like as std-types.

* other support crates, see docs inside
  - flipper0-build-cfg: Constants and configuration for build system
  - flipper0-fam-build: Manifest generator
  - flipper0-fap-build: Application Package build utils
  - fam: Flipper Application Package manifest format
  - flipper0-macro: Proc-macro `#[main]` for register and rustify entry point function.

* [examples](//github.com/boozook/flipper0/tree/master/examples)


- - -


# Bindings for Flipper Zero

Automatically generated bindings (or "externs") for [Flipper Zero Fw][] with some little hand-crafted wrappers and additions as upper abstraction layer.

See __[crate description][sys crate description]__ and __[examples][]__.


## State

![Maintenance Status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

Current state of the project is WiP. _Highly & dirty work-in-progress._
Any contribution are appreciated, you know.


## Build

There's three options: using pre-built bindings,
  build from existing clone of flipper-firmware repo,
  clone and build from source.

### Using pre-built bindings

Needed:
- Rust toolchain (`nightly` channel)
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang] (because bindgen as build-dependency can't be optional)

### Example

_crate manifest:_
```toml
[dependencies]
flipper0-sys = "*"
```

_lib.rs:_
```rust
#![crate_type = "staticlib"] #![no_main] #![no_std]

extern crate flipper0_sys;
use flipper0_sys::ffi::*;

#[no_mangle]
pub unsafe extern "C" fn init(_: *mut u8) -> i32 {
	static MESSAGE: &[u8] = b"Hello, World!";
	furi_thread_stdout_write(MESSAGE.as_ptr() as _, MESSAGE.len());
	0
}
```

Build with `cargo +nightly build --target=thumbv7em-none-eabihf`.


### Build bindings from source

Needed:
- Rust toolchain (`nightly` channel)
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang]
- clone of [Flipper Zero firmware][Flipper Zero Fw]
  - Env var `FLIPPER_FW_SRC_PATH` points to the root of the fw repo
- ARM toolchain, run `fbt` to easily get it

_crate manifest:_
```toml
[dependencies.flipper0-sys]
version = "*"
default-features = false # disable prebuild
features = [
  "allocator-global",
  "oom-global",
  "panic",
  "use-local-sdk" # enable build from source
  # ...
]
```

Example code same as above.

Build with `FLIPPER_FW_SRC_PATH=~/path/to/flipperzero-firmware/ cargo +nightly build --target=thumbv7em-none-eabihf`


Also check out instructions for __[examples][]__.


### Build bindings from source with auto- clone remote repository

Needed:
- Rust toolchain (`nightly` channel)
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang]
- Env var `FLIPPER_REPO_BRANCH` with name of branch for the firmware repository (optional)
- Env var `FLIPPER_REPO_REV` with name of tag/revision for the firmware repository (optional)
- Env var `FLIPPER_REPO_CLONE_PATH` points to directory where the clone should be placed (optional)

Example code same as above, __but using `use-remote-sdk`__ instead of `use-local-sdk`.

_crate manifest:_
```toml
[dependencies.flipper0-sys]
version = "*"
default-features = false # disable prebuild
features = [
  # same as above
  "use-remote-sdk" # enable build from source
]
```

Build with specified place:
```bash
FLIPPER_REPO_CLONE_PATH=/abs/path/ \
  cargo +nightly build --target=thumbv7em-none-eabihf
```
And/or with specified branch:
```bash
FLIPPER_REPO_BRANCH=release \
  cargo +nightly build --target=thumbv7em-none-eabihf
```
And/or with specified tag or revision:
```bash
FLIPPER_REPO_REV="0.70.1" \
  cargo +nightly build --target=thumbv7em-none-eabihf
```

Note:
1. `FLIPPER_REPO_CLONE_PATH` there used instead of `FLIPPER_FW_SRC_PATH`
1. By default `FLIPPER_REPO_CLONE_PATH` points to `$OUT_DIR/sdk`
1. Without specified branch or rev will be checked out head of default branch.



## Build Configuration

### Environment variables:
| Feature                   | Required | Description                                                                                                                                               | Use with feature                  |
| ------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `FLIPPER_FW_SRC_PATH`     | required | Needed to build from source in local working copy of [firmware repo][Flipper Zero Fw], points to root of the repo.                                        | `use-local-sdk`                   |
| `ARM_TOOLCHAIN`           | optional | If omitted build-script will search it in the working copy of the [firmware repo][Flipper Zero Fw]. Typically should points to "arm-none-eabi" directory. | `use-local-sdk`, `use-remote-sdk` |
| `FLIPPER_REPO_REV`        | optional | Revision or tag.                                                                                                                                          | `use-remote-sdk`                  |
| `FLIPPER_REPO_BRANCH`     | optional | Name of branch.                                                                                                                                           | `use-remote-sdk`                  |
| `FLIPPER_REPO_CLONE_PATH` | optional | Path points to directory where the SDK repository will be cloned. Default is `OUT_DIR/flipperzero-firmware`.                                              | `use-remote-sdk`                  |


### Features:

- `allocator`: include allocator implementation
- `allocator-global`: default, include __global allocator__ implementation
- `oom-global`: default, out-of-mem handler. Disable it to use you custom handler or `#![feature(default_alloc_error_handler)]`.
- `panic`: default, include global panic & OoM handler
- `macro`: include `#[main]` macro for FAP entry point.


### Bindings gen customization features:

_Can be used with `use-local-sdk` or `use-remote-sdk` features._

- `derive-default`
- `derive-eq`
- `derive-copy`
- `derive-hash`
- `derive-ord`
- `derive-partialeq`
- `derive-partialord`
- `derive-debug` - derive `Debug`, enabled by default for debug profile

All of these `derive-`features are used for bindgen configuration.


### Build methods features:

| Feature          | Default | Description                                                            | Used ENV vars                                                                                             |
| ---------------- | ------- | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `prebuild`       | +       | use pre-generated bindings                                             |                                                                                                           |
| `use-local-sdk`  | +       | look at `FLIPPER_FW_SRC_PATH`, build from source                       | `FLIPPER_FW_SRC_PATH` (required), `ARM_TOOLCHAIN` (optional)                                              |
| `use-remote-sdk` | -       | clone remote git repo, initial setup with fbt, then build from source. | `FLIPPER_REPO_REV`, `FLIPPER_REPO_BRANCH`, `FLIPPER_REPO_CLONE_PATH`, `ARM_TOOLCHAIN` (all vars optional) |

_`prebuild` is default feature just for ability to build crate out-of-the-box._



- - -

Other documentation useful for development:

- [ci pipeline documentation][]
- [build system explanation][]



[bingen+clang]: https://github.com/rust-lang/rust-bindgen/issues/918
[Flipper Zero Fw]: https://github.com/flipperdevices/flipperzero-firmware/
[examples]: https://github.com/boozook/flipper0/blob/master/examples/
[sys crate description]: https://github.com/boozook/flipper0/blob/master/crates.io.md

[ci pipeline documentation]: https://github.com/boozook/flipper0/tree/master/.github#readme
[build system explanation]: #TODO
