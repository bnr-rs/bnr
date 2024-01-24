//! BNR device status types.

use std::fmt;

use crate::xfs::{method_response::XfsMethodResponse, params::XfsParam};
use crate::{impl_xfs_struct, Error, Result};

mod content;
mod device;
mod dispenser;
mod hardware;
mod inter_stacker;
mod position;
mod position_capabilities;
mod position_status;
mod safe_door;
mod shutter;
mod transport;

pub use content::*;
pub use device::*;
pub use dispenser::*;
pub use hardware::*;
pub use inter_stacker::*;
pub use position::*;
pub use position_capabilities::*;
pub use position_status::*;
pub use safe_door::*;
pub use shutter::*;
pub use transport::*;

/// Represents the CDR status returned by the
/// [`get_status`](crate::device_handle::DeviceHandle::get_status) call.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrStatus {
    pub device_status: DeviceStatus,
    pub dispenser_status: DispenserStatus,
    pub intermediate_stacker_status: IntermediateStackerStatus,
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
            intermediate_stacker_status: IntermediateStackerStatus::new(),
            safe_door_status: SafeDoorStatus::new(),
            shutter_status: ShutterStatus::new(),
            transport_status: TransportStatus::new(),
            position_status_list: CdrPositionStatusList::new(),
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

impl_xfs_struct!(
    CdrStatus,
    "status",
    [
        device_status: DeviceStatus,
        dispenser_status: DispenserStatus,
        intermediate_stacker_status: IntermediateStackerStatus,
        safe_door_status: SafeDoorStatus,
        shutter_status: ShutterStatus,
        transport_status: TransportStatus,
        position_status_list: CdrPositionStatusList
    ]
);

impl From<&CdrStatus> for XfsParam {
    fn from(val: &CdrStatus) -> Self {
        Self::create(val.into())
    }
}

impl From<CdrStatus> for XfsParam {
    fn from(val: CdrStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsParam> for CdrStatus {
    type Error = Error;

    fn try_from(val: &XfsParam) -> Result<Self> {
        val.value().try_into()
    }
}

impl TryFrom<XfsParam> for CdrStatus {
    type Error = Error;

    fn try_from(val: XfsParam) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsMethodResponse> for CdrStatus {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|p| p.inner())
            .find(|p| p.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected CdrStatus XfsMethodResponse, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for CdrStatus {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CdrStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"device_status": "{}","#, self.device_status)?;
        write!(f, r#" "dispenser_status": "{}","#, self.dispenser_status)?;
        write!(
            f,
            r#" "intermediate_stacker_status": "{}","#,
            self.intermediate_stacker_status
        )?;
        write!(f, r#" "safe_door_status": "{}","#, self.safe_door_status)?;
        write!(f, r#" "shutter_status": "{}","#, self.shutter_status)?;
        write!(f, r#" "transport_status": "{}","#, self.transport_status)?;
        write!(
            f,
            r#" "position_status_list": {}}}"#,
            self.position_status_list
        )
    }
}
