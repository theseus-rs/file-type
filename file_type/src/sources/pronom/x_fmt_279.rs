use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_279: FileFormat = FileFormat {
    id: 425,
    puid: "x-fmt/279",
    name: "MPEG 1/2 Audio Layer 3 Streaming",
    extensions: &["m3u"],
    media_types: &["audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 923,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
