use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_149: FileFormat = FileFormat {
    id: 210,
    puid: "x-fmt/149",
    name: "Desktop Color Separation File",
    extensions: &["dcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
