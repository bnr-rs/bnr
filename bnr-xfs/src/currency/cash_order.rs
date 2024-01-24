use std::fmt;

use crate::Result;
use crate::{device_handle::CallbackArg, impl_xfs_struct};

use super::{Currency, Denomination};

/// Represents a cash order event initiated by the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashOrder {
    currency: Currency,
    denomination: Denomination,
}

impl CashOrder {
    /// Creates a new [CashOrder].
    pub const fn new() -> Self {
        Self {
            currency: Currency::new(),
            denomination: Denomination::new(),
        }
    }

    /// Creates a new [CashOrder] from the provided parameters.
    pub const fn create(currency: Currency, denomination: Denomination) -> Self {
        Self {
            currency,
            denomination,
        }
    }

    /// Gets a reference to the [Currency].
    pub const fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Gets a mutable reference to the [Currency].
    pub fn currency_mut(&mut self) -> &mut Currency {
        &mut self.currency
    }

    /// Sets the [Currency].
    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }

    /// Builder function that sets the [Currency].
    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.set_currency(currency);
        self
    }

    /// Gets a reference to the [Denomination].
    pub const fn denomination(&self) -> &Denomination {
        &self.denomination
    }

    /// Gets a mutable reference to the [Denomination].
    pub fn denomination_mut(&mut self) -> &mut Denomination {
        &mut self.denomination
    }

    /// Sets the [Denomination].
    pub fn set_denomination(&mut self, denomination: Denomination) {
        self.denomination = denomination;
    }

    /// Builder function that sets the [Denomination].
    pub fn with_denomination(mut self, denomination: Denomination) -> Self {
        self.set_denomination(denomination);
        self
    }
}

impl fmt::Display for CashOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""currency": {}, "#, self.currency)?;
        write!(f, r#""denomination": {}"#, self.denomination)?;
        write!(f, "}}")
    }
}

impl CallbackArg for CashOrder {
    fn value(&self) -> i32 {
        let cash_order = self as &CashOrder;
        cash_order
            .currency()
            .from_mdu_value(cash_order.denomination().amount()) as i32
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_cash_order(&self) -> bool {
        true
    }

    fn as_cash_order(&self) -> Result<&CashOrder> {
        Ok(self as &CashOrder)
    }

    fn as_cash_order_mut(&mut self) -> Result<&mut CashOrder> {
        Ok(self as &mut CashOrder)
    }
}

impl_xfs_struct!(CashOrder, "cashOrder", [currency: Currency, denomination: Denomination]);
