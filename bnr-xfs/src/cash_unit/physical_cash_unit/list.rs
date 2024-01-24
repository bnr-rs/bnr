use std::{cmp, fmt};

use crate::{arrays, impl_xfs_array, impl_xfs_struct};

use super::*;

/// Maximum number of [PhysicalCashUnit]s that can be present on a device.
pub const PCU_LIST_LEN: usize = 10;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PhysicalCashUnitItems {
    size: Size,
    #[serde(with = "arrays")]
    items: [PhysicalCashUnit; PCU_LIST_LEN],
}

impl PhysicalCashUnitItems {
    /// Creates a new [PhysicalCashUnitItems] list.
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [PhysicalCashUnit::new(); PCU_LIST_LEN],
        }
    }

    /// Creates a new [PhysicalCashUnitItems] list from the provided parameter.
    pub const fn create(items: [PhysicalCashUnit; PCU_LIST_LEN]) -> Self {
        Self {
            size: Size::create(PCU_LIST_LEN as u32),
            items,
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        PCU_LIST_LEN
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

    /// Gets a list of the [PhysicalCashUnit]s.
    pub fn items(&self) -> &[PhysicalCashUnit] {
        let size = self.size() as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..size].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable list of the [PhysicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [PhysicalCashUnit] {
        let size = self.size() as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..size].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets a list of the [PhysicalCashUnit]s.
    pub fn set_items(&mut self, items: &[PhysicalCashUnit]) {
        let len = cmp::min(items.len(), PCU_LIST_LEN);

        self.set_size(len as u32);
        self.items[..len].copy_from_slice(items[..len].as_ref());
    }

    /// Builder function that sets a list of the [PhysicalCashUnit]s.
    pub fn with_items(mut self, items: &[PhysicalCashUnit]) -> Self {
        self.set_items(items);
        self
    }
}

impl_xfs_array!(PhysicalCashUnitItems, "items");

/// Represents a list of [PhysicalCashUnit]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PhysicalCashUnitList {
    max_size: MaxSize,
    size: Size,
    items: PhysicalCashUnitItems,
}

impl PhysicalCashUnitList {
    /// Creates a new [PhysicalCashUnitList].
    pub const fn new() -> Self {
        Self {
            max_size: MaxSize::create(PCU_LIST_LEN as u32),
            size: Size::new(),
            items: PhysicalCashUnitItems::new(),
        }
    }

    /// Creates a new [PhysicalCashUnitList] from the provided parameter.
    pub const fn create(items: [PhysicalCashUnit; PCU_LIST_LEN]) -> Self {
        Self {
            max_size: MaxSize::create(PCU_LIST_LEN as u32),
            size: Size::create(PCU_LIST_LEN as u32),
            items: PhysicalCashUnitItems::create(items),
        }
    }

    /// Gets the max size.
    pub const fn max_size(&self) -> u32 {
        self.max_size.inner()
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: u32) {
        self.size.set_inner(size);
        self.items.set_size(size);
    }

    /// Builder function that sets the size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets a list of the [PhysicalCashUnit]s.
    pub fn items(&self) -> &[PhysicalCashUnit] {
        self.items.items()
    }

    /// Gets a mutable list of the [PhysicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [PhysicalCashUnit] {
        self.items.items_mut()
    }

    /// Sets a list of the [PhysicalCashUnit]s.
    pub fn set_items(&mut self, items: &[PhysicalCashUnit]) {
        self.items.set_items(items);
        self.set_size(self.items.size());
    }

    /// Builder function that sets a list of the [PhysicalCashUnit]s.
    pub fn with_items(mut self, items: &[PhysicalCashUnit]) -> Self {
        self.set_items(items);
        self
    }
}

impl fmt::Display for PhysicalCashUnitList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size":{},"#, self.max_size)?;
        write!(f, r#""size":{},"#, self.size)?;

        write!(f, r#""items":["#)?;
        let item_len = self.items().len();
        for (i, item) in self.items().iter().enumerate() {
            write!(f, r#"{item}"#)?;

            if i != item_len - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]}}")
    }
}

impl_xfs_struct!(PhysicalCashUnitList, "physicalCashUnits", [size: Size, items: PhysicalCashUnitItems]);
