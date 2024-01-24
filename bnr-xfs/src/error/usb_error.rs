use std::fmt;

const BXR_USB_ERROR_OFFSET: u32 = 10_000;
const BXR_USB_CRC: u32 = BXR_USB_ERROR_OFFSET + 0x01;
const BXR_USB_BTSTUFF: u32 = BXR_USB_ERROR_OFFSET + 0x02;
const BXR_USB_DATA_TOGGLE_MISMATCH: u32 = BXR_USB_ERROR_OFFSET + 0x03;
const BXR_USB_STALL_PID: u32 = BXR_USB_ERROR_OFFSET + 0x04;
const BXR_USB_DEV_NOT_RESPONDING: u32 = BXR_USB_ERROR_OFFSET + 0x05;
const BXR_USB_PID_CHECK_FAILURE: u32 = BXR_USB_ERROR_OFFSET + 0x06;
const BXR_USB_UNEXPECTED_PID: u32 = BXR_USB_ERROR_OFFSET + 0x07;
const BXR_USB_DATA_OVERRUN: u32 = BXR_USB_ERROR_OFFSET + 0x08;
const BXR_USB_DATA_UNDERRUN: u32 = BXR_USB_ERROR_OFFSET + 0x09;
const BXR_USB_BUFFER_OVERRUN: u32 = BXR_USB_ERROR_OFFSET + 0x0C;
const BXR_USB_BUFFER_UNDERRUN: u32 = BXR_USB_ERROR_OFFSET + 0x0D;
const BXR_USB_NOT_ACCESSED: u32 = BXR_USB_ERROR_OFFSET + 0x0F;
const BXR_USB_FIFO: u32 = BXR_USB_ERROR_OFFSET + 0x10;
const BXR_USB_ENDPOINT_HALTED: u32 = BXR_USB_ERROR_OFFSET + 0x30;
const BXR_USB_NO_MEMORY: u32 = BXR_USB_ERROR_OFFSET + 0x100;
const BXR_USB_INVALID_URB_FUNCTION: u32 = BXR_USB_ERROR_OFFSET + 0x101;
const BXR_USB_INVALID_PARAMETER: u32 = BXR_USB_ERROR_OFFSET + 0x102;
const BXR_USB_ERROR_BUSY: u32 = BXR_USB_ERROR_OFFSET + 0x103;
const BXR_USB_REQUEST_FAILED: u32 = BXR_USB_ERROR_OFFSET + 0x104;
const BXR_USB_INVALID_PIPE_HANDLE: u32 = BXR_USB_ERROR_OFFSET + 0x105;
const BXR_USB_NO_BANDWIDTH: u32 = BXR_USB_ERROR_OFFSET + 0x106;
const BXR_USB_INTERNAL_HC_ERROR: u32 = BXR_USB_ERROR_OFFSET + 0x107;
const BXR_USB_ERROR_SHORT_TRANSFER: u32 = BXR_USB_ERROR_OFFSET + 0x108;
const BXR_USB_BAD_START_FRAME: u32 = BXR_USB_ERROR_OFFSET + 0x109;
const BXR_USB_ISOCH_REQUEST_FAILED: u32 = BXR_USB_ERROR_OFFSET + 0x10A;
const BXR_USB_FRAME_CONTROL_OWNED: u32 = BXR_USB_ERROR_OFFSET + 0x10B;
const BXR_USB_FRAME_CONTROL_NOT_OWNED: u32 = BXR_USB_ERROR_OFFSET + 0x10C;
const BXR_USB_CANCELED: u32 = BXR_USB_ERROR_OFFSET + 0x200;
const BXR_USB_CANCELING: u32 = BXR_USB_ERROR_OFFSET + 0x201;
const BXR_USB_ALREADY_CONFIGURED: u32 = BXR_USB_ERROR_OFFSET + 0x202;
const BXR_USB_UNCONFIGURED: u32 = BXR_USB_ERROR_OFFSET + 0x203;
const BXR_USB_NO_SUCH_DEVICE: u32 = BXR_USB_ERROR_OFFSET + 0x204;
const BXR_USB_DEVICE_NOT_FOUND: u32 = BXR_USB_ERROR_OFFSET + 0x205;
const BXR_USB_NOT_SUPPORTED: u32 = BXR_USB_ERROR_OFFSET + 0x206;
const BXR_USB_IO_PENDING: u32 = BXR_USB_ERROR_OFFSET + 0x207;
const BXR_USB_IO_TIMEOUT: u32 = BXR_USB_ERROR_OFFSET + 0x208;
const BXR_USB_DEVICE_REMOVED: u32 = BXR_USB_ERROR_OFFSET + 0x209;
const BXR_USB_PIPE_NOT_LINKED: u32 = BXR_USB_ERROR_OFFSET + 0x20A;
const BXR_USB_CONNECTED_PIPES: u32 = BXR_USB_ERROR_OFFSET + 0x20B;
const BXR_USB_DEVICE_LOCKED: u32 = BXR_USB_ERROR_OFFSET + 0x20C;

/// Represents USB error codes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UsbError {
    CRC = BXR_USB_CRC,
    BTStuff = BXR_USB_BTSTUFF,
    DataToggleMismatch = BXR_USB_DATA_TOGGLE_MISMATCH,
    StallPid = BXR_USB_STALL_PID,
    DevNotResponding = BXR_USB_DEV_NOT_RESPONDING,
    PidCheckFailure = BXR_USB_PID_CHECK_FAILURE,
    UnexpectedPid = BXR_USB_UNEXPECTED_PID,
    DataOverrun = BXR_USB_DATA_OVERRUN,
    DataUnderrun = BXR_USB_DATA_UNDERRUN,
    BufferOverrun = BXR_USB_BUFFER_OVERRUN,
    BufferUnderrun = BXR_USB_BUFFER_UNDERRUN,
    NotAccessed = BXR_USB_NOT_ACCESSED,
    Fifo = BXR_USB_FIFO,
    EndpointHalted = BXR_USB_ENDPOINT_HALTED,
    NoMemory = BXR_USB_NO_MEMORY,
    InvalidUrbFunction = BXR_USB_INVALID_URB_FUNCTION,
    InvalidParameter = BXR_USB_INVALID_PARAMETER,
    ErrorBusy = BXR_USB_ERROR_BUSY,
    RequestFailed = BXR_USB_REQUEST_FAILED,
    InvalidPipeHandle = BXR_USB_INVALID_PIPE_HANDLE,
    NoBandwidth = BXR_USB_NO_BANDWIDTH,
    InternalHcError = BXR_USB_INTERNAL_HC_ERROR,
    ErrorShortTransfer = BXR_USB_ERROR_SHORT_TRANSFER,
    BadStartFrame = BXR_USB_BAD_START_FRAME,
    IsochRequestFailed = BXR_USB_ISOCH_REQUEST_FAILED,
    FrameControlOwned = BXR_USB_FRAME_CONTROL_OWNED,
    FrameControlNotOwned = BXR_USB_FRAME_CONTROL_NOT_OWNED,
    Canceled = BXR_USB_CANCELED,
    Canceling = BXR_USB_CANCELING,
    AlreadyConfigured = BXR_USB_ALREADY_CONFIGURED,
    Unconfigured = BXR_USB_UNCONFIGURED,
    NoSuchDevice = BXR_USB_NO_SUCH_DEVICE,
    DeviceNotFound = BXR_USB_DEVICE_NOT_FOUND,
    NotSupported = BXR_USB_NOT_SUPPORTED,
    IoPending = BXR_USB_IO_PENDING,
    IoTimeout = BXR_USB_IO_TIMEOUT,
    DeviceRemoved = BXR_USB_DEVICE_REMOVED,
    PipeNotLinked = BXR_USB_PIPE_NOT_LINKED,
    ConnectedPipes = BXR_USB_CONNECTED_PIPES,
    DeviceLocked = BXR_USB_DEVICE_LOCKED,
}

impl From<u32> for UsbError {
    fn from(val: u32) -> Self {
        match val {
            BXR_USB_CRC => Self::CRC,
            BXR_USB_BTSTUFF => Self::BTStuff,
            BXR_USB_DATA_TOGGLE_MISMATCH => Self::DataToggleMismatch,
            BXR_USB_STALL_PID => Self::StallPid,
            BXR_USB_DEV_NOT_RESPONDING => Self::DevNotResponding,
            BXR_USB_PID_CHECK_FAILURE => Self::PidCheckFailure,
            BXR_USB_UNEXPECTED_PID => Self::UnexpectedPid,
            BXR_USB_DATA_OVERRUN => Self::DataOverrun,
            BXR_USB_DATA_UNDERRUN => Self::DataUnderrun,
            BXR_USB_BUFFER_OVERRUN => Self::BufferOverrun,
            BXR_USB_BUFFER_UNDERRUN => Self::BufferUnderrun,
            BXR_USB_NOT_ACCESSED => Self::NotAccessed,
            BXR_USB_FIFO => Self::Fifo,
            BXR_USB_ENDPOINT_HALTED => Self::EndpointHalted,
            BXR_USB_NO_MEMORY => Self::NoMemory,
            BXR_USB_INVALID_URB_FUNCTION => Self::InvalidUrbFunction,
            BXR_USB_INVALID_PARAMETER => Self::InvalidParameter,
            BXR_USB_ERROR_BUSY => Self::ErrorBusy,
            BXR_USB_REQUEST_FAILED => Self::RequestFailed,
            BXR_USB_INVALID_PIPE_HANDLE => Self::InvalidPipeHandle,
            BXR_USB_NO_BANDWIDTH => Self::NoBandwidth,
            BXR_USB_INTERNAL_HC_ERROR => Self::InternalHcError,
            BXR_USB_ERROR_SHORT_TRANSFER => Self::ErrorShortTransfer,
            BXR_USB_BAD_START_FRAME => Self::BadStartFrame,
            BXR_USB_ISOCH_REQUEST_FAILED => Self::IsochRequestFailed,
            BXR_USB_FRAME_CONTROL_OWNED => Self::FrameControlOwned,
            BXR_USB_FRAME_CONTROL_NOT_OWNED => Self::FrameControlNotOwned,
            BXR_USB_CANCELED => Self::Canceled,
            BXR_USB_CANCELING => Self::Canceling,
            BXR_USB_ALREADY_CONFIGURED => Self::AlreadyConfigured,
            BXR_USB_UNCONFIGURED => Self::Unconfigured,
            BXR_USB_NO_SUCH_DEVICE => Self::NoSuchDevice,
            BXR_USB_DEVICE_NOT_FOUND => Self::DeviceNotFound,
            BXR_USB_NOT_SUPPORTED => Self::NotSupported,
            BXR_USB_IO_PENDING => Self::IoPending,
            BXR_USB_IO_TIMEOUT => Self::IoTimeout,
            BXR_USB_DEVICE_REMOVED => Self::DeviceRemoved,
            BXR_USB_PIPE_NOT_LINKED => Self::PipeNotLinked,
            BXR_USB_CONNECTED_PIPES => Self::ConnectedPipes,
            BXR_USB_DEVICE_LOCKED => Self::DeviceLocked,
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
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
