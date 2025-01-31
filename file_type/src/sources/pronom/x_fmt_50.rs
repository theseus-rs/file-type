use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_50: FileFormat = FileFormat {
    id: 82,
    puid: "x-fmt/50",
    name: "AutoCAD Drawing Standards File",
    extensions: &["dws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
