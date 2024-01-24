use std::fmt;

/// Defines how the BNR perform the self tests.
///
/// Integration recommendations:
///
/// If the device mode [Device](SelfTestMode::Device) is used, the Host should send a [self_test](crate::maintenance::self_test) command at every time that internal tests and movements of the BNR can be allowed.
///
/// It is recommended to send [self_test](crate::maintenance::self_test), after a [present](crate::cash::present) command to allow the BNR to refloat the recyclers.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SelfTestMode {
    /// This is the default and recommended mode. In this mode, the BNR does self tests automatically.
    /// After [reset](crate::init::reset) or [present](crate::cash::present), the BNR refloats the Recyclers from the Loader, when necessary.
    #[default]
    Auto = 0,
    /// In this mode, the BNR doesn’t do anything automatically, but waits for a [self_test](crate::maintenance::self_test) command to do all self test actions.
    Device,
}

impl SelfTestMode {
    /// Creates a new [SelfTestMode].
    pub const fn new() -> Self {
        Self::Auto
    }
}

impl From<u32> for SelfTestMode {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Auto,
            1 => Self::Device,
            _ => Self::Auto,
        }
    }
}

impl From<SelfTestMode> for u32 {
    fn from(val: SelfTestMode) -> Self {
        val as u32
    }
}

impl From<&SelfTestMode> for &'static str {
    fn from(val: &SelfTestMode) -> Self {
        match val {
            SelfTestMode::Auto => "auto",
            SelfTestMode::Device => "device",
        }
    }
}

impl From<SelfTestMode> for &'static str {
    fn from(val: SelfTestMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for SelfTestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
