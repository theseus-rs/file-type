use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_380: FileFormat = FileFormat {
    id: 380,
    source_type: SourceType::Pronom,
    name: "WordStar for Windows Document",
    extensions: &["ws", "wsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 286,
    }],
};
