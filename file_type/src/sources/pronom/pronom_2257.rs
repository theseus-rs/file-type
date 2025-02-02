use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2257: FileFormat = FileFormat {
    id: 2_257,
    source_type: SourceType::Pronom,
    name: "Apple iWork Pages",
    extensions: &["pages"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
