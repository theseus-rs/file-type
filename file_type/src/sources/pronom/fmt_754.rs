use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_754: FileFormat = FileFormat {
    id: 1_553,
    puid: "fmt/754",
    name: "Microsoft Word Document (Password Protected)",
    extensions: &["wbk", "doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_554,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 690,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_401,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 688,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
