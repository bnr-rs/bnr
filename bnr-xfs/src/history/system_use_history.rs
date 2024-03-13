use crate::xfs::method_response::XfsMethodResponse;
use crate::{create_xfs_array, create_xfs_date_time, create_xfs_i4, create_xfs_struct};
use crate::{Error, Result};

const SENSOR_TEMPS_LEN: usize = 4;
const SENSOR_TEMP_DEFAULT: SensorTemperature = SensorTemperature::new();

create_xfs_date_time!(
    CurrentDateTime,
    "currentDateTime",
    "Represents the device current date time."
);

create_xfs_i4!(UpTime, "upTime", "Represents the device up time.");

create_xfs_i4!(
    TotalUpTime,
    "totalUpTime",
    "Represents the device total up time."
);

create_xfs_i4!(
    TimeSinceOperational,
    "timeSinceOperational",
    "Represents the device time since operational."
);

create_xfs_i4!(
    SystemCycleCount,
    "systemCycleCount",
    "Represents the device system cycle count."
);

create_xfs_i4!(
    SystemTemperature,
    "systemTemperature",
    "Represents the device system temperature."
);

create_xfs_i4!(
    SensorTemperature,
    "",
    "Represents a device sensor temperature."
);

create_xfs_array!(
    RecognitionSensorTemperatures,
    "recognitionSensorTemperatures",
    SensorTemperature,
    SENSOR_TEMPS_LEN,
    SENSOR_TEMP_DEFAULT,
    "Represents the device recognition sensor temperatures."
);

create_xfs_i4!(
    PowerSupplyVoltage,
    "powerSupplyVoltage",
    "Represents a device power supply voltage."
);

create_xfs_struct!(
    SystemUseHistory,
    "systemUseHistory",
    [
        current_date_time: CurrentDateTime,
        up_time: UpTime,
        total_up_time: TotalUpTime,
        time_since_operational: TimeSinceOperational,
        system_cycle_count: SystemCycleCount,
        system_temperature: SystemTemperature,
        recognition_sensor_temperatures: RecognitionSensorTemperatures,
        power_supply_voltage: PowerSupplyVoltage
    ],
    "Represents the system use history."
);

impl TryFrom<&XfsMethodResponse> for SystemUseHistory {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected SystemUseHistory XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for SystemUseHistory {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
