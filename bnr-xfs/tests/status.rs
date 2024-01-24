use bnr_xfs::{
    xfs::{from_str, xfs_struct::XfsStruct},
    CdrStatus, Result,
};

#[test]
fn test_status_serde() -> Result<()> {
    let xfs = include_bytes!("xml/status.xml");
    let xfs_str = std::str::from_utf8(xfs).unwrap_or("");

    let xfs_struct = from_str::<XfsStruct>(xfs_str)?;
    let caps = CdrStatus::try_from(xfs_struct)?;

    log::debug!("Deserialized Capabilities: {caps}");

    Ok(())
}
