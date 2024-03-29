use std::fmt;
use std::sync::mpsc;

mod bnr_error;
mod usb_error;

pub use bnr_error::*;
pub use usb_error::*;

use time as datetime;

/// Convenience alias for a `Result` type for the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error type for the crate.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Error {
    Generic(i64),
    Serde(String),
    Parsing(String),
    Enum(String),
    Usb(String),
    Io(String),
    Json(String),
    Xfs(String),
    DateTime(String),
    Bnr(BnrError),
    BnrUsb(UsbError),
}

impl From<serde_xml::Error> for Error {
    fn from(err: serde_xml::Error) -> Self {
        Self::Serde(format!("{err}"))
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::Parsing(format!("{err}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(format!("{err}"))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Parsing(format!("{err}"))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Parsing(format!("{err}"))
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Self::Io(format!("{err}"))
    }
}

impl From<datetime::Error> for Error {
    fn from(err: datetime::Error) -> Self {
        Self::DateTime(format!("{err}"))
    }
}

impl From<datetime::error::Format> for Error {
    fn from(err: datetime::error::Format) -> Self {
        Self::DateTime(format!("{err}"))
    }
}

impl From<datetime::error::Parse> for Error {
    fn from(err: datetime::error::Parse) -> Self {
        Self::DateTime(format!("{err}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(format!("{err}"))
    }
}

impl<E> From<mpsc::SendError<E>> for Error {
    fn from(err: mpsc::SendError<E>) -> Self {
        Self::Io(format!("mpsc send error: {err}"))
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::DateTime(format!("time error: {err}"))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(err) => write!(f, "Generic error: {err}"),
            Self::Serde(err) => write!(f, "Serialization error: {err}"),
            Self::Parsing(err) => write!(f, "Parsing error: {err}"),
            Self::Enum(err) => write!(f, "Enum error: {err}"),
            Self::Usb(err) => write!(f, "USB error: {err}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Xfs(err) => write!(f, "XFS error: {err}"),
            Self::DateTime(err) => write!(f, "DateTime error: {err}"),
            Self::Bnr(err) => write!(f, "BNR error: {err}"),
            Self::BnrUsb(err) => write!(f, "BNR USB error: {err}"),
            Self::Json(err) => write!(f, "JSON error: {err}"),
        }
    }
}

impl From<nusb::transfer::TransferError> for Error {
    fn from(err: nusb::transfer::TransferError) -> Self {
        Self::Usb(format!("{err}"))
    }
}
