# API for Flipper Zero

Minimal supported version:
- FW: [0.70.1](https://github.com/flipperdevices/flipperzero-firmware/releases/tag/0.70.1)
- API: [7.0](https://github.com/flipperdevices/flipperzero-firmware/blob/release/firmware/targets/f7/api_symbols.csv#L2)

## Crate contains

- Re-exports low-level bindings
- `#[main]` macro
- File System rusty API
- Some things such as stdout, print(ln), OsString, etc..


For detailed information see:

- [Examples][]
- [Sys crate][]



[Examples]: https://github.com/boozook/flipper0/blob/master/examples/
[Sys crate]: https://crates.io/crates/flipper0-sys
