//! Types and functions for handling currency sets.

use std::fmt;

pub use currency_iso4217::Currency as CurrencyCode;

mod cash_type;
mod denomination;
mod mix;

pub use cash_type::*;
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

    /// Converts a standard value to a MDU value.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use bnr_xfs::{Currency, CurrencyCode};
    /// let value = 10;
    /// let currency = Currency::from(CurrencyCode::USD);
    /// assert_eq!(currency.to_mdu_value(value), 1000);
    /// ```
    pub fn to_mdu_value(&self, value: u32) -> u32 {
        (value as f32 * 10f32.powf(self.exponent.abs() as f32)) as u32
    }

    /// Converts a MDU value to a standard value.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use bnr_xfs::{Currency, CurrencyCode};
    /// let mdu_value = 1000;
    /// let currency = Currency::from(CurrencyCode::USD);
    /// assert_eq!(currency.from_mdu_value(mdu_value), 10);
    /// ```
    pub fn from_mdu_value(&self, value: u32) -> u32 {
        (value as f32 * 10f32.powf(self.exponent as f32)) as u32
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

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""currency": {{"currency_code": "{}", "exponent": {}}}"#,
            self.currency_code, self.exponent
        )
    }
}
