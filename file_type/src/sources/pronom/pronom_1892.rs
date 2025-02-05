use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1892: FileFormat = FileFormat {
    id: 1_892,
    source_type: SourceType::Pronom,
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
