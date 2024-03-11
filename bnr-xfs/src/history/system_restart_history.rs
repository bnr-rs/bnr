use crate::create_xfs_struct;
use crate::xfs::method_response::XfsMethodResponse;
use crate::{
    BillIntakeCoverCount, CashModulesLockCount, Error, InternalResetCount,
    InternalResetWithBillStoppedCount, PowerDownWithBillStoppedCount, PowerUpCount,
    RecognitionSensorCoverCount, Result, SpineCoverCount, SystemOpeningCount, WithBillStoppedCount,
};

create_xfs_struct!(
    SystemOpeningDetails,
    "systemOpeningDetails",
    [
        with_bill_stopped_count: WithBillStoppedCount,
        cash_modules_lock_count: CashModulesLockCount,
        bill_intake_cover_count: BillIntakeCoverCount,
        recognition_sensor_cover_count: RecognitionSensorCoverCount,
        spine_cover_count: SpineCoverCount
    ],
    "Represents the system opening details."
);

create_xfs_struct!(
    SystemRestartHistory,
    "systemRestartHistory",
    [
        power_up_count: PowerUpCount,
        power_down_with_bill_stopped_count: PowerDownWithBillStoppedCount,
        internal_reset_count: InternalResetCount,
        internal_reset_with_bill_stopped_count: InternalResetWithBillStoppedCount,
        system_opening_count: SystemOpeningCount,
        system_opening_details: SystemOpeningDetails
    ],
    "Represents the history of system restart events."
);

impl TryFrom<&XfsMethodResponse> for SystemRestartHistory {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected SystemRestartHistory XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for SystemRestartHistory {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
