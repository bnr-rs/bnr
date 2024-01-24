//! XFS `method response` types.

use std::fmt;

use super::fault::XfsFault;
use super::params::{XfsParam, XfsParams};
use crate::{Error, Result};

/// Represents an XFS method response containing (one of):
///
/// - a list of [params](XfsParams).
/// - an XFS [fault](XfsFault).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "methodResponse")]
pub enum XfsMethodResponse {
    #[serde(rename = "params")]
    Params(XfsParams),
    #[serde(rename = "fault")]
    Fault(XfsFault),
}

impl XfsMethodResponse {
    /// Creates a new [XfsMethodResponse::Params] variant with the provided [XfsParams].
    pub fn new_params<P: Into<Vec<XfsParam>>>(params: P) -> Self {
        Self::Params(XfsParams::create(params.into()))
    }

    /// Creates a new [XfsMethodResponse::Fault] variant with the provided fault code.
    pub fn new_fault<S: Into<String>>(code: i32, string: S) -> Self {
        Self::Fault(XfsFault::create(code, string))
    }

    /// Gets the async callback ID from the [XfsMethodResponse].
    ///
    /// Returns: `Ok(i32)` on success, `Err(Error)` on failure
    pub fn call_id(&self) -> Result<i32> {
        self.as_params()?
            .params()
            .iter()
            .find(|&p| p.inner().value().i4().is_some())
            .ok_or(Error::Xfs("missing response ID".into()))?
            .inner()
            .value()
            .i4()
            .cloned()
            .ok_or(Error::Xfs("missing response ID".into()))
    }
}

inner_enum!(XfsMethodResponse, Params, XfsParams);
inner_enum!(XfsMethodResponse, Fault, XfsFault);

impl fmt::Display for XfsMethodResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Params(res) => write!(f, "{res}"),
            Self::Fault(res) => write!(f, "{res}"),
        }
    }
}

/// Represents an XFS method response wrapper around [XfsMethodResponse].
///
/// This is an implementation detail for correct serialization.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "methodResponse")]
pub struct XfsMethodResponseStruct {
    #[serde(rename = "$value")]
    response: XfsMethodResponse,
}

impl XfsMethodResponseStruct {
    /// Creates a new [XfsMethodResponseStruct] wrapper.
    pub const fn new(response: XfsMethodResponse) -> Self {
        Self { response }
    }

    /// Gets a reference to the [XfsMethodResponseStruct] inner representation.
    pub const fn inner(&self) -> &XfsMethodResponse {
        &self.response
    }

    /// Gets a mutable reference to the [XfsMethodResponseStruct] inner representation.
    pub fn inner_mut(&mut self) -> &mut XfsMethodResponse {
        &mut self.response
    }

    /// Converts the [XfsMethodResponseStruct] to its inner representation.
    pub fn into_inner(self) -> XfsMethodResponse {
        self.response
    }
}

impl From<XfsMethodResponse> for XfsMethodResponseStruct {
    fn from(val: XfsMethodResponse) -> Self {
        Self::new(val)
    }
}

impl From<&XfsMethodResponse> for XfsMethodResponseStruct {
    fn from(val: &XfsMethodResponse) -> Self {
        Self::new(val.clone())
    }
}

impl From<XfsMethodResponseStruct> for XfsMethodResponse {
    fn from(val: XfsMethodResponseStruct) -> Self {
        val.into_inner()
    }
}

impl From<&XfsMethodResponseStruct> for XfsMethodResponse {
    fn from(val: &XfsMethodResponseStruct) -> Self {
        val.clone().into()
    }
}

impl fmt::Display for XfsMethodResponseStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.inner();
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        xfs::{self, value::XfsValue},
        Result,
    };

    #[test]
    fn test_method_response_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><methodResponse><fault><value><struct><member><name>faultCode</name><value><i4>6072</i4></value></member><member><name>faultString</name><value><string></string></value></member></struct></value></fault></methodResponse>"#;
        let res = XfsMethodResponseStruct::from(XfsMethodResponse::new_fault(6072, ""));

        assert_eq!(xfs::to_string(&res)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsMethodResponseStruct>(exp_xml)?, res);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><methodResponse><params><param><value><i4>32</i4></value></param></params></methodResponse>"#;
        let res = XfsMethodResponseStruct::from(XfsMethodResponse::new_params([XfsParam::create(
            XfsValue::new().with_i4(32),
        )]));

        assert_eq!(xfs::to_string(&res)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsMethodResponseStruct>(exp_xml)?, res);

        Ok(())
    }

    #[test]
    fn test_method_response_accessors() -> Result<()> {
        let fault = XfsMethodResponse::new_fault(1010, "");
        let exp_fault = XfsFault::create(1010, "");

        assert!(fault.is_fault());
        assert_eq!(fault.as_fault()?, &exp_fault);
        assert_eq!(fault.into_fault()?, exp_fault);

        let params = XfsMethodResponse::new_params([XfsParam::create(XfsValue::new().with_i4(32))]);
        let exp_params = XfsParams::create([XfsParam::create(XfsValue::new().with_i4(32))]);

        assert!(params.is_params());
        assert_eq!(params.as_params()?, &exp_params);
        assert_eq!(params.into_params()?, exp_params);

        Ok(())
    }

    #[test]
    fn test_nested_response() -> Result<()> {
        let fuck_xml = r#"<?xml version="1.0"?>
        <methodResponse>
          <params>
            <param>
              <value>
                <struct>
                  <member>
                    <name>
                      outerStruct
                    </name>
                    <value>
                      <array>
                        <data>
                          <value>
                            <struct>
                              <member>
                                <name>
                                  innerStruct
                                </name>
                                <value>
                                  <array>
                                    <data>
                                      <value>
                                        <i4>0</i4>
                                      </value>
                                      <value>
                                        <i4>1</i4>
                                      </value>
                                    </data>
                                  </array>
                                </value>
                              </member>
                            </struct>
                          </value>
                        </data>
                      </array>
                    </value>
                  </member>
                </struct>
              </value>
            </param>
          </params>
        </methodResponse>
        "#;

        xfs::from_str::<XfsMethodResponseStruct>(fuck_xml)?;

        Ok(())
    }

    #[test]
    fn test_null_param_value_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><methodResponse><params><param><value/></param></params></methodResponse>"#;
        let exp_res =
            XfsMethodResponseStruct::from(XfsMethodResponse::Params(XfsParams::create([
                XfsParam::new(),
            ])));

        assert_eq!(xfs::to_string(&exp_res)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsMethodResponseStruct>(exp_xml)?, exp_res);

        Ok(())
    }
}
