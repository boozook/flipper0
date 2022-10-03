#![allow(dead_code)]
#![allow(unused_imports)]

macro_rules! doc_export {
	(mod $item:ident) => {
		#[cfg(doc)]
		pub mod $item;
		#[cfg(not(doc))]
		pub(crate) mod $item;
	};
	($item:ident::*) => {
		#[cfg(doc)]
		pub use $item::*;
		#[cfg(not(doc))]
		pub(crate) use $item::*;
	};
	($item:path) => {
		#[cfg(doc)]
		pub use $item;
		#[cfg(not(doc))]
		pub(crate) use $item;
	};
}


mod env {
	/// Env var name for internal use, points to generated bindings.
	pub const BINDINGS_ENV: &'static str = "BINDINGS";
	/// Env var name, value should contain path to the root of the Flipper Zero firmware repository.
	pub const FLIPPER_SDK_PATH_ENV: &'static str = "FLIPPER_REPO_PATH";
	/// Env var name. Optional. Should points to ARM toolchain, `arm-none-eabi` directory.
	pub const ARM_TOOLCHAIN_PATH_ENV: &'static str = "ARM_TOOLCHAIN";
}


mod support {
	/// Minimal supported SDK version.
	pub const SDK_VERSION: &str = "~0.68.1";
	/// Tested with API version.
	pub const API_VERSION: &str = "=1.13";

	/// Tested with ARM toolchain version.
	#[allow(dead_code)] // for future validation
	pub const TOOLCHAIN_VERSION: &str = "15";
}


doc_export!(env::FLIPPER_SDK_PATH_ENV);
doc_export!(env::ARM_TOOLCHAIN_PATH_ENV);
pub(crate) use env::BINDINGS_ENV;

doc_export!(support::*);
