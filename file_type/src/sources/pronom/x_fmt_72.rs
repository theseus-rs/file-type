use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_72: FileFormat = FileFormat {
    id: 114,
    puid: "x-fmt/72",
    name: "AutoCAD Template Menu File",
    extensions: &["mnu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
