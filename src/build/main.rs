#![feature(box_syntax)]
#![feature(fs_try_exists)]
#![feature(io_error_other)]
#![feature(option_result_contains)]
#![feature(exit_status_error)]

extern crate bindgen;

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};
use semver::VersionReq;

mod macros;
mod consts;
mod api_table;
mod source;


fn main() -> Result<(), Box<dyn Error>> {
	let cargo_profile = env::var("PROFILE").expect("PROFILE cargo env var");
	let debug = cargo_profile.to_lowercase().eq("debug");
	let output_filename = bindings_filename(debug);

	// crate features:
	let prebuild = feature("prebuild");
	let use_local_sdk = feature("use-local-sdk");
	let use_remote_sdk = feature("use-remote-sdk");

	if prebuild {
		let root = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR cargo env var");
		let path = PathBuf::from(root).join("gen").join(output_filename);
		println!("cargo:rustc-env={}={}", consts::BINDINGS_ENV, path.display().to_string());
		return Ok(());
	}

	if use_local_sdk {
		let result = source::local_sdk::try_build();
		// if we have no next steps:
		if result.is_ok() | !use_remote_sdk {
			return result;
		}
	}

	if use_remote_sdk {
		let path = crate::source::remote_sdk::git_clone_sdk()?;
		println!("SDK from git: {} successfully installed", path.display());
		env::set_var(consts::FLIPPER_SDK_PATH_ENV, path);
		source::local_sdk::try_build()?;
	}

	Ok(())
}


fn check_version<S: std::fmt::Display>(version: &str, supported: VersionReq, name: S) {
	let parsed = semver::Version::parse(version.trim()).or_else(|_| {
		                                                   // dummy fix version to semantic-version fmt:
		                                                   let mut version = version.trim().to_string();
		                                                   let count = version.chars().filter(|c| c == &'.').count();
		                                                   (0..(2 - count)).for_each(|_| version.push_str(".0"));
		                                                   semver::Version::parse(&version)
	                                                   });

	if let Ok(current) = parsed {
		if !supported.matches(&current) {
			println!("cargo:warning={name} version not supported. Required version '{supported}' does not matches {version}.");
		}
	} else {
		println!("cargo:warning={name} version `{version}` is invalid.");
	}
}


fn is_debug() -> bool {
	let cargo_profile = env::var("PROFILE").expect("PROFILE cargo env var");
	cargo_profile.to_lowercase().eq("debug")
}


#[inline(always)]
fn env_var_bool<K: AsRef<OsStr>>(key: K) -> bool {
	env::var(key).ok()
	             .filter(|s| s == "1" || s.to_lowercase() == "true")
	             .is_some()
}


fn feature(feature: &str) -> bool {
	let name = feature.to_uppercase().replace("-", "_");
	env_var_bool(format!("CARGO_FEATURE_{name}"))
}


fn bindings_filename(debug: bool) -> &'static str {
	if debug {
		"flipper0-debug.rs"
	} else {
		"flipper0-release.rs"
	}
}
