use std::fmt;

use crate::status::{CdrPosition, CdrPositionCapabilitiesList};
use crate::xfs::{
    method_response::XfsMethodResponse,
    params::{XfsParam, XfsParams},
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::{Error, Result};

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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
            recognition_sensor_type: b'B',
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

impl From<&Capabilities> for XfsValue {
    fn from(val: &Capabilities) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<Capabilities> for XfsValue {
    fn from(val: Capabilities) -> Self {
        (&val).into()
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

impl From<&Capabilities> for XfsStruct {
    fn from(val: &Capabilities) -> Self {
        Self::create([
            XfsMember::create(
                "autoPresent",
                XfsValue::new().with_boolean(val.auto_present as u8),
            ),
            val.anti_fishing_level.into(),
            XfsMember::create(
                "allowUsbFrontSwitch",
                XfsValue::new().with_boolean(val.allow_usb_front_switch as u8),
            ),
            val.cd_type.into(),
            val.euro_art6_capability.into(),
            XfsMember::create(
                "trustedUser",
                XfsValue::new().with_boolean(val.auto_present as u8),
            ),
            XfsMember::create(
                "maxInBills",
                XfsValue::new().with_i4(val.max_in_bills as i32),
            ),
            XfsMember::create(
                "maxOutBills",
                XfsValue::new().with_i4(val.max_out_bills as i32),
            ),
            XfsMember::create(
                "shutterCmd",
                XfsValue::new().with_boolean(val.shutter_cmd as u8),
            ),
            XfsMember::create("retract", XfsValue::new().with_boolean(val.retract as u8)),
            XfsMember::create(
                "safeDoorCmd",
                XfsValue::new().with_boolean(val.safe_door_cmd as u8),
            ),
            XfsMember::create("cashBox", XfsValue::new().with_boolean(val.cash_box as u8)),
            XfsMember::create("refill", XfsValue::new().with_boolean(val.refill as u8)),
            XfsMember::create("dispense", XfsValue::new().with_boolean(val.dispense as u8)),
            XfsMember::create("deposit", XfsValue::new().with_boolean(val.deposit as u8)),
            XfsMember::create(
                "intermediateStacker",
                XfsValue::new().with_boolean(val.intermediate_stacker as u8),
            ),
            XfsMember::create(
                "billsTakenSensor",
                XfsValue::new().with_boolean(val.bill_taken_sensor as u8),
            ),
            XfsMember::create("escrow", XfsValue::new().with_boolean(val.escrow as u8)),
            XfsMember::create(
                "escrowSize",
                XfsValue::new().with_i4(val.escrow_size as i32),
            ),
            XfsMember::create("detector", XfsValue::new().with_boolean(val.detector as u8)),
            XfsMember::create(
                "defaultRollbackPosition",
                val.default_rollback_position.into(),
            ),
            val.position_capabilities_list.into(),
            XfsMember::create(
                "recognitionSensorType",
                XfsValue::new()
                    .with_string(std::str::from_utf8(&[val.recognition_sensor_type]).unwrap_or("")),
            ),
            val.reporting_mode.into(),
            val.self_test_mode.into(),
            XfsMember::create(
                "reportUsbConsumption",
                XfsValue::new().with_boolean(val.report_usb_consumption as u8),
            ),
            val.secured_comm_level.into(),
            XfsMember::create(
                "autoRetractAtInlet",
                XfsValue::new().with_boolean(val.auto_retract as u8),
            ),
            XfsMember::create(
                "rejectViaOutlet",
                XfsValue::new().with_boolean(val.reject_via_outlet as u8),
            ),
        ])
    }
}

impl From<Capabilities> for XfsStruct {
    fn from(val: Capabilities) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsStruct> for Capabilities {
    type Error = Error;

    fn try_from(val: &XfsStruct) -> Result<Self> {
        let members = val.members();

        let auto_pres = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "autoPresent" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "autoPresent""#.into()))?;

        let allow_usb = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "allowUsbFrontSwitch" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "allowUsbFrontSwitch""#.into(),
            ))?;

        let anti_fish = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == AntiFishingLevel::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "antiFishingLevel""#.into(),
            ))?;

        let cdr_type = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == CdrType::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "cdType""#.into()))?;

        let eur_art = members
            .iter()
            .map(|m| m.inner())
            .find(|m| {
                m.name() == EuroArt6Capability::xfs_name() && m.value().xfs_struct().is_some()
            })
            .ok_or(Error::Xfs(
                r#"Capabilities missing "eurArt6Capabilities""#.into(),
            ))?;

        let trusted_user = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "trustedUser" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "trustedUser""#.into()))?;

        let max_in = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "maxInBills" && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "maxInBills""#.into()))?;

        let max_out = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "maxOutBills" && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "maxOutBills""#.into()))?;

        let shutter_cmd = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "shutterCmd" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "shutterCmd""#.into()))?;

        let retract = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "retract" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "retract""#.into()))?;

        let safe_cmd = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "safeDoorCmd" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "safeDoorCmd""#.into()))?;

        let cash_box = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "cashBox" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "cashBox""#.into()))?;

        let refill = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "refill" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "refill""#.into()))?;

        let dispense = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "dispense" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "dispense""#.into()))?;

        let deposit = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "deposit" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "deposit""#.into()))?;

        let int_stacker = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "intermediateStacker" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "intermediateStacker""#.into(),
            ))?;

        let bills_sensor = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "billsTakenSensor" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "billsTakenSensor""#.into(),
            ))?;

        let escrow = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "escrow" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "escrow""#.into()))?;

        let escrow_size = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "escrowSize" && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "escrowSize""#.into()))?;

        let detector = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "detector" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "detector""#.into()))?;

        let default_rollback = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "defaultRollbackPosition" && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "defaultRollbackPosition""#.into(),
            ))?;

        let pos_caps = members
            .iter()
            .map(|m| m.inner())
            .find(|m| {
                m.name() == CdrPositionCapabilitiesList::xfs_name() && m.value().array().is_some()
            })
            .ok_or(Error::Xfs(
                r#"Capabilities missing "positionCapabilitiesList""#.into(),
            ))?;

        let recog_sensor = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "recognitionSensorType" && m.value().string().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "recognitionSensorType""#.into(),
            ))?;

        let reporting_mode = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == ReportingMode::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "reportingMode""#.into()))?;

        let self_test = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == SelfTestMode::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(r#"Capabilities missing "selfTestMode""#.into()))?;

        let report_usb = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "reportUsbConsumption" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "reportUsbConsumption""#.into(),
            ))?;

        let sec_comm = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == SecuredCommLevel::xfs_name() && m.value().i4().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "securedCommLevel""#.into(),
            ))?;

        let auto_retract = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "autoRetractAtInlet" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "autoRetractAtInlet""#.into(),
            ))?;

        let reject_out = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "rejectViaOutlet" && m.value().boolean().is_some())
            .ok_or(Error::Xfs(
                r#"Capabilities missing "rejectViaOutlet""#.into(),
            ))?;

        let euro_art6_capability: EuroArt6Capability = eur_art.try_into()?;
        let cd_type: CdrType = cdr_type.try_into()?;

        let recognition_sensor_type = match recog_sensor.value().string() {
            Some(rs) if rs == "B" => b'B',
            rs => {
                return Err(Error::Xfs(format!(
                    r#"Expected recognition sensor type "B", have {rs:?}"#
                )))
            }
        };

        let default_rollback_position: CdrPosition =
            default_rollback.value().i4().unwrap_or(&0).into();
        let anti_fishing_level: AntiFishingLevel = anti_fish.try_into()?;
        let self_test_mode: SelfTestMode = self_test.try_into()?;
        let reporting_mode: ReportingMode = reporting_mode.try_into()?;
        let position_capabilities_list: CdrPositionCapabilitiesList = pos_caps.try_into()?;
        let secured_comm_level: SecuredCommLevel = sec_comm.try_into()?;

        Ok(Self {
            auto_present: auto_pres.value().boolean() == Some(&1),
            cd_type,
            euro_art6_capability,
            trusted_user: trusted_user.value().boolean() == Some(&1),
            max_in_bills: *max_in.value().i4().unwrap_or(&1) as u32,
            max_out_bills: *max_out.value().i4().unwrap_or(&15) as u32,
            shutter_cmd: shutter_cmd.value().boolean() == Some(&1),
            retract: retract.value().boolean() == Some(&1),
            safe_door_cmd: safe_cmd.value().boolean() == Some(&1),
            cash_box: cash_box.value().boolean() == Some(&1),
            refill: refill.value().boolean() == Some(&1),
            dispense: dispense.value().boolean() == Some(&1),
            deposit: deposit.value().boolean() == Some(&1),
            intermediate_stacker: int_stacker.value().boolean() == Some(&1),
            bill_taken_sensor: bills_sensor.value().boolean() == Some(&1),
            escrow: escrow.value().boolean() == Some(&1),
            escrow_size: *escrow_size.value().i4().unwrap_or(&15) as u32,
            detector: detector.value().boolean() == Some(&1),
            default_rollback_position,
            position_capabilities_list,
            self_test_mode,
            recognition_sensor_type,
            anti_fishing_level,
            allow_usb_front_switch: allow_usb.value().boolean() == Some(&1),
            reporting_mode,
            report_usb_consumption: report_usb.value().boolean() == Some(&1),
            auto_retract: auto_retract.value().boolean() == Some(&1),
            reject_via_outlet: reject_out.value().boolean() == Some(&1),
            secured_comm_level,
        })
    }
}

impl TryFrom<XfsStruct> for Capabilities {
    type Error = Error;

    fn try_from(val: XfsStruct) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsValue> for Capabilities {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.xfs_struct()
            .ok_or(Error::Xfs(format!(
                "Expected Capabilities XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for Capabilities {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
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
