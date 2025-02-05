use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2251: FileFormat = FileFormat {
    id: 2_251,
    source_type: SourceType::Pronom,
    name: "Minitab Worksheet",
    extensions: &["mtw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
