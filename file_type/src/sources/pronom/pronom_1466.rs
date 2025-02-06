use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1466: FileFormat = FileFormat {
    id: 1_466,
    source_type: SourceType::Pronom,
    name: "Photoshop Curve File",
    extensions: &["acv", "atf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
