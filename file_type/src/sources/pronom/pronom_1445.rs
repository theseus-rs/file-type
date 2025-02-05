use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1445: FileFormat = FileFormat {
    id: 1_445,
    source_type: SourceType::Pronom,
    name: "Apple iWork Keynote",
    extensions: &["key"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
