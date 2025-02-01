use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_423: FileFormat = FileFormat {
    id: 810,
    puid: "x-fmt/423",
    name: "JavaScript file",
    extensions: &["js"],
    media_types: &["application/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
