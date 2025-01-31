use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100243992: FileFormat = FileFormat {
    id: 100_243_992,
    puid: "wikidata/100243992",
    name: "Student Writing Center Sign",
    extensions: &["sg", "sgt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
