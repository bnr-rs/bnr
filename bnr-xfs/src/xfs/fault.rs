//! XFS `fault` entry types.

use std::fmt;

use super::{
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::impl_default;

/// Represents an XFS `fault` entry in an
/// [XfsMethodResponse](super::method_response::XfsMethodResponse).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "fault")]
pub struct XfsFault {
    value: XfsValue,
}

impl fmt::Display for XfsFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl XfsFault {
    /// Creates a new [XfsFault].
    pub fn new() -> Self {
        Self {
            value: XfsValue::new().with_xfs_struct(XfsStruct::create([
                XfsMember::create("faultCode", XfsValue::new().with_i4(0)),
                XfsMember::create("faultString", XfsValue::new().with_string(String::new())),
            ])),
        }
    }

    /// Creates a new [XfsFault] with the provided fault code.
    pub fn create<S: Into<String>>(code: i32, string: S) -> Self {
        Self {
            value: XfsValue::new().with_xfs_struct(XfsStruct::create([
                XfsMember::create("faultCode", XfsValue::new().with_i4(code)),
                XfsMember::create("faultString", XfsValue::new().with_string(string.into())),
            ])),
        }
    }

    /// Gets the fault code value.
    pub fn code(&self) -> i32 {
        match self.value.xfs_struct() {
            Some(fc) => match fc.members().get(0) {
                Some(m) => match m.inner().value().i4() {
                    Some(val) => *val,
                    None => 0,
                },
                None => 0,
            },
            None => 0,
        }
    }

    /// Sets the fault code value.
    pub fn set_code(&mut self, code: i32) {
        if let Some(fc) = self.value.xfs_struct.as_mut() {
            if let Some(m) = fc.members_mut().get_mut(0) {
                m.inner_mut().set_value(XfsValue::new().with_i4(code));
            }
        }
    }

    /// Builder function that sets the fault code value.
    pub fn with_code(mut self, code: i32) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the fault string value.
    pub fn fault_string(&self) -> &str {
        match self.value.xfs_struct() {
            Some(fc) => match fc.members().get(1) {
                Some(m) => match m.inner().value().string() {
                    Some(val) => val,
                    None => "",
                },
                None => "",
            },
            None => "",
        }
    }

    /// Sets the fault code value.
    pub fn set_fault_string<S: Into<String>>(&mut self, string: S) {
        if let Some(fc) = self.value.xfs_struct.as_mut() {
            if let Some(m) = fc.members_mut().get_mut(1) {
                m.inner_mut()
                    .set_value(XfsValue::new().with_string(string.into()));
            }
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
    use super::*;
    use crate::{xfs, Result};

    #[test]
    fn test_fault_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><fault><value><struct><member><name>faultCode</name><value><i4>0</i4></value></member><member><name>faultString</name><value><string></string></value></member></struct></value></fault>"#;
        let exp_fault = XfsFault::new();
        let xml_str = xfs::to_string(&exp_fault)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsFault>(exp_xml)?, exp_fault);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><fault><value><struct><member><name>faultCode</name><value><i4>1010</i4></value></member><member><name>faultString</name><value><string></string></value></member></struct></value></fault>"#;
        let exp_fault = XfsFault::create(1010, "");
        let xml_str = xfs::to_string(&exp_fault)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsFault>(exp_xml)?, exp_fault);

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
