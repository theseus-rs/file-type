use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_281: FileFormat = FileFormat {
    id: 281,
    source_type: SourceType::Pronom,
    name: "WordPerfect for Windows Document",
    extensions: &["w52", "wp", "wpd", "wp5"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 75,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 737,
        },
    ],
};
