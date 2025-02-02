use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_630: FileFormat = FileFormat {
    id: 630,
    source_type: SourceType::Pronom,
    name: "Rich Text Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 631,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 633,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 753,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_101,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 631,
        },
    ],
};
