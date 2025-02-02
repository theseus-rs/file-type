use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1624: FileFormat = FileFormat {
    id: 1_624,
    source_type: SourceType::Pronom,
    name: "Apple iWork Pages",
    extensions: &["pages"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
