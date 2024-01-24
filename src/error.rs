use std::fmt;

mod bnr_error;
mod usb_error;

pub use bnr_error::*;
pub use usb_error::*;

/// Convenience alias for the library `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Check the result from a bnr-sys function call.
pub fn check_res(res: i32, name: &str) -> Result<()> {
    if res < 0 {
        let err_str = format!("BNR {name} failure: {res}");
        log::error!("{err_str}");
        Err(Error::Hal(err_str))
    } else {
        log::debug!("BNR {name} success: {res}");
        Ok(())
    }
}

/// Error types for the library.
#[derive(Debug, PartialEq)]
pub enum Error {
    Cash(String),
    Io(String),
    Rsa(String),
    Hal(String),
    Json(String),
    Bnr(BnrError),
    Usb(UsbError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(format!("{err}"))
    }
}

impl From<rsa::Error> for Error {
    fn from(err: rsa::Error) -> Self {
        Self::Rsa(format!("{err}"))
    }
}

impl From<rsa::pkcs1::Error> for Error {
    fn from(err: rsa::pkcs1::Error) -> Self {
        Self::Rsa(format!("{err}"))
    }
}

impl From<rsa::pkcs8::Error> for Error {
    fn from(err: rsa::pkcs8::Error) -> Self {
        Self::Rsa(format!("{err}"))
    }
}

impl From<rsa::pkcs8::spki::Error> for Error {
    fn from(err: rsa::pkcs8::spki::Error) -> Self {
        Self::Rsa(format!("{err}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(format!("{err}"))
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Self::Io(format!("{err}"))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cash(err) => write!(f, "Cash error: {err}"),
            Self::Io(err) => write!(f, "I/O error: {err})"),
            Self::Rsa(err) => write!(f, "RSA error: {err})"),
            Self::Hal(err) => write!(f, "HAL error: {err})"),
            Self::Json(err) => write!(f, "JSON error: {err})"),
            Self::Bnr(err) => write!(f, "BNR device error: {err})"),
            Self::Usb(err) => write!(f, "USB error: {err})"),
        }
    }
}
