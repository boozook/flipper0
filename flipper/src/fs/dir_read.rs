use core::{ptr::NonNull, ffi::CStr};
use crate::AsPtr;
use sys::ffi;
use sys::error::fs::Error;
use sys::error::fs::Status;
use super::record::Record;
use super::Storage;
use super::Metadata;
use super::File;
use super::Path;
use super::FILE_NAME_LEN_MAX;
use super::Result;


/// Iterator over the entries in a directory.
///
/// This iterator is returned from the [`Storage::read_dir`] function and
/// will yield instances of <code>[crate::fs::Result]<[Entry]></code>. Through a [`Entry`]
/// information like the entry's filename and possibly other metadata can be
/// learned.
///
///
/// # Difference from same thing in the std
///
/// [ReadDir::rewind] methods restarts the iterator.
///
/// [Entry] have no `path`, only filename and metadata.
///
/// Metadata is optional and disabled by default, can be enabled with [ReadDir::with_info] method.
///
/// Entries contains buffer for the filename string,
/// its length can be set with <code>[ReadDir::with_buf_len]<usize></code> method.
///
///
/// # Errors
///
/// This [`crate::fs::Result`] will be an [`Err`] if there's some sort of intermittent
/// IO/FS error during iteration.
///
/// Also returns <code>Err([crate::fs::Error::Internal])</code> when received null-pointer from the underneath cffi API.
pub struct ReadDir<const NAME_BUF_LEN: usize = { FILE_NAME_LEN_MAX as usize }, const WITH_INFO: bool = false>(File);


impl<T: Record> Storage<T> where [(); T::LEN]: Sized {
	/// Note: `path` of directory must not ends with a trailing slash.
	pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
		let path = path.as_ref();

		unsafe {
			let dir = ffi::storage_file_alloc(self.0.as_ptr());

			// TODO: remove trailing slash from path
			if ffi::storage_dir_open(dir, path.as_ptr() as _) {
			} else {
				ffi::storage_file_get_error(dir)?;
			}

			NonNull::new(dir).map(File).ok_or(Error::Internal).map(ReadDir::new)
		}
	}
}


impl<const NAME_BUF_LEN: usize, const WITH_INFO: bool> ReadDir<NAME_BUF_LEN, WITH_INFO> {
	pub const fn new(file: File) -> Self { Self(file) }
}
impl<const NAME_BUF_LEN: usize> ReadDir<NAME_BUF_LEN, false> {
	pub fn with_info(self) -> ReadDir<NAME_BUF_LEN, true> { ReadDir(self.0) }
}
impl<const CURRENT_BUF_LEN: usize, const WITH_INFO: bool> ReadDir<CURRENT_BUF_LEN, WITH_INFO> {
	pub fn with_buf_len<const LEN: usize>(self) -> ReadDir<LEN, WITH_INFO> { ReadDir(self.0) }
}


impl<const LEN: usize, const INFO: bool> ReadDir<LEN, INFO> {
	pub fn rewind(&mut self) -> Result<(), sys::error::fs::Error> {
		let ptr = self.0.as_ptr();
		unsafe {
			if ffi::storage_dir_rewind(ptr) {
				Ok(())
			} else {
				Ok(ffi::storage_file_get_error(ptr)?)
			}
		}
	}
}


impl<const LEN: usize> Iterator for ReadDir<LEN, true> {
	type Item = Result<Entry<ffi::FileInfo, LEN>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut entry = Entry::default();

		unsafe {
			let success = ffi::storage_dir_read(
			                                    self.0.as_ptr(),
			                                    (&mut entry.info) as *mut _,
			                                    entry.name.as_mut_ptr() as _,
			                                    LEN as u16,
			);

			if success {
				Some(Ok(entry))
			} else {
				match ffi::storage_file_get_error(self.0.as_ptr()) {
					Status::FSE_OK | Status::FSE_NOT_EXIST => None,
					err => Some(Err(err.into())),
				}
			}
		}
	}
}

impl<const LEN: usize> Iterator for ReadDir<LEN, false> {
	type Item = Result<Entry<(), LEN>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut entry = Entry::default();

		unsafe {
			let success = ffi::storage_dir_read(
			                                    self.0.as_ptr(),
			                                    core::ptr::null_mut(),
			                                    entry.name.as_mut_ptr() as _,
			                                    LEN as u16,
			);

			if success {
				Some(Ok(entry))
			} else {
				match ffi::storage_file_get_error(self.0.as_ptr()) {
					Status::FSE_OK | Status::FSE_NOT_EXIST => None,
					err => Some(Err(err.into())),
				}
			}
		}
	}
}


/// Entries returned by the [`ReadDir`] iterator.
///
/// An instance of `Entry` represents an entry inside of a directory on the
/// filesystem. Each entry can be inspected via methods to learn about the filename
/// or possibly other metadata.
#[derive(Debug)]
pub struct Entry<Info, const LEN: usize> {
	name: [u8; LEN],
	info: Info,
}


impl<Info, const LEN: usize> Entry<Info, LEN> {
	pub fn file_name(&self) -> Result<&CStr, core::ffi::FromBytesUntilNulError> { CStr::from_bytes_until_nul(self.name.as_slice()) }
}


impl<const LEN: usize> Entry<ffi::FileInfo, LEN> {
	pub fn metadata(&self) -> Metadata { Metadata(&self.info) }
}

impl<Info, const LEN: usize> Default for Entry<Info, LEN> {
	fn default() -> Self {
		Self { name: [0; LEN],
		       info: unsafe { core::mem::zeroed() } }
	}
}

impl<Info, const LEN: usize> core::fmt::Display for Entry<Info, LEN> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:?}", self.file_name().map_err(|_| core::fmt::Error)?,)
	}
}
