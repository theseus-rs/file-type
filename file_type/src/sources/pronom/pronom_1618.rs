use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1618: FileFormat = FileFormat {
    id: 1_618,
    source_type: SourceType::Pronom,
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
