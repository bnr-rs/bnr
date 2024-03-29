use std::{cmp, fmt};

use crate::{create_xfs_bool, create_xfs_i4, impl_xfs_array, impl_xfs_struct, ShutterCmd, Size};

use super::CdrPosition;

pub const CDR_POS_CAP_LIST_LEN: usize = 2;

create_xfs_bool!(
    ShutterStatusSupported,
    "shutterStatusSupported",
    "Specifies whether shutter status reporting is supported."
);
create_xfs_bool!(
    ContentStatusSupported,
    "contentStatusSupported",
    "Specifies whether there is a sensor to detect if the position is empty."
);
create_xfs_i4!(
    MaxItems,
    "maxItems",
    "Maximum number of items which this position can hold."
);
create_xfs_bool!(
    Input,
    "input",
    "Specifies whether this position can be used as source for an accept command."
);
create_xfs_bool!(
    Output,
    "output",
    "Specifies whether this position can be used as target for a dispense command."
);
create_xfs_bool!(Rollback, "rollback", "Specifies whether this position can be used as target for[cash_in_rollback](crate::cash::cash_in_rollback) command.");
create_xfs_bool!(Refusal, "refusal", "Specifies whether refused notes can be moved to this position during [cash_in](crate::cash::cash_in) command.");

/// Characteristics of an input/output position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrPositionCapabilities {
    /// Position fo the unit.
    pub position: CdrPosition,
    pub content_status_supported: ContentStatusSupported,
    pub shutter_status_supported: ShutterStatusSupported,
    /// Defines if the shutter has to be explicitly controlled by the application.
    pub shutter_cmd: ShutterCmd,
    pub max_items: MaxItems,
    pub input: Input,
    pub output: Output,
    pub rollback: Rollback,
    pub refusal: Refusal,
}

impl CdrPositionCapabilities {
    /// Creates a new [CdrPositionCapabilities].
    pub const fn new() -> Self {
        Self {
            position: CdrPosition::new(),
            shutter_status_supported: ShutterStatusSupported::new(),
            shutter_cmd: ShutterCmd::new(),
            content_status_supported: ContentStatusSupported::new(),
            max_items: MaxItems::new(),
            input: Input::new(),
            output: Output::new(),
            rollback: Rollback::new(),
            refusal: Refusal::new(),
        }
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

impl_xfs_struct!(
    CdrPositionCapabilities,
    "positionCapabilities",
    [
        position: CdrPosition,
        shutter_status_supported: ShutterStatusSupported,
        shutter_cmd: ShutterCmd,
        content_status_supported: ContentStatusSupported,
        max_items: MaxItems,
        input: Input,
        output: Output,
        rollback: Rollback,
        refusal: Refusal
    ]
);

/// List of position capabilties.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CdrPositionCapabilitiesList {
    size: Size,
    items: [CdrPositionCapabilities; CDR_POS_CAP_LIST_LEN],
}

impl CdrPositionCapabilitiesList {
    /// Creates a new [CdrPositionCapabilitiesList].
    pub const fn new() -> Self {
        Self {
            size: Size::new(),
            items: [CdrPositionCapabilities::new(); CDR_POS_CAP_LIST_LEN],
        }
    }

    /// Creates a new [CdrPositionCapabilitiesList] from the provided parameter.
    pub const fn create(items: [CdrPositionCapabilities; CDR_POS_CAP_LIST_LEN]) -> Self {
        Self {
            size: Size::create(CDR_POS_CAP_LIST_LEN as u32),
            items,
        }
    }

    /// Gets the max size.
    pub const fn max_size() -> usize {
        CDR_POS_CAP_LIST_LEN
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size.inner()
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: u32) {
        if size as usize <= Self::max_size() {
            self.size.set_inner(size);
        }
    }

    /// Gets a reference to the [CdrPositionCapabilities] list.
    pub fn items(&self) -> &[CdrPositionCapabilities] {
        let len = self.size.inner() as usize;
        if len <= Self::max_size() {
            self.items[..len].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a reference to the [CdrPositionCapabilities] list.
    pub fn items_mut(&mut self) -> &mut [CdrPositionCapabilities] {
        let len = self.size.inner() as usize;
        if len <= Self::max_size() {
            self.items[..len].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets the [CdrPositionCapabilities] list items.
    pub fn set_items(&mut self, items: &[CdrPositionCapabilities]) {
        let len = cmp::min(items.len(), Self::max_size());
        self.items[..len].copy_from_slice(&items[..len]);
        self.size.set_inner(len as u32);
    }

    /// Builder function that sets the [CdrPositionCapabilities] list items.
    pub fn with_items(mut self, items: &[CdrPositionCapabilities]) -> Self {
        self.set_items(items);
        self
    }
}

impl fmt::Display for CdrPositionCapabilitiesList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let items = self.items();
        for (i, item) in items.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{item}")?;
        }

        write!(f, "]")
    }
}

impl_xfs_array!(CdrPositionCapabilitiesList, "positionCapabilitiesList");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xfs::{
        array::XfsArray,
        value::XfsValue,
        xfs_struct::{XfsMember, XfsStruct},
    };
    use crate::Result;

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
            size: Size::create(1),
            items: [
                CdrPositionCapabilities {
                    position: CdrPosition::Top,
                    shutter_status_supported: false.into(),
                    shutter_cmd: false.into(),
                    content_status_supported: true.into(),
                    max_items: 1u32.into(),
                    input: true.into(),
                    output: false.into(),
                    rollback: false.into(),
                    refusal: true.into(),
                },
                CdrPositionCapabilities::new(),
            ],
        };

        assert_eq!(
            CdrPositionCapabilitiesList::try_from(&exp_xfs)?,
            exp_cap_list
        );

        let xfs_list = XfsMember::from(exp_cap_list);
        for (exp, item) in exp_xfs
            .value()
            .array()
            .unwrap()
            .data()
            .iter()
            .map(|m| m.inner().xfs_struct().unwrap())
            .zip(
                xfs_list
                    .value()
                    .array()
                    .unwrap()
                    .data()
                    .iter()
                    .map(|m| m.inner().xfs_struct().unwrap()),
            )
        {
            assert_eq!(
                exp.find_member(CdrPosition::xfs_name())?,
                item.find_member(CdrPosition::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ShutterStatusSupported::xfs_name())?,
                item.find_member(ShutterStatusSupported::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ShutterCmd::xfs_name())?,
                item.find_member(ShutterCmd::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ContentStatusSupported::xfs_name())?,
                item.find_member(ContentStatusSupported::xfs_name())?
            );
            assert_eq!(
                exp.find_member(MaxItems::xfs_name())?,
                item.find_member(MaxItems::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Input::xfs_name())?,
                item.find_member(Input::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Output::xfs_name())?,
                item.find_member(Output::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Rollback::xfs_name())?,
                item.find_member(Rollback::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Refusal::xfs_name())?,
                item.find_member(Refusal::xfs_name())?
            );
        }

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
            size: Size::create(2),
            items: [
                CdrPositionCapabilities {
                    position: CdrPosition::Top,
                    content_status_supported: true.into(),
                    shutter_status_supported: false.into(),
                    shutter_cmd: false.into(),
                    max_items: 1u32.into(),
                    input: true.into(),
                    output: false.into(),
                    rollback: false.into(),
                    refusal: true.into(),
                },
                CdrPositionCapabilities {
                    position: CdrPosition::Bottom,
                    content_status_supported: true.into(),
                    shutter_status_supported: true.into(),
                    shutter_cmd: true.into(),
                    max_items: 15u32.into(),
                    input: false.into(),
                    output: true.into(),
                    rollback: true.into(),
                    refusal: false.into(),
                },
            ],
        };

        assert_eq!(
            CdrPositionCapabilitiesList::try_from(&exp_xfs)?,
            exp_cap_list
        );

        let xfs_list = XfsMember::from(exp_cap_list);
        for (exp, item) in exp_xfs
            .value()
            .array()
            .unwrap()
            .data()
            .iter()
            .map(|m| m.inner().xfs_struct().unwrap())
            .zip(
                xfs_list
                    .value()
                    .array()
                    .unwrap()
                    .data()
                    .iter()
                    .map(|m| m.inner().xfs_struct().unwrap()),
            )
        {
            assert_eq!(
                exp.find_member(CdrPosition::xfs_name())?,
                item.find_member(CdrPosition::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ShutterStatusSupported::xfs_name())?,
                item.find_member(ShutterStatusSupported::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ShutterCmd::xfs_name())?,
                item.find_member(ShutterCmd::xfs_name())?
            );
            assert_eq!(
                exp.find_member(ContentStatusSupported::xfs_name())?,
                item.find_member(ContentStatusSupported::xfs_name())?
            );
            assert_eq!(
                exp.find_member(MaxItems::xfs_name())?,
                item.find_member(MaxItems::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Input::xfs_name())?,
                item.find_member(Input::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Output::xfs_name())?,
                item.find_member(Output::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Rollback::xfs_name())?,
                item.find_member(Rollback::xfs_name())?
            );
            assert_eq!(
                exp.find_member(Refusal::xfs_name())?,
                item.find_member(Refusal::xfs_name())?
            );
        }

        Ok(())
    }
}
