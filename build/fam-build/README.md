# Flipper Zero Application Manifest __Generator__

Builds FAM from metadata in the crate manifest (usually Cargo.toml).
Uses `cargo metadata`.


## Usage

_Cargo.toml:_
```toml
[package]
build = "build.rs"
# ...

[package.metadata.fap]
main = "init"
name = "Hello, FAP"                # optional, default is crate name
# id = "hello-fap"                 # optional, default is crate name
# type = "FlipperAppType.EXTERNAL" # optional, default is FlipperAppType.EXTERNAL
# icon = "icon_10px.png"           # optional
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

	println!("Exported FAM path: {}", path.display());
}
```


- - -

[Official format documentation](https://github.com/flipperdevices/flipperzero-firmware/blob/release-candidate/documentation/AppManifests.md).
