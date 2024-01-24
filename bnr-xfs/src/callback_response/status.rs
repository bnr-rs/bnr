use std::fmt;

use crate::{create_xfs_i4, impl_xfs_struct};

create_xfs_i4!(
    CallbackStatus,
    "status",
    r#"Represents the identification ID for a particular callback operation.

This uniquely identifies the specific call that was made."#
);

create_xfs_i4!(
    CallbackResult,
    "result",
    "Represents the result for a particular callback operation."
);

/// Represents a response to a callback call made by the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallbackStatusResponse {
    status: CallbackStatus,
    result: CallbackResult,
}

impl CallbackStatusResponse {
    /// Creates a new [CallbackStatusResponse].
    pub const fn new() -> Self {
        Self {
            status: CallbackStatus::new(),
            result: CallbackResult::new(),
        }
    }

    /// Creates a new [CallbackStatusResponse] from the provided parameters.
    pub const fn create(status: i32, result: i32) -> Self {
        Self {
            status: CallbackStatus::create(status as u32),
            result: CallbackResult::create(result as u32),
        }
    }

    /// Gets the [CallbackStatus] value.
    pub const fn status(&self) -> u32 {
        self.status.inner()
    }

    /// Sets the [CallbackStatus] value.
    pub fn set_status(&mut self, val: i32) {
        self.status.set_inner(val as u32);
    }

    /// Builder function that sets the [CallbackStatus] value.
    pub fn with_status(mut self, val: i32) -> Self {
        self.set_status(val);
        self
    }

    /// Gets the [CallbackResult] value.
    pub const fn result(&self) -> u32 {
        self.result.inner()
    }

    /// Sets the [CallbackResult] value.
    pub fn set_result(&mut self, val: i32) {
        self.result.set_inner(val as u32)
    }

    /// Builder function that sets the [CallbackResult] value.
    pub fn with_result(mut self, val: i32) -> Self {
        self.set_result(val);
        self
    }
}

impl fmt::Display for CallbackStatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""status": {}, "#, self.status())?;
        write!(f, r#""result": {}"#, self.result())?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(CallbackStatusResponse, "callbackResponse", [status: CallbackStatus, result: CallbackResult]);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::xfs::{
        self,
        method_response::{XfsMethodResponse, XfsMethodResponseStruct},
        params::XfsParam,
    };
    use crate::Result;

    #[test]
    fn test_callback_response_serde() -> Result<()> {
        let exp_xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?><methodResponse><params><param><value><struct><member><name>status</name><value><i4>6192</i4></value></member><member><name>result</name><value><i4>-1</i4></value></member></struct></value></param></params></methodResponse>"#;
        let exp_res =
            XfsMethodResponseStruct::new(XfsMethodResponse::new_params([XfsParam::create(
                CallbackStatusResponse::create(6192, -1).into(),
            )]));

        assert_eq!(xfs::to_iso_string(&exp_res)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsMethodResponseStruct>(exp_xml)?, exp_res);

        Ok(())
    }
}
