#![allow(dead_code)]


pub mod env {
	/// Env var name for internal use, points to generated bindings.
	pub const BINDINGS_ENV: &'static str = "BINDINGS";
	/// Env var name for internal use, contains doc-line for bindings.
	pub const BINDINGS_METADATA_DOC_ENV: &'static str = "BINDINGS_METADATA_DOC";

	// local
	/// Env var name, value should contain path to the root of the Flipper Zero firmware repository.
	pub const FLIPPER_SDK_PATH_ENV: &'static str = "FLIPPER_FW_SRC_PATH";
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
