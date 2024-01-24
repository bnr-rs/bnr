//! XFS `array` types.

use std::fmt;

use super::value::{ListValue, XfsValue};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct XfsData {
    #[serde(rename = "$value", default)]
    value: Vec<ListValue>,
}

impl XfsData {
    /// Creates a new [XfsData].
    pub const fn new() -> Self {
        Self { value: Vec::new() }
    }

    /// Creates a new [XfsData] from the provided parameter.
    pub fn create<D: IntoIterator<Item = XfsValue>>(data: D) -> Self {
        Self {
            value: data.into_iter().map(ListValue::from).collect(),
        }
    }
}

impl AsRef<[ListValue]> for XfsData {
    fn as_ref(&self) -> &[ListValue] {
        self.value.as_ref()
    }
}

impl fmt::Display for XfsData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.value.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{v}")?;
        }
        write!(f, "]")
    }
}

/// Represents an XFS `array` containing a list of [XfsValues].
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "array")]
pub struct XfsArray {
    data: XfsData,
}

impl XfsArray {
    /// Creates a new [XfsArray].
    pub const fn new() -> Self {
        Self {
            data: XfsData::new(),
        }
    }

    /// Creates a new [XfsArray] from the provided [XfsValue] list.
    pub fn create<D: IntoIterator<Item = XfsValue>>(data: D) -> Self {
        Self {
            data: XfsData::create(data),
        }
    }

    /// Gets the data list of [XfsValues].
    pub fn data(&self) -> &[ListValue] {
        self.data.as_ref()
    }

    /// Sets the data list of [XfsValues].
    pub fn set_data<D: IntoIterator<Item = XfsValue>>(&mut self, data: D) {
        self.data = XfsData::create(data);
    }

    /// Builder function that sets the data list of [XfsValues].
    pub fn with_data<D: IntoIterator<Item = XfsValue>>(mut self, data: D) -> Self {
        self.set_data(data);
        self
    }
}

impl fmt::Display for XfsArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = &self.data;
        write!(f, r#"{{"data": {d}}}"#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xfs::{self, value::XfsValue};
    use crate::Result;

    #[test]
    fn test_array_empty_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data/></array>"#;
        let exp_arr = XfsArray::new();

        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data/></array>"#;
        let exp_arr = XfsArray::create([]);

        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);

        Ok(())
    }

    #[test]
    fn test_array_serde() -> Result<()> {
        let exp_xml =
            r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value/></data></array>"#;

        let exp_arr = XfsArray::create([XfsValue::new()]);

        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value><i4>0</i4></value></data></array>"#;

        let exp_arr = XfsArray::create([XfsValue::new().with_i4(0)]);

        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value/><value><i4>1</i4></value></data></array>"#;

        let exp_arr = XfsArray::create([XfsValue::new(), XfsValue::new().with_i4(1)]);

        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value><i4>0</i4></value><value><i4>1</i4></value></data></array>"#;

        let exp_arr = XfsArray::create([XfsValue::new().with_i4(0), XfsValue::new().with_i4(1)]);

        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_array_struct_serde() -> Result<()> {
        /*
            let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value><struct><member><name>test</name><value><int>0</int></value></member></struct></value></data></array>"#;

            let test_member = XfsMember::create("test".into(), XfsValue::new());
            let test_struct = XfsValue::Struct(XfsStruct::create([test_member.into()].into()));
            let test_arr = XfsArray::create([test_struct].into_iter());

            let xml_str = xfs::to_string(&test_arr)?;

            assert_eq!(xml_str.as_str(), exp_xml);
        */

        Ok(())
    }

    #[test]
    fn test_value_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><array><data><value/></data></array></value>"#;

        let exp_arr = XfsValue::new().with_array(XfsArray::create([XfsValue::new()]));

        assert_eq!(xfs::from_str::<XfsValue>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(&exp_arr)?.as_str(), exp_xml);
        /*
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><array><data><value><struct><member><name>test</name><value><int>0</int></value></member></struct></value></data></array></value>"#;

        let test_member = XfsMember::create("test".into(), XfsValue::new());
        let test_struct = XfsStruct::create([test_member.into()].into());
        let test_values = [XfsValue::Struct(test_struct)];
        let test_arr = XfsValueArray::create(XfsArray::create(test_values.into_iter()));

        let xml_str = xfs::to_string(&test_arr)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        */
        Ok(())
    }
}
