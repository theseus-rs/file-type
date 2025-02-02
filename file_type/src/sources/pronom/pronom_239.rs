use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_239: FileFormat = FileFormat {
    id: 239,
    source_type: SourceType::Pronom,
    name: "Adobe PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
