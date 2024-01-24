use std::fmt;

use currency_iso4217::Currency as IsoCurrency;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

use super::Exponent;

/// Represents an ISO 4217 currency code.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrencyCode(IsoCurrency);

impl CurrencyCode {
    /// Creates a new [CurrencyCode].
    pub const fn new() -> Self {
        Self(IsoCurrency::new())
    }

    /// Creates a new [CurrencyCode] from the provided parameter.
    pub const fn create(val: IsoCurrency) -> Self {
        Self(val)
    }

    /// Gets the [XfsMember] name.
    pub const fn xfs_name() -> &'static str {
        "currencyCode"
    }
}

impl From<&CurrencyCode> for Exponent {
    fn from(val: &CurrencyCode) -> Self {
        Self::create(match val.0 {
            IsoCurrency::AUD
            | IsoCurrency::CAD
            | IsoCurrency::CHF
            | IsoCurrency::EUR
            | IsoCurrency::GBP
            | IsoCurrency::USD => -2,
            IsoCurrency::JPY | IsoCurrency::MXN => -1,
            IsoCurrency::AMD => 0,
            // FIXME: fill out with more actual values
            _ => 1,
        })
    }
}

impl From<CurrencyCode> for Exponent {
    fn from(val: CurrencyCode) -> Self {
        (&val).into()
    }
}

impl From<&str> for CurrencyCode {
    fn from(val: &str) -> Self {
        Self(val.into())
    }
}

impl From<&CurrencyCode> for &'static str {
    fn from(val: &CurrencyCode) -> Self {
        val.0.into()
    }
}

impl From<CurrencyCode> for &'static str {
    fn from(val: CurrencyCode) -> Self {
        (&val).into()
    }
}

impl From<&CurrencyCode> for XfsValue {
    fn from(val: &CurrencyCode) -> Self {
        Self::new().with_string(<&str>::from(val))
    }
}

impl From<CurrencyCode> for XfsValue {
    fn from(val: CurrencyCode) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for CurrencyCode {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        Ok(val
            .string()
            .ok_or(Error::Xfs(format!(
                "Expected CurrencyCode XfsValue, have: {val}"
            )))?
            .into())
    }
}

impl TryFrom<XfsValue> for CurrencyCode {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&CurrencyCode> for XfsMember {
    fn from(val: &CurrencyCode) -> Self {
        Self::create(CurrencyCode::xfs_name(), val.into())
    }
}

impl From<CurrencyCode> for XfsMember {
    fn from(val: CurrencyCode) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for CurrencyCode {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        match (val.name(), val.value().string()) {
            (n, Some(v)) if n == CurrencyCode::xfs_name() => Ok(v.into()),
            _ => Err(Error::Xfs(format!(
                "Expected CurrencyCode XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for CurrencyCode {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.0)
    }
}
