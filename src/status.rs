use crate::{Error, Result};

/// Status of a CDR stacker.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PositionStatus {
    pub position: u32,
    pub content_status: u32,
    pub shutter_status: u32,
}

impl PositionStatus {
    pub const fn new() -> Self {
        Self {
            position: 0,
            content_status: 0,
            shutter_status: 0,
        }
    }
}

impl From<PositionStatus> for bnr_sys::PositionStatus {
    fn from(val: PositionStatus) -> Self {
        Self {
            position: val.position,
            contentStatus: val.content_status,
            shutterStatus: val.shutter_status,
        }
    }
}

impl From<bnr_sys::PositionStatus> for PositionStatus {
    fn from(val: bnr_sys::PositionStatus) -> Self {
        Self {
            position: val.position,
            content_status: val.contentStatus,
            shutter_status: val.shutterStatus,
        }
    }
}

/// List of CDR stacker status by position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct XfsCdrPositionStatusList {
    pub max_size: u32,
    pub size: u32,
    pub items: [PositionStatus; 2],
}

impl XfsCdrPositionStatusList {
    /// Creates a new [XfsCdrPositionStatusList].
    pub const fn new() -> Self {
        Self {
            max_size: 2,
            size: 0,
            items: [PositionStatus::new(); 2],
        }
    }
}

impl From<XfsCdrPositionStatusList> for bnr_sys::XfsCdrPositionStatusList {
    fn from(val: XfsCdrPositionStatusList) -> Self {
        Self {
            maxSize: val.max_size,
            size: val.size,
            items: [val.items[0].into(), val.items[1].into()],
        }
    }
}

impl From<bnr_sys::XfsCdrPositionStatusList> for XfsCdrPositionStatusList {
    fn from(val: bnr_sys::XfsCdrPositionStatusList) -> Self {
        Self {
            max_size: val.maxSize,
            size: val.size,
            items: [val.items[0].into(), val.items[1].into()],
        }
    }
}

/// Represents the CDR status returned by the [`bnr_GetStatus`](bnr_sys::bnr_GetStatus) call.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct XfsCdrStatus {
    pub device_status: u32,
    pub dispenser_status: u32,
    pub intermediate_stacker_status: u32,
    pub safe_door_status: u32,
    pub shutter_status: u32,
    pub transport_status: u32,
    pub position_status_list: XfsCdrPositionStatusList,
}

impl XfsCdrStatus {
    /// Creates a new [XfsCdrStatus].
    pub const fn new() -> Self {
        Self {
            device_status: 0,
            dispenser_status: 0,
            intermediate_stacker_status: 0,
            safe_door_status: 0,
            shutter_status: 0,
            transport_status: 0,
            position_status_list: XfsCdrPositionStatusList::new(),
        }
    }

    /// Gets the [XfsCdrStatus] as a const pointer to a FFI [XfsCdrStatus](bnr_sys::XfsCdrStatus).
    pub fn as_raw_ptr(&self) -> *const bnr_sys::XfsCdrStatus {
        self as *const _ as *const _
    }

    /// Gets the [XfsCdrStatus] as a mutable pointer to a FFI [XfsCdrStatus](bnr_sys::XfsCdrStatus).
    pub fn as_raw_ptr_mut(&mut self) -> *mut bnr_sys::XfsCdrStatus {
        self as *mut _ as *mut _
    }
}

impl From<XfsCdrStatus> for bnr_sys::XfsCdrStatus {
    fn from(val: XfsCdrStatus) -> Self {
        Self {
            deviceStatus: val.device_status,
            dispenserStatus: val.dispenser_status,
            intermediateStackerStatus: val.intermediate_stacker_status,
            safeDoorStatus: val.safe_door_status, 
            shutterStatus: val.shutter_status, 
            transportStatus: val.transport_status, 
            positionStatusList: val.position_status_list.into(), 
        }
    }
}

impl From<bnr_sys::XfsCdrStatus> for XfsCdrStatus {
    fn from(val: bnr_sys::XfsCdrStatus) -> Self {
        Self {
            device_status: val.deviceStatus,
            dispenser_status: val.dispenserStatus,
            intermediate_stacker_status: val.intermediateStackerStatus,
            safe_door_status: val.safeDoorStatus, 
            shutter_status: val.shutterStatus, 
            transport_status: val.transportStatus, 
            position_status_list: val.positionStatusList.into(), 
        }
    }
}

/// Gets the status of the CDR device.
pub fn get_status() -> Result<XfsCdrStatus> {
    let mut status = XfsCdrStatus::new();
    let res = unsafe { bnr_sys::bnr_GetStatus(status.as_raw_ptr_mut()) };

    if res < 0 {
        Err(Error::Hal(res))
    } else {
        Ok(status)
    }
}
