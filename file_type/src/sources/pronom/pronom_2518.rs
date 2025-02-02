use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2518: FileFormat = FileFormat {
    id: 2_518,
    source_type: SourceType::Pronom,
    name: "EndNote Library",
    extensions: &["enl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_520,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_521,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_070,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 2_519,
        },
    ],
};
