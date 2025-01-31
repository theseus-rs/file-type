use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960118: FileFormat = FileFormat {
    id: 27_960_118,
    puid: "wikidata/27960118",
    name: "Sony Wave64",
    extensions: &["w64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
