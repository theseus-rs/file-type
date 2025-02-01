use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_206: FileFormat = FileFormat {
    id: 286,
    puid: "x-fmt/206",
    name: "WordStar for Windows Document",
    extensions: &["wsd", "ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 380,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
