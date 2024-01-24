use std::{cmp, fmt};

use crate::xfs::{
    array::XfsArray,
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::{Error, Result};

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
    /// Creates a new [PositionStatus].
    pub const fn new() -> Self {
        Self {
            position: CdrPosition::new(),
            content_status: ContentStatus::new(),
            shutter_status: ShutterStatus::new(),
        }
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "positionStatus"
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

impl From<&PositionStatus> for XfsStruct {
    fn from(val: &PositionStatus) -> Self {
        Self::create([
            val.position.into(),
            val.content_status.into(),
            val.shutter_status.into(),
        ])
    }
}

impl From<PositionStatus> for XfsStruct {
    fn from(val: PositionStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsStruct> for PositionStatus {
    type Error = Error;

    fn try_from(val: &XfsStruct) -> Result<Self> {
        let members = val.members();

        let position: CdrPosition = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == CdrPosition::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"PositionStatus missing "position""#.into()))?
            .try_into()?;

        let content_status: ContentStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == ContentStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"PositionStatus missing "contentStatus""#.into(),
            ))?
            .try_into()?;

        let shutter_status: ShutterStatus = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == ShutterStatus::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"PositionStatus missing "shutterStatus""#.into(),
            ))?
            .try_into()?;

        Ok(Self {
            position,
            content_status,
            shutter_status,
        })
    }
}

impl TryFrom<XfsStruct> for PositionStatus {
    type Error = Error;

    fn try_from(val: XfsStruct) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&PositionStatus> for XfsValue {
    fn from(val: &PositionStatus) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<PositionStatus> for XfsValue {
    fn from(val: PositionStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for PositionStatus {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.xfs_struct()
            .ok_or(Error::Xfs(format!(
                "Expected PositionStatus XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for PositionStatus {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&PositionStatus> for XfsMember {
    fn from(val: &PositionStatus) -> Self {
        Self::create(PositionStatus::xfs_name(), val.into())
    }
}

impl From<PositionStatus> for XfsMember {
    fn from(val: PositionStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for PositionStatus {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().xfs_struct()) {
            (n, Some(xfs)) if n == Self::xfs_name() => xfs.try_into(),
            _ => Err(Error::Xfs(format!(
                "Expected PositionStatus XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for PositionStatus {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
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
    max_size: u32,
    size: u32,
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

    pub const fn xfs_name() -> &'static str {
        "positionStatusList"
    }

    /// Gets the populated size of the [PositionStatus] list.
    pub const fn size(&self) -> usize {
        self.size as usize
    }

    /// Sets the populated size of the [PositionStatus] list.
    ///
    /// **NOTE** `size` must be less-than-or-equal to `POS_STATUS_LIST_LEN`.
    pub fn set_size(&mut self, size: usize) {
        if size <= POS_STATUS_LIST_LEN {
            self.size = size as u32;
        }
    }

    /// Builder function that sets the populated size of the [PositionStatus] list.
    ///
    /// **NOTE** `size` must be less-than-or-equal to `POS_STATUS_LIST_LEN`.
    pub fn with_size(mut self, size: usize) -> Self {
        self.set_size(size);
        self
    }

    /// Gets the [PositionStatus] list.
    pub fn items(&self) -> &[PositionStatus] {
        &self.items[..self.size()]
    }

    /// Sets the [PositionStatus] list.
    ///
    /// **NOTE** sets at most `POS_STATUS_LIST_LEN` items.
    pub fn set_items(&mut self, items: &[PositionStatus]) {
        let len = cmp::min(items.len(), POS_STATUS_LIST_LEN);
        self.size = len as u32;
        self.items[..len].copy_from_slice(&items[..len]);
    }

    /// Builder function that sets the [PositionStatus] list.
    ///
    /// **NOTE** sets at most `POS_STATUS_LIST_LEN` items.
    pub fn with_items(mut self, items: &[PositionStatus]) -> Self {
        self.set_items(items);
        self
    }
}

impl From<CdrPositionStatusList> for HardwareStatus {
    fn from(val: CdrPositionStatusList) -> Self {
        val.items.map(Self::from).into()
    }
}

impl From<&CdrPositionStatusList> for XfsArray {
    fn from(val: &CdrPositionStatusList) -> Self {
        Self::create(val.items().iter().map(XfsValue::from))
    }
}

impl From<CdrPositionStatusList> for XfsArray {
    fn from(val: CdrPositionStatusList) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsArray> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: &XfsArray) -> Result<Self> {
        let data = val.data();
        let len = cmp::min(data.len(), POS_STATUS_LIST_LEN);

        let max_size = POS_STATUS_LIST_LEN as u32;
        let size = len as u32;
        let mut items = [PositionStatus::new(); POS_STATUS_LIST_LEN];

        for (dst, src) in items[..len].iter_mut().zip(data[..len].iter()) {
            *dst = src.inner().try_into()?;
        }

        Ok(Self {
            max_size,
            size,
            items,
        })
    }
}

impl TryFrom<XfsArray> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: XfsArray) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&CdrPositionStatusList> for XfsValue {
    fn from(val: &CdrPositionStatusList) -> Self {
        Self::new().with_array(val.into())
    }
}

impl From<CdrPositionStatusList> for XfsValue {
    fn from(val: CdrPositionStatusList) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.array()
            .ok_or(Error::Xfs(format!(
                "Expected CdrPositionStatusList XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&CdrPositionStatusList> for XfsMember {
    fn from(val: &CdrPositionStatusList) -> Self {
        Self::create(CdrPositionStatusList::xfs_name(), val.into())
    }
}

impl From<CdrPositionStatusList> for XfsMember {
    fn from(val: CdrPositionStatusList) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().array()) {
            (n, Some(xfs)) if n == Self::xfs_name() => xfs.try_into(),
            _ => Err(Error::Xfs(format!(
                "Expected CdrPositionStatusList XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for CdrPositionStatusList {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
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
