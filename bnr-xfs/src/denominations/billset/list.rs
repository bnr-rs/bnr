use std::{cmp, fmt};

use crate::impl_xfs_array;
use crate::xfs::method_response::XfsMethodResponse;
use crate::{Error, Result};

use super::BillsetInfo;

pub const BILLSET_ID_LIST_LEN: usize = 61;

const BILLSET_INFO_DEFAULT: BillsetInfo = BillsetInfo::new();

/// Represents a list of [BillsetInfo].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BillsetIdList {
    size: usize,
    items: [BillsetInfo; BILLSET_ID_LIST_LEN],
}

impl BillsetIdList {
    /// Creates a new [BillsetIdList].
    pub const fn new() -> Self {
        Self {
            size: 0,
            items: [BILLSET_INFO_DEFAULT; BILLSET_ID_LIST_LEN],
        }
    }

    /// Gets the maximum number of [BillsetInfo] items.
    pub const fn max_size() -> usize {
        BILLSET_ID_LIST_LEN
    }

    /// Gets the size of the [BillsetIdList].
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Sets the size of the [BillsetIdList].
    ///
    /// No-op if `val` is larger than [BILLSET_ID_LIST_LEN].
    pub fn set_size(&mut self, val: u32) {
        let size = val as usize;
        if size <= BILLSET_ID_LIST_LEN {
            self.size = size;
        }
    }

    /// Builder function that sets the size of the [BillsetIdList].
    ///
    /// No-op if `val` is larger than [BILLSET_ID_LIST_LEN].
    pub fn with_size(mut self, val: u32) -> Self {
        self.set_size(val);
        self
    }

    /// Gets a reference to the list of set [BillsetInfo] items.
    pub fn items(&self) -> &[BillsetInfo] {
        let len = cmp::min(self.size, BILLSET_ID_LIST_LEN);
        self.items[..len].as_ref()
    }

    /// Sets the list of [BillsetInfo] items.
    pub fn set_items(&mut self, val: &[BillsetInfo]) {
        let len = cmp::min(val.len(), BILLSET_ID_LIST_LEN);
        self.items[..len]
            .iter_mut()
            .zip(val[..len].iter())
            .for_each(|(dst, src)| *dst = src.clone());
        self.items[len..]
            .iter_mut()
            .for_each(|item| *item = BillsetInfo::new());
        self.size = len;
    }

    /// Builder function that sets the list of [BillsetInfo] items.
    pub fn with_items(mut self, val: &[BillsetInfo]) -> Self {
        self.set_items(val);
        self
    }

    /// Pushes a [BillsetInfo] onto the end of the list.
    ///
    /// No-op if the list is at capacity.
    pub fn push_item(&mut self, val: BillsetInfo) {
        if self.size < BILLSET_ID_LIST_LEN {
            self.items[self.size] = val;
            self.size += 1;
        }
    }
}

impl Default for BillsetIdList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BillsetIdList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"items":["#)?;
        for (i, item) in self.items().iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]}}")
    }
}

impl_xfs_array!(BillsetIdList, "billsetIdList");

impl TryFrom<&XfsMethodResponse> for BillsetIdList {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().array().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected BillsetIdList XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for BillsetIdList {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
