#![allow(dead_code)]


pub mod env {
	pub use build_cfg::consts::env::*;
}


pub mod support {
	/// Minimal supported SDK version.
	pub const SDK_VERSION: &str = "^0.70.1";
	/// Tested with API version.
	pub const API_VERSION: &str = "^7.0";

	/// Tested with ARM toolchain version.
	pub const TOOLCHAIN_VERSION: &str = "15";
}
