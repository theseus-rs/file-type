use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_247: FileFormat = FileFormat {
    id: 247,
    source_type: SourceType::Pronom,
    name: "PageMaker PC Document",
    extensions: &["pm6", "pt6"],
    media_types: &["application/vnd.pagemaker"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 2_555,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_680,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 254,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_554,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 246,
        },
    ],
};
