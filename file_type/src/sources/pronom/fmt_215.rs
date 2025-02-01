use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_215: FileFormat = FileFormat {
    id: 941,
    puid: "fmt/215",
    name: "Microsoft Powerpoint for Windows",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_311,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_681,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_428,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_429,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_430,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_431,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_432,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_436,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
