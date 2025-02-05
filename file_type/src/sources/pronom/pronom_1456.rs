use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1456: FileFormat = FileFormat {
    id: 1_456,
    source_type: SourceType::Pronom,
    name: "Open XML Paper Specification",
    extensions: &["xps", "oxps"],
    media_types: &["application/oxps"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_630,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_782,
        },
    ],
};
