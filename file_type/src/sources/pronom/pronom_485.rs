use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_485: FileFormat = FileFormat {
    id: 485,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
