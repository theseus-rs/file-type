use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_60: FileFormat = FileFormat {
    id: 101,
    puid: "x-fmt/60",
    name: "AutoCAD Linetype Definition File",
    extensions: &["lin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
