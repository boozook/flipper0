#![crate_type = "staticlib"]
#![feature(error_in_core)]
#![no_main]
#![no_std]


#[macro_use]
extern crate flipper0;
extern crate alloc;

use core::ffi::CStr;
use alloc::boxed::Box;
use flipper0::fs::{Metadata, Storage};
use flipper0::path::{PathBuf, Path};


type Result<T = (), E = Box<dyn core::error::Error>> = core::result::Result<T, E>;


#[main]
pub unsafe fn main() -> Result {
	// Create, open FS interface:
	let storage = Storage::open_default()?;

	// Get info for external flash (SD):
	let info = storage.info(Path::from_ptr(flipper0::ffi::STORAGE_EXT_PATH_PREFIX.as_ptr() as _));
	println!("Storage info: {info:#?}");

	// There are simple read_dir example with custom buffer size using `with_buf_len`.
	//
	// Create path from null-terminated string:
	// let path = PathBuf::from_vec_with_nul_unchecked(b"/ext/apps/Misc\0".to_vec());
	let path = PathBuf::from_vec_with_nul_unchecked(b"/ext/apps\0".to_vec());
	println!("read_dir: {path:?}");

	for entry in storage.read_dir(path)?.with_buf_len::<64>().with_info() {
		match entry {
			Ok(entry) => println!("  entry: {entry}"),
			Err(err) => println!("  err: {err}"),
		}
	}


	// Now same but recursive with filter, using walk_dir.
	//
	// Create path from null-terminated string:
	let path = PathBuf::from_vec_with_nul_unchecked(b"/ext/apps\0".to_vec());
	println!("walk_dir: {path:?}");
	// Create filter predicate (name, info) -> bool:
	let filter = |name: &CStr, info: Option<Metadata>| {
		println!(" [filter]: {name:?}, info: {info:#?}");
		true
	};

	for entry in storage.walk_dir(path)?.recursive(true).filter(filter) {
		match entry {
			Ok(entry) => println!("  entry: {entry}"),
			Err(err) => println!("  err: {err}"),
		}
	}


	println!("complete");
	Ok(())
}
