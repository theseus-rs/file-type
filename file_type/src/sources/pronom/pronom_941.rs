use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_941: FileFormat = FileFormat {
    id: 941,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint for Windows",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_311,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_681,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_428,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_429,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_430,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_431,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_432,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_436,
        },
    ],
};
