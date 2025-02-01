use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_287: FileFormat = FileFormat {
    id: 438,
    puid: "x-fmt/287",
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
