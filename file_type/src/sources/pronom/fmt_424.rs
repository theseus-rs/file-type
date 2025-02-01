use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_424: FileFormat = FileFormat {
    id: 1_206,
    puid: "fmt/424",
    name: "OpenDocument Database Format",
    extensions: &["odb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 777,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_231,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
