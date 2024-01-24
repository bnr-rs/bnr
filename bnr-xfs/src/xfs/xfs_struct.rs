//! XFS `struct` types.

use super::value::XfsValue;
use std::fmt;

/// Represents an XFS `struct` containing an [XfsMember].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "struct")]
pub struct XfsStruct {
    #[serde(rename = "$value")]
    members: Vec<XfsMembers>,
}

impl XfsStruct {
    /// Creates a new [XfsStruct].
    pub const fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    /// Creates a new [XfsStruct] with the provided [XfsMember].
    pub const fn create(members: Vec<XfsMembers>) -> Self {
        Self { members }
    }

    /// Gets a reference to the list of [XfsMember].
    pub fn members(&self) -> &[XfsMembers] {
        self.members.as_ref()
    }

    /// Gets a mutable reference to the list of [XfsMember].
    pub fn members_mut(&mut self) -> &mut [XfsMembers] {
        self.members.as_mut()
    }

    /// Sets the list of [XfsMember].
    pub fn set_members<M: Into<Vec<XfsMembers>>>(&mut self, members: M) {
        self.members = members.into();
    }

    /// Builder function that sets the [XfsMember].
    pub fn with_members<M: Into<Vec<XfsMembers>>>(mut self, members: M) -> Self {
        self.set_members(members);
        self
    }
}

impl fmt::Display for XfsStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, member) in self.members.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{member}")?;
        }
        write!(f, "}}")
    }
}

/// Represents an XFS `struct` containing an [XfsMember].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "member")]
pub struct XfsMember {
    name: String,
    value: XfsValue,
}

impl XfsMember {
    /// Creates a new [XfsMember].
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            value: XfsValue::new(),
        }
    }

    /// Creates a new [XfsMember] with the provided parameters.
    pub const fn create(name: String, value: XfsValue) -> Self {
        Self { name, value }
    }

    /// Gets a reference to the [XfsMember] name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Sets the [XfsMember] name.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Builder function that sets the [XfsMember] name.
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a reference to the [XfsValue].
    pub const fn value(&self) -> &XfsValue {
        &self.value
    }

    /// Sets the [XfsValue].
    pub fn set_value(&mut self, value: XfsValue) {
        self.value = value;
    }

    /// Builder function that sets the [XfsValue].
    pub fn with_value(mut self, value: XfsValue) -> Self {
        self.set_value(value);
        self
    }
}

impl fmt::Display for XfsMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""name": {}, "#, self.name)?;
        write!(f, r#""value": {}"#, self.value)?;
        write!(f, "}}")
    }
}

/// Wrapper for an [XfsMember] value used in a collection.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum XfsMembers {
    #[serde(rename = "member")]
    Member(XfsMember),
}

impl XfsMembers {
    pub const fn from_inner(m: XfsMember) -> Self {
        Self::Member(m)
    }

    pub fn as_inner(&self) -> &XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }

    pub fn as_inner_mut(&mut self) -> &mut XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }

    pub fn into_inner(self) -> XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }
}

impl From<&XfsMember> for XfsMembers {
    fn from(val: &XfsMember) -> Self {
        val.clone().into()
    }
}

impl From<XfsMember> for XfsMembers {
    fn from(val: XfsMember) -> Self {
        Self::from_inner(val)
    }
}

impl From<&XfsMembers> for XfsMember {
    fn from(val: &XfsMembers) -> Self {
        val.clone().into()
    }
}

impl From<XfsMembers> for XfsMember {
    fn from(val: XfsMembers) -> Self {
        val.into_inner()
    }
}

impl fmt::Display for XfsMembers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Member(s) => write!(f, "{s}"),
        }
    }
}

/// Wrapper for an [XfsStruct] value used in a collection.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum XfsStructs {
    #[serde(rename = "struct")]
    Struct(XfsStruct),
}

impl XfsStructs {
    /// Creates a new [XfsStructs].
    pub const fn new() -> Self {
        Self::Struct(XfsStruct::new())
    }

    /// Creates a new [XfsStructs].
    pub const fn create(xfs: XfsStruct) -> Self {
        Self::Struct(xfs)
    }

    /// Deconstructs the [XfsStructs] enum into its inner representation.
    pub const fn as_inner(&self) -> &XfsStruct {
        match self {
            Self::Struct(s) => s,
        }
    }

    /// Deconstructs the [XfsStructs] enum into its mutable inner representation.
    pub fn as_inner_mut(&mut self) -> &mut XfsStruct {
        match self {
            Self::Struct(s) => s,
        }
    }
}

impl fmt::Display for XfsStructs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct(s) => write!(f, "{s}"),
        }
    }
}

/// Represents an XFS `value` containing an [XfsStruct].
///
/// Separated from the [XfsValue] enum to avoid infinite size inference by the compiler.
///
/// **NOTE** indirect wrapper [XfsStructs] is an implementation detail needed for correct serialization.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum XfsValueStruct {
    #[serde(rename = "value")]
    Value(XfsStructs),
}

impl XfsValueStruct {
    /// Creates a new [XfsValueStruct].
    pub const fn new() -> Self {
        Self::Value(XfsStructs::new())
    }

    /// Creates a new [XfsValueStruct] with the provided [XfsStruct].
    pub const fn create(item: XfsStruct) -> Self {
        Self::Value(XfsStructs::create(item))
    }

    /// Deconstructs the [XfsValueStruct] enum into its inner representation.
    pub const fn as_inner(&self) -> &XfsStruct {
        match self {
            Self::Value(s) => s.as_inner(),
        }
    }

    /// Deconstructs the [XfsValueStruct] enum into its mutable inner representation.
    pub fn as_inner_mut(&mut self) -> &mut XfsStruct {
        match self {
            Self::Value(s) => s.as_inner_mut(),
        }
    }
}

impl fmt::Display for XfsValueStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs as xml;

    use super::*;
    use crate::Result;

    #[test]
    fn test_xfs_value_struct_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="utf-8"?><value><struct><member><name>test</name><value><i4>16</i4></value></member></struct></value>"#;

        let exp_struct = XfsValueStruct::create(XfsStruct::create(
            [XfsMember::create("test".into(), XfsValue::Int4(16)).into()].into(),
        ));

        /*
        let xml_str = xml::to_string(&exp_struct)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        */

        assert_eq!(xml::from_str::<XfsValueStruct>(exp_xml)?, exp_struct);

        Ok(())
    }
}
