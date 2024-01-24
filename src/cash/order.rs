use crate::currency::{Currency, Denomination};

/// Represents a cash order operation in the CDR.
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

    /// Gets the [CashOrder] [Currency].
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Gets the [CashOrder] [Denomination].
    pub fn denomination(&self) -> Denomination {
        self.denomination
    }

    /// Gets the real value amount of the [CashOrder].
    pub fn amount(&self) -> u32 {
        self.denomination
            .amount()
            .saturating_mul(10f32.powf(self.currency.exponent() as f32) as u32)
    }
}
