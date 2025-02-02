use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1206: FileFormat = FileFormat {
    id: 1_206,
    source_type: SourceType::Pronom,
    name: "OpenDocument Database Format",
    extensions: &["odb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 777,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 778,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_231,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 778,
        },
    ],
};
