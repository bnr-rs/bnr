use std::fmt;

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
}

impl From<&bnr_sys::XfsEuroArt6Capability> for EuroArt6Capability {
    fn from(val: &bnr_sys::XfsEuroArt6Capability) -> Self {
        Self {
            category2: val.category2 != 0,
            category3: val.category3 != 0,
            unfit: val.unfit != 0,
        }
    }
}

impl From<bnr_sys::XfsEuroArt6Capability> for EuroArt6Capability {
    fn from(val: bnr_sys::XfsEuroArt6Capability) -> Self {
        (&val).into()
    }
}

impl From<&EuroArt6Capability> for bnr_sys::XfsEuroArt6Capability {
    fn from(val: &EuroArt6Capability) -> Self {
        Self {
            category2: val.category2.into(),
            category3: val.category3.into(),
            unfit: val.unfit.into(),
        }
    }
}

impl From<EuroArt6Capability> for bnr_sys::XfsEuroArt6Capability {
    fn from(val: EuroArt6Capability) -> Self {
        (&val).into()
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
