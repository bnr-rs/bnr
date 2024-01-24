use std::fmt;

use super::CurrencyCode;

pub const CASH_TYPE_LIST_LEN: usize = 14;

/// Represents a cash type ISO currency code, value, and variant.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashType {
    currency_code: CurrencyCode,
    value: u32,
    variant: u32,
}

impl CashType {
    /// Creates a new [CashType].
    pub const fn new() -> Self {
        Self {
            currency_code: CurrencyCode::new(),
            value: 0,
            variant: 0,
        }
    }

    /// Gets the [CurrencyCode].
    pub const fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }

    /// Gets the value.
    pub const fn value(&self) -> u32 {
        self.value
    }

    /// Gets the variant.
    pub const fn variant(&self) -> u32 {
        self.variant
    }
}

impl fmt::Display for CashType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""currency_code":{},"#, self.currency_code)?;
        write!(f, r#""value":{},"#, self.value)?;
        write!(f, r#""variant":{}"#, self.variant)?;
        write!(f, "}}")
    }
}

/// [CashType] list used for LCU's [secondary cash_type](LogicalCashUnit::secondary_cash_types).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashTypeList {
    size: u8,
    items: [CashType; CASH_TYPE_LIST_LEN],
}

impl CashTypeList {
    /// Creates a new [CashTypeList].
    pub const fn new() -> Self {
        Self {
            size: 0,
            items: [CashType::new(); CASH_TYPE_LIST_LEN],
        }
    }

    /// Gets the size.
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Gets whether the size is within bounds.
    pub const fn size_is_valid(&self) -> bool {
        self.size as usize <= CASH_TYPE_LIST_LEN
    }

    /// Gets a reference to the list of [CashType]s.
    pub fn items(&self) -> &[CashType] {
        if self.size_is_valid() {
            self.items[..self.size as usize].as_ref()
        } else {
            self.items.as_ref()
        }
    }
}

impl fmt::Display for CashTypeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""size":{},"#, self.size)?;

        write!(f, r#""items":["#)?;
        for (i, item) in self.items().iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]}}")
    }
}
