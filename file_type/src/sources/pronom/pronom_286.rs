use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_286: FileFormat = FileFormat {
    id: 286,
    source_type: SourceType::Pronom,
    name: "WordStar for Windows Document",
    extensions: &["wsd", "ws", "wsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 380,
    }],
};
