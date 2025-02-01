use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_21: FileFormat = FileFormat {
    id: 50,
    puid: "x-fmt/21",
    name: "7-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
