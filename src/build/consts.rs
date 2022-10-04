#![allow(dead_code)]
#![allow(unused_imports)]


mod env {
	/// Env var name for internal use, points to generated bindings.
	pub const BINDINGS_ENV: &'static str = "BINDINGS";
	/// Env var name for internal use, contains doc-line for bindings.
	pub const BINDINGS_METADATA_DOC_ENV: &'static str = "BINDINGS_METADATA_DOC";

	// local
	/// Env var name, value should contain path to the root of the Flipper Zero firmware repository.
	pub const FLIPPER_SDK_PATH_ENV: &'static str = "FLIPPER_REPO_PATH";
	/// Env var name. Optional. Should points to ARM toolchain, `arm-none-eabi` directory.
	pub const ARM_TOOLCHAIN_PATH_ENV: &'static str = "ARM_TOOLCHAIN";

	// remote
	/// Env var name. Optional. Revision or tag, used with feature `use-remote-sdk`.
	pub const FLIPPER_NET_SDK_REV_ENV: &'static str = "FLIPPER_REPO_REV";
	/// Env var name. Optional. Name of branch, used with feature `use-remote-sdk`.
	pub const FLIPPER_NET_SDK_BRANCH_ENV: &'static str = "FLIPPER_REPO_BRANCH";
	/// Env var name. Optional. Path points to directory where the SDK repository will be cloned.
	/// Default: `OUT_DIR/flipperzero-firmware`. Used with feature `use-remote-sdk` only.
	pub const FLIPPER_NET_SDK_PATH: &'static str = "FLIPPER_REPO_CLONE_PATH";
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


pub(crate) use env::BINDINGS_ENV;
pub(crate) use env::BINDINGS_METADATA_DOC_ENV;

doc_export!(env::FLIPPER_SDK_PATH_ENV);
doc_export!(env::ARM_TOOLCHAIN_PATH_ENV);

doc_export!(env::FLIPPER_NET_SDK_REV_ENV);
doc_export!(env::FLIPPER_NET_SDK_BRANCH_ENV);
doc_export!(env::FLIPPER_NET_SDK_PATH);

doc_export!(support::*);
