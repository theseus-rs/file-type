use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_373: FileFormat = FileFormat {
    id: 549,
    puid: "x-fmt/373",
    name: "XYWrite Document",
    extensions: &["xy4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
