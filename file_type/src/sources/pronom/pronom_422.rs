use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_422: FileFormat = FileFormat {
    id: 422,
    source_type: SourceType::Pronom,
    name: "Real Video",
    extensions: &["rv"],
    media_types: &["video/vnd.rn-realvideo"],
    signatures: &[],
    related_formats: &[],
};
