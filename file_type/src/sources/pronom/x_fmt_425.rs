use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_425: FileFormat = FileFormat {
    id: 812,
    puid: "x-fmt/425",
    name: "Generic Library File",
    extensions: &["lib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
