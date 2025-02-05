use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_609: FileFormat = FileFormat {
    id: 609,
    source_type: SourceType::Pronom,
    name: "Tagged Image File Format",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 672,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 673,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 752,
        },
    ],
};
