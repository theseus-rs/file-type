use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2250: FileFormat = FileFormat {
    id: 2_250,
    source_type: SourceType::Pronom,
    name: "Minitab Worksheet",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
