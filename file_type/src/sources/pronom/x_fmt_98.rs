use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_98: FileFormat = FileFormat {
    id: 146,
    puid: "x-fmt/98",
    name: "AutoCAD ACIS Export File",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
