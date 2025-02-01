use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_422: FileFormat = FileFormat {
    id: 809,
    puid: "x-fmt/422",
    name: "Java Language Source Code File",
    extensions: &["java"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
