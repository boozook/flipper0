extern crate fam;
extern crate serde;
extern crate serde_json;

use crate::Result;


/// Modifies existing or creates new manifest.
pub fn export_main_to_manifest(entry_point: &str) -> Result {
	let (path, mut manifest) = match fam::IntermediateManifest::from_json_out_dir() {
		Ok(intermediate) => (intermediate.product, intermediate.manifest),
		Err(err) => {
			println!("Unable to load cached manifest. That's ok, will try to create a new one. ({err})");
			let manifest = fam::manifest()?;
			let path = manifest.save_to_out_dir()?;
			(path, manifest)
		},
	};

	if let Some(main) = manifest.main() && main != entry_point && main != fam::DEFAULT_MAIN {
		use std::io::{Error, ErrorKind};
		let message = format!("Entry-point name must be the same as in other source (Crate metadata or Flipper.toml), `{main}` != `{entry_point}`.");
		return Err(box Error::new(ErrorKind::AlreadyExists, message));
	}

	*(manifest.main_mut()) = Some(entry_point.to_owned());
	manifest.save_to(&path)?;
	Ok(())
}
