use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_214: FileFormat = FileFormat {
    id: 940,
    puid: "fmt/214",
    name: "Microsoft Excel for Windows",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_679,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_387,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_390,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_426,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_427,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
