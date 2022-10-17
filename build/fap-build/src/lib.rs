#![feature(fs_try_exists)]

use std::fs::try_exists;
use std::{env, fs};
use std::path::Path;
use std::path::PathBuf;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use build_cfg::consts::env::FLIPPER_SDK_PATH_ENV;


type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;


/// Build Flipper Application Package (FAP).
///
/// 1. Build FAP manifest (fam),
/// 1. read env var [`FLIPPER_SDK_PATH_ENV`][],
/// 1. copy/link (now just link) the manifest (fam) and assets to `$FLIPPER_FW_SRC_PATH/applications_user/{fap-id}/`,
/// 1. then execute `fbt firmware_{fap-id}` (TODO: impl post-build somehow).
///
/// `force` means that we can overwrite anything in the `$FLIPPER_FW_SRC_PATH/applications_user/{fap-id}`.
///
/// [`FLIPPER_SDK_PATH_ENV`]: https://docs.rs/flipper0-build-cfg/latest/flipper0_build_cfg/consts/env/constant.FLIPPER_SDK_PATH_ENV.html
pub fn build(force: bool) -> Result {
	let fam = fam::manifest()?;
	let id = fam.id().ok_or(IoError::new(IoErrorKind::NotFound, "manifest.appid"))?;
	let path = fam.save_to_out_dir()?;
	println!("Exported FAM path: {}", path.display());

	link_package_dir(id, force)?;

	// TODO: configure post-build env for future rustc wrapper or linker
	// something like this:
	env::set_var("FBT_RUNNER_FAP_ID", id);

	Ok(())
}


fn get_flipper_sdk_path() -> Result<PathBuf> {
	env::var(FLIPPER_SDK_PATH_ENV).map(PathBuf::from)
	                              .map_err(|err| {
		                              println!("cargo:warning=`Fail when read {FLIPPER_SDK_PATH_ENV}` env var. Error: {err}");
		                              err.into()
	                              })
	                              .and_then(|path| {
		                              try_exists(&path).and_then(|exists| {
			                                               if exists {
				                                               Ok(path)
			                                               } else {
				                                               Err(IoError::new(IoErrorKind::NotFound, path.display().to_string())).into()
			                                               }
		                                               })
		                                               .map_err(Into::into)
	                              })
}


/// Creates symlink `$FLIPPER_FW_SRC_PATH/applications_user/{fap-id}` -> `$OUT_DIR` directory.
///
/// __Attention:__ it affects fs outside of `$OUT_DIR` directory.
///
/// _Safety:_
/// - it fails if:
/// 	- `$FLIPPER_FW_SRC_PATH` points to non-existing directory
/// 	- `$FLIPPER_FW_SRC_PATH/applications_user` does not exists
/// 	- `$FLIPPER_FW_SRC_PATH/applications_user/{fap-id}` existing but is not a symlink
/// 	- `$FLIPPER_FW_SRC_PATH/applications_user/{fap-id}` existing symlink but `force` arg is `false`
/// - it can just create symlink
/// - it can override existing symlink only with `force` arg is `true`
pub fn link_package_dir<S: AsRef<str>>(id: S, force: bool) -> Result {
	let package: PathBuf = env::var("OUT_DIR")?.into();
	let target = get_flipper_sdk_path()?.join("applications_user").join(id.as_ref());

	if fs::try_exists(&target)? || target.is_symlink() {
		let link = fs::read_link(&target).map_err(|err| {
			                                 println!("cargo:warning=Error: {err}");
			                                 println!(
			                    "cargo:warning=Application `{}` already existing at `{}`, that's not a link, so I cannot overwrite it.",
			                    id.as_ref(),
			                    target.display()
			);
			                                 IoError::new(IoErrorKind::AlreadyExists, target.display().to_string())
		                                 })?;

		println!("Existing link points to: {}", link.display());

		// if that is a link, check where it points to.
		if link != package {
			if !force {
				return Err(IoError::new(
					IoErrorKind::AlreadyExists,
					format!("Unable to overwrite `{}` without `force`.", target.display()),
				).into());
			}
			println!("Overwriting existing link, there was: {}", link.display());
			fs::remove_file(&target)?;
			soft_link(package, target)?;
		} else {
			println!("Doing nothing, link already points to the current package dir (`OUT_DIR`).");
		}
	} else {
		soft_link(package, target)?;
	}

	Ok(())
}


#[rustfmt::skip]
fn soft_link<P: AsRef<Path>>(original: P, link: P) -> Result {
	#[cfg(unix)] use std::os::unix::fs::symlink;
	#[cfg(windows)] use std::os::windows::fs::symlink_dir as symlink;
	symlink(original.as_ref(), link.as_ref()).map_err(Into::into)
}
