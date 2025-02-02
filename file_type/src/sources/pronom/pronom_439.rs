use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_439: FileFormat = FileFormat {
    id: 439,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
