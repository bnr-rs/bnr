//! XFS `params` collection and `param` entry types.

use std::fmt;

use super::value::XfsValue;

/// Represents an XFS `params` entry containing a list of [param](XfsParam).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "params")]
pub struct XfsParams {
    #[serde(rename = "$value", default)]
    params: Vec<ListParam>,
}

impl XfsParams {
    /// Creates a new [XfsParams].
    pub const fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// Creates a new [XfsParams] with the provided list of [XfsParam].
    pub fn create<P: IntoIterator<Item = XfsParam>>(p: P) -> Self {
        Self {
            params: p.into_iter().map(ListParam::from).collect(),
        }
    }

    /// Gets a reference to the list of [XfsParam]s.
    pub fn params(&self) -> &[ListParam] {
        self.params.as_ref()
    }
}

impl fmt::Display for XfsParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, param) in self.params.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, r#""param": {param}"#)?;
        }
        write!(f, "}}")
    }
}

/// Represents an XFS parameter that contains a list of [values](XfsValue).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "param")]
pub struct XfsParam {
    value: XfsValue,
}

impl XfsParam {
    /// Creates a new [XfsParam].
    pub const fn new() -> Self {
        Self {
            value: XfsValue::new(),
        }
    }

    /// Creates a new [XfsParam] with the provided [XfsValues].
    pub fn create(value: XfsValue) -> Self {
        Self { value: value }
    }

    /// Gets a reference to the [XfsValue].
    pub const fn value(&self) -> &XfsValue {
        &self.value
    }
}

impl fmt::Display for XfsParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = self.value();
        write!(f, "{p}")
    }
}

/// Wrapper type for [XfsParam] used in a collection.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ListParam {
    #[serde(rename = "param")]
    Param(XfsParam),
}

impl ListParam {
    /// Creates a new [ListParam].
    pub const fn new() -> Self {
        Self::Param(XfsParam::new())
    }

    /// Creates a new [ListParam] from the provided [XfsParam].
    pub const fn create(value: XfsParam) -> Self {
        Self::Param(value)
    }

    /// Gets a reference to the inner [XfsParam].
    pub const fn inner(&self) -> &XfsParam {
        match self {
            Self::Param(v) => v,
        }
    }

    /// Converts into the inner [XfsParam].
    pub fn into_inner(self) -> XfsParam {
        match self {
            Self::Param(v) => v,
        }
    }
}

impl From<XfsParam> for ListParam {
    fn from(val: XfsParam) -> Self {
        Self::create(val)
    }
}

impl From<&XfsParam> for ListParam {
    fn from(val: &XfsParam) -> Self {
        val.clone().into()
    }
}

impl From<ListParam> for XfsParam {
    fn from(val: ListParam) -> Self {
        val.into_inner()
    }
}

impl From<&ListParam> for XfsParam {
    fn from(val: &ListParam) -> Self {
        val.clone().into()
    }
}

impl fmt::Display for ListParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.inner();
        write!(f, "{v}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        xfs::{
            self,
            xfs_struct::{XfsMember, XfsStruct},
        },
        Result,
    };

    #[test]
    fn test_params_empty_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params/>"#;
        let exp_params = XfsParams::new();

        assert_eq!(xfs::to_string(&exp_params)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsParams>(exp_xml)?, exp_params);

        Ok(())
    }

    #[test]
    fn test_params_serde() -> Result<()> {
        let exp_xml =
            r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value/></param></params>"#;
        let exp_params = XfsParams::create([XfsParam::new()]);

        assert_eq!(xfs::to_string(&exp_params)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsParams>(exp_xml)?, exp_params);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value><struct><member><name>test</name><value><i4>16</i4></value></member></struct></value></param></params>"#;

        let exp_struct =
            XfsStruct::create([XfsMember::create("test", XfsValue::new().with_i4(16))]);
        let exp_params = XfsParams::create([XfsParam::create(
            XfsValue::new().with_xfs_struct(exp_struct),
        )]);

        assert_eq!(xfs::from_str::<XfsParams>(exp_xml)?, exp_params);
        assert_eq!(xfs::to_string(&exp_params)?.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_multiple_params_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value/></param><param><value/></param></params>"#;
        let exp_params = XfsParams::create([XfsParam::new(), XfsParam::new()]);
        let xml_str = xfs::to_string(&exp_params)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsParams>(exp_xml)?, exp_params);

        Ok(())
    }
}
