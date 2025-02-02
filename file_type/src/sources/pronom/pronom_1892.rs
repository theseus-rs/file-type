use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1892: FileFormat = FileFormat {
    id: 1_892,
    source_type: SourceType::Pronom,
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
