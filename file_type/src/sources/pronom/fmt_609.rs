use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_609: FileFormat = FileFormat {
    id: 1_401,
    puid: "fmt/609",
    name: "Microsoft Word (Generic)",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 76,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 688,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 690,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_553,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_554,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
