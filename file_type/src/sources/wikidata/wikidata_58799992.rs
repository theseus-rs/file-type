use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58799992: FileFormat = FileFormat {
    id: 58_799_992,
    puid: "wikidata/58799992",
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
