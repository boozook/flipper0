// TODO: generate it automatically with derive-proc-macro, call inject from build-script.

use core::ops::{Try, FromResidual};
use core::convert::Infallible;
use core::ops::ControlFlow;
use crate::ffi::FS_Error;


pub type Status = FS_Error;


impl const From<i32> for Status {
	fn from(v: i32) -> Self {
		match v {
			0 => Self::FSE_OK,
			1 => Self::FSE_NOT_READY,
			2 => Self::FSE_EXIST,
			3 => Self::FSE_NOT_EXIST,
			4 => Self::FSE_INVALID_PARAMETER,
			5 => Self::FSE_DENIED,
			6 => Self::FSE_INVALID_NAME,
			7 => Self::FSE_INTERNAL,
			8 => Self::FSE_NOT_IMPLEMENTED,
			9 => Self::FSE_ALREADY_OPEN,
			_ => Self::FSE_NOT_IMPLEMENTED,
		}
	}
}

impl const From<i32> for Error {
	fn from(v: i32) -> Self { unsafe { core::mem::transmute(v) } }
}

impl const From<Error> for Status {
	fn from(err: Error) -> Self { unsafe { core::mem::transmute(err) } }
}

impl const Try for Status {
	type Output = ();
	type Residual = Result<Infallible, Error>;

	fn from_output(output: Self::Output) -> Self { Status::FSE_OK }

	fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
		match self {
			Status::FSE_OK => ControlFlow::Continue(()),
			Status::FSE_NOT_READY => ControlFlow::Break(Err(Error::NotReady)),
			Status::FSE_EXIST => ControlFlow::Break(Err(Error::Exist)),
			Status::FSE_NOT_EXIST => ControlFlow::Break(Err(Error::NotExist)),
			Status::FSE_INVALID_PARAMETER => ControlFlow::Break(Err(Error::InvalidParameter)),
			Status::FSE_DENIED => ControlFlow::Break(Err(Error::Denied)),
			Status::FSE_INVALID_NAME => ControlFlow::Break(Err(Error::InvalidName)),
			Status::FSE_INTERNAL => ControlFlow::Break(Err(Error::Internal)),
			Status::FSE_NOT_IMPLEMENTED => ControlFlow::Break(Err(Error::NotImplemented)),
			Status::FSE_ALREADY_OPEN => ControlFlow::Break(Err(Error::AlreadyOpen)),
			_ => ControlFlow::Break(Err(Error::NotImplemented)),
		}
	}
}

impl const FromResidual for Status {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		match residual {
			Ok(_) => Self::FSE_OK,
			Err(code) => code.into(),
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone)]
pub enum Error {
	#[doc = "FS not ready"]
	NotReady = Status::FSE_NOT_READY as _,
	#[doc = "File/Dir already exist"]
	Exist = Status::FSE_EXIST as _,
	#[doc = "File/Dir does not exist"]
	NotExist = Status::FSE_NOT_EXIST as _,
	#[doc = "Invalid API parameter"]
	InvalidParameter = Status::FSE_INVALID_PARAMETER as _,
	#[doc = "Access denied"]
	Denied = Status::FSE_DENIED as _,
	#[doc = "Invalid name/path"]
	InvalidName = Status::FSE_INVALID_NAME as _,
	#[doc = "Internal error"]
	Internal = Status::FSE_INTERNAL as _,
	#[doc = "Function not implemented"]
	NotImplemented = Status::FSE_NOT_IMPLEMENTED as _,
	#[doc = "File/Dir already opened"]
	AlreadyOpen = Status::FSE_ALREADY_OPEN as _,
}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { todo!("fs-error display") }
}


impl const From<FS_Error> for Error {
	fn from(value: FS_Error) -> Self {
		assert!(!matches!(value, FS_Error::FSE_OK));
		unsafe { core::mem::transmute(value) }
	}
}
