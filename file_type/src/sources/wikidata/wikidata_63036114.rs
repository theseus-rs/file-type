use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63036114: FileFormat = FileFormat {
    id: 63_036_114,
    source_type: SourceType::Wikidata,
    name: "8-bit ANSI Text",
    extensions: &["ans"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
