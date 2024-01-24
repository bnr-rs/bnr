use std::fmt;

use super::{ContentStatus, HardwareStatus, ShutterStatus};

/// Status of a CDR stacker.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PositionStatus {
    pub position: CdrPosition,
    pub content_status: ContentStatus,
    pub shutter_status: ShutterStatus,
}

impl PositionStatus {
    pub const fn new() -> Self {
        Self {
            position: CdrPosition::new(),
            content_status: ContentStatus::new(),
            shutter_status: ShutterStatus::new(),
        }
    }
}

impl From<PositionStatus> for bnr_sys::PositionStatus {
    fn from(val: PositionStatus) -> Self {
        Self {
            position: val.position.into(),
            contentStatus: val.content_status.into(),
            shutterStatus: val.shutter_status.into(),
        }
    }
}

impl From<bnr_sys::PositionStatus> for PositionStatus {
    fn from(val: bnr_sys::PositionStatus) -> Self {
        Self {
            position: val.position.into(),
            content_status: val.contentStatus.into(),
            shutter_status: val.shutterStatus.into(),
        }
    }
}

impl From<PositionStatus> for HardwareStatus {
    fn from(val: PositionStatus) -> Self {
        [
            Self::from(val.content_status),
            Self::from(val.shutter_status),
        ]
        .into()
    }
}

impl fmt::Display for PositionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"position": "{}", "#, self.position)?;
        write!(f, r#""content_status": "{}", "#, self.content_status)?;
        write!(f, r#""shutter_status": "{}"}}"#, self.shutter_status)
    }
}

/// List of CDR stacker status by position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrPositionStatusList {
    pub max_size: u32,
    pub size: u32,
    pub items: [PositionStatus; 2],
}

impl CdrPositionStatusList {
    /// Creates a new [XfsCdrPositionStatusList].
    pub const fn new() -> Self {
        Self {
            max_size: 2,
            size: 0,
            items: [PositionStatus::new(); 2],
        }
    }
}

impl From<CdrPositionStatusList> for bnr_sys::XfsCdrPositionStatusList {
    fn from(val: CdrPositionStatusList) -> Self {
        Self {
            maxSize: val.max_size,
            size: val.size,
            items: val.items.map(bnr_sys::PositionStatus::from),
        }
    }
}

impl From<bnr_sys::XfsCdrPositionStatusList> for CdrPositionStatusList {
    fn from(val: bnr_sys::XfsCdrPositionStatusList) -> Self {
        Self {
            max_size: val.maxSize,
            size: val.size,
            items: val.items.map(PositionStatus::from),
        }
    }
}

impl From<CdrPositionStatusList> for HardwareStatus {
    fn from(val: CdrPositionStatusList) -> Self {
        val.items.map(Self::from).into()
    }
}

impl fmt::Display for CdrPositionStatusList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""{{"max_size": {}, "size": {}"#,
            self.max_size, self.size
        )?;
        write!(f, r#", "items": ["#)?;

        let items_len = self.items.len();
        for (i, item) in self.items.iter().enumerate() {
            write!(f, "{item}")?;

            if i != items_len - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]}}")
    }
}

/// Represents a CDR position
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CdrPosition {
    /// Outlet position
    #[default]
    Bottom = bnr_sys::XfsCdrPosition_XFS_C_CDR_POS_BOTTOM,
    /// Inlet position
    Top = bnr_sys::XfsCdrPosition_XFS_C_CDR_POS_TOP,
}

impl CdrPosition {
    /// Creates a new [CdrPosition].
    pub const fn new() -> Self {
        Self::Bottom
    }
}

impl From<u32> for CdrPosition {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XfsCdrPosition_XFS_C_CDR_POS_BOTTOM => Self::Bottom,
            ds if ds == bnr_sys::XfsCdrPosition_XFS_C_CDR_POS_TOP => Self::Top,
            _ => Self::Bottom,
        }
    }
}

impl From<CdrPosition> for u32 {
    fn from(val: CdrPosition) -> Self {
        val as u32
    }
}

impl From<CdrPosition> for &'static str {
    fn from(val: CdrPosition) -> Self {
        match val {
            CdrPosition::Bottom => "bottom",
            CdrPosition::Top => "top",
        }
    }
}

impl From<&CdrPosition> for &'static str {
    fn from(val: &CdrPosition) -> Self {
        (*val).into()
    }
}

impl fmt::Display for CdrPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
