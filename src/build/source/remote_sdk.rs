use std::env;
use std::error::Error;
use std::fs::try_exists;
use std::path::Path;
use std::path::PathBuf;
use crate::consts;
use crate::source::local_sdk::find_arm_toolchain;


const URI: &'static str = "https://github.com/flipperdevices/flipperzero-firmware.git";


pub fn git_clone_sdk() -> Result<PathBuf, Box<dyn Error>> {
	let rev = env::var(consts::env::FLIPPER_NET_SDK_REV_ENV).ok();
	let branch = env::var(consts::env::FLIPPER_NET_SDK_BRANCH_ENV).ok();

	println!("cargo:rerun-if-env-changed={}", consts::env::FLIPPER_NET_SDK_REV_ENV);
	println!("cargo:rerun-if-env-changed={}", consts::env::FLIPPER_NET_SDK_BRANCH_ENV);
	println!("cargo:rerun-if-env-changed={}", consts::env::FLIPPER_NET_SDK_PATH);

	let path = PathBuf::from(env::var("OUT_DIR")?).join("flipperzero-firmware");

	let repo = if try_exists(&path)? {
		println!("SDK repo clone aleady exists");
		rustygit::Repository::new(&path)
	} else {
		println!("Cloning SDK repository...");
		let repo = rustygit::Repository::clone(URI.parse()?, &path)?;
		println!("Fetching submodules...");
		repo.cmd(["fetch", "--recurse-submodules", "-j6"])?;
		println!("SDK repository cloned successfully");
		repo
	};


	if let Some(branch) = branch.as_deref() {
		println!("SDK repo checkout branch: {branch}...");
		repo.cmd_out(["checkout", branch, "--recurse-submodules"])?;
		println!("SDK repo checkout complete");
	}

	if let Some(revision) = rev.as_deref() {
		println!("SDK repo checkout revision: {revision}...");
		repo.cmd_out(["checkout", revision, "--recurse-submodules"])?;
		println!("SDK repo checkout complete");
	}

	try_setup(&path)?;

	Ok(PathBuf::from(path))
}


/// Initial setup.
/// Run fbt, it downloads arm-toolchain if needed
fn try_setup<P: AsRef<Path>>(root: P) -> Result<(), Box<dyn Error>> {
	use std::process::Command;

	// check toolchain first
	let orig_flipper_sdk_path = env::var_os(consts::env::FLIPPER_SDK_PATH_ENV);
	env::set_var(consts::env::FLIPPER_SDK_PATH_ENV, root.as_ref());
	let toolchain_not_found = find_arm_toolchain(&root).is_err();
	if let Some(var) = orig_flipper_sdk_path {
		env::set_var(consts::env::FLIPPER_SDK_PATH_ENV, var)
	} else {
		env::remove_var(consts::env::FLIPPER_SDK_PATH_ENV)
	}

	if !toolchain_not_found {
		println!("SDK and toolchain already exists");
		return Ok(());
	}

	// run fbt:
	if toolchain_not_found && !try_exists(&root.as_ref().join("toolchain/"))? {
		println!("Calling fbt...");
		let target_opts = if crate::is_debug() {
			("COMPACT=1", "DEBUG=0")
		} else {
			("COMPACT=0", "DEBUG=1")
		};

		Command::new("./fbt").args([target_opts.0, target_opts.1])
		                     .current_dir(&root)
		                     .status()
		                     .map_err(|err| {
			                     println!("cargo:warning=fbt initial run failed: {err}");
			                     std::io::Error::other(err)
		                     })?
		                     .exit_ok()?;
	}

	Ok(())
}
