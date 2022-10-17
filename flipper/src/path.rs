/// Temporarily aliased CString -> PathBuf
pub type PathBuf = sys::alloc::ffi::CString;
pub type Path = core::ffi::CStr;
// TODO: impl PathBuf with `ffi::path_append` and etc..
