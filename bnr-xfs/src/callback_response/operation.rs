use std::fmt;

use crate::{create_xfs_i4, impl_xfs_struct, xfs::OperationId};

create_xfs_i4!(
    IdentificationId,
    "identificationId",
    "Represents the specific call instance for a particular callback operation."
);

/// Represents a response to a callback call made by the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallbackOperationResponse {
    operation_id: OperationId,
    identification_id: IdentificationId,
}

impl CallbackOperationResponse {
    /// Creates a new [CallbackOperationResponse].
    pub const fn new() -> Self {
        Self {
            operation_id: OperationId::new(),
            identification_id: IdentificationId::new(),
        }
    }

    /// Creates a new [CallbackOperationResponse] from the provided parameters.
    pub const fn create(operation_id: i32, identification_id: i32) -> Self {
        Self {
            operation_id: OperationId::create(operation_id as u32),
            identification_id: IdentificationId::create(identification_id as u32),
        }
    }

    /// Gets the [OperationId] value.
    pub const fn operation_id(&self) -> u32 {
        self.operation_id.inner()
    }

    /// Sets the [OperationId] value.
    pub fn set_operation_id(&mut self, val: i32) {
        self.operation_id = OperationId::create(val as u32);
    }

    /// Builder function that sets the [OperationId] value.
    pub fn with_operation_id(mut self, val: i32) -> Self {
        self.set_operation_id(val);
        self
    }

    /// Gets the [IdentificationId] value.
    pub const fn identification_id(&self) -> u32 {
        self.identification_id.inner()
    }

    /// Sets the [IdentificationId] value.
    pub fn set_identification_id(&mut self, val: i32) {
        self.identification_id.set_inner(val as u32)
    }

    /// Builder function that sets the [IdentificationId] value.
    pub fn with_identification_id(mut self, val: i32) -> Self {
        self.set_identification_id(val);
        self
    }
}

impl fmt::Display for CallbackOperationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""operation_id": {}, "#, self.operation_id())?;
        write!(f, r#""identification_id": {}"#, self.identification_id())?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(CallbackOperationResponse, "callbackResponse", [operation_id: OperationId, identification_id: IdentificationId]);

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
        let exp_xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?><methodResponse><params><param><value><struct><member><name>operationId</name><value><i4>6121</i4></value></member><member><name>identificationId</name><value><i4>4</i4></value></member></struct></value></param></params></methodResponse>"#;
        let exp_res =
            XfsMethodResponseStruct::new(XfsMethodResponse::new_params([XfsParam::create(
                CallbackOperationResponse::create(6121, 4).into(),
            )]));

        assert_eq!(xfs::to_iso_string(&exp_res)?.as_str(), exp_xml);
        assert_eq!(xfs::from_str::<XfsMethodResponseStruct>(exp_xml)?, exp_res);

        Ok(())
    }
}

