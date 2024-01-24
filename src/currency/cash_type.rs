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

impl From<bnr_sys::XfsCashType> for CashType {
    fn from(val: bnr_sys::XfsCashType) -> Self {
        Self {
            currency_code: val.currencyCode.into(),
            value: val.value,
            variant: val.variant,
        }
    }
}

impl From<CashType> for bnr_sys::XfsCashType {
    fn from(val: CashType) -> Self {
        Self {
            currencyCode: val.currency_code.into(),
            value: val.value,
            variant: val.variant,
        }
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
}

impl From<&CashTypeList> for bnr_sys::XfsCashTypeList {
    fn from(val: &CashTypeList) -> Self {
        Self {
            size: val.size,
            items: val.items.map(bnr_sys::XfsCashType::from),
        }
    }
}

impl From<CashTypeList> for bnr_sys::XfsCashTypeList {
    fn from(val: CashTypeList) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsCashTypeList> for CashTypeList {
    fn from(val: &bnr_sys::XfsCashTypeList) -> Self {
        Self {
            size: val.size,
            items: val.items.map(CashType::from),
        }
    }
}

impl From<bnr_sys::XfsCashTypeList> for CashTypeList {
    fn from(val: bnr_sys::XfsCashTypeList) -> Self {
        (&val).into()
    }
}
