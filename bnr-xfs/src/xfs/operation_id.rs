use std::fmt;

use crate::impl_xfs_enum;

const OFFSET: u32 = 6106;
const DENOMINATE: u32 = 6107;
const DISPENSE: u32 = 6108;
const QUERY_CASH_UNIT: u32 = 6114;
const RESET: u32 = 6117;
const UPDATE_CASH_UNIT: u32 = 6118;
const GET_DATE_TIME: u32 = 6119;
const SET_DATE_TIME: u32 = 6120;
const QUERY_DENOMINATIONS: u32 = 6181;
const UPDATE_DENOMINATIONS: u32 = 6182;
const CASH_IN_START: u32 = 6121;
const CASH_IN: u32 = 6122;
const CASH_IN_END: u32 = 6123;
const CASH_IN_ROLLBACK: u32 = 6124;
const EMPTY: u32 = 6125;
const PRESENT: u32 = 6126;
const REJECT: u32 = 6127;
const RETRACT: u32 = 6128;
const CASHUNIT_CHANGED: u32 = 6153;
const CASHUNIT_THRESHOLD: u32 = 6155;

/// Represents an XFS operation ID.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum OperationId {
    #[default]
    Offset = OFFSET,
    Denominate = DENOMINATE,
    Dispense = DISPENSE,
    QueryCashUnit = QUERY_CASH_UNIT,
    Reset = RESET,
    UpdateCashUnit = UPDATE_CASH_UNIT,
    GetDateTime = GET_DATE_TIME,
    SetDateTime = SET_DATE_TIME,
    QueryDenominations = QUERY_DENOMINATIONS,
    UpdateDenominations = UPDATE_DENOMINATIONS,
    CashInStart = CASH_IN_START,
    CashIn = CASH_IN,
    CashInEnd = CASH_IN_END,
    CashInRollback = CASH_IN_ROLLBACK,
    Empty = EMPTY,
    Present = PRESENT,
    Reject = REJECT,
    Retract = RETRACT,
    CashUnitChanged = CASHUNIT_CHANGED,
    CashUnitThreshold = CASHUNIT_THRESHOLD,
}

impl OperationId {
    /// Creates a new [OperationId].
    pub const fn new() -> Self {
        Self::Offset
    }

    /// Creates a new [OperationId] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            OFFSET => Self::Offset,
            DENOMINATE => Self::Denominate,
            DISPENSE => Self::Dispense,
            QUERY_CASH_UNIT => Self::QueryCashUnit,
            RESET => Self::Reset,
            UPDATE_CASH_UNIT => Self::UpdateCashUnit,
            GET_DATE_TIME => Self::GetDateTime,
            SET_DATE_TIME => Self::SetDateTime,
            QUERY_DENOMINATIONS => Self::QueryDenominations,
            UPDATE_DENOMINATIONS => Self::UpdateDenominations,
            CASH_IN_START => Self::CashInStart,
            CASH_IN => Self::CashIn,
            CASH_IN_END => Self::CashInEnd,
            CASH_IN_ROLLBACK => Self::CashInRollback,
            EMPTY => Self::Empty,
            PRESENT => Self::Present,
            REJECT => Self::Reject,
            RETRACT => Self::Retract,
            CASHUNIT_CHANGED => Self::CashUnitChanged,
            CASHUNIT_THRESHOLD => Self::CashUnitThreshold,
            _ => Self::Offset,
        }
    }
}

impl fmt::Display for OperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_enum!(OperationId, "operationId");
