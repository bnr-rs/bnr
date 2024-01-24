use std::fmt;

use crate::xfs::xfs_struct::XfsMember;
use crate::{impl_xfs_struct, Error, Result};

mod counts;

pub use counts::*;

/// Represents extended counters for a cash unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ExtendedCounters {
    deposit: Option<DepositCounters>,
    dispense: Option<DispenseCounters>,
}

impl ExtendedCounters {
    /// Creates a new [ExtendedCounters].
    pub const fn new() -> Self {
        Self {
            deposit: None,
            dispense: None,
        }
    }
}

impl From<DepositCounters> for ExtendedCounters {
    fn from(val: DepositCounters) -> Self {
        Self {
            deposit: Some(val),
            dispense: None,
        }
    }
}

impl From<&DepositCounters> for ExtendedCounters {
    fn from(val: &DepositCounters) -> Self {
        (*val).into()
    }
}

impl From<DispenseCounters> for ExtendedCounters {
    fn from(val: DispenseCounters) -> Self {
        Self {
            deposit: None,
            dispense: Some(val),
        }
    }
}

impl From<&DispenseCounters> for ExtendedCounters {
    fn from(val: &DispenseCounters) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ExtendedCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        if let Some(d) = self.deposit.as_ref() {
            write!(f, r#""deposit":{d}"#)?;
        }

        if let Some(d) = self.dispense.as_ref() {
            if self.deposit.is_some() {
                write!(f, ",")?;
            }

            write!(f, r#""dispense":{d}"#)?;
        }

        write!(f, "}}")
    }
}

impl_xfs_struct!(
    ExtendedCounters,
    "extendedCounters",
    [deposit: DepositCounters, dispense: DispenseCounters]
);

/// Represents counters for deposits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DepositCounters {
    deposit_count: DepositCount,
    retracted_count: RetractedCount,
    emptied_count: EmptiedCount,
    forgery_count: ForgeryCount,
    disappeared_count: DisappearedCount,
}

impl DepositCounters {
    /// Creates a new [DepositCounters].
    pub const fn new() -> Self {
        Self {
            deposit_count: DepositCount::new(),
            retracted_count: RetractedCount::new(),
            emptied_count: EmptiedCount::new(),
            forgery_count: ForgeryCount::new(),
            disappeared_count: DisappearedCount::new(),
        }
    }
}

impl fmt::Display for DepositCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""deposit_count":{},"#, self.deposit_count)?;
        write!(f, r#""retracted_count":{},"#, self.retracted_count)?;
        write!(f, r#""emptied_count":{},"#, self.emptied_count)?;
        write!(f, r#""forgery_count":{},"#, self.forgery_count)?;
        write!(f, r#""disappeared_count":{}"#, self.disappeared_count)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    DepositCounters,
    "depositCounters",
    [
        deposit_count: DepositCount,
        retracted_count: RetractedCount,
        emptied_count: EmptiedCount,
        forgery_count: ForgeryCount,
        disappeared_count: DisappearedCount
    ]
);

impl From<Option<&DepositCounters>> for XfsMember {
    fn from(val: Option<&DepositCounters>) -> Self {
        match val {
            Some(v) => Self::create(DepositCounters::xfs_name(), v.into()),
            None => Self::new().with_name(DepositCounters::xfs_name()),
        }
    }
}

impl From<Option<DepositCounters>> for XfsMember {
    fn from(val: Option<DepositCounters>) -> Self {
        val.as_ref().into()
    }
}

impl TryFrom<&XfsMember> for Option<DepositCounters> {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().xfs_struct()) {
            (n, Some(v)) if n == DepositCounters::xfs_name() => Ok(Some(v.try_into()?)),
            (_n, None) => Ok(None),
            _ => Err(Error::Xfs(format!(
                "Expected DepositCounters XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for Option<DepositCounters> {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

/// Represents counters for dispensed notes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DispenseCounters {
    dispense_count: DispenseCount,
    reject_count: RejectCount,
}

impl DispenseCounters {
    /// Creates a new [DispenseCounters].
    pub const fn new() -> Self {
        Self {
            dispense_count: DispenseCount::new(),
            reject_count: RejectCount::new(),
        }
    }
}

impl fmt::Display for DispenseCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""dispense_count":{}"#, self.dispense_count)?;
        write!(f, r#""reject_count":{}"#, self.reject_count)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    DispenseCounters,
    "dispenseCounters",
    [dispense_count: DispenseCount, reject_count: RejectCount]
);

impl From<Option<&DispenseCounters>> for XfsMember {
    fn from(val: Option<&DispenseCounters>) -> Self {
        match val {
            Some(v) => Self::create(DispenseCounters::xfs_name(), v.into()),
            None => Self::new().with_name(DispenseCounters::xfs_name()),
        }
    }
}

impl From<Option<DispenseCounters>> for XfsMember {
    fn from(val: Option<DispenseCounters>) -> Self {
        val.as_ref().into()
    }
}

impl TryFrom<&XfsMember> for Option<DispenseCounters> {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().xfs_struct()) {
            (n, Some(v)) if n == DispenseCounters::xfs_name() => Ok(Some(v.try_into()?)),
            (_n, None) => Ok(None),
            _ => Err(Error::Xfs(format!(
                "Expected DispenseCounters XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for Option<DispenseCounters> {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}
