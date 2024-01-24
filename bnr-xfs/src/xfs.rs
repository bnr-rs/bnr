//! XFS message types for BNR communication.

use crate::Result;

pub mod array;
pub mod fault;
pub mod method_call;
pub mod method_response;
mod operation_id;
pub mod params;
pub mod value;
pub mod xfs_struct;

pub use operation_id::*;

pub fn to_string<S: serde::Serialize>(s: S) -> Result<String> {
    let mut sink = Vec::with_capacity(0xff);

    let event_writer = xml::EventWriter::new_with_config(
        &mut sink,
        xml::EmitterConfig::new().pad_self_closing(false),
    );

    let mut ser = serde_xml_rs::Serializer::new_from_writer(event_writer);
    s.serialize(&mut ser)?;

    Ok(String::from_utf8(sink)?)
}

pub fn from_str<'de, T: serde::Deserialize<'de>>(xml_str: &'de str) -> Result<T> {
    let mut de = serde_xml_rs::Deserializer::new(xml::EventReader::new_with_config(
        xml_str.as_bytes(),
        xml::ParserConfig::new()
            .trim_whitespace(true)
            .whitespace_to_characters(false)
            .cdata_to_characters(true)
            .coalesce_characters(true),
    ));

    Ok(T::deserialize(&mut de)?)
}
