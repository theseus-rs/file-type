use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2618: FileFormat = FileFormat {
    id: 2_618,
    source_type: SourceType::Pronom,
    name: "C Source Code File",
    extensions: &["c"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
