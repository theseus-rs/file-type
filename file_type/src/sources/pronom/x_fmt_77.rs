use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_77: FileFormat = FileFormat {
    id: 119,
    puid: "x-fmt/77",
    name: "AutoCAD Plot Configuration File",
    extensions: &["pc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
