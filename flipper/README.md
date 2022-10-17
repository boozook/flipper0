# API for Flipper Zero

## Crate contains

- Re-exports low-level bindings
- Re-shares `#[main]` macro
- File System rusty API
- Some things such as stdout, print(ln), OsString, etc..

Not yet implemented:

- [ ] Threading rusty API
- [ ] Async-await on top of above
- [ ] Neat logging
- [ ] Some little things like `PathBuf`.


For detailed information see:

- [Examples][]
- [Sys crate][]



[Examples]: https://github.com/boozook/flipper0/blob/master/examples/
[Sys crate]: https://crates.io/crates/flipper0-sys
