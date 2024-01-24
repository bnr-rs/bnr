//! Types for handling dispense requests.

use std::fmt;

use crate::currency::{Currency, Denomination, MixNumber};
use crate::impl_xfs_struct;

/// Structure that defines the parameters of `bnr_Dispense()` or `bnr_Denominate()`, to specify
/// either an amount or a list of banknotes to dispense.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DispenseRequest {
    mix_number: MixNumber,
    denomination: Denomination,
    currency: Currency,
}

impl DispenseRequest {
    /// Creates a new [DispenseRequest].
    pub const fn new() -> Self {
        Self {
            mix_number: MixNumber::new(),
            denomination: Denomination::new(),
            currency: Currency::new(),
        }
    }

    /// Gets the [MixNumber] of the [DispenseRequest].
    pub const fn mix_number(&self) -> MixNumber {
        self.mix_number
    }

    /// Sets the [MixNumber] of the [DispenseRequest].
    pub fn set_mix_number(&mut self, mix: MixNumber) {
        self.mix_number = mix;
    }

    /// Builder function that sets the [MixNumber] of the [DispenseRequest].
    pub fn with_mix_number(mut self, mix: MixNumber) -> Self {
        self.set_mix_number(mix);
        self
    }

    /// Gets the [Denomination] of the [DispenseRequest].
    pub const fn denomination(&self) -> &Denomination {
        &self.denomination
    }

    /// Sets the [Denomination] of the [DispenseRequest].
    pub fn set_denomination(&mut self, denomination: Denomination) {
        self.denomination = denomination;
    }

    /// Builder function that sets the [Denomination] of the [DispenseRequest].
    pub fn with_denomination(mut self, denomination: Denomination) -> Self {
        self.set_denomination(denomination);
        self
    }

    /// Gets the [Currency] of the [DispenseRequest].
    pub const fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Sets the [Currency] of the [DispenseRequest].
    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }

    /// Builder function that sets the [Currency] of the [DispenseRequest].
    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.set_currency(currency);
        self
    }
}

impl fmt::Display for DispenseRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""mix_number": {}, "#, self.mix_number)?;
        write!(f, r#""denomination": {}, "#, self.denomination)?;
        write!(f, r#""currency": {}"#, self.currency)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    DispenseRequest,
    "dispenseRequest",
    [
        mix_number: MixNumber,
        denomination: Denomination,
        currency: Currency
    ]
);
