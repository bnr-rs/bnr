use std::{cmp, fmt};

use crate::{MaxSize, Size};

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
        write!(f, "{{")?;
        write!(f, r#""position": {}, "#, self.position)?;
        write!(f, r#""content_status": {}, "#, self.content_status)?;
        write!(f, r#""shutter_status": {}"#, self.shutter_status)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    PositionStatus,
    "positionStatus",
    [
        position: CdrPosition,
        content_status: ContentStatus,
        shutter_status: ShutterStatus
    ]
);

/// List of CDR stacker status by position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PositionStatusItems {
    size: Size,
    items: [PositionStatus; POS_STATUS_LIST_LEN],
}

impl PositionStatusItems {
    /// Creates a new [PositionStatusItems].
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [PositionStatus::new(); POS_STATUS_LIST_LEN],
        }
    }

    /// Creates a new [PositionStatusItems] from the provided parameter.
    pub const fn create(items: [PositionStatus; POS_STATUS_LIST_LEN]) -> Self {
        Self {
            size: Size::create(POS_STATUS_LIST_LEN as u32),
            items,
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        POS_STATUS_LIST_LEN
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: u32) {
        if size as usize <= Self::max_size() {
            self.size.set_inner(size);
        }
    }

    /// Builder function that sets the size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets a reference to the list of [PositionStatus] items.
    pub fn items(&self) -> &[PositionStatus] {
        let len = self.size() as usize;
        if len <= Self::max_size() {
            self.items[..len].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable reference to the list of [PositionStatus] items.
    pub fn items_mut(&mut self) -> &mut [PositionStatus] {
        let len = self.size() as usize;
        if len <= Self::max_size() {
            self.items[..len].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets the list of [PositionStatus] items.
    pub fn set_items(&mut self, items: &[PositionStatus]) {
        let len = cmp::min(items.len(), Self::max_size());
        self.items[..len].copy_from_slice(&items[..len]);
    }

    /// Builder function that sets the list of [PositionStatus] items.
    pub fn with_items(mut self, items: &[PositionStatus]) -> Self {
        self.set_items(items);
        self
    }
}

impl fmt::Display for PositionStatusItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, item) in self.items().iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }

        write!(f, "]")
    }
}

impl_xfs_array!(PositionStatusItems, "items");

/// List of CDR stacker status by position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrPositionStatusList {
    max_size: MaxSize,
    size: Size,
    items: PositionStatusItems,
}

impl CdrPositionStatusList {
    /// Creates a new [CdrPositionStatusList].
    pub const fn new() -> Self {
        Self {
            max_size: MaxSize::create(POS_STATUS_LIST_LEN as u32),
            size: Size::new(),
            items: PositionStatusItems::new(),
        }
    }

    /// Creates a new [XfsCdrPositionStatusList] from the provided parameter.
    pub const fn create(items: [PositionStatus; POS_STATUS_LIST_LEN]) -> Self {
        Self {
            max_size: MaxSize::create(POS_STATUS_LIST_LEN as u32),
            size: Size::create(POS_STATUS_LIST_LEN as u32),
            items: PositionStatusItems::create(items),
        }
    }

    /// Gets the populated size of the [PositionStatus] list.
    pub const fn size(&self) -> usize {
        self.size.inner() as usize
    }

    /// Sets the populated size of the [PositionStatus] list.
    ///
    /// **NOTE** `size` must be less-than-or-equal to `POS_STATUS_LIST_LEN`.
    pub fn set_size(&mut self, size: usize) {
        if size <= POS_STATUS_LIST_LEN {
            self.size.set_inner(size as u32);
            self.items.set_size(size as u32);
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
        self.items.items()
    }

    /// Sets the [PositionStatus] list.
    ///
    /// **NOTE** sets at most `POS_STATUS_LIST_LEN` items.
    pub fn set_items(&mut self, items: &[PositionStatus]) {
        self.items.set_items(items);
        self.size.set_inner(self.items.size());
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
        val.items.items.map(Self::from).into()
    }
}

impl fmt::Display for CdrPositionStatusList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""{{"max_size": {}, "size": {}, "items": {}}}"#,
            self.max_size, self.size, self.items
        )
    }
}

impl_xfs_struct!(
    CdrPositionStatusList,
    "positionStatusList",
    [
        size: Size,
        items: PositionStatusItems
    ]
);
