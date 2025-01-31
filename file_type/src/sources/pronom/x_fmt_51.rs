use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_51: FileFormat = FileFormat {
    id: 83,
    puid: "x-fmt/51",
    name: "AutoCAD Drawing Template",
    extensions: &["dwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
