use crate::xfs::method_response::XfsMethodResponse;
use crate::{create_xfs_array, create_xfs_struct};
use crate::{
    BillErrorCount, BillJamCount, BillTooLongInBottomTransportBwCount, BillTooLongInSpineFwCount,
    BillTooShortInBottomTransportBwCount, BillTooShortInSpineFwCount, BottomTransportCount,
    BundlerCount, ConfigurationErrorCount, EnvironmentErrorCount, Error, HardwareFailureCount,
    HardwareFailureWithBillStoppedCount, IncompatibleSoftwareCount, InterfaceCount,
    MissingModuleCount, ModuleCount, OperationalDegradedCount, PositionerCount,
    RecognitionSystemCount, ResetWithCoverOpenCount, ResetWithInterlockOpenCount, Result,
    SpineCount, TransportErrorCount, UnnamedCount,
};

const BILL_ENDING_COUNTERS_LEN: usize = 4;
const INCIDENT_START_COUNTERS_LEN: usize = 25;

const UNNAMED_COUNT_DEFAULT: UnnamedCount = UnnamedCount::new();

const MODULE_COUNT_LIST_LEN: usize = 10;
const MODULE_COUNT_DEFAULT: ModuleCount = ModuleCount::new();

const INTERFACE_COUNT_LIST_LEN: usize = 10;
const INTERFACE_COUNT_DEFAULT: InterfaceCount = InterfaceCount::new();

create_xfs_struct!(
    MainModuleSectionCounters,
    "mainModuleSectionCounters",
    [
        positioner_count: PositionerCount,
        recognition_system_count: RecognitionSystemCount,
        bottom_transport_count: BottomTransportCount,
        bundler_count: BundlerCount
    ],
    "Represents the main module section counters."
);

create_xfs_struct!(
    IncidentStartSectionCounters,
    "incidentStartSectionCounters",
    [
        positioner_count: PositionerCount,
        recognition_system_count: RecognitionSystemCount,
        bottom_transport_count: BottomTransportCount,
        bundler_count: BundlerCount,
        spine_count: SpineCount,
        module_counts: ModuleCountList,
        interface_counts: InterfaceCountList
    ],
    "Represents the incident start section counters."
);

create_xfs_array!(
    ModuleCountList,
    "moduleCountList",
    ModuleCount,
    MODULE_COUNT_LIST_LEN,
    MODULE_COUNT_DEFAULT,
    "Represents a list of module count items."
);

create_xfs_array!(
    InterfaceCountList,
    "interfaceCountList",
    InterfaceCount,
    INTERFACE_COUNT_LIST_LEN,
    INTERFACE_COUNT_DEFAULT,
    "Represents a list of interface count items."
);

create_xfs_array!(
    BillEndingInMMSectionCounters,
    "billEndingInMMSectionCounters",
    UnnamedCount,
    BILL_ENDING_COUNTERS_LEN,
    UNNAMED_COUNT_DEFAULT,
    "Represents a list of bill ending in main module section count items."
);

create_xfs_array!(
    IncidentStartSectionCountersList,
    "incidentStartSectionCounters",
    UnnamedCount,
    INCIDENT_START_COUNTERS_LEN,
    UNNAMED_COUNT_DEFAULT,
    "Represents a list of incident start section count items."
);

create_xfs_struct!(
    PerSectionHistoryInternal,
    "perSectionHistory",
    [
        bill_ending_in_mm_section_counters: BillEndingInMMSectionCounters,
        incident_start_section: IncidentStartSectionCountersList
    ],
    "Represents the internal XFS representation of per-section history."
);

create_xfs_struct!(
    PerSectionHistory,
    "perSectionHistory",
    [
        bill_ending_in_mm_section_counters: MainModuleSectionCounters,
        incident_start_section: IncidentStartSectionCounters
    ],
    "Represents the internal XFS representation of per-section history."
);

impl From<&BillEndingInMMSectionCounters> for MainModuleSectionCounters {
    fn from(val: &BillEndingInMMSectionCounters) -> Self {
        let items = val.items();
        Self {
            positioner_count: items
                .first()
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            recognition_system_count: items
                .get(1)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            bottom_transport_count: items
                .get(2)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            bundler_count: items
                .get(3)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
        }
    }
}

impl From<BillEndingInMMSectionCounters> for MainModuleSectionCounters {
    fn from(val: BillEndingInMMSectionCounters) -> Self {
        (&val).into()
    }
}

impl From<&IncidentStartSectionCountersList> for IncidentStartSectionCounters {
    fn from(val: &IncidentStartSectionCountersList) -> Self {
        let items = val.items();
        Self {
            positioner_count: items
                .first()
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            recognition_system_count: items
                .get(1)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            bottom_transport_count: items
                .get(2)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            bundler_count: items
                .get(3)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            spine_count: items
                .get(4)
                .cloned()
                .unwrap_or(UnnamedCount::new())
                .inner()
                .into(),
            module_counts: ModuleCountList::new().with_items(
                items
                    .iter()
                    .skip(5)
                    .take(10)
                    .map(|i| ModuleCount::from(i.inner()))
                    .collect::<Vec<ModuleCount>>()
                    .as_ref(),
            ),
            interface_counts: InterfaceCountList::new().with_items(
                items
                    .iter()
                    .skip(15)
                    .take(10)
                    .map(|i| InterfaceCount::from(i.inner()))
                    .collect::<Vec<InterfaceCount>>()
                    .as_ref(),
            ),
        }
    }
}

impl From<IncidentStartSectionCountersList> for IncidentStartSectionCounters {
    fn from(val: IncidentStartSectionCountersList) -> Self {
        (&val).into()
    }
}

impl From<&PerSectionHistoryInternal> for PerSectionHistory {
    fn from(val: &PerSectionHistoryInternal) -> Self {
        Self {
            bill_ending_in_mm_section_counters: val.bill_ending_in_mm_section_counters().into(),
            incident_start_section: val.incident_start_section().into(),
        }
    }
}

impl From<PerSectionHistoryInternal> for PerSectionHistory {
    fn from(val: PerSectionHistoryInternal) -> Self {
        (&val).into()
    }
}

create_xfs_struct!(
    SystemFailureHistory,
    "systemFailureHistory",
    [
        hardware_failure_count: HardwareFailureCount,
        hardware_failure_with_bill_stopped_count: HardwareFailureWithBillStoppedCount,
        operational_degraded_count: OperationalDegradedCount,
        bill_jam_count: BillJamCount,
        environment_error_count: EnvironmentErrorCount,
        bill_error_count: BillErrorCount,
        transport_error_count: TransportErrorCount,
        bill_too_short_in_bottom_transport_bw_count: BillTooShortInBottomTransportBwCount,
        bill_too_long_in_bottom_transport_bw_count: BillTooLongInBottomTransportBwCount,
        bill_too_short_in_spine_fw_count: BillTooShortInSpineFwCount,
        bill_too_long_in_spine_fw_count: BillTooLongInSpineFwCount,
        missing_module_count: MissingModuleCount,
        configuration_error_count: ConfigurationErrorCount,
        incompatible_software_count: IncompatibleSoftwareCount,
        reset_with_cover_open_count: ResetWithCoverOpenCount,
        reset_with_interlock_open_count: ResetWithInterlockOpenCount,
        per_section_history: PerSectionHistory
    ],
    "Represents the history of system failure events."
);

create_xfs_struct!(
    SystemFailureHistoryInternal,
    "systemFailureHistory",
    [
        hardware_failure_count: HardwareFailureCount,
        hardware_failure_with_bill_stopped_count: HardwareFailureWithBillStoppedCount,
        operational_degraded_count: OperationalDegradedCount,
        bill_jam_count: BillJamCount,
        environment_error_count: EnvironmentErrorCount,
        bill_error_count: BillErrorCount,
        transport_error_count: TransportErrorCount,
        bill_too_short_in_bottom_transport_bw_count: BillTooShortInBottomTransportBwCount,
        bill_too_long_in_bottom_transport_bw_count: BillTooLongInBottomTransportBwCount,
        bill_too_short_in_spine_fw_count: BillTooShortInSpineFwCount,
        bill_too_long_in_spine_fw_count: BillTooLongInSpineFwCount,
        missing_module_count: MissingModuleCount,
        configuration_error_count: ConfigurationErrorCount,
        incompatible_software_count: IncompatibleSoftwareCount,
        reset_with_cover_open_count: ResetWithCoverOpenCount,
        reset_with_interlock_open_count: ResetWithInterlockOpenCount,
        per_section_history: PerSectionHistoryInternal
    ],
    "Alternative internal representation the history of system failure events."
);

impl From<&SystemFailureHistoryInternal> for SystemFailureHistory {
    fn from(val: &SystemFailureHistoryInternal) -> Self {
        Self {
            hardware_failure_count: val.hardware_failure_count,
            hardware_failure_with_bill_stopped_count: val.hardware_failure_with_bill_stopped_count,
            operational_degraded_count: val.operational_degraded_count,
            bill_jam_count: val.bill_jam_count,
            environment_error_count: val.environment_error_count,
            bill_error_count: val.bill_error_count,
            transport_error_count: val.transport_error_count,
            bill_too_short_in_bottom_transport_bw_count: val
                .bill_too_short_in_bottom_transport_bw_count,
            bill_too_long_in_bottom_transport_bw_count: val
                .bill_too_long_in_bottom_transport_bw_count,
            bill_too_short_in_spine_fw_count: val.bill_too_short_in_spine_fw_count,
            bill_too_long_in_spine_fw_count: val.bill_too_long_in_spine_fw_count,
            missing_module_count: val.missing_module_count,
            configuration_error_count: val.configuration_error_count,
            incompatible_software_count: val.incompatible_software_count,
            reset_with_cover_open_count: val.reset_with_cover_open_count,
            reset_with_interlock_open_count: val.reset_with_interlock_open_count,
            per_section_history: val.per_section_history().into(),
        }
    }
}

impl From<SystemFailureHistoryInternal> for SystemFailureHistory {
    fn from(val: SystemFailureHistoryInternal) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMethodResponse> for SystemFailureHistory {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        let xfs_struct = val
            .as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected SystemFailureHistory XfsMethodResponse, have: {val}"
            )))?
            .value();

        match SystemFailureHistory::try_from(xfs_struct) {
            Ok(ret) => Ok(ret),
            Err(err) => {
                log::warn!("Error parsing SystemFailureHistory: {err}");
                log::debug!("Trying SystemFailureHistoryInternal representation");

                Ok(SystemFailureHistoryInternal::try_from(xfs_struct)?.into())
            }
        }
    }
}

impl TryFrom<XfsMethodResponse> for SystemFailureHistory {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
