use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51802172: FileFormat = FileFormat {
    id: 51_802_172,
    puid: "wikidata/51802172",
    name: "Speller Custom Dictionary",
    extensions: &["dic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
