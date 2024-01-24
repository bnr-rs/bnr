//! XFS `method call` types.

use std::fmt;

use super::params::{XfsParam, XfsParams};
use crate::{Error, Result};

/// Represents an XFS method call containing a list of [params](XfsParams).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "methodCall")]
pub struct XfsMethodCall {
    #[serde(rename = "methodName")]
    name: String,
    params: XfsParams,
}

impl XfsMethodCall {
    /// Creates a new [XfsMethodCall].
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            params: XfsParams::new(),
        }
    }

    /// Creates a new [XfsMethodCall] with the provided parameters.
    pub fn create<P: Into<Vec<XfsParam>>>(name: XfsMethodName, params: P) -> Self {
        Self {
            name: <&str>::from(name).into(),
            params: XfsParams::create(params.into()),
        }
    }

    /// Gets the [name](XfsMethodName).
    ///
    /// Returns:
    ///
    /// - `Ok(XfsMethodName)` if [XfsMethodCall] has a valid [XfsMethodName] set.
    /// - `Err(_)` otherwise
    pub fn name(&self) -> Result<XfsMethodName> {
        self.name.as_str().try_into()
    }

    /// Gets the [name](XfsMethodName) as a string.
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    /// Sets the [name](XfsMethodName).
    pub fn set_name(&mut self, name: XfsMethodName) {
        self.name = <&str>::from(name).into();
    }

    /// Builder function that sets the [name](XfsMethodName).
    pub fn with_name(mut self, name: XfsMethodName) -> Self {
        self.set_name(name);
        self
    }

    /// Gets the [params](XfsParams).
    pub const fn params(&self) -> &XfsParams {
        &self.params
    }

    /// Sets the [params](XfsParams).
    pub fn set_params(&mut self, params: XfsParams) {
        self.params = params;
    }

    /// Builder function that sets the [params](XfsParams).
    pub fn with_params(mut self, params: XfsParams) -> Self {
        self.set_params(params);
        self
    }
}

impl From<&XfsMethodName> for XfsMethodCall {
    fn from(val: &XfsMethodName) -> Self {
        Self {
            name: <&str>::from(val).into(),
            params: XfsParams::new(),
        }
    }
}

impl From<XfsMethodName> for XfsMethodCall {
    fn from(val: XfsMethodName) -> Self {
        (&val).into()
    }
}

/// Represents the [XfsMethodCall] name used in a procedure call to a BNR device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum XfsMethodName {
    #[serde(rename = "bnr.getdatetime")]
    GetDateTime,
    #[serde(rename = "bnr.setdatetime")]
    SetDateTime,
    #[serde(rename = "bnr.cashinstart")]
    CashInStart,
    #[serde(rename = "bnr.cashin")]
    CashIn,
    #[serde(rename = "bnr.cashinrollback")]
    CashInRollback,
    #[serde(rename = "bnr.cashinend")]
    CashInEnd,
    #[serde(rename = "module.getidentification")]
    #[default]
    GetIdentification,
    #[serde(rename = "bnr.getstatus")]
    GetStatus,
    #[serde(rename = "bnr.stopsession")]
    StopSession,
    #[serde(rename = "bnr.reset")]
    Reset,
    #[serde(rename = "bnr.reboot")]
    Reboot,
    #[serde(rename = "bnr.cancel")]
    Cancel,
    #[serde(rename = "bnr.park")]
    Park,
    #[serde(rename = "bnr.empty")]
    Empty,
    #[serde(rename = "bnr.eject")]
    Eject,
    #[serde(rename = "bnr.querycashunit")]
    QueryCashUnit,
    #[serde(rename = "bnr.configurecashunit")]
    ConfigureCashUnit,
    #[serde(rename = "bnr.updatecashunit")]
    UpdateCashUnit,
    #[serde(rename = "bnr.denominate")]
    Denominate,
    #[serde(rename = "bnr.dispense")]
    Dispense,
    #[serde(rename = "bnr.present")]
    Present,
    #[serde(rename = "bnr.cancelwaitingcashtaken")]
    CancelWaitingCashTaken,
    #[serde(rename = "bnr.retract")]
    Retract,
    #[serde(rename = "bnr.setcapabilities")]
    SetCapabilities,
    // **NOTE**: `Occured` is not a typo here, it is a mispelling in the protocol message that we have to replicate
    #[serde(rename = "BnrListener.operationCompleteOccured")]
    OperationCompleteOccurred,
    // **NOTE**: `Occured` is not a typo here, it is a mispelling in the protocol message that we have to replicate
    #[serde(rename = "BnrListener.statusOccured")]
    StatusOccurred,
}

impl XfsMethodName {
    /// Creates a new [XfsMethodName].
    pub const fn new() -> Self {
        Self::GetIdentification
    }
}

impl From<&XfsMethodName> for &'static str {
    fn from(val: &XfsMethodName) -> Self {
        match val {
            XfsMethodName::GetDateTime => "bnr.getdatetime",
            XfsMethodName::SetDateTime => "bnr.setdatetime",
            XfsMethodName::CashInStart => "bnr.cashinstart",
            XfsMethodName::CashIn => "bnr.cashin",
            XfsMethodName::CashInRollback => "bnr.cashinrollback",
            XfsMethodName::CashInEnd => "bnr.cashinend",
            XfsMethodName::GetIdentification => "module.getidentification",
            XfsMethodName::GetStatus => "bnr.getstatus",
            XfsMethodName::StopSession => "bnr.stopsession",
            XfsMethodName::Reset => "bnr.reset",
            XfsMethodName::Reboot => "bnr.reboot",
            XfsMethodName::Cancel => "bnr.cancel",
            XfsMethodName::Park => "bnr.park",
            XfsMethodName::Empty => "bnr.empty",
            XfsMethodName::Eject => "bnr.eject",
            XfsMethodName::QueryCashUnit => "bnr.querycashunit",
            XfsMethodName::ConfigureCashUnit => "bnr.configurecashunit",
            XfsMethodName::UpdateCashUnit => "bnr.updatecashunit",
            XfsMethodName::Denominate => "bnr.denominate",
            XfsMethodName::Dispense => "bnr.dispense",
            XfsMethodName::Present => "bnr.present",
            XfsMethodName::CancelWaitingCashTaken => "bnr.cancelwaitingcashtaken",
            XfsMethodName::Retract => "bnr.retract",
            XfsMethodName::SetCapabilities => "bnr.setcapabilities",
            XfsMethodName::OperationCompleteOccurred => "BnrListener.operationCompleteOccured",
            XfsMethodName::StatusOccurred => "BnrListener.statusOccured",
        }
    }
}

impl From<XfsMethodName> for &'static str {
    fn from(val: XfsMethodName) -> Self {
        (&val).into()
    }
}

impl TryFrom<&str> for XfsMethodName {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val.to_lowercase().as_str() {
            "bnr.getdatetime" => Ok(Self::GetDateTime),
            "bnr.setdatetime" => Ok(Self::SetDateTime),
            "bnr.cashinstart" => Ok(Self::CashInStart),
            "bnr.cashin" => Ok(Self::CashIn),
            "bnr.cashinrollback" => Ok(Self::CashInRollback),
            "bnr.cashinend" => Ok(Self::CashInEnd),
            "module.getidentification" => Ok(Self::GetIdentification),
            "bnr.getstatus" => Ok(Self::GetStatus),
            "bnr.stopsession" => Ok(Self::StopSession),
            "bnr.reset" => Ok(Self::Reset),
            "bnr.reboot" => Ok(Self::Reboot),
            "bnr.cancel" => Ok(Self::Cancel),
            "bnr.park" => Ok(Self::Park),
            "bnr.empty" => Ok(Self::Empty),
            "bnr.eject" => Ok(Self::Eject),
            "bnr.querycashunit" => Ok(Self::QueryCashUnit),
            "bnr.configurecashunit" => Ok(Self::ConfigureCashUnit),
            "bnr.updatecashunit" => Ok(Self::UpdateCashUnit),
            "bnr.denominate" => Ok(Self::Denominate),
            "bnr.dispense" => Ok(Self::Dispense),
            "bnr.present" => Ok(Self::Present),
            "bnr.cancelwaitingcashtaken" => Ok(Self::CancelWaitingCashTaken),
            "bnr.retract" => Ok(Self::Retract),
            "bnr.setcapabilities" => Ok(Self::SetCapabilities),
            "bnrlistener.operationcompleteoccured" => Ok(Self::OperationCompleteOccurred),
            "bnrlistener.statusoccured" => Ok(Self::StatusOccurred),
            _ => Err(Error::Parsing(format!("unknown method name: {val}"))),
        }
    }
}

impl fmt::Display for XfsMethodName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{xfs, Result};

    #[test]
    fn test_method_call_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><methodCall><methodName></methodName><params/></methodCall>"#;
        let xml_str = xfs::to_string(&XfsMethodCall::new())?;

        assert_eq!(xml_str.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><methodCall><methodName>module.getidentification</methodName><params/></methodCall>"#;
        let xml_str = xfs::to_string(&XfsMethodCall::from(XfsMethodName::GetIdentification))?;

        assert_eq!(xml_str.as_str(), exp_xml);

        Ok(())
    }
}
