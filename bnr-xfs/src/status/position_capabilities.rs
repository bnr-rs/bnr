use std::{cmp, fmt};

use crate::xfs::{
    array::XfsArray,
    method_response::XfsMethodResponse,
    params::XfsParam,
    value::XfsValue,
    xfs_struct::{XfsMember, XfsStruct},
};
use crate::{Error, Result};

use super::CdrPosition;

pub const CDR_POS_CAP_LIST_LEN: usize = 2;

/// Characteristics of an input/output position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CdrPositionCapabilities {
    /// Position fo the unit.
    pub position: CdrPosition,
    /// Specifies whether shutter status reporting is supported.
    pub shutter_status_supported: bool,
    /// Defines if the shutter has to be explicitly controlled by the application.
    pub shutter_cmd: bool,
    /// Specifies whether there is a sensor to detect if the position is empty.
    pub content_status_supported: bool,
    /// Maximum number of items which this position can hold.
    pub max_items: u32,
    /// Specifies whether this position can be used as source for an accept command.
    pub input: bool,
    /// Specifies whether this position can be used as target for a dispense command.
    pub output: bool,
    /// Specifies whether this position can be used as target for
    /// [cash_in_rollback](crate::cash::cash_in_rollback) command.
    pub rollback: bool,
    /// Specifies whether refused notes can be moved to this position during
    /// [cash_in](crate::cash::cash_in) command.
    pub refusal: bool,
}

impl CdrPositionCapabilities {
    /// Creates a new [CdrPositionCapabilities].
    pub const fn new() -> Self {
        Self {
            position: CdrPosition::new(),
            shutter_status_supported: false,
            shutter_cmd: false,
            content_status_supported: false,
            max_items: 0,
            input: false,
            output: false,
            rollback: false,
            refusal: false,
        }
    }

    /// Gets the [XfsMember] name of the [CdrPositionCapabilities].
    pub const fn xfs_name() -> &'static str {
        "positionCapabilities"
    }
}

impl From<&CdrPositionCapabilities> for XfsStruct {
    fn from(val: &CdrPositionCapabilities) -> Self {
        Self::create([
            val.position.into(),
            XfsMember::create(
                "contentStatusSupported",
                XfsValue::new().with_boolean(val.content_status_supported as u8),
            ),
            XfsMember::create(
                "shutterStatusSupported",
                XfsValue::new().with_boolean(val.shutter_status_supported as u8),
            ),
            XfsMember::create(
                "shutterCmd",
                XfsValue::new().with_boolean(val.shutter_status_supported as u8),
            ),
            XfsMember::create("maxItems", XfsValue::new().with_i4(val.max_items as i32)),
            XfsMember::create("input", XfsValue::new().with_boolean(val.input as u8)),
            XfsMember::create("output", XfsValue::new().with_boolean(val.output as u8)),
            XfsMember::create("rollback", XfsValue::new().with_boolean(val.rollback as u8)),
            XfsMember::create("refusal", XfsValue::new().with_boolean(val.refusal as u8)),
        ])
    }
}

impl From<CdrPositionCapabilities> for XfsStruct {
    fn from(val: CdrPositionCapabilities) -> Self {
        (&val).into()
    }
}

impl From<&CdrPositionCapabilities> for XfsValue {
    fn from(val: &CdrPositionCapabilities) -> Self {
        Self::new().with_xfs_struct(val.into())
    }
}

impl From<CdrPositionCapabilities> for XfsValue {
    fn from(val: CdrPositionCapabilities) -> Self {
        (&val).into()
    }
}

impl From<&CdrPositionCapabilities> for XfsMember {
    fn from(val: &CdrPositionCapabilities) -> Self {
        XfsMember::create(CdrPositionCapabilities::xfs_name(), val.into())
    }
}

impl From<CdrPositionCapabilities> for XfsMember {
    fn from(val: CdrPositionCapabilities) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsValue> for CdrPositionCapabilities {
    type Error = Error;

    fn try_from(val: &XfsValue) -> Result<Self> {
        val.xfs_struct()
            .ok_or(Error::Xfs(format!(
                "Expected CdrPositionCapabilities XfsValue, have: {val}"
            )))?
            .try_into()
    }
}

impl TryFrom<XfsValue> for CdrPositionCapabilities {
    type Error = Error;

    fn try_from(val: XfsValue) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsStruct> for CdrPositionCapabilities {
    type Error = Error;

    fn try_from(val: &XfsStruct) -> Result<Self> {
        let members = val.members();

        let pos = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "position" && m.value().i4().is_some());
        let content_stat_support = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "contentStatusSupported" && m.value().boolean().is_some());
        let shutter_stat_support = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "shutterStatusSupported" && m.value().boolean().is_some());
        let shutter_cmd = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "shutterCmd" && m.value().boolean().is_some());
        let max_items = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "maxItems" && m.value().i4().is_some());
        let input = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "input" && m.value().boolean().is_some());
        let output = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "output" && m.value().boolean().is_some());
        let rollback = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "rollback" && m.value().boolean().is_some());
        let refusal = members
            .iter()
            .map(|m| m.inner())
            .find(|m| m.name() == "refusal" && m.value().boolean().is_some());

        match (
            pos,
            content_stat_support,
            shutter_stat_support,
            shutter_cmd,
            max_items,
            input,
            output,
            rollback,
            refusal,
        ) {
            (Some(pos),
            Some(css),
            Some(sss),
            Some(sc),
            Some(mi),
            Some(i),
            Some(o),
            Some(rb),
            Some(rf),
            ) => {
                Ok(Self {
                    position: pos.value().i4().unwrap_or(&0).into(),
                    shutter_status_supported: sss.value().boolean().unwrap_or(&0) == &1,
                    shutter_cmd: sc.value().boolean().unwrap_or(&0) == &1,
                    content_status_supported: css.value().boolean().unwrap_or(&0) == &1,
                    max_items: *mi.value().i4().unwrap_or(&0) as u32,
                    input: i.value().boolean().unwrap_or(&0) == &1,
                    output: o.value().boolean().unwrap_or(&0) == &1,
                    rollback: rb.value().boolean().unwrap_or(&0) == &1,
                    refusal: rf.value().boolean().unwrap_or(&0) == &1,
                })
            }
            _ => Err(Error::Xfs(format!("Invalid CdrPositionCapabilities fields, position: {pos:?}, content status support: {content_stat_support:?}, shutter status support: {shutter_stat_support:?}, chutter command: {shutter_cmd:?}, max items: {max_items:?}, input: {input:?}, output: {output:?}, rollback: {rollback:?}, refusal: {refusal:?}"))),
        }
    }
}

impl TryFrom<XfsStruct> for CdrPositionCapabilities {
    type Error = Error;

    fn try_from(val: XfsStruct) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CdrPositionCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""position":{},"#, self.position)?;
        write!(
            f,
            r#""shutter_status_supported":{},"#,
            self.shutter_status_supported
        )?;
        write!(f, r#""shutter_cmd":{},"#, self.shutter_cmd)?;
        write!(
            f,
            r#""content_status_supported":{},"#,
            self.content_status_supported
        )?;
        write!(f, r#""max_items":{},"#, self.max_items)?;
        write!(f, r#""input":{},"#, self.input)?;
        write!(f, r#""output":{},"#, self.output)?;
        write!(f, r#""rollback":{},"#, self.rollback)?;
        write!(f, r#""refusal":{}"#, self.refusal)?;
        write!(f, "}}")
    }
}

/// List of position capabilties.
///
/// One for each position supported by the device.
///
/// See [Capabilities](crate::capabilities::Capabilities).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CdrPositionCapabilitiesList {
    max_size: u32,
    size: u32,
    items: [CdrPositionCapabilities; CDR_POS_CAP_LIST_LEN],
}

impl CdrPositionCapabilitiesList {
    /// Creates a new [CdrPositionCapabilitiesList].
    pub const fn new() -> Self {
        Self {
            max_size: CDR_POS_CAP_LIST_LEN as u32,
            size: 0,
            items: [CdrPositionCapabilities::new(); CDR_POS_CAP_LIST_LEN],
        }
    }

    /// Gets the maximum capacity of the [CdrPositionCapabilitiesList].
    pub const fn capacity() -> usize {
        CDR_POS_CAP_LIST_LEN
    }

    /// Gets the maximum size of the list.
    pub const fn max_size(&self) -> u32 {
        self.max_size
    }

    /// Gets the size of the list.
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Sets the size of the list.
    ///
    /// No-op if the size is invalid (> [CDR_POS_CAP_LIST_LEN]).
    pub fn set_size(&mut self, size: u32) {
        if (size as usize) <= CDR_POS_CAP_LIST_LEN {
            self.size = size;
        }
    }

    /// Builder function that sets the size of the list.
    ///
    /// No-op if the size is invalid (> [CDR_POS_CAP_LIST_LEN]).
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets whether the size of the list is valid.
    pub const fn size_is_valid(&self) -> bool {
        (self.size as usize) <= CDR_POS_CAP_LIST_LEN
    }

    /// Gets a reference to the [CdrPositionCapabilities] list items.
    pub fn items(&self) -> &[CdrPositionCapabilities] {
        if self.size_is_valid() {
            self.items[..self.size as usize].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Sets the [CdrPositionCapabilities] list items.
    ///
    /// Only sets up to [max_size](CDR_POS_CAP_LIST_LEN) items.
    pub fn set_items(&mut self, items: &[CdrPositionCapabilities]) {
        let len = cmp::min(CDR_POS_CAP_LIST_LEN, items.len());
        self.items[..len].copy_from_slice(&items[..len]);
    }

    /// Builder function that sets the [CdrPositionCapabilities] list items.
    ///
    /// Only sets up to [max_size](CDR_POS_CAP_LIST_LEN) items.
    pub fn with_items(mut self, items: &[CdrPositionCapabilities]) -> Self {
        self.set_items(items);
        self
    }

    pub const fn xfs_name() -> &'static str {
        "positionCapabilitiesList"
    }
}

impl From<&CdrPositionCapabilitiesList> for XfsArray {
    fn from(val: &CdrPositionCapabilitiesList) -> Self {
        XfsArray::create(val.items.map(XfsValue::from))
    }
}

impl From<&CdrPositionCapabilitiesList> for XfsValue {
    fn from(val: &CdrPositionCapabilitiesList) -> Self {
        XfsValue::new().with_array(val.into())
    }
}

impl From<CdrPositionCapabilitiesList> for XfsValue {
    fn from(val: CdrPositionCapabilitiesList) -> Self {
        (&val).into()
    }
}

impl From<&CdrPositionCapabilitiesList> for XfsParam {
    fn from(val: &CdrPositionCapabilitiesList) -> Self {
        Self::create(val.into())
    }
}

impl From<CdrPositionCapabilitiesList> for XfsParam {
    fn from(val: CdrPositionCapabilitiesList) -> Self {
        (&val).into()
    }
}

impl From<&CdrPositionCapabilitiesList> for XfsMember {
    fn from(val: &CdrPositionCapabilitiesList) -> Self {
        XfsMember::create(CdrPositionCapabilitiesList::xfs_name(), val.into())
    }
}

impl From<CdrPositionCapabilitiesList> for XfsMember {
    fn from(val: CdrPositionCapabilitiesList) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        let name = val.name();
        let value = val.value().array();

        match (name, value) {
            (n, Some(array)) if n == Self::xfs_name() => {
                let data = array.data();
                let len = cmp::min(data.len(), Self::capacity());

                let mut ret = Self::new().with_size(len as u32);

                for (dst, src) in ret.items[..len].iter_mut().zip(data[..len].iter()) {
                    *dst = src.inner().try_into()?;
                }

                Ok(ret)
            }
            _ => Err(Error::Xfs(format!(
                "Expected CdrPositionCapabilitiesList XfsMember, have: {val}"
            ))),
        }
    }
}

impl TryFrom<XfsMember> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsParam> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: &XfsParam) -> Result<Self> {
        val.value()
            .xfs_struct()
            .unwrap()
            .members()
            .iter()
            .find(|m| m.inner().name() == CdrPositionCapabilitiesList::xfs_name())
            .ok_or(Error::Xfs(format!(
                "Expected CdrPositionCapabilitiesList XfsParam, have: {val}"
            )))?
            .inner()
            .try_into()
    }
}

impl TryFrom<XfsParam> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: XfsParam) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&XfsMethodResponse> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .find(|p| p.inner().value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected CdrPositionCapabilitiesList XfsMethodResponse, have: {val}"
            )))?
            .inner()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for CdrPositionCapabilitiesList {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CdrPositionCapabilitiesList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size":{},"#, self.max_size)?;
        write!(f, r#""size":{},"#, self.size)?;
        write!(f, r#""items":["#)?;

        let items = self.items();
        let items_len = items.len();
        for (i, item) in items.iter().enumerate() {
            write!(f, "{item}")?;
            if i != items_len - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, "]}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdr_position_capabitilities_xfs() -> Result<()> {
        let exp_xfs = XfsMember::create(
            CdrPositionCapabilitiesList::xfs_name(),
            XfsValue::new().with_array(XfsArray::create([XfsValue::new().with_xfs_struct(
                XfsStruct::create([
                    XfsMember::create("position", XfsValue::new().with_i4(1)),
                    XfsMember::create("contentStatusSupported", XfsValue::new().with_boolean(1)),
                    XfsMember::create("shutterStatusSupported", XfsValue::new().with_boolean(0)),
                    XfsMember::create("shutterCmd", XfsValue::new().with_boolean(0)),
                    XfsMember::create("maxItems", XfsValue::new().with_i4(1)),
                    XfsMember::create("input", XfsValue::new().with_boolean(1)),
                    XfsMember::create("output", XfsValue::new().with_boolean(0)),
                    XfsMember::create("rollback", XfsValue::new().with_boolean(0)),
                    XfsMember::create("refusal", XfsValue::new().with_boolean(1)),
                ]),
            )])),
        );
        let exp_cap_list = CdrPositionCapabilitiesList {
            max_size: 2,
            size: 1,
            items: [
                CdrPositionCapabilities {
                    position: CdrPosition::Top,
                    content_status_supported: true,
                    shutter_status_supported: false,
                    shutter_cmd: false,
                    max_items: 1,
                    input: true,
                    output: false,
                    rollback: false,
                    refusal: true,
                },
                CdrPositionCapabilities::new(),
            ],
        };

        assert_eq!(
            CdrPositionCapabilitiesList::try_from(&exp_xfs)?,
            exp_cap_list
        );

        let xfs = XfsMember::from(exp_cap_list);
        let arr = xfs.value().array().unwrap();
        let mem0 = arr.data()[0].inner().xfs_struct().unwrap();

        let exp_arr = exp_xfs.value().array().unwrap();
        let exp_mem0 = exp_arr.data()[0].inner().xfs_struct().unwrap();

        assert_eq!(mem0, exp_mem0);

        Ok(())
    }

    #[test]
    fn test_cdr_position_capabitilities_xfs_full() -> Result<()> {
        let exp_xfs = XfsMember::create(
            CdrPositionCapabilitiesList::xfs_name(),
            XfsValue::new().with_array(XfsArray::create([
                XfsValue::new().with_xfs_struct(XfsStruct::create([
                    XfsMember::create("position", XfsValue::new().with_i4(1)),
                    XfsMember::create("contentStatusSupported", XfsValue::new().with_boolean(1)),
                    XfsMember::create("shutterStatusSupported", XfsValue::new().with_boolean(0)),
                    XfsMember::create("shutterCmd", XfsValue::new().with_boolean(0)),
                    XfsMember::create("maxItems", XfsValue::new().with_i4(1)),
                    XfsMember::create("input", XfsValue::new().with_boolean(1)),
                    XfsMember::create("output", XfsValue::new().with_boolean(0)),
                    XfsMember::create("rollback", XfsValue::new().with_boolean(0)),
                    XfsMember::create("refusal", XfsValue::new().with_boolean(1)),
                ])),
                XfsValue::new().with_xfs_struct(XfsStruct::create([
                    XfsMember::create("position", XfsValue::new().with_i4(2)),
                    XfsMember::create("contentStatusSupported", XfsValue::new().with_boolean(1)),
                    XfsMember::create("shutterStatusSupported", XfsValue::new().with_boolean(1)),
                    XfsMember::create("shutterCmd", XfsValue::new().with_boolean(1)),
                    XfsMember::create("maxItems", XfsValue::new().with_i4(15)),
                    XfsMember::create("input", XfsValue::new().with_boolean(0)),
                    XfsMember::create("output", XfsValue::new().with_boolean(1)),
                    XfsMember::create("rollback", XfsValue::new().with_boolean(1)),
                    XfsMember::create("refusal", XfsValue::new().with_boolean(0)),
                ])),
            ])),
        );
        let exp_cap_list = CdrPositionCapabilitiesList {
            max_size: 2,
            size: 2,
            items: [
                CdrPositionCapabilities {
                    position: CdrPosition::Top,
                    content_status_supported: true,
                    shutter_status_supported: false,
                    shutter_cmd: false,
                    max_items: 1,
                    input: true,
                    output: false,
                    rollback: false,
                    refusal: true,
                },
                CdrPositionCapabilities {
                    position: CdrPosition::Bottom,
                    content_status_supported: true,
                    shutter_status_supported: true,
                    shutter_cmd: true,
                    max_items: 15,
                    input: false,
                    output: true,
                    rollback: true,
                    refusal: false,
                },
            ],
        };

        assert_eq!(
            CdrPositionCapabilitiesList::try_from(&exp_xfs)?,
            exp_cap_list
        );

        let xfs = XfsMember::from(exp_cap_list);
        let arr = xfs.value().array().unwrap();
        let mem0 = arr.data()[0].inner().xfs_struct().unwrap();
        let mem1 = arr.data()[1].inner().xfs_struct().unwrap();

        let exp_arr = exp_xfs.value().array().unwrap();
        let exp_mem0 = exp_arr.data()[0].inner().xfs_struct().unwrap();
        let exp_mem1 = exp_arr.data()[1].inner().xfs_struct().unwrap();

        assert_eq!(mem0, exp_mem0);
        assert_eq!(mem1, exp_mem1);

        Ok(())
    }
}
