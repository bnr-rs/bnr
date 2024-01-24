//! Device communication library for MEI/CPI BNR devices.
//!
//! This library is for communicating with BNR cash devices using the BNR/XFS communication
//! protocol over a USB connection.
//!
//! Users can either use the stand-alone functions exposed by the module APIs, or create a local
//! instance of a [DeviceHandle].
//!
//! Creating a local instance of a [DeviceHandle] allows the user to interact using the method
//! calls directly.
//!
//! The module APIs more closely mirror the C library provided by MEI/CPI, and utilize a global
//! instance of a [DeviceHandle]. These APIs are more useful for backwards compatibility with
//! applications already using the MEI/CPI libraries.

pub extern crate bnr_xfs;
pub extern crate time;

pub use bnr_xfs::*;

pub mod arrays;
pub mod cash;
pub mod init;
pub mod maintenance;
pub mod status;
pub mod sys_config;

use std::sync::{Mutex, MutexGuard};

// Global [DeviceHandle] instance for bnr-sys compatibility API.
pub(crate) static HANDLE: Mutex<Option<DeviceHandle>> = Mutex::new(None);

pub(crate) fn init_handle(handle: DeviceHandle) -> Result<()> {
    let mut lock = HANDLE.lock()?;

    if lock.is_some() {
        Err(Error::Usb("Global DeviceHandle already initialized".into()))
    } else {
        lock.replace(handle);
        Ok(())
    }
}

pub(crate) fn deinit_handle() -> Result<()> {
    let mut handle = HANDLE.lock()?;

    handle.take();

    Ok(())
}

pub(crate) fn lock_handle() -> Result<MutexGuard<'static, Option<DeviceHandle>>> {
    Ok(HANDLE.lock()?)
}

/// Locks the global [DeviceHandle] instance, and calls the provided callback.
pub fn with_handle<T>(f: impl Fn(&DeviceHandle) -> Result<T>) -> Result<T> {
    let handle = lock_handle()?;
    f(handle
        .as_ref()
        .ok_or(Error::Usb("Uninitialized device handle".into()))?)
}
