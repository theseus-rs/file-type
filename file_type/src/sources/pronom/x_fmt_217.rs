use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_217: FileFormat = FileFormat {
    id: 308,
    puid: "x-fmt/217",
    name: "Adobe ACD",
    extensions: &["acd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
