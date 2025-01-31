use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_210: FileFormat = FileFormat {
    id: 291,
    puid: "x-fmt/210",
    name: "XYWrite Document",
    extensions: &["xy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
