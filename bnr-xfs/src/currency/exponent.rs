use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Represents the exponent field of a [Currency].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Exponent(i32);

impl Exponent {
    /// Creates a new [Exponent].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Exponent] from the provided parameter.
    pub const fn create(val: i32) -> Self {
        Self(val)
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "exponent"
    }

    /// Gets the inner representation of the [Exponent].
    pub const fn inner(&self) -> i32 {
        self.0
    }

    /// Sets the inner representation of the [Exponent].
    pub fn set_inner(&mut self, val: i32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Exponent].
    pub fn into_inner(self) -> i32 {
        self.0
    }
}

impl From<&Exponent> for i32 {
    fn from(val: &Exponent) -> Self {
        val.inner()
    }
}

impl From<Exponent> for i32 {
    fn from(val: Exponent) -> Self {
        val.into_inner()
    }
}

impl From<&Exponent> for u32 {
    fn from(val: &Exponent) -> Self {
        val.inner() as u32
    }
}

impl From<Exponent> for u32 {
    fn from(val: Exponent) -> Self {
        val.into_inner() as u32
    }
}

impl From<u32> for Exponent {
    fn from(val: u32) -> Self {
        (val as i32).into()
    }
}

impl From<&u32> for Exponent {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for Exponent {
    fn from(val: i32) -> Self {
        Self::create(val)
    }
}

impl From<&i32> for Exponent {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<&Exponent> for XfsValue {
    fn from(val: &Exponent) -> Self {
        Self::new().with_i4(val.inner())
    }
}

impl From<Exponent> for XfsValue {
    fn from(val: Exponent) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for Exponent {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        Ok(val
            .i4()
            .ok_or(Error::Xfs(format!(
                "Expected Exponent XfsValue, have: {val}"
            )))?
            .into())
    }
}

impl From<&Exponent> for XfsMember {
    fn from(val: &Exponent) -> Self {
        Self::create(Exponent::xfs_name(), val.into())
    }
}

impl From<Exponent> for XfsMember {
    fn from(val: Exponent) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for Exponent {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().i4()) {
            (n, Some(v)) if n == Exponent::xfs_name() => Ok(v.into()),
            _ => Err(Error::Xfs(format!(
                "Expected Exponent XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for Exponent {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for Exponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}
