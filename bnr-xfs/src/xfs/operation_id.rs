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
}

impl OperationId {
    /// Creates a new [OperationId].
    pub const fn new() -> Self {
        Self::Offset
    }

    /// Creates a new [OperationId] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            v if v == OFFSET => Self::Offset,
            v if v == DENOMINATE => Self::Denominate,
            v if v == DISPENSE => Self::Dispense,
            v if v == QUERY_CASH_UNIT => Self::QueryCashUnit,
            v if v == RESET => Self::Reset,
            v if v == UPDATE_CASH_UNIT => Self::UpdateCashUnit,
            v if v == GET_DATE_TIME => Self::GetDateTime,
            v if v == SET_DATE_TIME => Self::SetDateTime,
            v if v == QUERY_DENOMINATIONS => Self::QueryDenominations,
            v if v == UPDATE_DENOMINATIONS => Self::UpdateDenominations,
            v if v == CASH_IN_START => Self::CashInStart,
            v if v == CASH_IN => Self::CashIn,
            v if v == CASH_IN_END => Self::CashInEnd,
            v if v == CASH_IN_ROLLBACK => Self::CashInRollback,
            v if v == EMPTY => Self::Empty,
            v if v == PRESENT => Self::Present,
            v if v == REJECT => Self::Reject,
            v if v == RETRACT => Self::Retract,
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
