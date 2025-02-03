use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_158: FileFormat = FileFormat {
    id: 158,
    source_type: SourceType::Pronom,
    name: "Scalable Vector Graphics Compressed",
    extensions: &["svgz"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
