use std::{cmp, fmt};

use crate::impl_xfs_array;
use crate::xfs::method_response::XfsMethodResponse;
use crate::{Error, Result};

use super::DenominationInfo;

pub const DENOMINATION_LIST_LEN: usize = 61;

const DENOM_INIT: DenominationInfo = DenominationInfo::new();

/// Represents a list of [DenominationInfo].
#[derive(Clone, Debug, PartialEq)]
pub struct DenominationList {
    size: usize,
    items: [DenominationInfo; DENOMINATION_LIST_LEN],
}

impl DenominationList {
    /// Creates a new [DenominationList].
    pub const fn new() -> Self {
        Self {
            size: 0,
            items: [DENOM_INIT; DENOMINATION_LIST_LEN],
        }
    }

    /// Gets a reference to the populated [DenominationInfo] items.
    pub fn items(&self) -> &[DenominationInfo] {
        let len = cmp::min(self.size, DENOMINATION_LIST_LEN);
        &self.items[..len]
    }

    /// Gets a mutable reference to the populated [DenominationInfo] items.
    pub fn items_mut(&mut self) -> &mut [DenominationInfo] {
        let len = cmp::min(self.size, DENOMINATION_LIST_LEN);
        &mut self.items[..len]
    }

    /// Adds a [DenominationInfo] item to the list.
    ///
    /// **NOTE**: If the [DenominationList] is at capacity, no item will be pushed.
    pub fn push(&mut self, item: DenominationInfo) {
        if self.size < DENOMINATION_LIST_LEN {
            self.items[self.size] = item;
            self.size += 1;
        }
    }

    /// Pops a [DenominationInfo] from the back of the list.
    ///
    /// **NOTE**: Returns `None` if no items remain in the list.
    pub fn pop(&mut self) -> Option<DenominationInfo> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.items[self.size].clone())
        } else {
            None
        }
    }

    /// Gets the maximum number of items.
    pub const fn max_size() -> usize {
        DENOMINATION_LIST_LEN
    }

    /// Gets a the size of populated [DenominationInfo] items.
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Sets a the size of populated [DenominationInfo] items.
    pub fn set_size(&mut self, size: u32) {
        let len = cmp::min(size as usize, DENOMINATION_LIST_LEN);
        self.size = len;
    }
}

impl_xfs_array!(DenominationList, "denominationList");

impl TryFrom<&XfsMethodResponse> for DenominationList {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().array().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected DenominationList XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for DenominationList {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for DenominationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""items": ["#)?;
        for (i, item) in self.items().iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]}}")
    }
}
