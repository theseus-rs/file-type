use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_68: FileFormat = FileFormat {
    id: 109,
    puid: "x-fmt/68",
    name: "AutoCAD Compiled Menu",
    extensions: &["mnc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
