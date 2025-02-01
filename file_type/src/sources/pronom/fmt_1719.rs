use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1719: FileFormat = FileFormat {
    id: 2_555,
    puid: "fmt/1719",
    name: "PageMaker Mac Document",
    extensions: &["pm6", "pt6"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 247,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 1_680,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_554,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_523,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
