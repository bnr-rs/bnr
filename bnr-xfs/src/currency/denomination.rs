use std::{cmp, fmt};

use crate::{impl_xfs_array, impl_xfs_i4, impl_xfs_struct, Count, Size};

pub const DENOM_ITEM_LEN: usize = 20;

/// Represents a denomination amount.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Amount(u32);

impl Amount {
    /// Creates a new [Amount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Amount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [Amount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of the [Amount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Amount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl_xfs_i4!(Amount, "amount");

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

/// Represents a denomination cashbox amount.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cashbox(u32);

impl Cashbox {
    /// Creates a new [Cashbox].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Cashbox] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [Cashbox].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of the [Cashbox].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Cashbox].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl_xfs_i4!(Cashbox, "cashBox");

impl fmt::Display for Cashbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

/// Represents a denomination unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Unit(u32);

impl Unit {
    /// Creates a new [Unit].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Unit] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [Unit].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Gets the inner representation of the [Unit].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Unit].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl_xfs_i4!(Unit, "unit");

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

/// This structure handles a list of [DenominationItem]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Denomination {
    /// Size of items array.
    size: Size,
    /// Amount in MDU
    amount: Amount,
    /// Amount the BNR cannot denominate or dispense
    cashbox: Cashbox,
    /// The [DenominationItem]s
    items: DenominationItems,
}

impl Denomination {
    /// Creates a new [Denomination].
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            amount: Amount::new(),
            cashbox: Cashbox::new(),
            items: DenominationItems::new(),
        }
    }

    /// Gets the size of the [Denomination].
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size of the [Denomination].
    pub fn set_size(&mut self, size: u32) {
        self.size.set_inner(size);
        self.items.set_size(size);
    }

    /// Builder function that sets the size of the [Denomination].
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets the amount of the [Denomination].
    pub const fn amount(&self) -> u32 {
        self.amount.inner()
    }

    /// Sets the amount of the [Denomination].
    pub fn set_amount(&mut self, amount: u32) {
        self.amount.set_inner(amount);
    }

    /// Builder function that sets the amount of the [Denomination].
    pub fn with_amount(mut self, amount: u32) -> Self {
        self.set_amount(amount);
        self
    }

    /// Gets the cashbox of the [Denomination].
    pub const fn cashbox(&self) -> u32 {
        self.cashbox.inner()
    }

    /// Sets the cashbox of the [Denomination].
    pub fn set_cashbox(&mut self, cashbox: u32) {
        self.cashbox.set_inner(cashbox);
    }

    /// Builder function that sets the cashbox of the [Denomination].
    pub fn with_cashbox(mut self, cashbox: u32) -> Self {
        self.set_cashbox(cashbox);
        self
    }

    /// Gets the list of [DenominationItem]s of the [Denomination].
    pub fn items(&self) -> &[DenominationItem] {
        self.items.items()
    }

    /// Gets the mutable list of [DenominationItem]s of the [Denomination].
    pub fn items_mut(&mut self) -> &mut [DenominationItem] {
        self.items.items_mut()
    }

    /// Sets the list of [DenominationItem]s of the [Denomination].
    pub fn set_items(&mut self, items: &[DenominationItem]) {
        self.items.set_items(items);
        self.size.set_inner(self.items.size());
    }

    /// Builder function that sets the list of [DenominationItem]s of the [Denomination].
    pub fn with_items(mut self, items: &[DenominationItem]) -> Self {
        self.set_items(items);
        self
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"size": {}, "amount": {}, "cashbox": {}, "items": {}"#,
            self.size, self.amount, self.cashbox, self.items
        )
    }
}

impl_xfs_struct!(
    Denomination,
    "denomination",
    [
        amount: Amount,
        cashbox: Cashbox,
        items: DenominationItems
    ]
);

/// This structure describes the number of bills stored to or dispensed from a Logical Cash Unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DenominationItem {
    /// Logical Cash Unit number
    unit: Unit,
    /// Bill count
    count: Count,
}

impl DenominationItem {
    /// Creates a new [DenominationItem].
    pub const fn new() -> Self {
        Self {
            unit: Unit::new(),
            count: Count::new(),
        }
    }

    /// Gets the [LogicalCashUnit] number.
    pub const fn unit(&self) -> u32 {
        self.unit.inner()
    }

    /// Sets the [LogicalCashUnit] number.
    pub fn set_unit(&mut self, unit: u32) {
        self.unit.set_inner(unit);
    }

    /// Builder function that sets the [LogicalCashUnit] number.
    pub fn with_unit(mut self, unit: u32) -> Self {
        self.set_unit(unit);
        self
    }

    /// Gets the count of [DenominationItem] notes.
    pub const fn count(&self) -> u32 {
        self.count.inner()
    }

    /// Sets the count of [DenominationItem] notes.
    pub fn set_count(&mut self, count: u32) {
        self.count.set_inner(count);
    }

    /// Builder function that sets the count of [DenominationItem] notes.
    pub fn with_count(mut self, count: u32) -> Self {
        self.set_count(count);
        self
    }
}

impl fmt::Display for DenominationItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"unit":{}, "count":{}}}"#, self.unit, self.count)
    }
}

impl_xfs_struct!(DenominationItem, "denominationItem", [unit: Unit, count: Count]);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DenominationItems {
    size: Size,
    items: [DenominationItem; DENOM_ITEM_LEN],
}

impl DenominationItems {
    /// Creates a new [DenominationItems].
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [DenominationItem::new(); DENOM_ITEM_LEN],
        }
    }

    /// Creates a new [DenominationItems] from the provided parameter.
    pub const fn create(items: [DenominationItem; DENOM_ITEM_LEN]) -> Self {
        Self {
            size: Size::create(DENOM_ITEM_LEN as u32),
            items,
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        DENOM_ITEM_LEN
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: u32) {
        self.size.set_inner(size);
    }

    /// Gets a reference to the [DenominationItem] list.
    pub fn items(&self) -> &[DenominationItem] {
        let size = self.size.inner() as usize;
        if size <= DENOM_ITEM_LEN {
            self.items[..size].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable reference to the [DenominationItem] list.
    pub fn items_mut(&mut self) -> &mut [DenominationItem] {
        let size = self.size.inner() as usize;
        if size <= DENOM_ITEM_LEN {
            self.items[..size].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets the [DenominationItem] list.
    pub fn set_items(&mut self, items: &[DenominationItem]) {
        let len = cmp::min(items.len(), DENOM_ITEM_LEN);
        self.items[..len]
            .iter_mut()
            .zip(items[..len].iter())
            .for_each(|(dst, &src)| {
                *dst = src;
            });
        self.size.set_inner(len as u32);
    }
}

impl fmt::Display for DenominationItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""size":{}, "#, self.size)?;
        write!(f, r#""items": ["#)?;

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }

        write!(f, "]}}")
    }
}

impl_xfs_array!(DenominationItems, "items");
