//! XFS `value` types.

use std::fmt;

use super::{array::XfsArray, xfs_struct::XfsStruct};

/// Represents an XFS parameter `value`.
#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(rename = "value")]
pub struct XfsValue {
    /// Represents a 64-bit signed integer value.
    #[serde(default)]
    pub(crate) int: Option<i64>,
    /// Represents a 32-bit signed integer value.
    #[serde(default)]
    pub(crate) i4: Option<i32>,
    /// Represents a Base-64 encoded string value.
    #[serde(default)]
    pub(crate) base64: Option<String>,
    /// Represents a date-time entry value, encoded according to ISO-8601: <https://en.wikipedia.org/wiki/ISO_8601>.
    #[serde(rename = "dateTime.iso8601", default)]
    pub(crate) date_time: Option<String>,
    /// Boolean integer type:
    ///
    /// - `0` is false
    /// - `1` is true
    #[serde(default)]
    pub(crate) boolean: Option<u8>,
    /// String value.
    #[serde(default)]
    pub(crate) string: Option<String>,
    /// XFS `struct` value.
    #[serde(rename = "struct", default)]
    pub(crate) xfs_struct: Option<XfsStruct>,
    /// XFS `array` value.
    #[serde(default)]
    pub(crate) array: Option<XfsArray>,
}

impl XfsValue {
    /// Creates a new [XfsValue].
    pub const fn new() -> Self {
        Self {
            int: None,
            i4: None,
            base64: None,
            date_time: None,
            boolean: None,
            string: None,
            xfs_struct: None,
            array: None,
        }
    }

    /// Gets whether all fields are empty.
    pub const fn is_empty(&self) -> bool {
        self.int.is_none()
            && self.i4.is_none()
            && self.base64.is_none()
            && self.date_time.is_none()
            && self.boolean.is_none()
            && self.string.is_none()
            && self.xfs_struct.is_none()
            && self.array.is_none()
    }

    /// Gets an optional reference to the `int` field.
    pub const fn int(&self) -> Option<&i64> {
        self.int.as_ref()
    }

    /// Sets the `int` field.
    pub fn set_int(&mut self, int: i64) {
        self.int = Some(int);
    }

    /// Unsets the `int` field.
    pub fn unset_int(&mut self) -> Option<i64> {
        self.int.take()
    }

    /// Builder function that sets the `int` field.
    pub fn with_int(mut self, int: i64) -> Self {
        self.set_int(int);
        self
    }

    /// Gets an optional reference to the `i4` field.
    pub const fn i4(&self) -> Option<&i32> {
        self.i4.as_ref()
    }

    /// Sets the `i4` field.
    pub fn set_i4(&mut self, i4: i32) {
        self.i4 = Some(i4);
    }

    /// Unsets the `i4` field.
    pub fn unset_i4(&mut self) -> Option<i32> {
        self.i4.take()
    }

    /// Builder function that sets the `i4` field.
    pub fn with_i4(mut self, i4: i32) -> Self {
        self.set_i4(i4);
        self
    }

    /// Gets an optional reference to the `base64` field.
    pub fn base64(&self) -> Option<&str> {
        self.base64.as_deref()
    }

    /// Sets the `base64` field.
    pub fn set_base64<S: Into<String>>(&mut self, base64: S) {
        self.base64 = Some(base64.into());
    }

    /// Unsets the `base64` field.
    pub fn unset_base64(&mut self) -> Option<String> {
        self.base64.take()
    }

    /// Builder function that sets the `base64` field.
    pub fn with_base64<S: Into<String>>(mut self, base64: S) -> Self {
        self.set_base64(base64);
        self
    }

    /// Gets an optional reference to the `date_time` field.
    pub fn date_time(&self) -> Option<&str> {
        self.date_time.as_deref()
    }

    /// Sets the `date_time` field.
    pub fn set_date_time<S: Into<String>>(&mut self, date_time: S) {
        self.date_time = Some(date_time.into());
    }

    /// Unsets the `date_time` field.
    pub fn unset_date_time(&mut self) -> Option<String> {
        self.date_time.take()
    }

    /// Builder function that sets the `date_time` field.
    pub fn with_date_time<S: Into<String>>(mut self, date_time: S) -> Self {
        self.set_date_time(date_time);
        self
    }

    /// Gets an optional reference to the `string` field.
    pub fn string(&self) -> Option<&str> {
        self.string.as_deref()
    }

    /// Sets the `string` field.
    pub fn set_string<S: Into<String>>(&mut self, string: S) {
        self.string = Some(string.into());
    }

    /// Unsets the `string` field.
    pub fn unset_string(&mut self) -> Option<String> {
        self.string.take()
    }

    /// Builder function that sets the `string` field.
    pub fn with_string<S: Into<String>>(mut self, string: S) -> Self {
        self.set_string(string);
        self
    }

    /// Gets an optional reference to the `struct` field.
    pub const fn xfs_struct(&self) -> Option<&XfsStruct> {
        self.xfs_struct.as_ref()
    }

    /// Sets the `struct` field.
    pub fn set_xfs_struct(&mut self, xfs_struct: XfsStruct) {
        self.xfs_struct = Some(xfs_struct.into());
    }

    /// Unsets the `struct` field.
    pub fn unset_xfs_struct(&mut self) -> Option<XfsStruct> {
        self.xfs_struct.take()
    }

    /// Builder function that sets the `struct` field.
    pub fn with_xfs_struct(mut self, xfs_struct: XfsStruct) -> Self {
        self.set_xfs_struct(xfs_struct);
        self
    }

    /// Gets an optional reference to the `boolean` field.
    pub const fn boolean(&self) -> Option<&u8> {
        self.boolean.as_ref()
    }

    /// Sets the `boolean` field.
    pub fn set_boolean(&mut self, boolean: u8) {
        self.boolean = Some(boolean);
    }

    /// Unsets the `boolean` field.
    pub fn unset_boolean(&mut self) -> Option<u8> {
        self.boolean.take()
    }

    /// Builder function that sets the `boolean` field.
    pub fn with_boolean(mut self, boolean: u8) -> Self {
        self.set_boolean(boolean);
        self
    }

    /// Gets an optional reference to the `array` field.
    pub const fn array(&self) -> Option<&XfsArray> {
        self.array.as_ref()
    }

    /// Sets the `array` field.
    pub fn set_array(&mut self, xfs_array: XfsArray) {
        self.array = Some(xfs_array.into());
    }

    /// Unsets the `array` field.
    pub fn unset_array(&mut self) -> Option<XfsArray> {
        self.array.take()
    }

    /// Builder function that sets the `array` field.
    pub fn with_array(mut self, xfs_array: XfsArray) -> Self {
        self.set_array(xfs_array);
        self
    }
}

impl serde::Serialize for XfsValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut ser = serializer.serialize_struct("value", 8)?;

        if let Some(v) = self.int() {
            ser.serialize_field("int", v)?;
        }

        if let Some(v) = self.i4() {
            ser.serialize_field("i4", v)?;
        }

        if let Some(v) = self.base64() {
            ser.serialize_field("base64", v)?;
        }

        if let Some(v) = self.date_time() {
            ser.serialize_field("dateTime.iso8601", v)?;
        }

        if let Some(v) = self.boolean() {
            ser.serialize_field("boolean", v)?;
        }

        if let Some(v) = self.string() {
            ser.serialize_field("string", v)?;
        }

        if let Some(v) = self.xfs_struct() {
            ser.serialize_field("struct", v)?;
        }

        if let Some(v) = self.array() {
            ser.serialize_field("array", v)?;
        }

        ser.end()
    }
}

impl_default!(XfsValue);

impl fmt::Display for XfsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut has_field = false;
        if let Some(v) = self.int() {
            write!(f, r#""int": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.i4() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""i4": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.base64() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""base64": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.date_time() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""date_time": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.boolean() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""boolean": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.string() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""string": "{v}""#)?;
            has_field = true;
        }
        if let Some(v) = self.xfs_struct() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""struct": {v}"#)?;
            has_field = true;
        }
        if let Some(v) = self.array() {
            if has_field {
                write!(f, ", ")?;
            }
            write!(f, r#""array": {v}"#)?;
        }
        write!(f, "}}")
    }
}

/// Wrapper type for `serde-xml-rs` to properly encode [XfsValue] in collections.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ListValue {
    #[serde(rename = "value")]
    Value(XfsValue),
}

impl ListValue {
    /// Creates a new [ListValue].
    pub const fn new() -> Self {
        Self::Value(XfsValue::new())
    }

    /// Creates a new [ListValue] from the provided [XfsValue].
    pub const fn create(value: XfsValue) -> Self {
        Self::Value(value)
    }

    /// Gets a reference to the inner [XfsValue].
    pub const fn inner(&self) -> &XfsValue {
        match self {
            Self::Value(v) => v,
        }
    }

    /// Converts into the inner [XfsValue].
    pub fn into_inner(self) -> XfsValue {
        match self {
            Self::Value(v) => v,
        }
    }
}

impl From<XfsValue> for ListValue {
    fn from(val: XfsValue) -> Self {
        Self::create(val)
    }
}

impl From<&XfsValue> for ListValue {
    fn from(val: &XfsValue) -> Self {
        val.clone().into()
    }
}

impl From<ListValue> for XfsValue {
    fn from(val: ListValue) -> Self {
        val.into_inner()
    }
}

impl From<&ListValue> for XfsValue {
    fn from(val: &ListValue) -> Self {
        val.clone().into()
    }
}

impl fmt::Display for ListValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.inner();
        write!(f, "{v}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        xfs::{self, xfs_struct::XfsMember},
        Result,
    };

    #[test]
    fn test_value_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value/>"#;
        let exp_val = XfsValue::new();

        assert_eq!(xfs::to_string(&exp_val)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsValue>(exp_xml)?, exp_val);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><i4>-1</i4></value>"#;
        let exp_val = XfsValue::new().with_i4(-1);

        assert_eq!(xfs::to_string(&exp_val)?.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_value_list_serde() -> Result<()> {
        #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename = "list")]
        struct List {
            #[serde(rename = "$value")]
            value: Vec<Value>,
        }

        #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        enum Value {
            #[serde(rename = "value")]
            Value(XfsValue),
        }

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><list><value/></list>"#;
        let exp_list = List {
            value: vec![Value::Value(XfsValue::new())],
        };

        assert_eq!(xfs::to_string(&exp_list)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<List>(exp_xml)?, exp_list);

        Ok(())
    }

    #[test]
    fn test_value_struct_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value>"#;
        let exp_val = ListValue::create(XfsValue::new().with_xfs_struct(XfsStruct::create([
            XfsMember::create("moduleType", XfsValue::new().with_i4(0)),
        ])));

        assert_eq!(xfs::from_str::<ListValue>(exp_xml)?, exp_val);

        Ok(())
    }

    #[test]
    fn test_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><array><data><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value></data></array>"#;
        let exp_arr = XfsArray::create([XfsValue::new().with_xfs_struct(XfsStruct::create([
            XfsMember::create("moduleType", XfsValue::new().with_i4(0)),
        ]))]);

        assert_eq!(xfs::from_str::<XfsArray>(exp_xml)?, exp_arr);
        assert_eq!(xfs::to_string(exp_arr)?.as_str(), exp_xml);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><value><array><data><value><struct><member><name>moduleType</name><value><i4>0</i4></value></member></struct></value></data></array></value>"#;
        let exp_val = ListValue::Value(XfsValue::new().with_array(XfsArray::create([
            XfsValue::new().with_xfs_struct(XfsStruct::create([XfsMember::create(
                "moduleType",
                XfsValue::new().with_i4(0),
            )])),
        ])));

        assert_eq!(xfs::from_str::<ListValue>(exp_xml)?, exp_val);
        assert_eq!(xfs::to_string(exp_val)?.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_nested_array_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0"?><value><array><data><value><struct><member><name>moduleType</name><value><array><data><value><struct><member><name>innerModule</name><value><i4>0</i4></value></member></struct></value></data></array></value></member></struct></value></data></array></value>"#;
        let exp_val =
            ListValue::Value(
                XfsValue::new().with_array(XfsArray::create([XfsValue::new().with_xfs_struct(
                    XfsStruct::create([XfsMember::create(
                        "moduleType",
                        XfsValue::new().with_array(XfsArray::create(
                            [XfsValue::new().with_xfs_struct(XfsStruct::create([
                                XfsMember::create("innerModule", XfsValue::new().with_i4(0)),
                            ]))]
                            .into_iter(),
                        )),
                    )]),
                )])),
            );

        assert_eq!(xfs::from_str::<ListValue>(exp_xml)?, exp_val);

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

        xfs::from_str::<ListValue>(exp_xml)?;

        Ok(())
    }
}
