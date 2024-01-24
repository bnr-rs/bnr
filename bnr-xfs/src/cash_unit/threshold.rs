use std::fmt;

use crate::xfs::{
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::{impl_xfs_i4, Error, Result};

mod mode;
mod status;

pub use mode::*;
pub use status::*;

/// The PCU [ThresholdStatus] becomes Full when the bill count is greater or equal to this value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ThresholdFull(u32);

impl ThresholdFull {
    /// Creates a new [ThresholdFull].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [ThresholdFull] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of [ThresholdFull].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of [ThresholdFull].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }
}

impl fmt::Display for ThresholdFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(ThresholdFull, "full");

/// The PCU [ThresholdStatus] becomes High when the bill count is greater than this value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ThresholdHigh(u32);

impl ThresholdHigh {
    /// Creates a new [ThresholdHigh].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [ThresholdHigh] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of [ThresholdHigh].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of [ThresholdHigh].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }
}

impl fmt::Display for ThresholdHigh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(ThresholdHigh, "high");

/// The PCU [ThresholdStatus] becomes Low when the bill count is lower to this value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ThresholdLow(u32);

impl ThresholdLow {
    /// Creates a new [ThresholdLow].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [ThresholdLow] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of [ThresholdLow].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of [ThresholdLow].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }
}

impl fmt::Display for ThresholdLow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(ThresholdLow, "low");

/// The PCU [ThresholdStatus] becomes Empty when the bill count is lower or equal to this value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ThresholdEmpty(u32);

impl ThresholdEmpty {
    /// Creates a new [ThresholdEmpty].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [ThresholdEmpty] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of [ThresholdEmpty].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of [ThresholdEmpty].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }
}

impl fmt::Display for ThresholdEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(ThresholdEmpty, "empty");

/// Structure that defines the levels determining a physical cash unit [ThresholdStatus].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Threshold {
    full: ThresholdFull,
    high: ThresholdHigh,
    low: ThresholdLow,
    empty: ThresholdEmpty,
}

impl Threshold {
    /// Creates a new [Threshold].
    pub const fn new() -> Self {
        Self {
            full: ThresholdFull::new(),
            high: ThresholdHigh::new(),
            low: ThresholdLow::new(),
            empty: ThresholdEmpty::new(),
        }
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "threshold"
    }

    /// Gets the full threshold limit.
    pub const fn full(&self) -> u32 {
        self.full.inner()
    }

    /// Sets the full threshold limit.
    pub fn set_full(&mut self, val: u32) {
        self.full.set_inner(val);
    }

    /// Builder function that sets the full threshold limit.
    pub fn with_full(mut self, val: u32) -> Self {
        self.set_full(val);
        self
    }

    /// Gets the high threshold limit.
    pub const fn high(&self) -> u32 {
        self.high.inner()
    }

    /// Sets the high threshold limit.
    pub fn set_high(&mut self, val: u32) {
        self.high.set_inner(val);
    }

    /// Builder function that sets the high threshold limit.
    pub fn with_high(mut self, val: u32) -> Self {
        self.set_high(val);
        self
    }

    /// Gets the low threshold limit.
    pub const fn low(&self) -> u32 {
        self.low.inner()
    }

    /// Sets the low threshold limit.
    pub fn set_low(&mut self, val: u32) {
        self.low.set_inner(val);
    }

    /// Builder function that sets the low threshold limit.
    pub fn with_low(mut self, val: u32) -> Self {
        self.set_low(val);
        self
    }

    /// Gets the empty threshold limit.
    pub const fn empty(&self) -> u32 {
        self.empty.inner()
    }

    /// Sets the empty threshold limit.
    pub fn set_empty(&mut self, val: u32) {
        self.empty.set_inner(val);
    }

    /// Builder function that sets the empty threshold limit.
    pub fn with_empty(mut self, val: u32) -> Self {
        self.set_empty(val);
        self
    }
}

impl From<&Threshold> for XfsStruct {
    fn from(val: &Threshold) -> Self {
        Self::create([
            val.full.into(),
            val.high.into(),
            val.low.into(),
            val.empty.into(),
        ])
    }
}

impl From<Threshold> for XfsStruct {
    fn from(val: Threshold) -> Self {
        (&val).into()
    }
}

impl From<&Threshold> for XfsValue {
    fn from(val: &Threshold) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<Threshold> for XfsValue {
    fn from(val: Threshold) -> Self {
        (&val).into()
    }
}

impl From<&Threshold> for XfsMember {
    fn from(val: &Threshold) -> Self {
        Self::create(Threshold::xfs_name(), val.into())
    }
}

impl From<Threshold> for XfsMember {
    fn from(val: Threshold) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for Threshold {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.xfs_struct()
            .ok_or(Error::Xfs(format!(
                "Expected Threshold XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for Threshold {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsMember> for Threshold {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().xfs_struct()) {
            (n, Some(s)) if n == Self::xfs_name() => s.try_into(),
            _ => Err(Error::Xfs(format!(
                "Expected Threshold XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for Threshold {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsStruct> for Threshold {
    type Error = Error;

    fn try_from(val: &XfsStruct) -> Result<Self> {
        let full: ThresholdFull = val.find_member(ThresholdFull::xfs_name())?.try_into()?;
        let high: ThresholdHigh = val.find_member(ThresholdHigh::xfs_name())?.try_into()?;
        let low: ThresholdLow = val.find_member(ThresholdLow::xfs_name())?.try_into()?;
        let empty: ThresholdEmpty = val.find_member(ThresholdEmpty::xfs_name())?.try_into()?;

        Ok(Self {
            full,
            high,
            low,
            empty,
        })
    }
}

impl TryFrom<XfsStruct> for Threshold {
    type Error = Error;

    fn try_from(val: XfsStruct) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for Threshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""full":{},"#, self.full)?;
        write!(f, r#""high":{},"#, self.high)?;
        write!(f, r#""low":{},"#, self.low)?;
        write!(f, r#""empty":{}"#, self.empty)?;
        write!(f, "}}")
    }
}
