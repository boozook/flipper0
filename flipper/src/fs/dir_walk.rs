use core::{ptr::NonNull, ffi::CStr};
use crate::alloc::boxed::Box;
use crate::AsPtr;
use crate::error::NullPointerError;
use crate::string::OsString;
use sys::ffi;
use sys::error::fs::Error;
use sys::error::fs::Status;
use super::record::Record;
use super::Storage;
use super::Metadata;
use super::Path;
use super::Result;


struct CloseOnDrop(NonNull<ffi::DirWalk>);
impl Drop for CloseOnDrop {
	fn drop(&mut self) {
		unsafe {
			ffi::dir_walk_close(self.0.as_ptr());
			ffi::dir_walk_free(self.0.as_ptr())
		}
	}
}
impl crate::AsPtr<ffi::DirWalk> for CloseOnDrop {
	#[inline]
	fn as_ptr(&self) -> *mut ffi::DirWalk { self.0.as_ptr() }
}


/// Iterator over the entries in a directory __recursively__.
///
/// This iterator is returned from the [`Storage::walk_dir`] function and
/// will yield instances of <code>[crate::fs::Result]<[Entry]></code>. Through a [`Entry`]
/// information like the entry's path and possibly other metadata can be
/// learned.
///
/// The order in which this iterator returns entries is platform and filesystem
/// dependent.
///
///
/// # Differences
///
/// Recursive traversal of directories is enabled by default,
/// but it can be set with [`WalkDir::recursive`] method.
///
/// Entries can be filtered before instantiating with filter using [`WalkDir::with_filter`] method.
///
/// Metadata is optional and disabled by default, can be enabled with [WalkDir::with_info] method.
///
///
/// # Errors
///
/// This [`crate::fs::Result`] will be an [`Err`] if there's some sort of intermittent
/// IO error during iteration.
///
/// Also returns <code>Err([crate::fs::Error::Internal])</code> when received null-pointer from the underneath cffi API.
pub struct WalkDir<const WITH_INFO: bool = false>(CloseOnDrop);


impl<T: Record> Storage<T> where [(); T::LEN]: Sized {
	/// Note: `path` of directory must not ends with a trailing slash.
	pub fn walk_dir<P: AsRef<Path>>(&self, path: P) -> Result<WalkDir> {
		let path = path.as_ref();

		unsafe {
			let dir = ffi::dir_walk_alloc(self.0.as_ptr());

			// TODO: remove trailing slash from path
			if ffi::dir_walk_open(dir, path.as_ptr() as _) {
				crate::println!("dir_walk_open success");
			} else {
				crate::println!("dir_walk_open FAIL");
				ffi::dir_walk_get_error(dir)?;
			}

			WalkDir::new(dir)
		}
	}
}


/// `Fn( name: &CStr, info: Option<Metadata> ) -> bool`
pub trait FilterFn = Fn(&CStr, Option<Metadata>) -> bool;


impl<const WITH_INFO: bool> WalkDir<WITH_INFO> {
	pub fn new(descriptor: *mut ffi::DirWalk) -> Result<Self> {
		Ok(Self(NonNull::new(descriptor).map(CloseOnDrop).ok_or(Error::Internal)?))
	}


	pub fn recursive(self, value: bool) -> Self {
		unsafe { ffi::dir_walk_set_recursive(self.0.as_ptr(), value) }
		self
	}


	#[inline]
	pub fn filter<F: FilterFn>(self, filter: F) -> Self { self.with_filter(Some(filter)) }

	pub fn with_filter<F: FilterFn>(self, filter: Option<F>) -> Self {
		unsafe extern "C" fn filter_proxy<F: FilterFn>(name: *const core::ffi::c_char,
		                                               info: *mut ffi::FileInfo,
		                                               ctx: *mut core::ffi::c_void)
		                                               -> bool {
			if ctx.is_null() {
				return false;
			};

			let f: Box<F> = Box::from_raw(ctx as _);
			let name = CStr::from_ptr(name);
			let info = info.as_ref().map(Metadata);
			f(name, info)
		}

		unsafe {
			let (filter, ctx) = if let Some(filter) = filter {
				let ctx = Box::into_raw(Box::new(filter));
				(Some(filter_proxy::<F> as _), ctx)
			} else {
				(None, core::ptr::null_mut())
			};

			ffi::dir_walk_set_filter_cb(self.0.as_ptr(), filter, ctx as _);
		}

		self
	}
}


impl WalkDir<false> {
	pub fn with_info(self) -> WalkDir<true> { WalkDir(self.0) }
}


impl<T: Sized, const WITH_INFO: bool> WalkDir<WITH_INFO> where Self: Iterator<Item = Result<Entry<T>>> {
	#[inline(always)]
	fn _next(&mut self, path: &mut OsString, info: *mut ffi::FileInfo) -> Option<Result<()>> {
		unsafe {
			let result = ffi::dir_walk_read(self.0.as_ptr(), path.as_ptr(), info);

			use ffi::DirWalkResult::*;
			match result {
				DirWalkOK => Some(Ok(())),
				DirWalkLast => None,
				// DirWalkError:
				_ => {
					match ffi::dir_walk_get_error(self.0.as_ptr()) {
						Status::FSE_OK => None,
						err => Some(Err(err.into())),
					}
				},
			}
		}
	}
}


impl Iterator for WalkDir<true> {
	type Item = Result<Entry<ffi::FileInfo>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut entry = Entry::new().ok()?;
		let info = (&mut entry.info) as *mut _;

		Some(self._next(&mut entry.path, info)?.map(|_| entry))
	}
}

impl Iterator for WalkDir<false> {
	type Item = Result<Entry<()>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut entry = Entry::new().ok()?;

		Some(self._next(&mut entry.path, core::ptr::null_mut())?.map(|_| entry))
	}
}


/// Entries returned by the [`WalkDir`] iterator.
///
/// An instance of `Entry` represents an entry inside of a directory on the
/// filesystem. Each entry can be inspected via methods to learn about the full
/// path or possibly other metadata.
#[derive(Debug)]
pub struct Entry<Info> {
	path: OsString,
	info: Info,
}


impl<Meta> Entry<Meta> {
	pub fn path(&self) -> &CStr { self.path.as_c_str() }
}


impl Entry<ffi::FileInfo> {
	pub fn metadata(&self) -> Metadata { Metadata(&self.info) }
}

impl<Info> Entry<Info> {
	fn new() -> Result<Self, NullPointerError> {
		OsString::new().map(|path| {
			               Self { path,
			                      info: unsafe { core::mem::zeroed() } }
		               })
	}
}

impl<Meta> core::fmt::Display for Entry<Meta> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self.path()) }
}
