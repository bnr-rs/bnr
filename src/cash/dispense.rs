use crate::currency::{Currency, Denomination, MixNumber};

/// Structure that defines the parameters of `bnr_Dispense()` or `bnr_Denominate()`, to specify
/// either an amount or a list of banknotes to dispense.
pub struct DispenseRequest {
    mix_number: MixNumber,
    denomination: Denomination,
    currency: Currency,
}

impl DispenseRequest {
    /// Creates a new [DispenseRequest].
    pub const fn new() -> Self {
        Self {
            mix_number: MixNumber::MinBills,
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

impl From<&bnr_sys::XfsDispenseRequest> for DispenseRequest {
    fn from(val: &bnr_sys::XfsDispenseRequest) -> Self {
        Self {
            mix_number: val.mixNumber.into(),
            denomination: val.denomination.into(),
            currency: val.currency.into(),
        }
    }
}

impl From<bnr_sys::XfsDispenseRequest> for DispenseRequest {
    fn from(val: bnr_sys::XfsDispenseRequest) -> Self {
        (&val).into()
    }
}

impl From<&DispenseRequest> for bnr_sys::XfsDispenseRequest {
    fn from(val: &DispenseRequest) -> Self {
        Self {
            mixNumber: val.mix_number.into(),
            denomination: val.denomination.into(),
            currency: val.currency.into(),
        }
    }
}

impl From<DispenseRequest> for bnr_sys::XfsDispenseRequest {
    fn from(val: DispenseRequest) -> Self {
        (&val).into()
    }
}
