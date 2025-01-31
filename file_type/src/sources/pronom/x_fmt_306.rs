use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_306: FileFormat = FileFormat {
    id: 464,
    puid: "x-fmt/306",
    name: "AutoSketch Drawing",
    extensions: &["skf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
