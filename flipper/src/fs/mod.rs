// TODO: Read & fix documentation.

use core::ptr::NonNull;
use crate::AsPtr;
use crate::error::NullPointerError;
use crate::path::Path;
use sys::ffi;
use sys::ffi::{FS_AccessMode, FS_OpenMode};
pub use sys::error::fs::Error;

pub use record::{Record, RecordExt};
pub use dir_read::ReadDir;
pub use dir_walk::WalkDir;


pub mod record;
pub mod dir_read;
pub mod dir_walk;


pub type Result<T, E = Error> = sys::result::Result<T, E>;
const FILE_NAME_LEN_MAX: u16 = 256;


// XXX: use Unique<T> instead?
pub struct Storage<T: Record = record::Storage>(NonNull<ffi::Storage>, record::CloseOnDrop<T>) where [(); T::LEN]: Sized;


impl Storage<record::Storage> {
	pub fn open_default() -> Result<Self, NullPointerError> { Self::open(record::Storage) }
}


impl<T: Record> Storage<T> where [(); T::LEN]: Sized {
	/// Allocate & open `Storage` instance.
	/// Returns `None` if got null pointer by API.
	pub fn open(record: T) -> Result<Storage<T>, NullPointerError> {
		let raw = unsafe { ffi::furi_record_open(record.tag().as_ptr() as _) };
		let p = NonNull::new(raw as *mut ffi::Storage);
		p.map(|p| Self(p, record::CloseOnDrop::new(record))).ok_or(NullPointerError)
	}

	#[inline]
	pub fn close(self) { core::mem::drop(self.1); }


	pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<MetadataOwned> {
		let path = path.as_ref();
		let mut result;

		unsafe {
			result = core::mem::zeroed();
			ffi::storage_common_stat(self.0.as_ptr(), path.as_ptr() as _, &mut result)?;
		}

		Ok(MetadataOwned(result))
	}

	pub fn remove<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		let path = path.as_ref();
		unsafe {
			ffi::storage_common_remove(self.0.as_ptr(), path.as_ptr() as _)?;
		}
		Ok(())
	}

	/// Move or rename a file or directory.
	pub fn rename<P: AsRef<Path>>(&self, from: P, to: P) -> Result<()> {
		let from = from.as_ref();
		let to = to.as_ref();
		unsafe {
			ffi::storage_common_rename(self.0.as_ptr(), from.as_ptr() as _, to.as_ptr() as _)?;
		}
		Ok(())
	}

	pub fn copy<P: AsRef<Path>>(&self, from: P, to: P) -> Result<()> {
		let from = from.as_ref();
		let to = to.as_ref();
		unsafe {
			ffi::storage_common_copy(self.0.as_ptr(), from.as_ptr() as _, to.as_ptr() as _)?;
		}
		Ok(())
	}

	/// Copy one folder contents into another with rename of all conflicting files.
	pub fn merge<P: AsRef<Path>>(&self, from: P, to: P) -> Result<()> {
		let from = from.as_ref();
		let to = to.as_ref();
		unsafe {
			ffi::storage_common_merge(self.0.as_ptr(), from.as_ptr() as _, to.as_ptr() as _)?;
		}
		Ok(())
	}

	pub fn create_dir<P: AsRef<Path>>(&self, from: P) -> Result<()> {
		let from = from.as_ref();
		unsafe {
			ffi::storage_common_mkdir(self.0.as_ptr(), from.as_ptr() as _)?;
		}
		Ok(())
	}

	/// File-system information.
	/// ```ignore
	/// use flipper0::fs::{Storage, Path};
	/// Storage::default().info(Path::from_ptr(flipper0::ffi::STORAGE_EXT_PATH_PREFIX.as_ptr() as _));
	/// ```
	pub fn info<P: AsRef<Path>>(&self, fs: P) -> Result<StorageInfo> {
		let path = fs.as_ref();
		let mut total: u64 = 0;
		let mut free: u64 = 0;
		unsafe {
			ffi::storage_common_fs_info(self.0.as_ptr(), path.as_ptr() as _, &mut total, &mut free)?;
		}
		Ok(StorageInfo { total_space: total,
		                 free_space: free })
	}


	/// Returns error for latest operation with specified `file`.
	/// Will be `Ok` if there is no error.
	pub fn last_error_for(file: &File) -> Result<()> {
		unsafe { ffi::storage_file_get_error(file.as_ptr()) }?;
		Ok(())
	}
}


#[derive(Debug)]
pub struct StorageInfo {
	pub total_space: u64,
	pub free_space: u64,
}


impl<T: Record> const crate::AsPtr<ffi::Storage> for Storage<T> where [(); T::LEN]: Sized {
	#[inline]
	fn as_ptr(&self) -> *mut ffi::Storage { self.0.as_ptr() }
}


/// Allocated `File` associated with `Storage`.
pub struct File(NonNull<ffi::File>);

impl File {
	/// Allocates `File` instance.
	/// Returns `None` if got null pointer by API.
	pub fn new<T: Record>(storage: &Storage<T>) -> Option<Self>
		where [(); T::LEN]: Sized {
		unsafe {
			let raw = ffi::storage_file_alloc(storage.as_ptr());
			NonNull::new(raw).map(Self)
		}
	}


	pub fn is_open(&self) -> bool { unsafe { ffi::storage_file_is_open(self.as_ptr()) } }
	pub fn is_dir(&self) -> bool { unsafe { ffi::storage_file_is_dir(self.as_ptr()) } }


	pub fn open<P: AsRef<Path>>(&mut self, path: P, access_mode: FS_AccessMode, open_mode: FS_OpenMode) -> Result<()> {
		let path = path.as_ref();
		unsafe {
			if ffi::storage_file_open(self.as_ptr(), path.as_ptr() as _, access_mode, open_mode) {
				Ok(())
			} else {
				ffi::storage_file_get_error(self.as_ptr())?;
				// should be unreachable
				unreachable!();
			}
		}
	}

	pub fn close<P: AsRef<Path>>(&mut self) -> Result<()> {
		unsafe {
			if ffi::storage_file_close(self.as_ptr()) {
				Ok(())
			} else {
				ffi::storage_file_get_error(self.as_ptr())?;
				// should be unreachable
				unreachable!();
			}
		}
	}


	pub fn read(&mut self, buf: &mut [u8]) -> Result<u16> {
		unsafe { Ok(ffi::storage_file_read(self.as_ptr(), buf.as_mut_ptr() as _, buf.len() as _)) }
	}

	pub fn write(&mut self, buf: &[u8]) -> Result<u16> {
		unsafe { Ok(ffi::storage_file_write(self.as_ptr(), buf.as_ptr() as _, buf.len() as _)) }
	}


	pub fn seek(&mut self, offset: u32, from_start: bool) -> Result<()> {
		unsafe {
			if ffi::storage_file_seek(self.as_ptr(), offset, from_start) {
				Ok(())
			} else {
				ffi::storage_file_get_error(self.as_ptr())?;
				// should be unreachable
				unreachable!();
			}
		}
	}

	pub fn tell(&mut self) -> u64 { unsafe { ffi::storage_file_tell(self.as_ptr()) } }
	pub fn size(&mut self) -> u64 { unsafe { ffi::storage_file_size(self.as_ptr()) } }
	pub fn eof(&mut self) -> bool { unsafe { ffi::storage_file_eof(self.as_ptr()) } }


	pub fn truncate(&mut self) -> Result<()> {
		unsafe {
			if ffi::storage_file_truncate(self.as_ptr()) {
				Ok(())
			} else {
				ffi::storage_file_get_error(self.as_ptr())?;
				// should be unreachable
				unreachable!();
			}
		}
	}


	/// Same as flush.
	pub fn sync(&mut self) -> Result<()> {
		unsafe {
			if ffi::storage_file_sync(self.as_ptr()) {
				Ok(())
			} else {
				ffi::storage_file_get_error(self.as_ptr())?;
				// should be unreachable
				unreachable!();
			}
		}
	}
}

impl const crate::AsPtr<ffi::File> for File {
	#[inline]
	fn as_ptr(&self) -> *mut ffi::File { self.0.as_ptr() }
}

impl core::fmt::Write for File {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		let bytes = s.as_bytes();
		if bytes.len() != self.write(bytes).map_err(|_| core::fmt::Error)? as usize {
			Err(core::fmt::Error)
		} else {
			Ok(())
		}
	}
}

/// Very gentle drop with close & dealloc.
impl Drop for File {
	fn drop(&mut self) {
		unsafe {
			let ptr = self.as_ptr();
			if !ptr.is_null() {
				let success = if ffi::storage_file_is_dir(ptr) {
					ffi::storage_dir_close(ptr)
				} else {
					ffi::storage_file_close(ptr)
				};

				// if `close` failed, then we need to destroy the file descriptor
				if !success && !ptr.is_null() {
					ffi::storage_file_free(ptr);
				}
			}
		}
	}
}


/// Information about a file.
///
/// This structure is returned from the [`metadata`] and represents known
/// metadata about a file such as its size.
#[derive(Clone)]
pub struct Metadata<'a>(&'a ffi::FileInfo);

pub struct MetadataOwned(ffi::FileInfo);
impl MetadataOwned {
	pub fn size(&self) -> u64 { self.0.size }
	pub fn is_dir(&self) -> bool {
		// XXX: use `ffi::FS_Flags::FSF_DIRECTORY`
		const FSF_DIRECTORY: u8 = 1 << 0;
		self.0.flags & FSF_DIRECTORY != 0
	}

	pub fn borrow(&self) -> Metadata { Metadata(&self.0) }
}

impl Metadata<'_> {
	pub fn size(&self) -> u64 { self.0.size }
	pub fn is_dir(&self) -> bool {
		// XXX: use `ffi::FS_Flags::FSF_DIRECTORY`
		const FSF_DIRECTORY: u8 = 1 << 0;
		self.0.flags & FSF_DIRECTORY != 0
	}

	pub fn to_owned(&self) -> MetadataOwned
		where ffi::FileInfo: Clone {
		MetadataOwned(self.0.clone())
	}
}

impl<'a> core::fmt::Display for Metadata<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let kind = if self.is_dir() { "f" } else { "d" };
		write!(f, "({kind}, size: {:?})", self.size())
	}
}

impl<'a> core::fmt::Debug for Metadata<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let kind = if self.is_dir() { "file" } else { "directory" };
		f.debug_struct("Metadata")
		 .field("kind", &kind)
		 .field("size", &self.size())
		 .finish()
	}
}
