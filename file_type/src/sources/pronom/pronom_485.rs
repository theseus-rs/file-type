use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_485: FileFormat = FileFormat {
    id: 485,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
