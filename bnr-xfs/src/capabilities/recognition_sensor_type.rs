use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Recognition sensor type identification. Always ‘B’ in the BNR.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RecognitionSensorType(u8);

impl RecognitionSensorType {
    /// Creates a new [RecognitionSensorType].
    pub const fn new() -> Self {
        Self(b'B')
    }

    /// Creates a new [RecognitionSensorType] from the provided parameter.
    pub const fn create(val: u8) -> Self {
        Self(val)
    }

    /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
    pub const fn xfs_name() -> &'static str {
        "recognitionSensorType"
    }

    /// Gets the inner representation of the [RecognitionSensorType].
    pub const fn inner(&self) -> u8 {
        self.0
    }

    /// Sets the inner representation of the [RecognitionSensorType].
    pub fn set_inner(&mut self, val: u8) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [RecognitionSensorType].
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl From<&RecognitionSensorType> for u8 {
    fn from(val: &RecognitionSensorType) -> Self {
        val.inner()
    }
}

impl From<RecognitionSensorType> for u8 {
    fn from(val: RecognitionSensorType) -> Self {
        val.into_inner()
    }
}

impl From<u8> for RecognitionSensorType {
    fn from(val: u8) -> Self {
        Self::create(val)
    }
}

impl From<&u8> for RecognitionSensorType {
    fn from(val: &u8) -> Self {
        (*val).into()
    }
}

impl From<&RecognitionSensorType> for XfsValue {
    fn from(val: &RecognitionSensorType) -> Self {
        Self::new().with_string(std::str::from_utf8(&[val.inner()]).unwrap_or(""))
    }
}

impl From<RecognitionSensorType> for XfsValue {
    fn from(val: RecognitionSensorType) -> Self {
        (&val).into()
    }
}

impl TryFrom<&str> for RecognitionSensorType {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        Ok(val
            .as_bytes()
            .first()
            .ok_or(Error::Xfs("Empty string".into()))?
            .into())
    }
}

impl TryFrom<&XfsValue> for RecognitionSensorType {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.string()
            .ok_or(Error::Xfs(format!(
                "Expected RecognitionSensorType XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl From<&RecognitionSensorType> for XfsMember {
    fn from(val: &RecognitionSensorType) -> Self {
        Self::create(RecognitionSensorType::xfs_name(), val.into())
    }
}

impl From<RecognitionSensorType> for XfsMember {
    fn from(val: RecognitionSensorType) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for RecognitionSensorType {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().string()) {
            (n, Some(v)) if n == Self::xfs_name() => v.try_into(),
            _ => Err(Error::Xfs(format!(
                "Expected RecognitionSensorType XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for RecognitionSensorType {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for RecognitionSensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.inner() as char)
    }
}
