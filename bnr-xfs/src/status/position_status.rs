use std::fmt;

use super::{CdrPosition, ContentStatus, HardwareStatus, ShutterStatus};

pub const POS_STATUS_LIST_LEN: usize = 2;

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
    pub items: [PositionStatus; POS_STATUS_LIST_LEN],
}

impl CdrPositionStatusList {
    /// Creates a new [XfsCdrPositionStatusList].
    pub const fn new() -> Self {
        Self {
            max_size: POS_STATUS_LIST_LEN as u32,
            size: 0,
            items: [PositionStatus::new(); POS_STATUS_LIST_LEN],
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

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }

        write!(f, "]}}")
    }
}
