use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_755: FileFormat = FileFormat {
    id: 1_554,
    puid: "fmt/755",
    name: "Microsoft Word Document Template (Password Protected)",
    extensions: &["dot"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 76,
            relationship_type: RelationshipType::HasPriorityOver,
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
            id: 1_553,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
