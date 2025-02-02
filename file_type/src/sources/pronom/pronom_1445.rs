use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1445: FileFormat = FileFormat {
    id: 1_445,
    source_type: SourceType::Pronom,
    name: "Apple iWork Keynote",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
