use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_203: FileFormat = FileFormat {
    id: 281,
    puid: "x-fmt/203",
    name: "WordPerfect for Windows Document",
    extensions: &["w52", "wp", "wpd", "wp5"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 75,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 737,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
