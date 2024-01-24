//! BNR device status types.

use std::fmt;

use crate::xfs::{
    method_response::XfsMethodResponse, params::XfsParam, value::XfsValue, xfs_struct::XfsStruct,
};
use crate::{Error, Result};

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

impl From<&CdrStatus> for XfsStruct {
    fn from(val: &CdrStatus) -> Self {
        Self::create([
            val.device_status.into(),
            val.dispenser_status.into(),
            val.intermediate_stacker_status.into(),
            val.safe_door_status.into(),
            val.shutter_status.into(),
            val.transport_status.into(),
            val.position_status_list.into(),
        ])
    }
}

impl From<CdrStatus> for XfsStruct {
    fn from(val: CdrStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsStruct> for CdrStatus {
    type Error = Error;

    fn try_from(val: &XfsStruct) -> Result<Self> {
        let members = val.members();

        let device_status: DeviceStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == DeviceStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"CdrStatus missing "deviceStatus""#.into()))?
            .try_into()?;

        let dispenser_status: DispenserStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == DispenserStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"CdrStatus missing "dispenserStatus""#.into()))?
            .try_into()?;

        let intermediate_stacker_status: InterStackerStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == InterStackerStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"CdrStatus missing "intermediateStackerStatus""#.into(),
            ))?
            .try_into()?;

        let safe_door_status: SafeDoorStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == SafeDoorStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"CdrStatus missing "safeDoorStatus""#.into()))?
            .try_into()?;

        let shutter_status: ShutterStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == ShutterStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"CdrStatus missing "shutterStatus""#.into()))?
            .try_into()?;

        let transport_status: TransportStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == TransportStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"CdrStatus missing "transportStatus""#.into()))?
            .try_into()?;

        let position_status_list: CdrPositionStatusList = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == CdrPositionStatusList::xfs_name() && m.value().array().is_some())
            .ok_or(Error::Xfs(
                r#"CdrStatus missing "positionStatusList""#.into(),
            ))?
            .try_into()?;

        Ok(Self {
            device_status,
            dispenser_status,
            intermediate_stacker_status,
            safe_door_status,
            shutter_status,
            transport_status,
            position_status_list,
        })
    }
}

impl TryFrom<XfsStruct> for CdrStatus {
    type Error = Error;

    fn try_from(val: XfsStruct) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&CdrStatus> for XfsValue {
    fn from(val: &CdrStatus) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<CdrStatus> for XfsValue {
    fn from(val: CdrStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for CdrStatus {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.xfs_struct()
            .ok_or(Error::Xfs(format!(
                "Expected CdrStatus XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for CdrStatus {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

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
