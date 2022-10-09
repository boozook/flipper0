extern crate fam;

use std::path::Path;
use std::path::PathBuf;
use crate::Result;
use crate::metadata::FapMetadata;
use serde_json::Value;
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Clone)]
pub enum Manifest {
	Metadata(FapMetadata),
	Manifest(fam::Manifest),
}


impl From<FapMetadata> for Manifest {
	fn from(metadata: FapMetadata) -> Self { Self::Metadata(metadata) }
}

impl From<fam::Manifest> for Manifest {
	fn from(manifest: fam::Manifest) -> Self { Self::Manifest(manifest) }
}

impl From<Value> for Manifest {
	fn from(metadata: Value) -> Self { Self::Manifest(metadata.into()) }
}


impl Manifest {
	pub fn id(&self) -> Option<&str> {
		match self {
			Manifest::Metadata(metadata) => metadata.id.as_deref(),
			Manifest::Manifest(manifest) => manifest.appid(),
		}
	}

	#[cfg(feature = "optional_entry_point")]
	pub fn main(&self) -> Option<&str> {
		match self {
			Manifest::Metadata(metadata) => metadata.main.as_deref(),
			Manifest::Manifest(manifest) => manifest.entry_point(),
		}
	}

	#[cfg(not(feature = "optional_entry_point"))]
	pub fn main(&self) -> &str {
		match self {
			Manifest::Metadata(metadata) => &metadata.main,
			Manifest::Manifest(manifest) => manifest.entry_point(),
		}
	}

	#[cfg(feature = "optional_entry_point")]
	pub fn main_mut(&mut self) -> &mut Option<String> {
		match self {
			Manifest::Metadata(metadata) => &mut metadata.main,
			Manifest::Manifest(_manifest) => todo!(),
		}
	}


	pub fn save_to_out_dir(&self) -> Result<PathBuf> { crate::fam_out_path().and_then(|path| self.save_to(&path).map(|_| path)) }

	pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result {
		let result = std::fs::write(&path, self.to_python_string()?).map_err(Into::into);

		{
			// Also we should save it in json format for internal purposes
			// such as using in next build steps like in flipper0-macro crate.
			// And we must use `crate::fam_out_path()` for it.
			// But also we should not fail entire build process if this thing fails, just warn.
			fn warn<S: std::fmt::Display>(message: S) { println!("cargo:warning=[fap.json]: {message}") }
			self.to_json_out_dir(&path).map_err(warn).ok();
		}
		result
	}


	fn to_python_string(&self) -> Result<String> {
		let source = match self {
			Manifest::Manifest(manifest) => manifest.try_to_string()?,
			Manifest::Metadata(metadata) => fam::render_raw_json(&serde_json::to_value(&metadata)?)?,
		};

		Ok(source)
	}


	#[doc(hidden)]
	pub fn to_json_out_dir<P: AsRef<Path>>(&self, fam_path: P) -> Result<()> {
		let product = IntermediateManifest::new(&self, fam_path.as_ref().to_owned());
		let path = IntermediateManifest::path()?;
		std::fs::write(path, &product.to_json_string()?).map_err(Into::into)
	}
}


#[doc(hidden)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct IntermediateManifest<T = Manifest> {
	/// Manifest in json
	pub manifest: T,
	/// Path of exported fam
	pub product: PathBuf,
}

impl<'a> IntermediateManifest<&'a Manifest> {
	fn new(manifest: &'a Manifest, product: PathBuf) -> Self { Self { manifest, product } }
	pub fn to_json_string(&self) -> Result<String> { serde_json::to_string(&self).map_err(Into::into) }
}

impl IntermediateManifest<Manifest> {
	pub fn from_json_out_dir() -> Result<IntermediateManifest<Manifest>> {
		let path = Self::path()?;
		let file = std::fs::File::open(path)?;
		serde_json::from_reader(file).map_err(Into::into)
	}
}

impl IntermediateManifest {
	pub fn path() -> Result<PathBuf> {
		use std::io::{Error as IoError, ErrorKind as IoErrorKind};

		let mut path = crate::fam_out_path()?;
		path = if path.set_extension("json") {
			Ok(path)
		} else {
			Err(IoError::new(IoErrorKind::Other, "Unable to set extension."))
		}?;
		Ok(path)
	}
}
