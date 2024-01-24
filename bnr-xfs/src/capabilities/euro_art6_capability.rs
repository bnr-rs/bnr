use std::fmt;

use crate::{create_xfs_bool, impl_xfs_struct};

create_xfs_bool!(Category2, "category2", "Euro Article 6, Category 2 support");
create_xfs_bool!(Category3, "category3", "Euro Article 6, Category 3 support");
create_xfs_bool!(Unfit, "unfit", "Euro Article 6, Unfit classification");

/// Capabilities for Euro Article 6.
///
/// See [Capabilities].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EuroArt6Capability {
    pub category2: Category2,
    pub category3: Category3,
    pub unfit: Unfit,
}

impl EuroArt6Capability {
    /// Creates a new [EuroArt6Capability].
    pub const fn new() -> Self {
        Self {
            category2: Category2::new(),
            category3: Category3::new(),
            unfit: Unfit::new(),
        }
    }
}

impl_xfs_struct!(
    EuroArt6Capability,
    "eurArt6Capabilities",
    [
        category2: Category2,
        category3: Category3,
        unfit: Unfit
    ]
);

impl fmt::Display for EuroArt6Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""category2": {}, "#, self.category2)?;
        write!(f, r#""category3": {}, "#, self.category3)?;
        write!(f, r#""unfit": {}"#, self.unfit)?;
        write!(f, "}}")
    }
}
