use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_422: FileFormat = FileFormat {
    id: 422,
    source_type: SourceType::Pronom,
    name: "Real Video",
    extensions: &["rv"],
    media_types: &["video/vnd.rn-realvideo"],
    internal_signatures: &[],
    related_formats: &[],
};
