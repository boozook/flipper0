use std::env;
use std::error::Error;
use std::fs::try_exists;
use std::path::Path;
use std::path::PathBuf;
use bindgen::EnumVariation;
use bindgen::RustTarget;
use semver::VersionReq;

use crate::consts;
use crate::api_table;
use crate::source::doc_sdk_metadata_row;


pub fn sdk_path() -> Result<PathBuf, Box<dyn Error>> {
	println!("cargo:rerun-if-env-changed={}", consts::env::FLIPPER_SDK_PATH_ENV);
	if let Some(sdk_path) = env::var_os(consts::env::FLIPPER_SDK_PATH_ENV) {
		Ok(PathBuf::from(sdk_path))
	} else {
		println!(
		         "cargo:warning=env var `{}` not found, please set it. \
		         You can enable `prebuild` feature to use prebuilt bindings.",
		         consts::env::FLIPPER_SDK_PATH_ENV
		);
		Err(env::VarError::NotPresent.into()).into()
	}
}


pub fn find_arm_toolchain<P: AsRef<Path>>(root: P) -> Result<PathBuf, Box<dyn Error>> {
	let find_arm_toolchain = |root: &Path| {
		println!("Searching ARM toolchain...");

		let glob = wax::Glob::new("toolchain/*/arm-none-eabi").unwrap();
		let result = glob.walk_with_behavior(&root, wax::LinkBehavior::ReadTarget)
		                 .not(["toolchain/.*/arm-none-eabi"])
		                 .map_err(|err| println!("cargo:warning=ERROR: Unable to walk fs. {err}"))
		                 .ok()?
		                 .filter_map(|entry| entry.map_err(|err| println!("cargo:warning=ERROR: {err}")).ok())
		                 .filter(|entry| try_exists(entry.path().join("include")).ok().unwrap_or_default())
		                 .map(|entry| {
			                 println!("Found ARM toolchain: {}", entry.to_candidate_path().to_string());
			                 entry.path().to_owned()
		                 })
		                 .next();
		result
	};

	let path = env::var(consts::env::ARM_TOOLCHAIN_PATH_ENV).map_or_else(
	                                                                |err| {
		                                                                println!("`{}` {err}.", consts::env::ARM_TOOLCHAIN_PATH_ENV);
		                                                                find_arm_toolchain(&root.as_ref())
	                                                                },
	                                                                |path| {
		                                                                let path = PathBuf::from(path);
		                                                                if path.exists() && path.is_dir() {
			                                                                Some(path)
		                                                                } else {
			                                                                println!(
			                                                                         "cargo:warning=`{}` points to non-existing dir.",
			                                                                         consts::env::ARM_TOOLCHAIN_PATH_ENV
			);
			                                                                find_arm_toolchain(&root.as_ref())
		                                                                }
	                                                                },
	);

	path.ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, consts::env::ARM_TOOLCHAIN_PATH_ENV).into())
}


pub fn try_build() -> Result<(), Box<dyn Error>> {
	let pwd = env::current_dir()?;
	let cargo_target_triple = env::var("TARGET").expect("TARGET cargo env var");

	let debug = crate::is_debug();
	let output_filename = crate::bindings_filename(debug);

	let root = sdk_path()?;

	let (sdk_tags, sdk_rev) = validate_sdk(&root)?;

	let (version, symbols) = api_table::find_read_api_table(&root)?;
	crate::check_version(&version.as_deref().unwrap_or("n/a"), consts::support::API_VERSION.parse()?, "API");

	let header = api_table::gen_api_table_header(&symbols)?;
	let extra = get_extra_headers(&symbols)?;
	let exclustions = exclustions([&header, &extra])?;

	let toolchain = find_arm_toolchain(&root)?;

	let builder = || -> bindgen::Builder {
		let mut builder = bindgen::Builder::default().rust_target(RustTarget::Nightly)
		                                             .use_core()
		                                             .ctypes_prefix("core::ffi")
		                                             .size_t_is_usize(true)
		                                             .no_convert_floats()
		                                             .translate_enum_integer_types(true)
		                                             .array_pointers_in_arguments(true)
		                                             .explicit_padding(false)
		                                             .default_enum_style(EnumVariation::Rust { non_exhaustive: true })
		                                             .layout_tests(true)
		                                             .conservative_inline_namespaces()
		                                             .enable_function_attribute_detection()
		                                             .allowlist_recursively(true)
		                                             .detect_include_paths(true)
		                                             .trust_clang_mangling(true)
		                                             .derive_debug(debug)
		                                             .derive_default(crate::feature("derive-default"))
		                                             .derive_copy(crate::feature("derive-copy"))
		                                             .derive_hash(crate::feature("derive-hash"))
		                                             .derive_eq(crate::feature("derive-eq"))
		                                             .derive_ord(crate::feature("derive-ord"))
		                                             .derive_partialeq(crate::feature("derive-partialeq"))
		                                             .derive_partialord(crate::feature("derive-partialord"))
		                                             .generate_comments(true)
		                                             .parse_callbacks(box DeriveCallbacks { exclude: exclustions.clone() });


		// metadata for documentation:
		let metadata = doc_sdk_metadata_row(sdk_rev.as_ref(), &sdk_tags, version.as_ref());
		println!("cargo:rustc-env={}={}", consts::env::BINDINGS_METADATA_DOC_ENV, metadata);
		builder = builder.raw_line(format!("/* {metadata} */"));


		// whitelist api-symbols:
		for sym in symbols.iter() {
			use api_table::ApiTableRow::*;
			match sym {
				Header { name, .. } => builder = builder.allowlist_file(name),
				Variable { name, .. } => builder = builder.allowlist_var(name),
				Function { name, .. } => builder = builder.allowlist_function(name),
			}
		}


		// ARM toolchain
		builder = builder.blocklist_file("stdlib.h")
		                 .clang_args(&["--include-directory", &toolchain.join("include").display().to_string()]);


		// specifically for the target:
		if let "thumbv7em-none-eabihf" = cargo_target_triple.as_ref() {
			builder = builder.clang_args(&["-target", &cargo_target_triple]);
		}


		// extra code-gen for `debug` feature:
		if debug {
			builder = builder.derive_debug(true).clang_args(&["-DFURI_DEBUG", "-DNDEBUG"]);
		} else {
			builder = builder.derive_debug(false).no_debug(".*");
		}
		builder
	};


	let try_build = |(builder, twd /* twd - target working directory */): (bindgen::Builder, Option<PathBuf>)| {
		if let Some(path) = twd {
			env::set_current_dir(&path)?; // pushd
		}

		let bindings = builder.generate();
		env::set_current_dir(&pwd)?; // popd
		bindings.map(|bindings| {
			        let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR cargo env var")).join(output_filename);
			        println!("bindings output: {}", out_path.display());
			        bindings.write_to_file(&out_path).expect("Couldn't write bindings!");
			        println!("cargo:rustc-env={}={}", consts::env::BINDINGS_ENV, out_path.display().to_string());
		        })
		        .map_err(|err| err.into())
	};


	// we have few possible sources:
	// - just headers from entire source code
	// - symbols.h generated by fbt on build
	// - api symbols table csv
	// - exported headers by `fbt sdk_tree`

	from_source(builder, &root, &header, Some(&extra)).and_then(try_build)
	                                                  .or_else(|err| {
		                                                  println!("cargo:warning=build from fw source failed: {err}");
		                                                  from_build(builder, &root, &header, Some(&extra)).and_then(try_build)
	                                                  })
	                                                  .or_else(|err| {
		                                                  println!("cargo:warning=build from fw build failed: {err}");
		                                                  from_sdk_tree(builder, &root, debug).and_then(try_build)
	                                                  })
	                                                  .map_err(|err| {
		                                                  println!("cargo:warning=build from sdk tree failed: {err}");
		                                                  err
	                                                  })?;

	Ok(())
}


pub fn validate_sdk<P: AsRef<Path>>(root: P) -> Result<FwVersion, Box<dyn Error>> {
	println!(
	         "cargo:rerun-if-changed={}",
	         root.as_ref().join(PathBuf::from(".git/HEAD")).display()
	);
	check_version_git(root.as_ref(), consts::support::SDK_VERSION.parse()?, "SDK")
}

// Unfortunately there are no VERSION file yet.
pub fn _check_version_file<S: std::fmt::Display>(path: &Path, supported: VersionReq, name: S) -> Result<(), Box<dyn Error>> {
	let version = std::fs::read_to_string(path)?;
	crate::check_version(&version, supported, name);
	Ok(())
}


/// (tags, commit)
type FwVersion = (Vec<String>, Option<String>);
pub fn check_version_git<S: std::fmt::Display>(root: &Path, supported: VersionReq, name: S) -> Result<FwVersion, Box<dyn Error>> {
	let root = PathBuf::from(root);
	let repo = rustygit::Repository::new(&root);
	let tags = repo.cmd_out(["tag", "--points-at", "HEAD"])?;
	let commit = repo.get_hash(false).ok();
	let version = tags.iter().filter_map(|t| semver::Version::parse(&t).ok()).next();

	if let Some(version) = version {
		println!("cargo:info=Flipper SDK version determined from git tag: {version:}");
		if !supported.matches(&version) {
			println!("cargo:warning={name} version probably not supported. Supported version '{supported}' does not matches {version}.");
		} // else OK
	} else {
		println!("cargo:warning=Flipper SDK version not found");
	}

	Ok((tags, commit))
}


fn get_extra_headers(symbols: &[api_table::ApiTableRow<String>]) -> Result<PathBuf, Box<dyn Error>> {
	let outdir = PathBuf::from(env::var("OUT_DIR")?).join("extras");
	std::fs::create_dir_all(&outdir)?;

	let mut result = String::new();

	result.push_str("#pragma once \n\n #include <gui/icon.h> \n\n");

	for v in symbols.iter()
	                .filter(|s| matches!(s, api_table::ApiTableRow::Variable { mut_ty, .. } if mut_ty.as_str() == "const Icon"))
	{
		let name = v.name();
		let ty = v.ty();
		let row = format!("extern {ty} {name}; \n");
		result.push_str(&row);
	}


	let path = outdir.join("assets_icons.h");
	std::fs::write(&path, result.as_bytes())?;

	Ok(outdir)
}


/// Build list of excluded files such as generated by this crate that cargo do not need to watch.
/// If path points to directory, files in it will be in the result list.
pub fn exclustions<I: IntoIterator<Item = P>, P: AsRef<Path>>(paths: I) -> Result<Vec<PathBuf>, Box<dyn Error>> {
	let res = paths.into_iter()
	               .flat_map(|p| {
		               let def = vec![p.as_ref().to_owned()].into_iter();
		               if p.as_ref().is_dir() {
			               let res = std::fs::read_dir(p.as_ref()).ok().map(|iter| {
				                                                           iter.filter_map(|result| result.ok())
				                                                               .filter(|entry| entry.path().is_file())
				                                                               .filter(|entry| {
					                                                               entry.path()
					                                                                    .extension()
					                                                                    .unwrap_or_default()
					                                                                    .to_str()
					                                                                    .unwrap_or_default()
					                                                                    .to_owned() ==
					                                                               "h"
				                                                               })
			                                                           });
			               if let Some(res) = res {
				               res.map(|entry| entry.path().to_owned()).collect::<Vec<_>>().into_iter()
			               } else {
				               def
			               }
		               } else {
			               def
		               }
	               })
	               .collect();

	Ok(res)
}


// allowing dead-code to ommit unnecessary `Debug` impl for `DeriveCallbacks`
// that needed to `ParseCallbacks`
#[allow(dead_code)]
#[derive(Debug)]
struct DeriveCallbacks<P: AsRef<Path>> {
	exclude: Vec<P>,
}

impl bindgen::callbacks::ParseCallbacks for DeriveCallbacks<PathBuf> {
	fn include_file(&self, filename: &str) {
		let path = PathBuf::from(filename);
		if !self.exclude.contains(&path) {
			bindgen::CargoCallbacks.include_file(filename)
		}
	}
}


fn from_source<P: AsRef<Path>, Builder: FnOnce() -> bindgen::Builder>(
	builder: Builder,
	root: P,
	symbols_header: P,
	extra_include: Option<P>)
	-> Result<(bindgen::Builder, Option<PathBuf>), Box<dyn Error>> {
	let root = root.as_ref();
	let symbols_header = symbols_header.as_ref();
	let mut builder = builder();

	if try_exists(&symbols_header)? {
		builder = builder.header(format!("{}", symbols_header.display()))
		                 .clang_args(&["--include-directory", &root.display().to_string()])
		                 .clang_arg(format!("-I{}", &PathBuf::from("furi").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/f7").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/f7/ble_glue").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/f7/fatfs").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/f7/furi_hal").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/f7/Inc").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("firmware/targets/furi_hal_include").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("applications/services").display()))
		                 .clang_arg(format!("-I{}", &PathBuf::from("applications/main").display()))
		                 .clang_arg(format!("-DSTM32WB"))
		                 .clang_arg(format!("-DSTM32WB55xx"))
		                 .clang_arg(format!("-DUSE_FULL_LL_DRIVER"))
		                 .clang_args(&["-x", "c"])
		                 .clang_arg("-ferror-limit=100000");

		if let Some(include) = extra_include {
			builder = builder.clang_arg(format!("-I{}", &include.as_ref().display()))
		}

		// add lib/*:
		builder = builder.clang_arg(format!("-I{}", &PathBuf::from("lib").display()));
		let libs = std::fs::read_dir(&root.join("lib"))?.filter_map(|result| result.ok())
		                                                .filter(|entry| entry.path().is_dir())
		                                                .filter(|entry| {
			                                                let filename = entry.path()
			                                                                    .file_name()
			                                                                    .unwrap_or_default()
			                                                                    .to_str()
			                                                                    .unwrap_or_default()
			                                                                    .to_owned();
			                                                !filename.starts_with(".") &&
			                                                !filename.to_uppercase().starts_with("STM32") &&
			                                                !filename.to_uppercase().starts_with("FREERTOS")
		                                                });

		for lib in libs {
			let path = PathBuf::from("lib").join(lib.path());
			builder = builder.clang_arg(format!("-I{}", path.display()));
			if try_exists(path.join("include"))? {
				builder = builder.clang_arg(format!("-I{}", path.join("include").display()));
			}
		}

		// XXX: hardcoded extra include-paths:
		let libs = [
		            "STM32CubeWB/Drivers/CMSIS/Device/ST/STM32WBxx/Include",
		            "STM32CubeWB/Drivers/CMSIS/Include",
		            "STM32CubeWB/Drivers/STM32WBxx_HAL_Driver/Inc",
		            "FreeRTOS-Kernel/include",
		            "FreeRTOS-Kernel/portable/GCC/ARM_CM4F",
		            "ST25RFAL002/source/st25r3916",
		            "libusb_stm32/inc",
		];
		for lib in libs {
			let path = PathBuf::from("lib").join(PathBuf::from(lib));
			builder = builder.clang_arg(format!("-I{}", &path.display()));
			if try_exists(path.join("include"))? {
				builder = builder.clang_arg(format!("-I{}", path.join("include").display()));
			}
		}


		Ok((builder, Some(root.to_owned())))
	} else {
		Err(std::io::Error::new(std::io::ErrorKind::NotFound, symbols_header.display().to_string()).into())
	}
}


fn from_build<P: AsRef<Path>, Builder: FnOnce() -> bindgen::Builder>(
	_builder: Builder,
	_root: P,
	_symbols_headerr: P,
	_extra_include: Option<P>)
	-> Result<(bindgen::Builder, Option<PathBuf>), Box<dyn Error>> {
	Err(std::io::Error::other("Not implemented yet").into())
}


fn from_sdk_tree<P: AsRef<Path>, Builder: FnOnce() -> bindgen::Builder>(
	builder: Builder,
	root: P,
	debug: bool)
	-> Result<(bindgen::Builder, Option<PathBuf>), Box<dyn Error>> {
	let dirname = if debug { "f7-firmware-D" } else { "f7-firmware-C" };
	let sdk = root.as_ref().join(PathBuf::from(format!("build/{dirname}/sdk/")));
	let opts = std::fs::read_to_string(sdk.join("sdk.opts"))?;


	let glob = wax::Glob::new("**/*.h").unwrap();
	let headers = glob.walk_with_behavior(&sdk, wax::LinkBehavior::ReadTarget)
	                  .not(["**/.*"])?
	                  .filter_map(|entry| entry.map_err(|err| println!("cargo:warning=ERROR: {err}")).ok())
	                  .map(|entry| entry.path().to_owned());

	let mut builder = builder();
	for path in headers {
		println!("+ {}", path.display().to_string());
		builder = builder.header(path.display().to_string());
	}
	builder = builder.blocklist_file("portmacro.h")
	                 .blocklist_file("FreeRTOS.h")
	                 .clang_args(opts.split_ascii_whitespace().filter(|s| {
		                                                          s.starts_with("-I") || s.starts_with("-D")
		                                                          // || s.starts_with("-m") || s.starts_with("-W")
	                                                          }));


	Ok((builder, Some(sdk)))
}
