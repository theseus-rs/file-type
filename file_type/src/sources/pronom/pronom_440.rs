use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_440: FileFormat = FileFormat {
    id: 440,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
