use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1618: FileFormat = FileFormat {
    id: 1_618,
    source_type: SourceType::Pronom,
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
