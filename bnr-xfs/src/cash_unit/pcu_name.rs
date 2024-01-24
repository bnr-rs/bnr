use std::{cmp, fmt};

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

pub const PCU_NAME_LEN: usize = 5;

/// Represents the name of a PCU.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PcuName([u8; 5]);

impl PcuName {
    /// Creates a new [PcuName].
    pub const fn new() -> Self {
        Self([0u8; PCU_NAME_LEN])
    }

    /// Creates a new [PcuName] from the provided parameter.
    pub fn create(s: &[u8]) -> Self {
        let len = cmp::min(s.len(), PCU_NAME_LEN);
        let mut inner = [0u8; PCU_NAME_LEN];

        inner[..len].copy_from_slice(&s[..len]);

        Self(inner)
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "name"
    }

    /// Gets a reference to the inner representation of the [PcuName].
    pub fn inner(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Gets a reference to the inner string representation of the [PcuName].
    pub fn inner_str(&self) -> &str {
        std::str::from_utf8(self.0.as_ref()).unwrap_or("")
    }

    /// Gets a mutable reference to the inner representation of the [PcuName].
    pub fn inner_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Converts into the inner representation of the [PcuName].
    pub fn into_inner(self) -> [u8; 5] {
        self.0
    }
}

impl fmt::Display for PcuName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.inner_str())
    }
}

impl From<&[u8]> for PcuName {
    fn from(val: &[u8]) -> Self {
        Self::create(val)
    }
}

impl From<&str> for PcuName {
    fn from(val: &str) -> Self {
        Self::create(val.as_bytes())
    }
}

impl From<&PcuName> for XfsValue {
    fn from(val: &PcuName) -> Self {
        Self::new().with_string(val.inner_str())
    }
}

impl From<PcuName> for XfsValue {
    fn from(val: PcuName) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for PcuName {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        Ok(val
            .string()
            .ok_or(Error::Xfs(format!(
                "Expected PcuName XfsValue, have: {val}"
            )))?
            .into())
    }
}

impl TryFrom<XfsValue> for PcuName {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&PcuName> for XfsMember {
    fn from(val: &PcuName) -> Self {
        Self::create(PcuName::xfs_name(), val.into())
    }
}

impl From<PcuName> for XfsMember {
    fn from(val: PcuName) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for PcuName {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().string()) {
            (n, Some(s)) if n == PcuName::xfs_name() => Ok(s.into()),
            _ => Err(Error::Xfs(format!(
                "Expected PcuName XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for PcuName {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}
