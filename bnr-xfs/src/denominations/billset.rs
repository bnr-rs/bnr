use std::fmt;

use crate::{impl_xfs_string, impl_xfs_struct, ComponentType, ModuleType, Version};

mod list;

pub use list::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BillsetId(String);

impl BillsetId {
    /// Creates a new [BillsetId].
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Gets a reference to the inner representation.
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }

    /// Converts the [BillsetId] into the inner representation.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for BillsetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BillsetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.inner())
    }
}

impl_xfs_string!(BillsetId, "billsetId");

/// Represents billset information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BillsetInfo {
    billset_id: BillsetId,
    module_type: ModuleType,
    component_type: ComponentType,
    version: Version,
}

impl BillsetInfo {
    /// Creates a new [BillsetInfo].
    pub const fn new() -> Self {
        Self {
            billset_id: BillsetId::new(),
            module_type: ModuleType::new(),
            component_type: ComponentType::new(),
            version: Version::new(),
        }
    }

    /// Gets a reference to the [BillsetId].
    pub const fn billset_id(&self) -> &BillsetId {
        &self.billset_id
    }

    /// Sets the [BillsetId].
    pub fn set_billset_id(&mut self, val: BillsetId) {
        self.billset_id = val;
    }

    /// Builder function that sets the [BillsetId].
    pub fn with_billset_id(mut self, val: BillsetId) -> Self {
        self.set_billset_id(val);
        self
    }

    /// Gets the [ModuleType].
    pub const fn module_type(&self) -> ModuleType {
        self.module_type
    }

    /// Sets the [ModuleType].
    pub fn set_module_type(&mut self, val: ModuleType) {
        self.module_type = val;
    }

    /// Builder function that sets the [ModuleType].
    pub fn with_module_type(mut self, val: ModuleType) -> Self {
        self.set_module_type(val);
        self
    }

    /// Gets the [ComponentType].
    pub const fn component_type(&self) -> ComponentType {
        self.component_type
    }

    /// Sets the [ComponentType].
    pub fn set_component_type(&mut self, val: ComponentType) {
        self.component_type = val;
    }

    /// Builder function that sets the [ComponentType].
    pub fn with_component_type(mut self, val: ComponentType) -> Self {
        self.set_component_type(val);
        self
    }

    /// Gets the [Version].
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Sets the [Version].
    pub fn set_version(&mut self, val: Version) {
        self.version = val;
    }

    /// Builder function that sets the [Version].
    pub fn with_version(mut self, val: Version) -> Self {
        self.set_version(val);
        self
    }
}

impl fmt::Display for BillsetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""billset_id":{},"#, self.billset_id)?;
        write!(f, r#""module_type":{},"#, self.module_type)?;
        write!(f, r#""component_type":{},"#, self.component_type)?;
        write!(f, r#""version":{}"#, self.version)?;
        write!(f, "}}")
    }
}

impl Default for BillsetInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl_xfs_struct!(
    BillsetInfo,
    "billsetInfo",
    [
        billset_id: BillsetId,
        module_type: ModuleType,
        component_type: ComponentType,
        version: Version
    ]
);
