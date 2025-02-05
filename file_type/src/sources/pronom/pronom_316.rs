use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_316: FileFormat = FileFormat {
    id: 316,
    source_type: SourceType::Pronom,
    name: "Cascading Style Sheet",
    extensions: &["css"],
    media_types: &["text/css"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 635,
    }],
};
