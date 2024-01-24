use std::fmt;

use crate::{create_xfs_i4, impl_xfs_struct};

create_xfs_i4!(
    IdentificationId,
    "identificationId",
    r#"Represents the identification ID for a particular callback operation.

This uniquely identifies the specific call that was made."#
);

create_xfs_i4!(
    OperationId,
    "operationId",
    r#"Represents the operation ID for a particular callback operation.

This uniquely identifies the specific operation method of a call that was made."#
);

/// Represents a response to a callback call made by the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallbackResponse {
    id: IdentificationId,
    op_id: OperationId,
}

impl CallbackResponse {
    /// Creates a new [CallbackResponse].
    pub const fn new() -> Self {
        Self {
            id: IdentificationId::new(),
            op_id: OperationId::new(),
        }
    }

    /// Creates a new [CallbackResponse] from the provided parameters.
    pub const fn create(id: i32, op_id: i32) -> Self {
        Self {
            id: IdentificationId::create(id as u32),
            op_id: OperationId::create(op_id as u32),
        }
    }

    /// Gets the [IdentificationId] value.
    pub const fn id(&self) -> u32 {
        self.id.inner()
    }

    /// Sets the [IdentificationId] value.
    pub fn set_id(&mut self, val: i32) {
        self.id.set_inner(val as u32)
    }

    /// Builder function that sets the [IdentificationId] value.
    pub fn with_id(mut self, val: i32) -> Self {
        self.set_id(val);
        self
    }

    /// Gets the [OperationId] value.
    pub const fn operation_id(&self) -> u32 {
        self.op_id.inner()
    }

    /// Sets the [OperationId] value.
    pub fn set_operation_id(&mut self, val: i32) {
        self.op_id.set_inner(val as u32)
    }

    /// Builder function that sets the [OperationId] value.
    pub fn with_operation_id(mut self, val: i32) -> Self {
        self.set_operation_id(val);
        self
    }
}

impl fmt::Display for CallbackResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""identification_id": {}, "#, self.id())?;
        write!(f, r#""operation_id": {}"#, self.operation_id())?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(CallbackResponse, "callbackResponse", [id: IdentificationId, op_id: OperationId]);
