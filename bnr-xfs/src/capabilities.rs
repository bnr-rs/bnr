use std::fmt;

use crate::status::CdrPositionCapabilitiesList;
use crate::xfs::{
    method_response::XfsMethodResponse,
    params::{XfsParam, XfsParams},
};
use crate::{create_xfs_bool, create_xfs_i4, impl_xfs_struct, Error, Result};

mod anti_fishing_level;
mod cdr_type;
mod default_rollback_position;
mod euro_art6_capability;
mod recognition_sensor_type;
mod reporting_mode;
mod secured_comm_level;
mod self_test_mode;

pub use anti_fishing_level::*;
pub use cdr_type::*;
pub use default_rollback_position::*;
pub use euro_art6_capability::*;
pub use recognition_sensor_type::*;
pub use reporting_mode::*;
pub use secured_comm_level::*;
pub use self_test_mode::*;

create_xfs_bool!(
    AutoPresent,
    "autoPresent",
    r#"This specifies whether cash will be automatically presented to the user on execution of a dispense (autoPresent set to TRUE), or whether the cash will only be transported to the Bundler.

In the latter case, a [present](crate::cash::present) command will need to be issued following the [dispense](crate::cash::dispense) command.

Default value is FALSE.
"#
);
create_xfs_bool!(TrustedUser, "trustedUser", "Reserved for future use.");
create_xfs_i4!(
    MaxInBills,
    "maxInBills",
    "Maximum number of bills to be accepted by one command. Allways 1 for the BNR."
);
create_xfs_i4!(MaxOutBills, "maxOutBills", "Maximum number of bills to be dispensed by one command. Equals to 20 in BNA6 configuration, otherwise equals to 15.");
create_xfs_bool!(
    ShutterCmd,
    "shutterCmd",
    "The shutter be accessed by commands. Allways FALSE in the BNR."
);
create_xfs_bool!(Retract, "retract", "The cash dispenser can retract presented bills. Always TRUE in the BNR since FW v1.3.0, FALSE with previous versions.");
create_xfs_bool!(
    SafeDoorCmd,
    "safeDoorCmd",
    "This device supports a safe door command. Always FALSE in the BNR."
);
create_xfs_bool!(
    CashBox,
    "cashBox",
    "The service can handle a cash box. Always TRUE in the BNR."
);
create_xfs_bool!(Refill, "refill", "Can the BNR be refilled by placing bills of the same size on the stack of the Loader. Always TRUE in the BNR.");
create_xfs_bool!(Dispense, "dispense", "The device can dispense cash. Equals to FALSE in BNA6 configuration, otherwise equals to TRUE.");
create_xfs_bool!(
    Deposit,
    "deposit",
    "The device can deposit cash. Always TRUE in the BNR."
);
create_xfs_bool!(
    IntermediateStacker,
    "intermediateStacker",
    "The device has a temporary storage before presenting bills. Always TRUE in the BNR."
);
create_xfs_bool!(
    BillsTakenSensor,
    "billsTakenSensor",
    "The device has a bills taken sensor. Always TRUE in the BNR."
);
create_xfs_bool!(
    Escrow,
    "escrow",
    "The device supports an escrow. Always TRUE in the BNR."
);
create_xfs_i4!(EscrowSize, "escrowSize", "Specifies the maximum number of bills on the escrow. Equals to 20 in BNA6 configuration, otherwise equals to 15.");
create_xfs_bool!(
    Detector,
    "detector",
    "The device supports a detector to verify accepted cash. Always TRUE in the BNR."
);
create_xfs_bool!(
    AllowUsbFrontSwitch,
    "allowUsbFrontSwitch",
    "Allows to use USB Front interface to communicate with the BNR. Default value is TRUE."
);
create_xfs_bool!(ReportUsbConsumption, "reportUsbConsumption", "Specifies whether real max USB line consumption is reported on usb configuration descriptor instead of 0mA. Default value is FALSE.");
create_xfs_bool!(AutoRetractAtInlet, "autoRetractAtInlet", "Specifies whether bill will be automatically retracted to positioner when jam occurred during bill presenting at inlet or outlet. Default value is FALSE.");
create_xfs_bool!(RejectViaOutlet, "rejectViaOutlet", "Specifies whether measured but unknown or inhibited notes are rejected via the BNR’s Outlet instead of the Inlet. Default value is FALSE.");

/// Describes the BNR capabilities.
///
/// By default, capabilities are read-only properties, the following are writable:
///
/// - `auto_present`
/// - `self_test_mode`
/// - `anti_fishing_level`
/// - `allow_usb_front_switch`
/// - `reporting_mode`
/// - `report_usb_consumption`
/// - `auto_retract`
/// - `reject_via_outlet`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Capabilities {
    pub auto_present: AutoPresent,
    ///  Type of device. Always [CdrType::ATM](CdrType) for the BNR.
    pub cd_type: CdrType,
    /// Reserved for future use.
    pub euro_art6_capability: EuroArt6Capability,
    pub trusted_user: TrustedUser,
    pub max_in_bills: MaxInBills,
    pub max_out_bills: MaxOutBills,
    pub shutter_cmd: ShutterCmd,
    pub retract: Retract,
    pub safe_door_cmd: SafeDoorCmd,
    pub cash_box: CashBox,
    pub refill: Refill,
    pub dispense: Dispense,
    pub deposit: Deposit,
    pub intermediate_stacker: IntermediateStacker,
    pub bills_taken_sensor: BillsTakenSensor,
    pub escrow: Escrow,
    pub escrow_size: EscrowSize,
    pub detector: Detector,
    /// Specifies the default output position to rollback cash. Always [CdrPosition::Bottom](CdrPosition::Bottom) in the BNR.
    pub default_rollback_position: DefaultRollbackPosition,
    /// Specifies the capabilities of each position supported by the device. Please refer to [CdrPositionCapabilities] for default values.
    pub position_capabilities_list: CdrPositionCapabilitiesList,
    /// Allows to choose when the BNR can perform the self tests. Default value is [Auto](SelfTestMode::Auto) (recommended).
    pub self_test_mode: SelfTestMode,
    pub recognition_sensor_type: RecognitionSensorType,
    /// Sensitivity level of string detection at Inlet.
    /// Default value is [Normal](AntiFishingLevel::Normal).
    pub anti_fishing_level: AntiFishingLevel,
    pub allow_usb_front_switch: AllowUsbFrontSwitch,
    /// Specifies the kind of report generated on failure detection with no bill transported. Default value is [Normal](ReportingMode::Normal).
    pub reporting_mode: ReportingMode,
    pub report_usb_consumption: ReportUsbConsumption,
    pub auto_retract_at_inlet: AutoRetractAtInlet,
    pub reject_via_outlet: RejectViaOutlet,
    /// Indicates the security level in communication between Host and Bnr. Defaut value is [Level1](SecuredCommLevel::Level1).
    pub secured_comm_level: SecuredCommLevel,
}

impl Capabilities {
    /// Creates a new [Capabilities].
    pub const fn new() -> Self {
        Self {
            auto_present: AutoPresent::new(),
            cd_type: CdrType::Atm,
            euro_art6_capability: EuroArt6Capability::new(),
            trusted_user: TrustedUser::new(),
            max_in_bills: MaxInBills::create(1),
            max_out_bills: MaxOutBills::create(15),
            shutter_cmd: ShutterCmd::new(),
            retract: Retract::create(true),
            safe_door_cmd: SafeDoorCmd::new(),
            cash_box: CashBox::create(true),
            refill: Refill::create(true),
            dispense: Dispense::create(true),
            deposit: Deposit::create(true),
            intermediate_stacker: IntermediateStacker::create(true),
            bills_taken_sensor: BillsTakenSensor::create(true),
            escrow: Escrow::create(true),
            escrow_size: EscrowSize::create(15),
            detector: Detector::create(true),
            default_rollback_position: DefaultRollbackPosition::Bottom,
            position_capabilities_list: CdrPositionCapabilitiesList::new(),
            self_test_mode: SelfTestMode::new(),
            recognition_sensor_type: RecognitionSensorType::create(b'B'),
            anti_fishing_level: AntiFishingLevel::new(),
            allow_usb_front_switch: AllowUsbFrontSwitch::create(true),
            reporting_mode: ReportingMode::new(),
            report_usb_consumption: ReportUsbConsumption::new(),
            auto_retract_at_inlet: AutoRetractAtInlet::new(),
            reject_via_outlet: RejectViaOutlet::new(),
            secured_comm_level: SecuredCommLevel::new(),
        }
    }
}

impl From<&Capabilities> for XfsParam {
    fn from(val: &Capabilities) -> Self {
        Self::create(val.into())
    }
}

impl From<Capabilities> for XfsParam {
    fn from(val: Capabilities) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsParam> for Capabilities {
    type Error = Error;

    fn try_from(val: &XfsParam) -> Result<Self> {
        val.value().try_into()
    }
}

impl TryFrom<XfsParam> for Capabilities {
    type Error = Error;

    fn try_from(val: XfsParam) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsParams> for Capabilities {
    type Error = Error;

    fn try_from(val: &XfsParams) -> Result<Self> {
        val.params()
            .get(0)
            .ok_or(Error::Xfs(format!("Missing param: {val}")))?
            .inner()
            .value()
            .try_into()
    }
}

impl TryFrom<XfsParams> for Capabilities {
    type Error = Error;

    fn try_from(val: XfsParams) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsMethodResponse> for Capabilities {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?.try_into()
    }
}

impl TryFrom<XfsMethodResponse> for Capabilities {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""auto_present":{},"#, self.auto_present)?;
        write!(f, r#""cd_type":{},"#, self.cd_type)?;
        write!(
            f,
            r#""euro_art6_capability":{},"#,
            self.euro_art6_capability
        )?;
        write!(f, r#""trusted_user":{},"#, self.trusted_user)?;
        write!(f, r#""max_in_bills":{},"#, self.max_in_bills)?;
        write!(f, r#""max_out_bills":{},"#, self.max_out_bills)?;
        write!(f, r#""shutter_cmd":{},"#, self.shutter_cmd)?;
        write!(f, r#""retract":{},"#, self.retract)?;
        write!(f, r#""safe_door_cmd":{},"#, self.safe_door_cmd)?;
        write!(f, r#""cash_box":{},"#, self.cash_box)?;
        write!(f, r#""refill":{},"#, self.refill)?;
        write!(f, r#""dispense":{},"#, self.dispense)?;
        write!(f, r#""deposit":{},"#, self.deposit)?;
        write!(
            f,
            r#""intermediate_stacker":{},"#,
            self.intermediate_stacker
        )?;
        write!(f, r#""bills_taken_sensor":{},"#, self.bills_taken_sensor)?;
        write!(f, r#""escrow":{},"#, self.escrow)?;
        write!(f, r#""escrow_size":{},"#, self.escrow_size)?;
        write!(f, r#""detector":{},"#, self.detector)?;
        write!(
            f,
            r#""default_rollback_position":{},"#,
            self.default_rollback_position
        )?;
        write!(
            f,
            r#""position_capabilities_list":{},"#,
            self.position_capabilities_list
        )?;
        write!(f, r#""self_test_mode":{},"#, self.self_test_mode)?;
        write!(
            f,
            r#""recognition_sensor_type":{},"#,
            self.recognition_sensor_type
        )?;
        write!(f, r#""anti_fishing_level":{},"#, self.anti_fishing_level)?;
        write!(
            f,
            r#""allow_usb_front_switch":{},"#,
            self.allow_usb_front_switch
        )?;
        write!(f, r#""reporting_mode":{},"#, self.reporting_mode)?;
        write!(
            f,
            r#""report_usb_consumption":{},"#,
            self.report_usb_consumption
        )?;
        write!(f, r#""auto_retract_at_inlet":{},"#, self.auto_retract_at_inlet)?;
        write!(f, r#""reject_via_outlet":{},"#, self.reject_via_outlet)?;
        write!(f, r#""secured_comm_level":{}"#, self.secured_comm_level)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    Capabilities,
    "capabilities",
    [
        auto_present: AutoPresent,
        cd_type: CdrType,
        euro_art6_capability: EuroArt6Capability,
        trusted_user: TrustedUser,
        max_in_bills: MaxInBills,
        max_out_bills: MaxOutBills,
        shutter_cmd: ShutterCmd,
        retract: Retract,
        safe_door_cmd: SafeDoorCmd,
        cash_box: CashBox,
        refill: Refill,
        dispense: Dispense,
        deposit: Deposit,
        intermediate_stacker: IntermediateStacker,
        bills_taken_sensor: BillsTakenSensor,
        escrow: Escrow,
        escrow_size: EscrowSize,
        detector: Detector,
        default_rollback_position: DefaultRollbackPosition,
        position_capabilities_list: CdrPositionCapabilitiesList,
        self_test_mode: SelfTestMode,
        recognition_sensor_type: RecognitionSensorType,
        anti_fishing_level: AntiFishingLevel,
        allow_usb_front_switch: AllowUsbFrontSwitch,
        reporting_mode: ReportingMode,
        report_usb_consumption: ReportUsbConsumption,
        auto_retract_at_inlet: AutoRetractAtInlet,
        reject_via_outlet: RejectViaOutlet,
        secured_comm_level: SecuredCommLevel
    ]
);
