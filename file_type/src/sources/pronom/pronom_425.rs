use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_425: FileFormat = FileFormat {
    id: 425,
    source_type: SourceType::Pronom,
    name: "MPEG 1/2 Audio Layer 3 Streaming",
    extensions: &["m3u"],
    media_types: &["audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 923,
    }],
};
