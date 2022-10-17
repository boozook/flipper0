use super::ffi;


#[const_trait]
pub trait Record {
	const LEN: usize;

	fn tag(&self) -> &[u8; Self::LEN];
}

#[const_trait]
pub trait RecordTagRef {
	fn tag_ref(&self) -> &[u8];
}
impl<T: ~const Record> const RecordTagRef for T where [(); Self::LEN]: Sized {
	fn tag_ref(&self) -> &[u8] { self.tag().as_slice() }
}


pub trait RecordExt: Record {
	fn exists(&self) -> bool;
}

impl<T: Record + RecordTagRef> RecordExt for T {
	default fn exists(&self) -> bool { unsafe { ffi::furi_record_exists(self.tag_ref().as_ptr() as _) } }
}

// TODO: Enable when full specialization stabilized.
#[cfg(feature = "specialization")]
impl<T: Record> RecordExt for T where [(); T::LEN]: Sized {
	fn exists(&self) -> bool { unsafe { ffi::furi_record_exists(self.tag().as_ptr() as _) } }
}


pub struct Storage;

impl const Record for Storage {
	const LEN: usize = ffi::RECORD_STORAGE.len();
	fn tag(&self) -> &[u8; Self::LEN] { ffi::RECORD_STORAGE }
}

impl RecordExt for Storage {}


pub(crate) struct CloseOnDrop<T>(T)
	where T: Record,
	      [(); T::LEN]: Sized;

impl<T: Record> Drop for CloseOnDrop<T> where [(); T::LEN]: Sized {
	fn drop(&mut self) { unsafe { ffi::furi_record_close(self.0.tag().as_ptr() as _) } }
}

impl<T> CloseOnDrop<T>
	where T: Record,
	      [(); T::LEN]: Sized
{
	#[inline(always)]
	pub fn new(rec: T) -> Self { Self(rec) }
}
