use bnr_xfs::{
    xfs::{
        from_str,
        xfs_struct::{XfsMember, XfsStruct},
    },
    Capabilities, CdrPositionCapabilitiesList, Result,
};

#[test]
fn test_caps_serde() -> Result<()> {
    let xfs = include_bytes!("xml/capabilities.xml");
    let xfs_str = std::str::from_utf8(xfs).unwrap_or("");

    let xfs_struct = from_str::<XfsStruct>(xfs_str)?;
    let caps = Capabilities::try_from(xfs_struct)?;

    log::debug!("Deserialized Capabilities: {caps}");

    Ok(())
}

#[test]
fn test_position_caps_list_serde() -> Result<()> {
    let xfs = include_bytes!("xml/position-capabilities.xml");
    let xfs_str = std::str::from_utf8(xfs).unwrap_or("");

    let xfs_mem = from_str::<XfsMember>(xfs_str)?;
    let caps = CdrPositionCapabilitiesList::try_from(xfs_mem)?;

    log::debug!("Deserialized PositionCapabilitiesList: {caps}");

    Ok(())
}
