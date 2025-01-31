use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_169: FileFormat = FileFormat {
    id: 241,
    puid: "x-fmt/169",
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
