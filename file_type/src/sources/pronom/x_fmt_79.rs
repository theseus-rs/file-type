use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_79: FileFormat = FileFormat {
    id: 121,
    puid: "x-fmt/79",
    name: "AutoCAD Plot Configuration File",
    extensions: &["pcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
