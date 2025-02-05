use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_239: FileFormat = FileFormat {
    id: 239,
    source_type: SourceType::Pronom,
    name: "Adobe PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
