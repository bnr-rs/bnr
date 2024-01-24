//! Types and functions for handling system status events.

use crate::{check_res, Result};

mod content;
mod device;
mod dispenser;
mod hardware;
mod inter_stacker;
mod position;
mod safe_door;
mod shutter;
mod transport;

pub use content::*;
pub use device::*;
pub use dispenser::*;
pub use hardware::*;
pub use inter_stacker::*;
pub use position::*;
pub use safe_door::*;
pub use shutter::*;
pub use transport::*;

/// Represents the CDR status returned by the [`bnr_GetStatus`](bnr_sys::bnr_GetStatus) call.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrStatus {
    pub device_status: DeviceStatus,
    pub dispenser_status: DispenserStatus,
    pub intermediate_stacker_status: InterStackerStatus,
    pub safe_door_status: SafeDoorStatus,
    pub shutter_status: ShutterStatus,
    pub transport_status: TransportStatus,
    pub position_status_list: CdrPositionStatusList,
}

impl CdrStatus {
    /// Creates a new [CdrStatus].
    pub const fn new() -> Self {
        Self {
            device_status: DeviceStatus::new(),
            dispenser_status: DispenserStatus::new(),
            intermediate_stacker_status: InterStackerStatus::new(),
            safe_door_status: SafeDoorStatus::new(),
            shutter_status: ShutterStatus::new(),
            transport_status: TransportStatus::new(),
            position_status_list: CdrPositionStatusList::new(),
        }
    }

    /// Gets the [CdrStatus] as a const pointer to a FFI [XfsCdrStatus](bnr_sys::XfsCdrStatus).
    pub fn as_raw_ptr(&self) -> *const bnr_sys::XfsCdrStatus {
        self as *const _ as *const _
    }

    /// Gets the [CdrStatus] as a mutable pointer to a FFI [XfsCdrStatus](bnr_sys::XfsCdrStatus).
    pub fn as_raw_ptr_mut(&mut self) -> *mut bnr_sys::XfsCdrStatus {
        self as *mut _ as *mut _
    }
}

impl From<CdrStatus> for bnr_sys::XfsCdrStatus {
    fn from(val: CdrStatus) -> Self {
        Self {
            deviceStatus: val.device_status.into(),
            dispenserStatus: val.dispenser_status.into(),
            intermediateStackerStatus: val.intermediate_stacker_status.into(),
            safeDoorStatus: val.safe_door_status.into(),
            shutterStatus: val.shutter_status.into(),
            transportStatus: val.transport_status.into(),
            positionStatusList: val.position_status_list.into(),
        }
    }
}

impl From<bnr_sys::XfsCdrStatus> for CdrStatus {
    fn from(val: bnr_sys::XfsCdrStatus) -> Self {
        Self {
            device_status: val.deviceStatus.into(),
            dispenser_status: val.dispenserStatus.into(),
            intermediate_stacker_status: val.intermediateStackerStatus.into(),
            safe_door_status: val.safeDoorStatus.into(),
            shutter_status: val.shutterStatus.into(),
            transport_status: val.transportStatus.into(),
            position_status_list: val.positionStatusList.into(),
        }
    }
}

impl From<CdrStatus> for HardwareStatus {
    fn from(val: CdrStatus) -> Self {
        [
            HardwareStatus::from(val.device_status),
            HardwareStatus::from(val.dispenser_status),
            HardwareStatus::from(val.intermediate_stacker_status),
            HardwareStatus::from(val.safe_door_status),
            HardwareStatus::from(val.shutter_status),
            HardwareStatus::from(val.transport_status),
            HardwareStatus::from(val.position_status_list),
        ]
        .into()
    }
}

/// Gets the status of the CDR device.
pub fn get_status() -> Result<CdrStatus> {
    let mut status = CdrStatus::new();

    check_res(
        unsafe { bnr_sys::bnr_GetStatus(status.as_raw_ptr_mut()) },
        "get_status",
    )?;

    Ok(status)
}
