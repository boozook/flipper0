pub use crate::error::gpio::Error as OsError;
pub use crate::error::furi::Error as FuriError;
pub use crate::error::fs::Error as FsError;

pub type Result<T, E = FuriError> = core::result::Result<T, E>;
