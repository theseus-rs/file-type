use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_54: FileFormat = FileFormat {
    id: 88,
    puid: "x-fmt/54",
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
