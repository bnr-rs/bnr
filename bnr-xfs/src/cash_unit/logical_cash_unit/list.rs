use std::{cmp, fmt};

use crate::cash_unit::{MaxSize, Size};
use crate::{arrays, impl_xfs_array, impl_xfs_struct};

use super::*;

/// Maximum number of [LogicalCashUnit]s that can be present on a device.
pub const LCU_LIST_LEN: usize = 83;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogicalCashUnitItems {
    size: Size,
    #[serde(with = "arrays")]
    items: [LogicalCashUnit; LCU_LIST_LEN],
}

impl LogicalCashUnitItems {
    /// Creates a new [LogicalCashUnitItems] list.
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [LogicalCashUnit::new(); LCU_LIST_LEN],
        }
    }

    /// Creates a new [LogicalCashUnitItems] list from the provided parameter.
    pub fn create(items: [LogicalCashUnit; LCU_LIST_LEN]) -> Self {
        Self {
            size: Size::create(LCU_LIST_LEN as u32),
            items,
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        LCU_LIST_LEN
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

    /// Gets a list of the [LogicalCashUnit]s.
    pub fn items(&self) -> &[LogicalCashUnit] {
        let size = self.size() as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..size].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable list of the [LogicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [LogicalCashUnit] {
        let size = self.size() as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..size].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets a list of the [LogicalCashUnit]s.
    pub fn set_items(&mut self, items: &[LogicalCashUnit]) {
        let len = cmp::min(items.len(), LCU_LIST_LEN);

        self.set_size(len as u32);
        self.items[..len].copy_from_slice(items[..len].as_ref());
    }

    /// Builder function that sets a list of the [LogicalCashUnit]s.
    pub fn with_items(mut self, items: &[LogicalCashUnit]) -> Self {
        self.set_items(items);
        self
    }
}

impl Default for LogicalCashUnitItems {
    fn default() -> Self {
        Self::new()
    }
}

impl_xfs_array!(LogicalCashUnitItems, "items");

/// Represents a list of [LogicalCashUnit]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogicalCashUnitList {
    max_size: MaxSize,
    size: Size,
    items: LogicalCashUnitItems,
}

impl LogicalCashUnitList {
    /// Creates a new [LogicalCashUnitList].
    pub const fn new() -> Self {
        Self {
            max_size: MaxSize::create(LCU_LIST_LEN as u32),
            size: Size::new(),
            items: LogicalCashUnitItems::new(),
        }
    }

    /// Gets the max size of the items list.
    pub const fn max_size(&self) -> u32 {
        self.max_size.inner()
    }

    /// Gets the size of the items list.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size of the items list.
    pub fn set_size(&mut self, size: u32) {
        self.size.set_inner(size);
        self.items.set_size(size);
    }

    /// Builder function that sets the size of the items list.
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets a list of the [LogicalCashUnit]s.
    pub fn items(&self) -> &[LogicalCashUnit] {
        self.items.items()
    }

    /// Gets a mutable list of the [LogicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [LogicalCashUnit] {
        self.items.items_mut()
    }

    /// Sets a list of [LogicalCashUnit]s.
    pub fn set_items(&mut self, items: &[LogicalCashUnit]) {
        self.items.set_items(items);
        self.size.set_inner(self.items.size());
    }

    /// Builder function that sets a list of [LogicalCashUnit]s.
    pub fn with_items(mut self, items: &[LogicalCashUnit]) -> Self {
        self.set_items(items);
        self
    }
}

impl Default for LogicalCashUnitList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LogicalCashUnitList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size": {},"#, self.max_size)?;
        write!(f, r#""size": {},"#, self.size)?;

        write!(f, r#""items": ["#)?;
        for (i, item) in self.items().iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{item}")?;
        }
        write!(f, "]}}")
    }
}

impl_xfs_struct!(LogicalCashUnitList, "logicalCashUnits", [size: Size, items: LogicalCashUnitItems]);
