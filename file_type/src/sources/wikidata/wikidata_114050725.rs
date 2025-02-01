use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114050725: FileFormat = FileFormat {
    id: 114_050_725,
    puid: "wikidata/114050725",
    name: "Canon CIF File",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
