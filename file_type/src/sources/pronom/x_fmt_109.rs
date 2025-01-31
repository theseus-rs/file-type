use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_109: FileFormat = FileFormat {
    id: 158,
    puid: "x-fmt/109",
    name: "Scalable Vector Graphics Compressed",
    extensions: &["svgz"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
