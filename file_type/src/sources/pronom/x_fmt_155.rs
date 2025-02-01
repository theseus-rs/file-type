use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_155: FileFormat = FileFormat {
    id: 218,
    puid: "x-fmt/155",
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
