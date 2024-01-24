use std::fmt;

/// Represents USB error codes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UsbError {
    CRC = bnr_sys::BXR_USB_CRC,
    BTStuff = bnr_sys::BXR_USB_BTSTUFF,
    DataToggleMismatch = bnr_sys::BXR_USB_DATA_TOGGLE_MISMATCH,
    StallPid = bnr_sys::BXR_USB_STALL_PID,
    DevNotResponding = bnr_sys::BXR_USB_DEV_NOT_RESPONDING,
    PidCheckFailure = bnr_sys::BXR_USB_PID_CHECK_FAILURE,
    UnexpectedPid = bnr_sys::BXR_USB_UNEXPECTED_PID,
    DataOverrun = bnr_sys::BXR_USB_DATA_OVERRUN,
    DataUnderrun = bnr_sys::BXR_USB_DATA_UNDERRUN,
    BufferOverrun = bnr_sys::BXR_USB_BUFFER_OVERRUN,
    BufferUnderrun = bnr_sys::BXR_USB_BUFFER_UNDERRUN,
    NotAccessed = bnr_sys::BXR_USB_NOT_ACCESSED,
    Fifo = bnr_sys::BXR_USB_FIFO,
    EndpointHalted = bnr_sys::BXR_USB_ENDPOINT_HALTED,
    NoMemory = bnr_sys::BXR_USB_NO_MEMORY,
    InvalidUrbFunction = bnr_sys::BXR_USB_INVALID_URB_FUNCTION,
    InvalidParameter = bnr_sys::BXR_USB_INVALID_PARAMETER,
    ErrorBusy = bnr_sys::BXR_USB_ERROR_BUSY,
    RequestFailed = bnr_sys::BXR_USB_REQUEST_FAILED,
    InvalidPipeHandle = bnr_sys::BXR_USB_INVALID_PIPE_HANDLE,
    NoBandwidth = bnr_sys::BXR_USB_NO_BANDWIDTH,
    InternalHcError = bnr_sys::BXR_USB_INTERNAL_HC_ERROR,
    ErrorShortTransfer = bnr_sys::BXR_USB_ERROR_SHORT_TRANSFER,
    BadStartFrame = bnr_sys::BXR_USB_BAD_START_FRAME,
    IsochRequestFailed = bnr_sys::BXR_USB_ISOCH_REQUEST_FAILED,
    FrameControlOwned = bnr_sys::BXR_USB_FRAME_CONTROL_OWNED,
    FrameControlNotOwned = bnr_sys::BXR_USB_FRAME_CONTROL_NOT_OWNED,
    Canceled = bnr_sys::BXR_USB_CANCELED,
    Canceling = bnr_sys::BXR_USB_CANCELING,
    AlreadyConfigured = bnr_sys::BXR_USB_ALREADY_CONFIGURED,
    Unconfigured = bnr_sys::BXR_USB_UNCONFIGURED,
    NoSuchDevice = bnr_sys::BXR_USB_NO_SUCH_DEVICE,
    DeviceNotFound = bnr_sys::BXR_USB_DEVICE_NOT_FOUND,
    NotSupported = bnr_sys::BXR_USB_NOT_SUPPORTED,
    IoPending = bnr_sys::BXR_USB_IO_PENDING,
    IoTimeout = bnr_sys::BXR_USB_IO_TIMEOUT,
    DeviceRemoved = bnr_sys::BXR_USB_DEVICE_REMOVED,
    PipeNotLinked = bnr_sys::BXR_USB_PIPE_NOT_LINKED,
    ConnectedPipes = bnr_sys::BXR_USB_CONNECTED_PIPES,
    DeviceLocked = bnr_sys::BXR_USB_DEVICE_LOCKED,
}

impl From<u32> for UsbError {
    fn from(val: u32) -> Self {
        match val {
            v if v == bnr_sys::BXR_USB_CRC => Self::CRC,
            v if v == bnr_sys::BXR_USB_BTSTUFF => Self::BTStuff,
            v if v == bnr_sys::BXR_USB_DATA_TOGGLE_MISMATCH => Self::DataToggleMismatch,
            v if v == bnr_sys::BXR_USB_STALL_PID => Self::StallPid,
            v if v == bnr_sys::BXR_USB_DEV_NOT_RESPONDING => Self::DevNotResponding,
            v if v == bnr_sys::BXR_USB_PID_CHECK_FAILURE => Self::PidCheckFailure,
            v if v == bnr_sys::BXR_USB_UNEXPECTED_PID => Self::UnexpectedPid,
            v if v == bnr_sys::BXR_USB_DATA_OVERRUN => Self::DataOverrun,
            v if v == bnr_sys::BXR_USB_DATA_UNDERRUN => Self::DataUnderrun,
            v if v == bnr_sys::BXR_USB_BUFFER_OVERRUN => Self::BufferOverrun,
            v if v == bnr_sys::BXR_USB_BUFFER_UNDERRUN => Self::BufferUnderrun,
            v if v == bnr_sys::BXR_USB_NOT_ACCESSED => Self::NotAccessed,
            v if v == bnr_sys::BXR_USB_FIFO => Self::Fifo,
            v if v == bnr_sys::BXR_USB_ENDPOINT_HALTED => Self::EndpointHalted,
            v if v == bnr_sys::BXR_USB_NO_MEMORY => Self::NoMemory,
            v if v == bnr_sys::BXR_USB_INVALID_URB_FUNCTION => Self::InvalidUrbFunction,
            v if v == bnr_sys::BXR_USB_INVALID_PARAMETER => Self::InvalidParameter,
            v if v == bnr_sys::BXR_USB_ERROR_BUSY => Self::ErrorBusy,
            v if v == bnr_sys::BXR_USB_REQUEST_FAILED => Self::RequestFailed,
            v if v == bnr_sys::BXR_USB_INVALID_PIPE_HANDLE => Self::InvalidPipeHandle,
            v if v == bnr_sys::BXR_USB_NO_BANDWIDTH => Self::NoBandwidth,
            v if v == bnr_sys::BXR_USB_INTERNAL_HC_ERROR => Self::InternalHcError,
            v if v == bnr_sys::BXR_USB_ERROR_SHORT_TRANSFER => Self::ErrorShortTransfer,
            v if v == bnr_sys::BXR_USB_BAD_START_FRAME => Self::BadStartFrame,
            v if v == bnr_sys::BXR_USB_ISOCH_REQUEST_FAILED => Self::IsochRequestFailed,
            v if v == bnr_sys::BXR_USB_FRAME_CONTROL_OWNED => Self::FrameControlOwned,
            v if v == bnr_sys::BXR_USB_FRAME_CONTROL_NOT_OWNED => Self::FrameControlNotOwned,
            v if v == bnr_sys::BXR_USB_CANCELED => Self::Canceled,
            v if v == bnr_sys::BXR_USB_CANCELING => Self::Canceling,
            v if v == bnr_sys::BXR_USB_ALREADY_CONFIGURED => Self::AlreadyConfigured,
            v if v == bnr_sys::BXR_USB_UNCONFIGURED => Self::Unconfigured,
            v if v == bnr_sys::BXR_USB_NO_SUCH_DEVICE => Self::NoSuchDevice,
            v if v == bnr_sys::BXR_USB_DEVICE_NOT_FOUND => Self::DeviceNotFound,
            v if v == bnr_sys::BXR_USB_NOT_SUPPORTED => Self::NotSupported,
            v if v == bnr_sys::BXR_USB_IO_PENDING => Self::IoPending,
            v if v == bnr_sys::BXR_USB_IO_TIMEOUT => Self::IoTimeout,
            v if v == bnr_sys::BXR_USB_DEVICE_REMOVED => Self::DeviceRemoved,
            v if v == bnr_sys::BXR_USB_PIPE_NOT_LINKED => Self::PipeNotLinked,
            v if v == bnr_sys::BXR_USB_CONNECTED_PIPES => Self::ConnectedPipes,
            v if v == bnr_sys::BXR_USB_DEVICE_LOCKED => Self::DeviceLocked,
            _ => Self::NotSupported,
        }
    }
}

impl From<UsbError> for u32 {
    fn from(err: UsbError) -> Self {
        err as u32
    }
}

impl From<&UsbError> for u32 {
    fn from(err: &UsbError) -> Self {
        (*err).into()
    }
}

impl From<UsbError> for &'static str {
    fn from(err: UsbError) -> Self {
        match err {
            UsbError::CRC => "CRC",
            UsbError::BTStuff => "BTStuff",
            UsbError::DataToggleMismatch => "DataToggleMismatch",
            UsbError::StallPid => "StallPid",
            UsbError::DevNotResponding => "DevNotResponding",
            UsbError::PidCheckFailure => "PidCheckFailure",
            UsbError::UnexpectedPid => "UnexpectedPid",
            UsbError::DataOverrun => "DataOverrun",
            UsbError::DataUnderrun => "DataUnderrun",
            UsbError::BufferOverrun => "BufferOverrun",
            UsbError::BufferUnderrun => "BufferUnderrun",
            UsbError::NotAccessed => "NotAccessed",
            UsbError::Fifo => "Fifo",
            UsbError::EndpointHalted => "EndpointHalted",
            UsbError::NoMemory => "NoMemory",
            UsbError::InvalidUrbFunction => "InvalidUrbFunction",
            UsbError::InvalidParameter => "InvalidParameter",
            UsbError::ErrorBusy => "ErrorBusy",
            UsbError::RequestFailed => "RequestFailed",
            UsbError::InvalidPipeHandle => "InvalidPipeHandle",
            UsbError::NoBandwidth => "NoBandwidth",
            UsbError::InternalHcError => "InternalHcError",
            UsbError::ErrorShortTransfer => "ErrorShortTransfer",
            UsbError::BadStartFrame => "BadStartFrame",
            UsbError::IsochRequestFailed => "IsochRequestFailed",
            UsbError::FrameControlOwned => "FrameControlOwned",
            UsbError::FrameControlNotOwned => "FrameControlNotOwned",
            UsbError::Canceled => "Canceled",
            UsbError::Canceling => "Canceling",
            UsbError::AlreadyConfigured => "AlreadyConfigured",
            UsbError::Unconfigured => "Unconfigured",
            UsbError::NoSuchDevice => "NoSuchDevice",
            UsbError::DeviceNotFound => "DeviceNotFound",
            UsbError::NotSupported => "NotSupported",
            UsbError::IoPending => "IoPending",
            UsbError::IoTimeout => "IoTimeout",
            UsbError::DeviceRemoved => "DeviceRemoved",
            UsbError::PipeNotLinked => "PipeNotLinked",
            UsbError::ConnectedPipes => "ConnectedPipes",
            UsbError::DeviceLocked => "DeviceLocked",
        }
    }
}

impl From<&UsbError> for &'static str {
    fn from(err: &UsbError) -> Self {
        (*err).into()
    }
}

impl fmt::Display for UsbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
