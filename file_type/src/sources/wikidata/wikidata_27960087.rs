use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960087: FileFormat = FileFormat {
    id: 27_960_087,
    puid: "wikidata/27960087",
    name: "Memory Stick Voice",
    extensions: &["msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
