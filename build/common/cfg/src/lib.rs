pub mod consts;

use std::env;
use std::path::PathBuf;
use std::error::Error;
use consts::env::FLIPPER_SDK_PATH_ENV;


pub fn flipper_sdk_path() -> Result<PathBuf, Box<dyn Error>> {
	println!("cargo:rerun-if-env-changed={}", FLIPPER_SDK_PATH_ENV);
	if let Some(sdk_path) = env::var_os(FLIPPER_SDK_PATH_ENV) {
		if cfg!(windows) {
			// return path as it is.
			println!("sdk_path: {sdk_path:?}");
			Ok(sdk_path.into())
		} else {
			let path = if sdk_path.to_string_lossy().starts_with("~/") {
				env::var_os("HOME").map(|home| {
					                   println!("HOME: {:?}", home);
					                   PathBuf::from(home).join(&sdk_path.to_string_lossy()[2..])
				                   })
				                   .ok_or(env::VarError::NotPresent)?
			} else {
				PathBuf::from(sdk_path)
			};

			Ok(path.canonicalize()
			       .map_err(|err| println!("cargo:warning=Failed canonicalization SDK path: {err}"))
			       .unwrap_or(path))
		}
	} else {
		println!(
		         "cargo:warning=env var `{}` not found, please set it. \
		         You can enable `prebuild` feature to use prebuilt bindings.",
		         FLIPPER_SDK_PATH_ENV
		);
		Err(env::VarError::NotPresent.into()).into()
	}
}
