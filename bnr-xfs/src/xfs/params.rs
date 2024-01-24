//! XFS `params` collection and `param` entry types.

use super::value::{XfsValue, XfsValues};
use std::fmt;

/// Represents an XFS `params` entry containing a list of [param](XfsParam).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "params")]
pub struct XfsParams {
    #[serde(rename = "$value")]
    params: Vec<XfsParam>,
}

impl XfsParams {
    /// Creates a new [XfsParam].
    pub const fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// Creates a new [XfsParams] with the provided list of [XfsParam].
    pub const fn create(params: Vec<XfsParam>) -> Self {
        Self { params }
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
pub enum XfsParam {
    #[serde(rename = "param")]
    Param(XfsValues),
}

impl XfsParam {
    /// Creates a new [XfsParam].
    pub const fn new() -> Self {
        Self::Param(XfsValues::new())
    }

    /// Creates a new [XfsParam] with the provided [XfsValues].
    pub const fn create_value(value: XfsValue) -> Self {
        Self::Param(XfsValues::create(value))
    }
}

impl fmt::Display for XfsParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Param(p) => write!(f, "{p}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs as xml;

    use super::*;
    use crate::{
        xfs::xfs_struct::{XfsMember, XfsStruct},
        Result,
    };

    #[test]
    fn test_params_empty_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params />"#;
        let xml_str = xml::to_string(&XfsParams::new())?;

        assert_eq!(xml_str.as_str(), exp_xml);

        Ok(())
    }

    #[test]
    fn test_params_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value><int>0</int></value></param></params>"#;
        let exp_params = XfsParams::create([XfsParam::new()].into());
        let xml_str = xml::to_string(&exp_params)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xml::from_str::<XfsParams>(exp_xml)?, exp_params);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value><int>0</int></value></param><param><value><int>0</int></value></param></params>"#;
        let exp_params = XfsParams::create([XfsParam::new(), XfsParam::new()].into());
        let xml_str = xml::to_string(&exp_params)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xml::from_str::<XfsParams>(exp_xml)?, exp_params);

        let exp_xml = r#"<?xml version="1.0" encoding="UTF-8"?><params><param><value><struct><member><name>test</name><value><i4>16</i4></value></member></struct></value></param></params>"#;

        let exp_struct =
            XfsStruct::create([XfsMember::create("test".into(), XfsValue::Int4(16)).into()].into());
        let exp_params =
            XfsParams::create([XfsParam::create_value(XfsValue::Struct(exp_struct))].into());

        let xml_str = xml::to_string(&exp_params)?;

        assert_eq!(xml_str.as_str(), exp_xml);
        assert_eq!(xml::from_str::<XfsParams>(exp_xml)?, exp_params);

        Ok(())
    }
}
