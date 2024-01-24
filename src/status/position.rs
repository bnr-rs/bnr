use std::{cmp, fmt};

use super::{ContentStatus, HardwareStatus, ShutterStatus};

pub const CDR_POS_CAP_LIST_LEN: usize = 2;

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
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Characteristics of an input/output position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CdrPositionCapabilities {
    /// Position fo the unit.
    pub position: CdrPosition,
    /// Specifies whether shutter status reporting is supported.
    pub shutter_status_supported: bool,
    /// Defines if the shutter has to be explicitly controlled by the application.
    pub shutter_cmd: bool,
    /// Specifies whether there is a sensor to detect if the position is empty.
    pub content_status_supported: bool,
    /// Maximum number of items which this position can hold.
    pub max_items: u32,
    /// Specifies whether this position can be used as source for an accept command.
    pub input: bool,
    /// Specifies whether this position can be used as target for a dispense command.
    pub output: bool,
    /// Specifies whether this position can be used as target for
    /// [cash_in_rollback](crate::cash::cash_in_rollback) command.
    pub rollback: bool,
    /// Specifies whether refused notes can be moved to this position during
    /// [cash_in](crate::cash::cash_in) command.
    pub refusal: bool,
}

impl CdrPositionCapabilities {
    /// Creates a new [CdrPositionCapabilities].
    pub const fn new() -> Self {
        Self {
            position: CdrPosition::new(),
            shutter_status_supported: false,
            shutter_cmd: false,
            content_status_supported: false,
            max_items: 0,
            input: false,
            output: false,
            rollback: false,
            refusal: false,
        }
    }
}

impl From<&CdrPositionCapabilities> for bnr_sys::XfsCdrPositionCapabilities {
    fn from(val: &CdrPositionCapabilities) -> Self {
        Self {
            position: val.position.into(),
            shutterStatusSupported: val.shutter_status_supported.into(),
            shutterCmd: val.shutter_cmd.into(),
            contentStatusSupported: val.content_status_supported.into(),
            maxItems: val.max_items,
            input: val.input.into(),
            output: val.output.into(),
            rollback: val.rollback.into(),
            refusal: val.refusal.into(),
        }
    }
}

impl From<CdrPositionCapabilities> for bnr_sys::XfsCdrPositionCapabilities {
    fn from(val: CdrPositionCapabilities) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsCdrPositionCapabilities> for CdrPositionCapabilities {
    fn from(val: &bnr_sys::XfsCdrPositionCapabilities) -> Self {
        Self {
            position: val.position.into(),
            shutter_status_supported: val.shutterStatusSupported != 0,
            shutter_cmd: val.shutterCmd != 0,
            content_status_supported: val.contentStatusSupported != 0,
            max_items: val.maxItems,
            input: val.input != 0,
            output: val.output != 0,
            rollback: val.rollback != 0,
            refusal: val.refusal != 0,
        }
    }
}

impl From<bnr_sys::XfsCdrPositionCapabilities> for CdrPositionCapabilities {
    fn from(val: bnr_sys::XfsCdrPositionCapabilities) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CdrPositionCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""position":{},"#, self.position)?;
        write!(
            f,
            r#""shutter_status_supported":{},"#,
            self.shutter_status_supported
        )?;
        write!(f, r#""shutter_cmd":{},"#, self.shutter_cmd)?;
        write!(
            f,
            r#""content_status_supported":{},"#,
            self.content_status_supported
        )?;
        write!(f, r#""max_items":{},"#, self.max_items)?;
        write!(f, r#""input":{},"#, self.input)?;
        write!(f, r#""output":{},"#, self.output)?;
        write!(f, r#""rollback":{},"#, self.rollback)?;
        write!(f, r#""refusal":{}"#, self.refusal)?;
        write!(f, "}}")
    }
}

/// List of position capabilties.
///
/// One for each position supported by the device.
///
/// See [Capabilities](crate::capabilities::Capabilities).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CdrPositionCapabilitiesList {
    max_size: u32,
    size: u32,
    items: [CdrPositionCapabilities; CDR_POS_CAP_LIST_LEN],
}

impl CdrPositionCapabilitiesList {
    /// Creates a new [CdrPositionCapabilitiesList].
    pub const fn new() -> Self {
        Self {
            max_size: CDR_POS_CAP_LIST_LEN as u32,
            size: 0,
            items: [CdrPositionCapabilities::new(); CDR_POS_CAP_LIST_LEN],
        }
    }

    /// Gets the maximum size of the list.
    pub const fn max_size(&self) -> u32 {
        self.max_size
    }

    /// Gets the size of the list.
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Sets the size of the list.
    ///
    /// No-op if the size is invalid (> [CDR_POS_CAP_LIST_LEN]).
    pub fn set_size(&mut self, size: u32) {
        if (size as usize) <= CDR_POS_CAP_LIST_LEN {
            self.size = size;
        }
    }

    /// Builder function that sets the size of the list.
    ///
    /// No-op if the size is invalid (> [CDR_POS_CAP_LIST_LEN]).
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets whether the size of the list is valid.
    pub const fn size_is_valid(&self) -> bool {
        (self.size as usize) <= CDR_POS_CAP_LIST_LEN
    }

    /// Gets a reference to the [CdrPositionCapabilities] list items.
    pub fn items(&self) -> &[CdrPositionCapabilities] {
        if self.size_is_valid() {
            self.items[..self.size as usize].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Sets the [CdrPositionCapabilities] list items.
    ///
    /// Only sets up to [max_size](CDR_POS_CAP_LIST_LEN) items.
    pub fn set_items(&mut self, items: &[CdrPositionCapabilities]) {
        let len = cmp::min(CDR_POS_CAP_LIST_LEN, items.len());
        self.items[..len].copy_from_slice(&items[..len]);
    }

    /// Builder function that sets the [CdrPositionCapabilities] list items.
    ///
    /// Only sets up to [max_size](CDR_POS_CAP_LIST_LEN) items.
    pub fn with_items(mut self, items: &[CdrPositionCapabilities]) -> Self {
        self.set_items(items);
        self
    }
}

impl From<&CdrPositionCapabilitiesList> for bnr_sys::XfsCdrPositionCapabilitiesList {
    fn from(val: &CdrPositionCapabilitiesList) -> Self {
        Self {
            maxSize: val.max_size,
            size: val.size,
            items: val.items.map(bnr_sys::XfsCdrPositionCapabilities::from),
        }
    }
}

impl From<CdrPositionCapabilitiesList> for bnr_sys::XfsCdrPositionCapabilitiesList {
    fn from(val: CdrPositionCapabilitiesList) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsCdrPositionCapabilitiesList> for CdrPositionCapabilitiesList {
    fn from(val: &bnr_sys::XfsCdrPositionCapabilitiesList) -> Self {
        Self {
            max_size: val.maxSize,
            size: val.size,
            items: val.items.map(CdrPositionCapabilities::from),
        }
    }
}

impl From<bnr_sys::XfsCdrPositionCapabilitiesList> for CdrPositionCapabilitiesList {
    fn from(val: bnr_sys::XfsCdrPositionCapabilitiesList) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CdrPositionCapabilitiesList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size":{},"#, self.max_size)?;
        write!(f, r#""size":{},"#, self.size)?;
        write!(f, r#""items":["#)?;

        let items = self.items();
        let items_len = items.len();
        for (i, item) in items.iter().enumerate() {
            write!(f, "{item}")?;
            if i != items_len - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, "]}}")
    }
}
