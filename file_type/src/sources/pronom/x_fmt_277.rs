use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_277: FileFormat = FileFormat {
    id: 422,
    puid: "x-fmt/277",
    name: "Real Video",
    extensions: &["rv"],
    media_types: &["video/vnd.rn-realvideo"],
    internal_signatures: &[],
    related_formats: &[],
};
