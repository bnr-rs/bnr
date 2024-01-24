use std::fmt;

use crate::xfs::{
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::{Error, Result};

/// Capabilities for Euro Article 6.
///
/// See [Capabilities].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EuroArt6Capability {
    pub category2: bool,
    pub category3: bool,
    pub unfit: bool,
}

impl EuroArt6Capability {
    /// Creates a new [EuroArt6Capability].
    pub const fn new() -> Self {
        Self {
            category2: false,
            category3: false,
            unfit: false,
        }
    }

    /// Gets the [XfsMember] name for [EuroArt6Capability].
    pub const fn xfs_name() -> &'static str {
        "eurArt6Capabilities"
    }
}

impl From<&EuroArt6Capability> for XfsStruct {
    fn from(val: &EuroArt6Capability) -> Self {
        Self::create([
            XfsMember::create(
                "category2",
                XfsValue::new().with_boolean(val.category2 as u8),
            ),
            XfsMember::create(
                "category3",
                XfsValue::new().with_boolean(val.category3 as u8),
            ),
            XfsMember::create("unfit", XfsValue::new().with_boolean(val.unfit as u8)),
        ])
    }
}

impl From<EuroArt6Capability> for XfsStruct {
    fn from(val: EuroArt6Capability) -> Self {
        (&val).into()
    }
}

impl From<&EuroArt6Capability> for XfsValue {
    fn from(val: &EuroArt6Capability) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<EuroArt6Capability> for XfsValue {
    fn from(val: EuroArt6Capability) -> Self {
        (&val).into()
    }
}

impl From<&EuroArt6Capability> for XfsMember {
    fn from(val: &EuroArt6Capability) -> Self {
        XfsMember::create(EuroArt6Capability::xfs_name(), val.into())
    }
}

impl From<EuroArt6Capability> for XfsMember {
    fn from(val: EuroArt6Capability) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for EuroArt6Capability {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        let name = val.name();
        let value = val.value().xfs_struct();

        match (name, value) {
            (n, Some(xfs_struct)) if n == Self::xfs_name() => {
                let members = xfs_struct.members();
                let cat2 = members
                    .iter()
                    .map(|m| m.inner())
                    .find(|m| m.name() == "category2" && m.value().boolean().is_some());
                let cat3 = members
                    .iter()
                    .map(|m| m.inner())
                    .find(|m| m.name() == "category3" && m.value().boolean().is_some());
                let unfit = members
                    .iter()
                    .map(|m| m.inner())
                    .find(|m| m.name() == "unfit" && m.value().boolean().is_some());
                match (cat2, cat3, unfit) {
                    (Some(c2), Some(c3), Some(un)) => {
                        Ok(Self {
                            category2: c2.value().boolean().unwrap_or(&0) == &1,
                            category3: c3.value().boolean().unwrap_or(&0) == &1,
                            unfit: un.value().boolean().unwrap_or(&0) == &1,
                        })
                    }
                    _ => Err(Error::Xfs(format!("Invalid EuroArt6Capability fields, category2: {cat2:?}, category3: {cat3:?}, unfit: {unfit:?}"))),
                }
            }
            _ => Err(Error::Xfs(format!(
                "Expected EuroArt6Capability XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for EuroArt6Capability {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for EuroArt6Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""category2":{},"#, self.category2)?;
        write!(f, r#""category3":{},"#, self.category3)?;
        write!(f, r#""unfit":{}"#, self.unfit)?;
        write!(f, "}}")
    }
}
