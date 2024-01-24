//! XFS `array` types.

use super::value::{XfsValue, XfsValues};

/// Represents an XFS `array` containing a list of [XfsValues].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "array")]
pub struct XfsArray {
    data: Vec<XfsValues>,
}

impl XfsArray {
    /// Creates a new [XfsArray].
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a new [XfsArray] from the provided [XfsValue] list.
    pub fn create<D: Iterator<Item = XfsValue>>(data: D) -> Self {
        Self {
            data: data.map(XfsValues::from).collect(),
        }
    }

    /// Gets the data list of [XfsValues].
    pub fn data(&self) -> &[XfsValues] {
        self.data.as_ref()
    }

    /// Sets the data list of [XfsValues].
    pub fn set_data<D: Iterator<Item = XfsValue>>(&mut self, data: D) {
        self.data = data.map(XfsValues::from).collect();
    }

    /// Builder function that sets the data list of [XfsValues].
    pub fn with_data<D: Iterator<Item = XfsValue>>(mut self, data: D) -> Self {
        self.set_data(data);
        self
    }
}

/// Represents an XFS `value` containing an [XfsArray].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "value")]
pub struct XfsValueArray {
    array: XfsArray,
}

impl XfsValueArray {
    /// Creates a new [XfsValueArray].
    pub const fn new() -> Self {
        Self {
            array: XfsArray::new(),
        }
    }

    /// Creates a new [XfsValueArray] with the provided [XfsArray].
    pub const fn create(array: XfsArray) -> Self {
        Self { array }
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs as xml;

    use super::*;
    use crate::xfs::{
        value::XfsValue,
        xfs_struct::{XfsMember, XfsStruct},
    };
    use crate::Result;

    #[test]
    fn test_array_empty_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array />"#;
        let xml_str = xml::to_string(&XfsArray::new())?;

        assert_eq!(xml_str.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value><struct><member><name>test</name><value><int>0</int></value></member></struct></value></data></array>"#;

        let test_member = XfsMember::create("test".into(), XfsValue::new());
        let test_struct = XfsValue::Struct(XfsStruct::create([test_member.into()].into()));
        let test_arr = XfsArray::create([test_struct].into_iter());

        let xml_str = xml::to_string(&test_arr)?;

        assert_eq!(xml_str.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_value_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><array><data><value><struct><member><name>test</name><value><int>0</int></value></member></struct></value></data></array></value>"#;

        let test_member = XfsMember::create("test".into(), XfsValue::new());
        let test_struct = XfsStruct::create([test_member.into()].into());
        let test_values = [XfsValue::Struct(test_struct)];
        let test_arr = XfsValueArray::create(XfsArray::create(test_values.into_iter()));

        let xml_str = xml::to_string(&test_arr)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        Ok(())
    }
}
