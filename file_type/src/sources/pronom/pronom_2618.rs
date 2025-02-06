use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2618: FileFormat = FileFormat {
    id: 2_618,
    source_type: SourceType::Pronom,
    name: "C Source Code File",
    extensions: &["c"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
