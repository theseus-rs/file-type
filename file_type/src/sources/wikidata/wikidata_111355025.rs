use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355025: FileFormat = FileFormat {
    id: 111_355_025,
    puid: "wikidata/111355025",
    name: "exponential 8-bit format",
    extensions: &["u255law"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
