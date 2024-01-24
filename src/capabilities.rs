use std::fmt;

use crate::status::{CdrPosition, CdrPositionCapabilitiesList};

mod anti_fishing_level;
mod cdr_type;
mod euro_art6_capability;
mod reporting_mode;
mod secured_comm_level;
mod self_test_mode;

pub use anti_fishing_level::*;
pub use cdr_type::*;
pub use euro_art6_capability::*;
pub use reporting_mode::*;
pub use secured_comm_level::*;
pub use self_test_mode::*;

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
pub struct Capabilities {
    /// This specifies whether cash will be automatically presented to the user on execution of a dispense (autoPresent set to TRUE), or whether the cash will only be transported to the Bundler.
    /// In the latter case, a [present](crate::cash::present) command will need to be issued following the [dispense](crate::cash::dispense) command.
    /// Default value is FALSE.
    pub auto_present: bool,
    ///  Type of device. Always [CdrType::ATM](CdrType) for the BNR.
    pub cd_type: CdrType,
    /// Reserved for future use.
    pub euro_art6_capability: EuroArt6Capability,
    /// Reserved for future use.
    pub trusted_user: bool,
    /// Maximum number of bills to be accepted by one command. Allways 1 for the BNR.
    pub max_in_bills: u32,
    /// Maximum number of bills to be dispensed by one command. Equals to 20 in BNA6 configuration, otherwise equals to 15.
    pub max_out_bills: u32,
    /// The shutter be accessed by commands. Allways FALSE in the BNR.
    pub shutter_cmd: bool,
    /// The cash dispenser can retract presented bills. Always TRUE in the BNR since FW v1.3.0, FALSE with previous versions.
    pub retract: bool,
    /// This device supports a safe door command. Always FALSE in the BNR.
    pub safe_door_cmd: bool,
    /// The service can handle a cash box. Always TRUE in the BNR.
    pub cash_box: bool,
    /// Can the BNR be refilled by placing bills of the same size on the stack of the Loader. Always TRUE in the BNR.
    pub refill: bool,
    /// The device can dispense cash. Equals to FALSE in BNA6 configuration, otherwise equals to TRUE.
    pub dispense: bool,
    /// The device can deposit cash. Always TRUE in the BNR.
    pub deposit: bool,
    /// The device has a temporary storage before presenting bills. Always TRUE in the BNR.
    pub intermediate_stacker: bool,
    /// The device has a bills taken sensor. Always TRUE in the BNR.
    pub bill_taken_sensor: bool,
    /// The device supports an escrow. Always TRUE in the BNR.
    pub escrow: bool,
    /// Specifies the maximum number of bills on the escrow. Equals to 20 in BNA6 configuration, otherwise equals to 15.
    pub escrow_size: u32,
    /// The device supports a detector to verify accepted cash. Always TRUE in the BNR.
    pub detector: bool,
    /// Specifies the default output position to rollback cash. Always [CdrPosition::Bottom](CdrPosition::Bottom) in the BNR.
    pub default_rollback_position: CdrPosition,
    /// Specifies the capabilities of each position supported by the device. Please refer to [CdrPositionCapabilities] for default values.
    pub position_capabilities_list: CdrPositionCapabilitiesList,
    /// Allows to choose when the BNR can perform the self tests. Default value is [Auto](SelfTestMode::Auto) (recommended).
    pub self_test_mode: SelfTestMode,
    /// Recognition sensor type identification. Always ‘B’ in the BNR.
    pub recognition_sensor_type: u8,
    /// Sensitivity level of string detection at Inlet.
    /// Default value is [Normal](AntiFishingLevel::Normal).
    pub anti_fishing_level: AntiFishingLevel,
    /// Allows to use USB Front interface to communicate with the BNR. Default value is TRUE.
    pub allow_usb_front_switch: bool,
    /// Specifies the kind of report generated on failure detection with no bill transported. Default value is [Normal](ReportingMode::Normal).
    pub reporting_mode: ReportingMode,
    /// Specifies whether real max USB line consumption is reported on usb configuration descriptor instead of 0mA. Default value is FALSE.
    pub report_usb_consumption: bool,
    /// Specifies whether bill will be automatically retracted to positioner when jam occurred during bill presenting at inlet or outlet. Default value is FALSE.
    pub auto_retract: bool,
    /// Specifies whether measured but unknown or inhibited notes are rejected via the BNR’s Outlet instead of the Inlet. Default value is FALSE.
    pub reject_via_outlet: bool,
    /// Indicates the security level in communication between Host and Bnr. Defaut value is [Level1](SecuredCommLevel::Level1).
    pub secured_comm_level: SecuredCommLevel,
}

impl Capabilities {
    /// Creates a new [Capabilities].
    pub const fn new() -> Self {
        Self {
            auto_present: false,
            cd_type: CdrType::Atm,
            euro_art6_capability: EuroArt6Capability::new(),
            trusted_user: false,
            max_in_bills: 1,
            max_out_bills: 15,
            shutter_cmd: false,
            retract: true,
            safe_door_cmd: false,
            cash_box: true,
            refill: true,
            dispense: true,
            deposit: true,
            intermediate_stacker: true,
            bill_taken_sensor: true,
            escrow: true,
            escrow_size: 15,
            detector: true,
            default_rollback_position: CdrPosition::Bottom,
            position_capabilities_list: CdrPositionCapabilitiesList::new(),
            self_test_mode: SelfTestMode::new(),
            recognition_sensor_type: b'b',
            anti_fishing_level: AntiFishingLevel::new(),
            allow_usb_front_switch: true,
            reporting_mode: ReportingMode::new(),
            report_usb_consumption: false,
            auto_retract: false,
            reject_via_outlet: false,
            secured_comm_level: SecuredCommLevel::new(),
        }
    }
}

impl From<&bnr_sys::XfsCapabilities> for Capabilities {
    fn from(val: &bnr_sys::XfsCapabilities) -> Self {
        Self {
            auto_present: val.autoPresent != 0,
            cd_type: val.cdType.into(),
            euro_art6_capability: val.euroArt6Capability.into(),
            trusted_user: val.trustedUser != 0,
            max_in_bills: val.maxInBills,
            max_out_bills: val.maxOutBills,
            shutter_cmd: val.shutterCmd != 0,
            retract: val.retract != 0,
            safe_door_cmd: val.safeDoorCmd != 0,
            cash_box: val.cashBox != 0,
            refill: val.refill != 0,
            dispense: val.dispense != 0,
            deposit: val.deposit != 0,
            intermediate_stacker: val.intermediateStacker != 0,
            bill_taken_sensor: val.billTakenSensor != 0,
            escrow: val.escrow != 0,
            escrow_size: val.escrowSize,
            detector: val.detector != 0,
            default_rollback_position: val.defaultRollbackPosition.into(),
            position_capabilities_list: val.positionCapabilitiesList.into(),
            self_test_mode: val.selfTestMode.into(),
            recognition_sensor_type: val.recognitionSensorType as u8,
            anti_fishing_level: val.antiFishingLevel.into(),
            allow_usb_front_switch: val.allowUsbFrontSwitch != 0,
            reporting_mode: val.reportingMode.into(),
            report_usb_consumption: val.reportUsbConsumption != 0,
            auto_retract: val.autoRetract != 0,
            reject_via_outlet: val.rejectViaOutlet != 0,
            secured_comm_level: val.securedCommLevel.into(),
        }
    }
}

impl From<bnr_sys::XfsCapabilities> for Capabilities {
    fn from(val: bnr_sys::XfsCapabilities) -> Self {
        (&val).into()
    }
}

impl From<&Capabilities> for bnr_sys::XfsCapabilities {
    fn from(val: &Capabilities) -> Self {
        Self {
            autoPresent: val.auto_present.into(),
            cdType: val.cd_type.into(),
            euroArt6Capability: val.euro_art6_capability.into(),
            trustedUser: val.trusted_user.into(),
            maxInBills: val.max_in_bills,
            maxOutBills: val.max_out_bills,
            shutterCmd: val.shutter_cmd.into(),
            retract: val.retract.into(),
            safeDoorCmd: val.safe_door_cmd.into(),
            cashBox: val.cash_box.into(),
            refill: val.refill.into(),
            dispense: val.dispense.into(),
            deposit: val.deposit.into(),
            intermediateStacker: val.intermediate_stacker.into(),
            billTakenSensor: val.bill_taken_sensor.into(),
            escrow: val.escrow.into(),
            escrowSize: val.escrow_size,
            detector: val.detector.into(),
            defaultRollbackPosition: val.default_rollback_position.into(),
            positionCapabilitiesList: val.position_capabilities_list.into(),
            selfTestMode: val.self_test_mode.into(),
            recognitionSensorType: val.recognition_sensor_type as i8,
            antiFishingLevel: val.anti_fishing_level.into(),
            allowUsbFrontSwitch: val.allow_usb_front_switch.into(),
            reportingMode: val.reporting_mode.into(),
            reportUsbConsumption: val.report_usb_consumption.into(),
            autoRetract: val.auto_retract.into(),
            rejectViaOutlet: val.reject_via_outlet.into(),
            securedCommLevel: val.secured_comm_level.into(),
        }
    }
}

impl From<Capabilities> for bnr_sys::XfsCapabilities {
    fn from(val: Capabilities) -> Self {
        (&val).into()
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
        write!(f, r#""bill_taken_sensor":{},"#, self.bill_taken_sensor)?;
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
        write!(f, r#""auto_retract":{},"#, self.auto_retract)?;
        write!(f, r#""reject_via_outlet":{},"#, self.reject_via_outlet)?;
        write!(f, r#""secured_comm_level":{}"#, self.secured_comm_level)?;
        write!(f, "}}")
    }
}
