// TODO: generate it automatically with derive-proc-macro, call inject from build-script.

use core::ops::{Try, FromResidual};
use core::convert::Infallible;
use core::ops::ControlFlow;
use crate::ffi::FuriStatus;


pub type Status = FuriStatus;


impl const From<i32> for FuriStatus {
	fn from(v: i32) -> Self {
		match v {
			0 => Self::FuriStatusOk,
			-1 => Self::FuriStatusError,
			-2 => Self::FuriStatusErrorTimeout,
			-3 => Self::FuriStatusErrorResource,
			-4 => Self::FuriStatusErrorParameter,
			-5 => Self::FuriStatusErrorNoMemory,
			-6 => Self::FuriStatusErrorISR,
			_ => Self::FuriStatusError,
		}
	}
}

impl const From<i32> for Error {
	fn from(v: i32) -> Self { unsafe { core::mem::transmute(v) } }
}

impl const From<Error> for FuriStatus {
	fn from(err: Error) -> Self { unsafe { core::mem::transmute(err) } }
}

impl const Try for FuriStatus {
	type Output = ();
	type Residual = Result<Infallible, Error>;

	fn from_output(output: Self::Output) -> Self { FuriStatus::FuriStatusOk }

	fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
		match self {
			FuriStatus::FuriStatusOk => ControlFlow::Continue(()),
			FuriStatus::FuriStatusError => ControlFlow::Break(Err(Error::Error)),
			FuriStatus::FuriStatusErrorTimeout => ControlFlow::Break(Err(Error::Timeout)),
			FuriStatus::FuriStatusErrorResource => ControlFlow::Break(Err(Error::Resource)),
			FuriStatus::FuriStatusErrorParameter => ControlFlow::Break(Err(Error::Parameter)),
			FuriStatus::FuriStatusErrorNoMemory => ControlFlow::Break(Err(Error::NoMemory)),
			FuriStatus::FuriStatusErrorISR => ControlFlow::Break(Err(Error::ISR)),
			_ => ControlFlow::Break(Err(Error::Error)),
		}
	}
}

impl const FromResidual for FuriStatus {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		match residual {
			Ok(_) => Self::FuriStatusOk,
			Err(code) => code.into(),
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone)]
pub enum Error {
	Error = FuriStatus::FuriStatusError as _,
	#[doc = "Operation not completed within the timeout period."]
	Timeout = FuriStatus::FuriStatusErrorTimeout as _,
	#[doc = "Resource not available."]
	Resource = FuriStatus::FuriStatusErrorResource as _,
	#[doc = "Parameter error."]
	Parameter = FuriStatus::FuriStatusErrorParameter as _,
	NoMemory = FuriStatus::FuriStatusErrorNoMemory as _,
	ISR = FuriStatus::FuriStatusErrorISR as _,
}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { todo!() }
}
