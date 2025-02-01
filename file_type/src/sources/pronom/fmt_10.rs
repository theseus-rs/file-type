use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_10: FileFormat = FileFormat {
    id: 612,
    puid: "fmt/10",
    name: "Tagged Image File Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 672,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 673,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 752,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 795,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 796,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 797,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 799,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
