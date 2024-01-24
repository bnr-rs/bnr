pub const DENOM_ITEM_LEN: usize = 20;

/// This structure handles a list of [DenominationItem]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Denomination {
    /// Size of items array.
    size: u32,
    /// Amount in MDU
    amount: u32,
    /// Amount the BNR cannot denominate or dispense
    cashbox: u32,
    /// The [DenominationItem]s
    items: [DenominationItem; DENOM_ITEM_LEN],
}

impl Denomination {
    /// Creates a new [Denomination].
    pub const fn new() -> Self {
        Self {
            size: 0,
            amount: 0,
            cashbox: 0,
            items: [DenominationItem::new(), DENOM_ITEM_LEN],
        }
    }

    ///  Gets the size of the [Denomination].
    pub const fn size(&self) -> u32 {
        self.size
    }

    ///  Gets the amount of the [Denomination].
    pub const fn amount(&self) -> u32 {
        self.amount
    }

    ///  Gets the cashbox of the [Denomination].
    pub const fn cashbox(&self) -> u32 {
        self.cashbox
    }

    ///  Gets the list of [DenominationItem]s of the [Denomination].
    pub const fn items(&self) -> &[DenominationItem] {
        &self.items[..]
    }
}

impl From<&Denomination> for bnr_sys::XfsDenomination {
    fn from(val: &Denomination) -> Self {
        Self {
            size: val.size,
            amount: val.amount,
            cashbox: val.cashbox,
            items: val.items.map(|i| bnr_sys::XfsDenominationItem::from(i)),
        }
    }
}


/// This structure describes the number of bills stored to or dispensed from a Logical Cash Unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DenominationItem {
    /// Logical Cash Unit number
    unit: u32,
    /// Bill count
    count: u32,
}

impl From<&DenominationItem> for bnr_sys::XfsDenominationItem {
    fn from(val: &DenominationItem) -> Self {
        Self {
            unit: val.unit,
            count: val.count,
        }
    }
}

impl From<DenominationItem> for bnr_sys::XfsDenominationItem {
    fn from(val: DenominationItem) -> Self {
        (&val).into()
    }
}
