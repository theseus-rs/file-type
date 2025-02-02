use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_940: FileFormat = FileFormat {
    id: 940,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel for Windows",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_679,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_387,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_390,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_426,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_427,
        },
    ],
};
