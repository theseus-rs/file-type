use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_286: FileFormat = FileFormat {
    id: 286,
    source_type: SourceType::Pronom,
    name: "WordStar for Windows Document",
    extensions: &["wsd", "ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 380,
    }],
};
