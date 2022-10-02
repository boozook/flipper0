# Rust bindings for Flipper Zero

Automatically generated bindings (or "externs") for [Flipper Zero Fw][] with some little hand-crafted wrappers and additions as upper abstraction layer.


## State

Current state of the project is WiP. _Highlly & dirty work-in-progress._

Supported (means "tested with") fw version: __0.68.0__ but should work normally with any 0.68.


## Prerequisites

- Rust toolchain (currently only `nightly` supported)
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang]
- clone of [Flipper Zero firmware][Flipper Zero Fw]
- ARM toolchain, run `fbt` to easily get it


Add this as dependency to your cargo manifest file:
```
[dependencies.flipper0]
version = "0.1"
default-features = false # disable prebuild
```

To build just add `FLIPPER_REPO_PATH` to your ENV anyhow (config, build-script, shell-rc, etc..), for example run:
```
FLIPPER_REPO_PATH=~/path/to/flipperzero-firmware/ cargo build --release
```


## Configuration

### Environment variables:
- `FLIPPER_REPO_PATH`: optional, needed when feature `prebuild` disabled, points to root of working copy of the [firmware repo][Flipper Zero Fw];
- `ARM_TOOLCHAIN`: optional, if omitted build-script will search it in the working copy of the [firmware repo][Flipper Zero Fw]. Typically should points to "arm-none-eabi" directory.


### Features:
- `prebuild`: default, use pre-generated bindings

_`prebuild`_ is default feature just for ability to build crate out-of-the-box.


## Development

Any contribution are appreciated.

TODO:
- [ ] logger & logging feature
- [ ] wrapper for stdout
- [ ] wrapper for threading
- [ ] wrapper for fs
- [x] impl panic handler
- [ ] impl global alocator
- [ ] get from web by api_symbols.csv with opaque types
- [x] gen from source
- [ ] gen from built firmware
- [ ] gen from product of `fbt sdk_tree`
- [ ] tool for apps to build & link elf (with or without fbt)
- [ ] __examples__
- [ ] proper documentation
- [ ] split api to modules under feature-gates, like "furi" or "gpio".
- [ ] __tests__
- [ ] CI/CD
- [ ] update to latest fw version




[bingen+clang]: https://github.com/rust-lang/rust-bindgen/issues/918
[Flipper Zero Fw]: https://github.com/flipperdevices/flipperzero-firmware/
