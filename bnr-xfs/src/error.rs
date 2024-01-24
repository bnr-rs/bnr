use std::fmt;

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
    Xfs(String),
    DateTime(String),
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Self::Usb(format!("{err}"))
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Self::Serde(format!("{err}"))
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::Parsing(format!("{err}"))
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
        }
    }
}
