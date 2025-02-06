use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_158: FileFormat = FileFormat {
    id: 158,
    source_type: SourceType::Pronom,
    name: "Scalable Vector Graphics Compressed",
    extensions: &["svgz"],
    media_types: &["image/svg+xml"],
    signatures: &[],
    related_formats: &[],
};
