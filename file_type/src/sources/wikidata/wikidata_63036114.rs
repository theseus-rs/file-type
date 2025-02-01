use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63036114: FileFormat = FileFormat {
    id: 63_036_114,
    puid: "wikidata/63036114",
    name: "8-bit ANSI Text",
    extensions: &["ans"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
