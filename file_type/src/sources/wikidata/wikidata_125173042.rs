use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125173042: FileFormat = FileFormat {
    id: 125_173_042,
    puid: "wikidata/125173042",
    name: "Tomboy note",
    extensions: &["note"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
