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

use manifest::Manifest;
use metadata::Metadata;


// TODO: two possible sources:
// - [ ] flipper.toml near by crate manifest
// - [x] crate metadata in the crate manifest


type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;

const CARGO_MANIFEST: &str = "Cargo.toml";
const TARGET_KIND: &str = "staticlib";

const FAM_FILENAME: &str = "application.fam";
const FLIPPER_TRIPLE: &str = "thumbv7em-none-eabihf";


/// Build Flipper Application Manifest (FAM).
/// Call typically from your build script.
pub fn manifest() -> Result<Manifest> { manifest_named(CARGO_MANIFEST) }

/// With custom manifest file name.
pub fn manifest_named<S: AsRef<str>>(filename: S) -> Result<Manifest> {
	let root = crate_root()?;
	let manifest = root.join(filename.as_ref());
	let name = crate_name()?;

	println!("cargo:rerun-if-changed={}", manifest.display());


	let (target_directory, mut crate_metadata) = {
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
	let target = targets.next().ok_or(IoError::new(IoErrorKind::NotFound, "target"))?;
	if targets.next().is_some() {
		println!("cargo:warning=Multiple targets (libs) does not supported.");
	}

	let fap_metadata = {
		let mut fap = None;
		if let Some(meta) = crate_metadata.metadata.as_mut() {
			if let Some(fap) = meta.fap.as_mut() {
				fap.set_defaults()?;
				fap.sources
				   .insert(0, crate_product(&target_directory, target)?.display().to_string());
			}
			fap = meta.fap.take();
		}
		fap.ok_or(IoError::new(IoErrorKind::NotFound, "fap metadata"))?
	};

	Ok(fap_metadata.into())
}


/// Output directory with compilation product.
/// Typically is `target/{triple}/{profile}/lib{name}.a`.
pub fn crate_product(dir: &Path, target: &meta::Target) -> Result<PathBuf> {
	let triple = env::var("TARGET")?;
	let profile = env::var("PROFILE")?;
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


fn staticlib_name_to_filename(name: &str) -> String { format!("lib{name}.a").replace("-", "_") }


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