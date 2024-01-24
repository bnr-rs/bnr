//! XFS `value` types.

use super::{
    array::{XfsArray, XfsValueArray},
    xfs_struct::XfsStruct,
};
use std::fmt;

/// Represents an XFS parameter `value`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "value")]
pub enum XfsValue {
    /// Represents a 64-bit signed integer value.
    #[serde(rename = "int")]
    Int(i64),
    /// Represents a 32-bit signed integer value.
    #[serde(rename = "i4")]
    Int4(i32),
    /// Represents a Base-64 encoded string value.
    #[serde(rename = "base64")]
    Base64(String),
    /// Represents a date-time entry value, encoded according to ISO-8601: <https://en.wikipedia.org/wiki/ISO_8601>.
    #[serde(rename = "dateTime.iso8601")]
    DateTime(String),
    /// Boolean integer type:
    ///
    /// - `0` is false
    /// - `1` is true
    #[serde(rename = "boolean")]
    Boolean(u8),
    /// String value.
    #[serde(rename = "string")]
    String(String),
    /// XFS `struct` value.
    #[serde(rename = "struct")]
    Struct(XfsStruct),
    /// XFS `array` value.
    #[serde(rename = "array")]
    Array(XfsArray),
}

impl XfsValue {
    /// Creates a new [XfsValue].
    pub const fn new() -> Self {
        Self::Int(0)
    }
}

impl_default!(XfsValue);

inner_enum!(XfsValue, Int, i64);
inner_enum!(XfsValue, Int4, i32);
inner_enum!(XfsValue, Base64, String);
inner_enum!(XfsValue, DateTime, String);
inner_enum!(XfsValue, Boolean, u8);
inner_enum!(XfsValue, String, String);
inner_enum!(XfsValue, Struct, XfsStruct);
inner_enum!(XfsValue, Array, XfsArray);

impl fmt::Display for XfsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        match self {
            Self::Int(v) => write!(f, r#""int": {v}"#)?,
            Self::Int4(v) => write!(f, r#""i4": {v}"#)?,
            Self::Base64(v) => write!(f, r#""base64": "{v}""#)?,
            Self::DateTime(v) => write!(f, r#""date_time": "{v}""#)?,
            Self::Boolean(v) => write!(f, r#""boolean": {v}"#)?,
            Self::String(v) => write!(f, r#""string": "{v}""#)?,
            Self::Struct(v) => write!(f, r#""struct": "{v}""#)?,
            Self::Array(v) => write!(f, r#""array": "{v:?}""#)?,
        }
        write!(f, "}}")
    }
}

/// Represents an XFS parameter `value` used in a collection of [XfsValue]s.
///
/// **NOTE** this indirect wrapper is an implementation detail needed for correct serialization.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "value")]
pub enum XfsValues {
    #[serde(rename = "value")]
    Value(XfsValue),
}

impl XfsValues {
    /// Creates a new [XfsValues].
    pub const fn new() -> Self {
        Self::Value(XfsValue::new())
    }

    /// Creates a new [XfsValues] with the provided [XfsValue].
    pub const fn create(value: XfsValue) -> Self {
        Self::Value(value)
    }
}

inner_enum!(XfsValues, Value, XfsValue);

impl From<XfsValue> for XfsValues {
    fn from(val: XfsValue) -> Self {
        Self::Value(val)
    }
}

impl From<&XfsValue> for XfsValues {
    fn from(val: &XfsValue) -> Self {
        val.clone().into()
    }
}

impl fmt::Display for XfsValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs as xml;

    use super::*;
    use crate::{
        xfs::{
            array::XfsArray,
            xfs_struct::{XfsMember, XfsMembers},
        },
        Result,
    };

    #[test]
    fn test_value_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="utf-8"?><value><int>0</int></value>"#;
        let xml_str = xml::to_string(&XfsValues::new())?;

        assert_eq!(xml_str.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="utf-8"?><value><i4>-1</i4></value>"#;
        let xml_str = xml::to_string(&XfsValues::create(XfsValue::Int4(-1)))?;

        assert_eq!(xml_str.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0"?><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value>"#;
        let exp_val = XfsValues::create(XfsValue::Struct(XfsStruct::create(
            [XfsMembers::Member(XfsMember::create(
                "moduleType".into(),
                XfsValue::Int4(0),
            ))]
            .into(),
        )));

        assert_eq!(xml::from_str::<XfsValues>(exp_xml)?, exp_val);

        Ok(())
    }

    #[test]
    fn test_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0"?><array><data><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value></data></array>"#;
        let exp_val = XfsValue::Array(XfsArray::create(
            [XfsValue::Struct(XfsStruct::create(
                [XfsMembers::Member(XfsMember::create(
                    "moduleType".into(),
                    XfsValue::Int4(0),
                ))]
                .into(),
            ))]
            .into_iter(),
        ));

        assert_eq!(xml::from_str::<XfsValue>(exp_xml)?, exp_val);

        let exp_xml = r#"<?xml version="1.0"?><value><array><data><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value></data></array></value>"#;
        let exp_val = XfsValues::Value(XfsValue::Array(XfsArray::create(
            [XfsValue::Struct(XfsStruct::create(
                [XfsMembers::Member(XfsMember::create(
                    "moduleType".into(),
                    XfsValue::Int4(0),
                ))]
                .into(),
            ))]
            .into_iter(),
        )));

        assert_eq!(xml::from_str::<XfsValues>(exp_xml)?, exp_val);

        Ok(())
    }

    #[test]
    fn test_nested_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0"?><value><array><data><value><struct><member><name>moduleType</name><value><array><data><value><struct><member><name>innerModule</name><value><i4>0</i4></value></member></struct></value></data></array></value></member></struct></value></data></array></value>"#;
        let exp_val = XfsValues::Value(XfsValue::Array(XfsArray::create(
            [XfsValue::Struct(XfsStruct::create(
                [XfsMembers::Member(XfsMember::create(
                    "moduleType".into(),
                    XfsValue::Array(XfsArray::create(
                        [XfsValue::Struct(XfsStruct::create(
                            [XfsMembers::Member(XfsMember::create(
                                "innerModule".into(),
                                XfsValue::Int4(0),
                            ))]
                            .into(),
                        ))]
                        .into_iter(),
                    )),
                ))]
                .into(),
            ))]
            .into_iter(),
        )));

        assert_eq!(xml::from_str::<XfsValues>(exp_xml)?, exp_val);

        Ok(())
    }

    #[test]
    fn test_nested_struct_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0"?>
        <value>
          <struct>
            <member>
              <name>outerStruct</name>
              <value>
                <array>
                  <data>
                    <value>
                      <struct>
                        <member>
                          <name>innerStruct</name>
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
        "#;

        xml::from_str::<XfsValues>(exp_xml)?;

        Ok(())
    }
}
