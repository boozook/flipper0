#![feature(fs_try_exists)]
#![feature(io_error_more)]
#![feature(stmt_expr_attributes)]

use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

extern crate serde;
extern crate serde_json;
extern crate crate_metadata as meta;
extern crate fam;

mod manifest;
mod metadata;

pub use manifest::Manifest;
pub use manifest::IntermediateManifest;
pub use metadata::Metadata;
pub use metadata::DEFAULT_MAIN;


type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;

#[cfg(feature = "toml")]
const FLIPPER_MANIFEST_TOML: &str = "Flipper.toml";
const CARGO_MANIFEST: &str = "Cargo.toml";
const TARGET_KIND: &str = "staticlib";
const FAM_FILENAME: &str = "application.fam";
const FLIPPER_TRIPLE: &str = "thumbv7em-none-eabihf";


/// Build Flipper Application Manifest (FAM).
/// Call typically from your build script.
pub fn manifest() -> Result<Manifest> { manifest_named(CARGO_MANIFEST) }

/// With custom crate manifest file name. Typically that is `Cargo.toml`.
pub fn manifest_named<S: AsRef<str>>(filename: S) -> Result<Manifest> {
	let root = crate_root()?;
	let manifest = root.join(filename.as_ref());
	let name = crate_name()?;

	println!("cargo:rerun-if-changed={}", manifest.display());


	let (target_directory, crate_metadata) = {
		let metadata = meta::crate_metadata_for::<Metadata, _>(&manifest, &name)?;
		let target = metadata.target_directory;
		let package = metadata.packages
		                      .into_iter()
		                      .next()
		                      .ok_or_else(|| IoError::new(IoErrorKind::NotFound, format!("package {name} not found")))?;
		(target, package)
	};

	let mut targets = {
		let kind = TARGET_KIND.to_owned();
		crate_metadata.targets
		              .iter()
		              .filter(move |t| t.kind.contains(&kind) && t.crate_types.contains(&kind))
	};

	// TODO: try get CARGO_PRIMARY_PACKAGE to determine whether crate is dep or finale.

	// TODO: support multiple targets.
	// Now get first, then check more and warn if it is.
	// Should probably fail instead of warning.
	let target = targets.next().ok_or_else(|| IoError::new(IoErrorKind::NotFound, "target"))?;
	if targets.next().is_some() {
		println!("cargo:warning=Multiple targets (libs) does not supported.");
	}

	#[allow(clippy::bind_instead_of_map)]
	let mut fap = crate_metadata.metadata
	                         .map(|meta| meta.fap)
	                         .take()
	                         .flatten()
	                         .ok_or_else(|| IoError::new(IoErrorKind::NotFound, "fap crate metadata"))
	                         .or_else(|_err| {
		                         #[cfg(feature = "toml")]
		                         {
			                         // try to get manifest from Flipper.toml near by Cargo.toml:
			                         let fap_toml_res = manifest_toml_from(&root.join(FLIPPER_MANIFEST_TOML));
			                         // report if both not found:
			                         if fap_toml_res.is_err() {
				                         println!("cargo:warning=Error: crate metadata not found in `{}`.", manifest.display());
				                         println!("cargo:warning=Error: `Flipper.toml` not found in `{}`.", root.display());
			                         }
			                         fap_toml_res
		                         }

		                         #[rustfmt::skip]
		                            #[cfg(not(feature = "toml"))] { Err(_err) }
	                         })?;

	fap.set_defaults()?;
	fap.sources
	   .insert(0, crate_product(&target_directory, target)?.display().to_string());
	// We should not fail here because in the custom build-pipeline assets can be delivered later.
	fap.canonicalize_assets(&root)
	   .map_err(|err| println!("cargo:warning=Unable to consolidate assets: {err}"))
	   .ok();


	Ok(fap.into())
}


/// Output directory with compilation product.
/// Typically is `target/{triple}/{profile}/lib{name}.a`.
pub fn crate_product(dir: &Path, target: &meta::Target) -> Result<PathBuf> {
	let triple = current_target()?;
	let profile = current_profile()?;
	let path = if triple == FLIPPER_TRIPLE {
		dir.join(&triple).join(&profile)
	} else {
		dir.join(&profile)
	};

	// We accept only static libraries.
	// Previously we filtered out others.
	let filename = staticlib_name_to_filename(&target.name);
	Ok(path.join(&filename))
}


fn staticlib_name_to_filename(name: &str) -> String { format!("lib{name}.a").replace('-', "_") }


pub fn current_target() -> Result<String> {
	env::var("TARGET").map(|target| {
		                  // re-share variable for future possible usage from macro (if used)
		                  println!("cargo:rustc-env=TARGET={target}");
		                  target
	                  })
	                  .or_else(|_| {
		                  let mut found = false;
		                  let mut result = None;
		                  for (_, arg) in std::env::args().enumerate() {
			                  if !found && arg == "--target" {
				                  found = true;
			                  } else if found {
				                  result = Some(arg);
				                  break;
			                  }
		                  }
		                  result.ok_or(env::VarError::NotPresent.into())
	                  })
}

pub fn current_profile() -> Result<String> {
	env::var("PROFILE").map(|profile| {
		                   // re-share variable for future possible usage from macro (if used)
		                   println!("cargo:rustc-env=PROFILE={profile}");
		                   profile
	                   })
	                   .map_err(Into::into)
}


pub fn crate_name() -> Result<String> { env::var("CARGO_PKG_NAME").map_err(Into::into) }
pub fn crate_descr() -> Result<String> { env::var("CARGO_PKG_DESCRIPTION").map_err(Into::into) }
pub fn crate_root() -> Result<PathBuf> { Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)) }
pub fn crate_version() -> Result<String> { env::var("CARGO_PKG_VERSION").map_err(Into::into) }
pub fn crate_version_parts() -> Result<(usize, usize, Option<usize>, Option<String>)> {
	use env::var;
	let parse = |v: String| v.parse().ok();

	let mut version = [
	                   "CARGO_PKG_VERSION_MAJOR",
	                   "CARGO_PKG_VERSION_MINOR",
	                   "CARGO_PKG_VERSION_PATCH",
	].into_iter()
	                  .map(|key| var(key).map(parse).ok().flatten());
	Ok((version.next().flatten().ok_or(env::VarError::NotPresent)?,
	    version.next().flatten().unwrap_or(0),
	    version.next().flatten(),
	    var("CARGO_PKG_VERSION_PRE").ok()))
}

pub fn crate_url() -> Result<String> {
	env::var("CARGO_PKG_HOMEPAGE").or_else(|_| env::var("CARGO_PKG_REPOSITORY"))
	                              .map_err(Into::into)
}

pub fn crate_authors() -> Result<String> { env::var("CARGO_PKG_AUTHORS").map_err(Into::into) }

pub fn fam_out_path() -> Result<PathBuf> {
	env::var("OUT_DIR").map(|p| PathBuf::from(p).join(FAM_FILENAME))
	                   .map_err(Into::into)
}


#[cfg(feature = "toml")]
pub fn manifest_toml_from(manifest: &Path) -> Result<metadata::FapMetadata> {
	if fs::try_exists(&manifest)? {
		let source = fs::read_to_string(&manifest)?;
		let mut data = toml::from_str::<metadata::MetadataStandalone>(&source)?.package;
		data.set_defaults()?;
		Ok(data)
	} else {
		Err(IoError::new(IoErrorKind::NotFound, manifest.display().to_string()).into())
	}
}


impl metadata::FapMetadata {
	/// Links all assets such as icon.
	///
	/// Currently only icon.
	///
	pub fn canonicalize_assets<P: AsRef<Path>>(&mut self, root: P) -> Result {
		let icon_override = if let Some(origin) = &self.icon {
			let path = PathBuf::from(&origin);
			let package: PathBuf = env::var("OUT_DIR")?.into();
			let filename = path.file_name()
			                   .ok_or_else(|| IoError::new(IoErrorKind::InvalidFilename, path.display().to_string()))?;

			if path.is_relative() {
				// We cannot link path as-is by first component because of it can contains relative components like `..`.

				// try consolidate or just concatenate with root:
				let full = {
					let concat = root.as_ref().join(&path);
					concat.canonicalize().or_else(|_| Ok::<_, IoError>(concat))?
				};

				soft_link_forced(&full, &package.join(filename))?;
				Some(filename.to_str().unwrap().to_owned())
			} else if path.is_absolute() && fs::try_exists(&path)? {
				soft_link_forced(&path, &package.join(filename))?;
				Some(filename.to_str().unwrap().to_owned())
			} else {
				println!("Asset {origin} has not been canonicalized.");
				None
			}
		} else {
			None
		};


		if icon_override.is_some() {
			self.icon = icon_override;
		}

		Ok(())
	}
}


/// Creates symlink.
///
/// Safety:
/// - operates in `$OUT_DIR` directory only
/// - able to overwrite existing symlink only.
fn soft_link_forced<P: AsRef<Path>>(origin: P, link: P) -> Result {
	assert!(link.as_ref().starts_with(env::var("OUT_DIR")?));

	let existing = fs::try_exists(&link)?;
	let symlink = link.as_ref().is_symlink();

	if !existing && !symlink {
		soft_link(origin, link)
	} else if existing || symlink {
		fs::remove_file(&link)?;
		soft_link(origin, link)
	} else {
		// There's anything but not symlink, so we should not touch it.
		Err(IoError::new(IoErrorKind::AlreadyExists, link.as_ref().display().to_string()).into())
	}
}

#[rustfmt::skip]
fn soft_link<P: AsRef<Path>>(origin: P, link: P) -> Result {
	#[cfg(unix)] use std::os::unix::fs::symlink;
	#[cfg(windows)] use std::os::windows::fs::symlink_dir as symlink;
	symlink(origin.as_ref(), link.as_ref()).map_err(Into::into)
}
