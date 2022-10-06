extern crate fam;

use std::path::Path;
use std::path::PathBuf;
use crate::Result;
use crate::metadata::FapMetadata;
use serde_json::Value;


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
			Manifest::Metadata(ref metadata) => metadata.id.as_deref(),
			Manifest::Manifest(ref manifest) => manifest.appid(),
		}
	}


	pub fn save_to_out_dir(&self) -> Result<PathBuf> { crate::fam_out_path().and_then(|path| self.save_to(&path).map(|_| path)) }

	pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result { std::fs::write(&path, self.try_to_string()?).map_err(Into::into) }


	pub fn try_to_string(&self) -> Result<String> {
		let source = match self {
			Manifest::Manifest(manifest) => manifest.try_to_string()?,
			Manifest::Metadata(metadata) => fam::render_raw(&serde_json::to_value(&metadata)?)?,
		};

		Ok(source)
	}
}
