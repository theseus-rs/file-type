use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125914662: FileFormat = FileFormat {
    id: 125_914_662,
    puid: "wikidata/125914662",
    name: "Sandboxels Save File",
    extensions: &["sbxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
