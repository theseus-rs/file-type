use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_380: FileFormat = FileFormat {
    id: 380,
    source_type: SourceType::Pronom,
    name: "WordStar for Windows Document",
    extensions: &["ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 286,
    }],
};
