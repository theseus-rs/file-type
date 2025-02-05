use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2114: FileFormat = FileFormat {
    id: 2_114,
    source_type: SourceType::Pronom,
    name: "Calendar Creator File",
    extensions: &["cc3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_115,
    }],
};
