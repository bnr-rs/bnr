//! XFS `struct` types.

use std::fmt;

use crate::{Error, Result};

use super::value::XfsValue;

/// Represents an XFS `struct` containing an [XfsMember].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "struct")]
pub struct XfsStruct {
    #[serde(rename = "$value", default)]
    members: Vec<ListMember>,
}

impl XfsStruct {
    /// Creates a new [XfsStruct].
    pub const fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    /// Creates a new [XfsStruct] with the provided [XfsMember].
    pub fn create<M: IntoIterator<Item = XfsMember>>(members: M) -> Self {
        Self {
            members: members.into_iter().map(ListMember::from).collect(),
        }
    }

    /// Gets a reference to the list of [XfsMember].
    pub fn members(&self) -> &[ListMember] {
        self.members.as_ref()
    }

    /// Gets a mutable reference to the list of [XfsMember].
    pub fn members_mut(&mut self) -> &mut [ListMember] {
        self.members.as_mut()
    }

    /// Sets the list of [XfsMember].
    pub fn set_members<M: IntoIterator<Item = XfsMember>>(&mut self, members: M) {
        self.members = members.into_iter().map(ListMember::from).collect();
    }

    /// Builder function that sets the [XfsMember].
    pub fn with_members<M: IntoIterator<Item = XfsMember>>(mut self, members: M) -> Self {
        self.set_members(members);
        self
    }

    /// Searches for an [XfsMember] with a matching `name`.
    pub fn find_member(&self, name: &str) -> Result<&XfsMember> {
        self.members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == name)
            .ok_or(Error::Xfs(format!(r#"Missing member "{name}""#)))
    }
}

impl fmt::Display for XfsStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"members": ["#)?;
        for (i, member) in self.members.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{member}")?;
        }
        write!(f, "]}}")
    }
}

/// Wrapper for an [XfsStruct] value used in a collection.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ListStruct {
    #[serde(rename = "struct")]
    Struct(XfsStruct),
}

impl ListStruct {
    /// Creates a new [ListStruct].
    pub const fn new() -> Self {
        Self::Struct(XfsStruct::new())
    }

    /// Creates a new [ListStruct].
    pub const fn create(xfs: XfsStruct) -> Self {
        Self::Struct(xfs)
    }

    /// Destructures the [ListStruct] enum into its inner representation.
    pub const fn inner(&self) -> &XfsStruct {
        match self {
            Self::Struct(s) => s,
        }
    }

    /// Destructures the [ListStruct] enum into its mutable inner representation.
    pub fn inner_mut(&mut self) -> &mut XfsStruct {
        match self {
            Self::Struct(s) => s,
        }
    }

    /// Converts the [ListStruct] into the inner [XfsStruct].
    pub fn into_inner(self) -> XfsStruct {
        match self {
            Self::Struct(s) => s,
        }
    }
}

impl fmt::Display for ListStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct(s) => write!(f, "{s}"),
        }
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
    pub fn create<S: Into<String>>(name: S, value: XfsValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
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
        write!(f, r#""name": "{}", "#, self.name)?;
        write!(f, r#""value": {}"#, self.value)?;
        write!(f, "}}")
    }
}

/// Wrapper for an [XfsMember] value used in a collection.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ListMember {
    #[serde(rename = "member")]
    Member(XfsMember),
}

impl ListMember {
    /// Creates a new [ListMember].
    pub const fn new() -> Self {
        Self::Member(XfsMember::new())
    }

    /// Creates a new [ListMember] from the provided [XfsMember].
    pub const fn create(m: XfsMember) -> Self {
        Self::Member(m)
    }

    /// Creates a new [ListMember] from the provided [XfsMember].
    pub const fn from_inner(m: XfsMember) -> Self {
        Self::Member(m)
    }

    /// Gets a reference to the inner [XfsMember].
    pub const fn inner(&self) -> &XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }

    /// Gets a mutable reference to the inner [XfsMember].
    pub fn inner_mut(&mut self) -> &mut XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }

    /// Converts the [ListMember] into an [XfsMmeber].
    pub fn into_inner(self) -> XfsMember {
        match self {
            Self::Member(m) => m,
        }
    }
}

impl From<&XfsMember> for ListMember {
    fn from(val: &XfsMember) -> Self {
        val.clone().into()
    }
}

impl From<XfsMember> for ListMember {
    fn from(val: XfsMember) -> Self {
        Self::from_inner(val)
    }
}

impl From<&ListMember> for XfsMember {
    fn from(val: &ListMember) -> Self {
        val.clone().into()
    }
}

impl From<ListMember> for XfsMember {
    fn from(val: ListMember) -> Self {
        val.into_inner()
    }
}

impl fmt::Display for ListMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Member(s) => write!(f, "{s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{xfs, Result};

    #[test]
    fn test_xfs_value_struct_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><struct><member><name>test</name><value><i4>16</i4></value></member></struct></value>"#;

        let exp_struct = XfsValue::new().with_xfs_struct(XfsStruct::create([XfsMember::create(
            "test",
            XfsValue::new().with_i4(16),
        )]));

        assert_eq!(xfs::to_string(&exp_struct)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsValue>(exp_xml)?, exp_struct);

        Ok(())
    }
}
