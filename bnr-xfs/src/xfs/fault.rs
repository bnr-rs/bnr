//! XFS `fault` entry types.

use super::{
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct, XfsValueStruct},
};
use crate::impl_default;
use std::fmt;

/// Represents an XFS `fault` entry in an
/// [XfsMethodResponse](super::method_response::XfsMethodResponse).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "fault")]
pub struct XfsFault {
    #[serde(rename = "$value")]
    fields: XfsValueStruct,
}

impl fmt::Display for XfsFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fields)
    }
}

impl XfsFault {
    /// Creates a new [XfsFault].
    pub fn new() -> Self {
        Self {
            fields: XfsValueStruct::create(XfsStruct::create(vec![
                XfsMember::create("faultCode".into(), XfsValue::Int4(0)).into(),
                XfsMember::create("faultString".into(), XfsValue::String(String::new())).into(),
            ])),
        }
    }

    /// Creates a new [XfsFault] with the provided fault code.
    pub fn create<S: Into<String>>(code: i32, string: S) -> Self {
        Self {
            fields: XfsValueStruct::create(XfsStruct::create(vec![
                XfsMember::create("faultCode".into(), XfsValue::Int4(code)).into(),
                XfsMember::create("faultString".into(), XfsValue::String(string.into())).into(),
            ])),
        }
    }

    /// Gets the fault code value.
    pub fn code(&self) -> i32 {
        if let Some(fc) = self.fields.as_inner().members().get(0) {
            match fc.as_inner().value() {
                XfsValue::Int4(val) => *val,
                _ => 0,
            }
        } else {
            0
        }
    }

    /// Sets the fault code value.
    pub fn set_code(&mut self, code: i32) {
        if let Some(fc) = self.fields.as_inner_mut().members_mut().get_mut(0) {
            fc.as_inner_mut().set_value(XfsValue::Int4(code));
        }
    }

    /// Builder function that sets the fault code value.
    pub fn with_code(mut self, code: i32) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the fault string value.
    pub fn fault_string(&self) -> &str {
        if let Some(fc) = self.fields.as_inner().members().get(1) {
            match fc.as_inner().value() {
                XfsValue::String(val) => val,
                _ => "",
            }
        } else {
            ""
        }
    }

    /// Sets the fault code value.
    pub fn set_fault_string<S: Into<String>>(&mut self, string: S) {
        if let Some(fc) = self.fields.as_inner_mut().members_mut().get_mut(1) {
            fc.as_inner_mut().set_value(XfsValue::String(string.into()));
        }
    }

    /// Builder function that sets the fault code value.
    pub fn with_fault_string<S: Into<String>>(mut self, string: S) -> Self {
        self.set_fault_string(string);
        self
    }
}

impl_default!(XfsFault);

#[cfg(test)]
mod tests {
    use serde_xml_rs as xml;

    use super::*;
    use crate::Result;

    #[test]
    fn test_fault_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><fault><value><struct><member><name>faultCode</name><value><i4>0</i4></value></member><member><name>faultString</name><value><string></string></value></member></struct></value></fault>"#;
        let exp_fault = XfsFault::new();
        let xml_str = xml::to_string(&exp_fault)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xml::from_str::<XfsFault>(exp_xml)?, exp_fault);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><fault><value><struct><member><name>faultCode</name><value><i4>1010</i4></value></member><member><name>faultString</name><value><string></string></value></member></struct></value></fault>"#;
        let exp_fault = XfsFault::create(1010, "");
        let xml_str = xml::to_string(&exp_fault)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xml::from_str::<XfsFault>(exp_xml)?, exp_fault);

        Ok(())
    }

    #[test]
    fn test_fault_accessors() -> Result<()> {
        let exp_code = 6080;
        let mut fault = XfsFault::create(exp_code, "");

        assert_eq!(fault.code(), exp_code);
        assert_eq!(XfsFault::new().code(), 0);
        assert_eq!(XfsFault::new().with_code(exp_code).code(), exp_code);

        let new_code = 1010;
        fault.set_code(new_code);

        assert_eq!(fault.code(), new_code);

        Ok(())
    }
}
