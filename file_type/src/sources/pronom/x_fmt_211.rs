use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_211: FileFormat = FileFormat {
    id: 292,
    puid: "x-fmt/211",
    name: "XYWrite Document",
    extensions: &["xy3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
