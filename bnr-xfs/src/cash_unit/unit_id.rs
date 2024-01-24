use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

pub const UNIT_ID_LEN: usize = 20;

/// Represents the ID of an LCU and/or PCU.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UnitId(u64);

impl UnitId {
    /// Creates a new [UnitId].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [UnitId] from the provided parameter.
    pub const fn create(id: u64) -> Self {
        Self(id)
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "unitId"
    }

    /// Gets the inner representation of the [UnitId].
    pub const fn inner(&self) -> u64 {
        self.0
    }

    /// Sets the inner representation of the [UnitId].
    pub fn set_inner(&mut self, val: u64) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [UnitId].
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl fmt::Display for UnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl From<u64> for UnitId {
    fn from(val: u64) -> Self {
        Self::create(val)
    }
}

impl From<&UnitId> for XfsValue {
    fn from(val: &UnitId) -> Self {
        Self::new().with_string(format!("{val}"))
    }
}

impl From<UnitId> for XfsValue {
    fn from(val: UnitId) -> Self {
        (&val).into()
    }
}

impl TryFrom<&str> for UnitId {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        Ok(val
            .parse::<u64>()
            .map_err(|_| Error::Xfs(format!("Expected UnitId string, have: {val}")))?
            .into())
    }
}

impl TryFrom<&XfsValue> for UnitId {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.string()
            .ok_or(Error::Xfs(format!("Expected UnitId XfsValue, have: {val}")))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for UnitId {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&UnitId> for XfsMember {
    fn from(val: &UnitId) -> Self {
        Self::create(UnitId::xfs_name(), val.into())
    }
}

impl From<UnitId> for XfsMember {
    fn from(val: UnitId) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for UnitId {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().string()) {
            (n, Some(s)) if n == Self::xfs_name() => s.try_into(),
            _ => Err(Error::Xfs(format!(
                "Expected UnitId XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for UnitId {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}
