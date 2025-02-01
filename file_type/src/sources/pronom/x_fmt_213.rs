use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_213: FileFormat = FileFormat {
    id: 300,
    puid: "x-fmt/213",
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
