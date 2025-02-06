use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2257: FileFormat = FileFormat {
    id: 2_257,
    source_type: SourceType::Pronom,
    name: "Apple iWork Pages",
    extensions: &["pages"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
