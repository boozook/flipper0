# Flipper Zero Application Manifest __Generator__

Builds FAM from one of two possible sources:
- Metadata in the crate manifest (usually Cargo.toml)
- Flipper.toml near by crate manifest

Uses `cargo metadata`.


## Usage

_Cargo.toml:_
```toml
[package]
build = "build.rs"
# ...

[package.metadata.fam]
main = "init"
name = "Hello, Flipper"            # optional, default is crate name
# id = "hello-flipper"             # optional, default is crate name
# type = "FlipperAppType.EXTERNAL" # optional, default is FlipperAppType.EXTERNAL
# icon-file = "icon_10px.png"      # optional, path relative to the root of crate
# category = "Misc"                # optional, default Misc

[build-dependencies.fam-build]
package = "flipper0-fam-build"
version = "*"
```

_build.rs:_
```rust
fn main() {
	let fam = fam_build::manifest().unwrap();
	let path = fam.save_to_out_dir().unwrap();
	// or...   fam.save_to(some other path)

	println!("Exported FAM path: {}", path.display());
}
```

### Entry-point `main`

Metadata field `package.metadata.fam.main` is required by default.

But it's __optional__ if feature `optional_entry_point` enabled.
That optionality needed in cases when this used in couple with [macro crate][flipper0-macro] that also writes to manifest. So you can omit `package.metadata.fam.main` in that case.


## Examples

- [Usage example with crate metadata](https://github.com/boozook/flipper0/tree/master/examples/app-manifest-meta).
- [Usage example with Flipper.toml](https://github.com/boozook/flipper0/tree/master/examples/app-manifest-toml).


- - -

[Official format documentation](https://github.com/flipperdevices/flipperzero-firmware/blob/release-candidate/documentation/AppManifests.md).


[flipper0-macro]: https://crates.io/crates/flipper0-macro
