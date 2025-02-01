use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_262: FileFormat = FileFormat {
    id: 380,
    puid: "x-fmt/262",
    name: "WordStar for Windows Document",
    extensions: &["ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 286,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
