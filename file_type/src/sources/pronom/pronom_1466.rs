use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1466: FileFormat = FileFormat {
    id: 1_466,
    source_type: SourceType::Pronom,
    name: "Photoshop Curve File",
    extensions: &["acv", "atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
