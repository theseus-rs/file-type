use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_112: FileFormat = FileFormat {
    id: 164,
    puid: "x-fmt/112",
    name: "AutoCAD External Database Configuration File",
    extensions: &["udl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
