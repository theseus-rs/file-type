use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58876002: FileFormat = FileFormat {
    id: 58_876_002,
    puid: "wikidata/58876002",
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
