use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_174: FileFormat = FileFormat {
    id: 247,
    puid: "x-fmt/174",
    name: "PageMaker PC Document",
    extensions: &["pm6", "pt6"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_555,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 1_680,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 254,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_554,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 246,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
