//! Types and functions for handling currency sets.

use std::fmt;

mod cash_type;
mod code;
mod denomination;
mod mix;

pub use cash_type::*;
pub use code::*;
pub use denomination::*;
pub use mix::*;

/// Represents a currency set used in the CDR.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Currency {
    currency_code: CurrencyCode,
    exponent: i32,
}

impl Currency {
    /// Creates a new [Currency].
    pub const fn new() -> Self {
        Self {
            currency_code: CurrencyCode::new(),
            exponent: 0,
        }
    }

    /// Gets the [CurrencyCode].
    pub fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }

    /// Gets the [Currency] exponent.
    ///
    /// Used to get the real value of a denomination by raising 10 to the exponent, and multiplying
    /// by the denomination amount.
    pub fn exponent(&self) -> i32 {
        self.exponent
    }
}

impl From<CurrencyCode> for Currency {
    fn from(val: CurrencyCode) -> Self {
        let exponent = match val {
            CurrencyCode::AUD
            | CurrencyCode::CAD
            | CurrencyCode::CHF
            | CurrencyCode::EUR
            | CurrencyCode::GBP
            | CurrencyCode::USD => -2,
            CurrencyCode::JPY | CurrencyCode::MXN => -1,
            CurrencyCode::AMD => 0,
            // FIXME: fill out with more actual values
            _ => 1,
        };

        Self {
            currency_code: val,
            exponent,
        }
    }
}

impl From<&Currency> for bnr_sys::XfsCurrency {
    fn from(val: &Currency) -> Self {
        Self {
            currencyCode: val.currency_code.into(),
            exponent: val.exponent,
        }
    }
}

impl From<Currency> for bnr_sys::XfsCurrency {
    fn from(val: Currency) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsCurrency> for Currency {
    fn from(val: &bnr_sys::XfsCurrency) -> Self {
        Self {
            currency_code: val.currencyCode.into(),
            exponent: val.exponent,
        }
    }
}

impl From<bnr_sys::XfsCurrency> for Currency {
    fn from(val: bnr_sys::XfsCurrency) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""currency": {{"currency_code": "{}", "exponent": {}}}"#,
            self.currency_code, self.exponent
        )
    }
}
