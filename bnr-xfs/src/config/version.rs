use std::fmt;

use crate::{create_xfs_i4, impl_xfs_struct};

create_xfs_i4!(
    Major,
    "major",
    "Represents the major component of the [Version]"
);
create_xfs_i4!(
    Minor,
    "minor",
    "Represents the minor component of the [Version]"
);

/// Simple version structure, used in version requirements.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version {
    major: Major,
    minor: Minor,
}

impl Version {
    /// Creates a new [Version].
    pub const fn new() -> Self {
        Self {
            major: Major::new(),
            minor: Minor::new(),
        }
    }

    /// Gets the [Major] component.
    pub const fn major(&self) -> Major {
        self.major
    }

    /// Sets the [Major] component.
    pub fn set_major(&mut self, val: Major) {
        self.major = val;
    }

    /// Builder function that sets the [Major] component.
    pub fn with_major(mut self, val: Major) -> Self {
        self.set_major(val);
        self
    }

    /// Gets the [Minor] component.
    pub const fn minor(&self) -> Minor {
        self.minor
    }

    /// Sets the [Minor] component.
    pub fn set_minor(&mut self, val: Minor) {
        self.minor = val;
    }

    /// Builder function that sets the [Minor] component.
    pub fn with_minor(mut self, val: Minor) -> Self {
        self.set_minor(val);
        self
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""major":{},"#, self.major)?;
        write!(f, r#""minor":{}"#, self.minor)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(Version, "version", [major: Major, minor: Minor]);

create_xfs_i4!(
    ModuleType,
    "moduleType",
    "Represents the module type of the [VersionRequirement]"
);
create_xfs_i4!(
    ComponentType,
    "componentType",
    "Represents the component type of the [VersionRequirement]"
);

/// Represents a version requirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VersionRequirement {
    module_type: ModuleType,
    component_type: ComponentType,
    version: Version,
}

impl VersionRequirement {
    /// Creates a new [VersionRequirement].
    pub const fn new() -> Self {
        Self {
            module_type: ModuleType::new(),
            component_type: ComponentType::new(),
            version: Version::new(),
        }
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

impl Default for VersionRequirement {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VersionRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""module_type":{},"#, self.module_type)?;
        write!(f, r#""component_type":{},"#, self.component_type)?;
        write!(f, r#""version":{}"#, self.version)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    VersionRequirement,
    "versionRequirement",
    [
        module_type: ModuleType,
        component_type: ComponentType,
        version: Version
    ]
);
