use std::fmt;

use crate::{create_xfs_i4, impl_xfs_array, impl_xfs_struct, Size};

use super::CurrencyCode;

pub const CASH_TYPE_LIST_LEN: usize = 14;

create_xfs_i4!(Value, "value", "Represents the value of a [CashType].");

create_xfs_i4!(
    Variant,
    "variant",
    "Represents the variant of a [CashType]."
);

/// Represents a cash type ISO currency code, value, and variant.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashType {
    currency_code: CurrencyCode,
    value: Value,
    variant: Variant,
}

impl CashType {
    /// Creates a new [CashType].
    pub const fn new() -> Self {
        Self {
            currency_code: CurrencyCode::new(),
            value: Value::new(),
            variant: Variant::new(),
        }
    }

    /// Gets the [CurrencyCode].
    pub const fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }

    /// Gets the value.
    pub const fn value(&self) -> u32 {
        self.value.inner()
    }

    /// Gets the variant.
    pub const fn variant(&self) -> u32 {
        self.variant.inner()
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

impl_xfs_struct!(CashType, "cashType", [currency_code: CurrencyCode, value: Value, variant: Variant]);

/// [CashType] list used for LCU's [secondary cash_type](LogicalCashUnit::secondary_cash_types).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashTypeList {
    size: Size,
    items: [CashType; CASH_TYPE_LIST_LEN],
}

impl CashTypeList {
    /// Creates a new [CashTypeList].
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [CashType::new(); CASH_TYPE_LIST_LEN],
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        CASH_TYPE_LIST_LEN
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size.
    pub fn set_size(&mut self, val: u32) {
        self.size.set_inner(val);
    }

    /// Gets whether the size is within bounds.
    pub const fn size_is_valid(&self) -> bool {
        self.size.inner() as usize <= CASH_TYPE_LIST_LEN
    }

    /// Gets a reference to the list of [CashType]s.
    pub fn items(&self) -> &[CashType] {
        let size = self.size.inner() as usize;
        if size <= CASH_TYPE_LIST_LEN {
            self.items[..size].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable reference to the list of [CashType]s.
    pub fn items_mut(&mut self) -> &mut [CashType] {
        let size = self.size.inner() as usize;
        if size <= CASH_TYPE_LIST_LEN {
            self.items[..size].as_mut()
        } else {
            self.items.as_mut()
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

impl_xfs_array!(CashTypeList, "secondaryCashTypes");
