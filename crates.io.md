# Bindings for Flipper Zero

Automatically generated bindings (or "externs") for [Flipper Zero Fw][] with some little hand-crafted wrappers and additions as upper abstraction layer.

This can be built with:
- without Flipper fw sources using default feature `prebuild`
- with modified fw using feature `use-local-sdk`
- with enabled feature `use-remote-sdk` official fw will be downloaded, then build as with `use-local-sdk` feature.


## Compatibility

Minimal supported version:
- FW: [0.70.1](https://github.com/flipperdevices/flipperzero-firmware/releases/tag/0.70.1)
- API: [7.0](https://github.com/flipperdevices/flipperzero-firmware/blob/release/firmware/targets/f7/api_symbols.csv#L2)

Latest supported version can be determined by [git tags](https://github.com/boozook/flipper0/tags) starting with `fw-`.



## Prerequisites

- Rust toolchain, `nightly`
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang]
- clone of [Flipper Zero firmware][Flipper Zero Fw] _(optional)_
- ARM toolchain, run `fbt` to easily get it _(optional)_


For build using pre-generated bindings (`prebuild` feature) just Rust toolchain is required, nightly channel.

For build using non-modified official fw just Rust toolchain and firmware sources are required.

For other cases see [documentation][readme-build] and [examples][].



## Hello-world

Just add dependency to your cargo manifest file:
```toml
[dependencies]
flipper0-sys = "*"
```

And follow instructions for __[examples][]__.


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



## Features:

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

By default `prebuild` is turned on. It uses pre-generated bindings, so fw not needed.


| Feature          | Default | Description                                                            | Used ENV vars                                                                                             |
| ---------------- | ------- | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `prebuild`       | +       | use pre-generated bindings                                             |                                                                                                           |
| `use-local-sdk`  | +       | look at `FLIPPER_FW_SRC_PATH`, build from source                       | `FLIPPER_FW_SRC_PATH` (required), `ARM_TOOLCHAIN` (optional)                                              |
| `use-remote-sdk` | -       | clone remote git repo, initial setup with fbt, then build from source. | `FLIPPER_REPO_REV`, `FLIPPER_REPO_BRANCH`, `FLIPPER_REPO_CLONE_PATH`, `ARM_TOOLCHAIN` (all vars optional) |





[Flipper Zero Fw]: https://github.com/flipperdevices/flipperzero-firmware/
[examples]: https://github.com/boozook/flipper0/blob/master/examples/
[readme-build]: https://github.com/boozook/flipper0/blob/master/README.md#build-configuration
[bingen+clang]: https://github.com/rust-lang/rust-bindgen/issues/918
