use crate::Result;
use serde::Deserialize;
use serde::Serialize;


const DEFAULT_MAIN: &str = "main";
const DEFAULT_TYPE: &str = "FlipperAppType.EXTERNAL";
const DEFAULT_CATEGORY: &str = "Misc";


#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
	pub fap: Option<FapMetadata>,
}


/// Same as `fam::Manifest` but more with optional fields that required there.
#[derive(Serialize, Deserialize, Debug)]
pub struct FapMetadata {
	/// Name that is displayed in menus
	pub name: Option<String>,

	/// String, application id within the build system. Used for specifying which applications to include in build configuration and to resolve dependencies and conflicts.
	/// Default is crate name.
	#[serde(alias = "appid")]
	#[serde(rename = "appid")]
	pub id: Option<String>,

	/// Member of FlipperAppType.* enumeration. Valid values are:
	/// - `SERVICE`: System service, created at early startup
	/// - `SYSTEM`: Application not being shown in any menus. Can be started by other apps or from CLI
	/// - `APP`: Regular application for main menu
	/// - `PLUGIN`: Application to be built as a part of firmware an to be placed in Plugins menu
	/// - `DEBUG`: Application only visible in Debug menu with debug mode enabled
	/// - `ARCHIVE`: One and only Archive app
	/// - `SETTINGS`: Application to be placed in System settings menu
	/// - `STARTUP`: Callback function to run at system startup. Does not define a separate app
	/// - `EXTERNAL`: Application to be built as .fap plugin
	/// - `METAPACKAGE`: Does not define any code to be run, used for declaring dependencies and application bundles
	/// Default is FlipperAppType.EXTERNAL.
	#[serde(alias = "type")]
	#[serde(alias = "apptype")]
	#[serde(rename = "apptype")]
	pub ty: Option<String>,

	/// Stack size, in bytes, to allocate for application on its startup. Note that allocating a stack that is too small for an app to run will cause system crash due to stack overflow, and allocating too much stack space will reduce usable heap memory size for apps to process data. Note: you can use ps and free CLI commands to profile your app's memory usage.
	#[serde(alias = "stack-size")]
	pub stack_size: Option<usize>,

	/// Animated icon name from built-in assets to be used when building app as a part of firmware.
	#[serde(alias = "icon-system")]
	#[serde(alias = "icon_system")]
	#[serde(alias = "icon-builtin")]
	#[serde(rename = "icon")]
	pub icon_builtin: Option<String>,

	/// C function to be used as application's entry point
	// TODO: Default determined via build-script.
	#[serde(rename = "entry_point")]
	#[serde(alias = "entry-point")]
	#[serde(alias = "start")]
	#[serde(alias = "main")]
	pub main: String,

	/// Internal flags for system apps. Do not use.
	#[serde(default)]
	pub flags: Vec<String>,

	/// C preprocessor definitions to declare globally for other apps when current application is included in active build configuration.
	#[serde(default)]
	pub cdefines: Vec<String>,

	/// List of application IDs to also include in build configuration, when current application is referenced in list of applications to build.
	#[serde(default)]
	pub requires: Vec<String>,

	/// List of application IDs that current application conflicts with. If any of them is found in constructed application list, fbt will abort firmware build process.
	#[serde(default)]
	pub conflicts: Vec<String>,

	/// Functionally identical to requires field.
	#[serde(default)]
	pub provides: Vec<String>,

	/// Order of an application within its group when sorting entries in it. The lower the order is, the closer to the start of the list the item is placed. Used for ordering startup hooks and menu entries.
	pub order: Option<isize>,

	/// List of C header files from this app's code to include in API definitions for external applications.
	#[serde(default)]
	#[serde(alias = "sdk-headers")]
	pub sdk_headers: Vec<String>,

	// External FAPs only:
	/// list of strings, file name masks, used for gathering sources within app folder. Default value of ["*.c*"] includes C and CPP source files.
	///
	/// Default determined via build-script.
	/// Put here additional sources.
	#[serde(default)]
	pub sources: Vec<String>,

	/// Tuple, 2 numbers in form of (x,y): application version to be embedded within .fap file. Default value is (0,1), meanig version "0.1".
	/// Default is crate version.
	#[serde(alias = "fap-version")]
	#[serde(alias = "fap_version")]
	#[serde(rename = "fap_version")]
	pub version: Option<Vec<usize>>,
	// version: Option<(String, String)>,
	/// Path to icon relative to the crate manifest directory (crate root).
	/// Name of a .png file, 1-bit color depth, 10x10px, to be embedded within .fap file.
	#[serde(alias = "fap-icon")]
	#[serde(rename = "fap_icon")]
	pub icon: Option<String>,

	/// List of extra libraries to link application against. Provides access to extra functions that are not exported as a part of main firmware at expense of increased .fap file size and RAM consumption.
	#[serde(default)]
	#[serde(alias = "fap-libs")]
	#[serde(rename = "fap_libs")]
	pub libs: Vec<String>,

	/// String, may be empty. App subcategory, also works as path of FAP within apps folder in the file system.
	/// Default is `Misc`
	#[serde(alias = "fap-category")]
	#[serde(rename = "fap_category")]
	pub category: Option<String>,

	/// String, may be empty. Short application description.
	/// Default is crate description.
	#[serde(alias = "fap-description")]
	#[serde(rename = "fap_description")]
	pub description: Option<String>,

	/// String, may be empty. Application's author.
	#[serde(alias = "fap-author")]
	#[serde(rename = "fap_author")]
	pub author: Option<String>,

	/// String, may be empty. Application's homepage.
	#[serde(alias = "fap-weburl")]
	#[serde(rename = "fap_weburl")]
	pub url: Option<String>,
}


impl Default for FapMetadata {
	fn default() -> Self {
		let version = crate::crate_version_parts().map(|(major, minor, patch, pre)| {
			                                          let mut result = vec![major, minor];
			                                          if let Some(v) = patch {
				                                          result.push(v);
			                                          }
			                                          if let Some(pre) = pre {
				                                          println!("cargo:warning=Lats component of the crate version will be ignored: {pre}");
			                                          }
			                                          result
		                                          })
		                                          .map_err(|err| println!("cargo:warning=Failed to get crate version: {err}"))
		                                          .ok();

		Self { id: crate::crate_name().ok(),
		       ty: Some(DEFAULT_TYPE.to_owned()),
		       version,
		       description: crate::crate_descr().ok()
		                                        .map(|s| s.trim().to_owned())
		                                        .filter(|s| !s.is_empty()),
		       name: crate::crate_name().ok(),
		       main: DEFAULT_MAIN.to_owned(),
		       icon: None,
		       category: Some(DEFAULT_CATEGORY.to_owned()),
		       stack_size: None,
		       icon_builtin: None,
		       sources: Default::default(),
		       flags: Default::default(),
		       cdefines: Default::default(),
		       requires: Default::default(),
		       conflicts: Default::default(),
		       provides: Default::default(),
		       order: None,
		       sdk_headers: Default::default(),
		       libs: Default::default(),
		       author: crate::crate_authors().ok(),
		       url: crate::crate_url().ok() }
	}
}


impl FapMetadata {
	/// Set default values to fields that `None`.
	pub fn set_defaults(&mut self) -> Result {
		let mut def = Self::default();

		if self.version.is_none() {
			self.version = def.version;
		}

		if self.description.is_none() {
			self.description = def.description;
		}

		if self.id.is_none() {
			self.id = def.id;
		}

		if self.ty.is_none() {
			self.ty = def.ty;
		}

		if self.sources.is_empty() {
			self.sources = def.sources;
		} else {
			self.sources.append(&mut def.sources)
		}

		if self.author.is_none() {
			self.author = def.author;
		}
		if self.url.is_none() {
			self.url = def.url;
		}

		if self.icon.is_none() {
			self.icon = def.icon;
		}

		if self.category.is_none() {
			self.category = def.category;
		}

		Ok(())
	}
}
