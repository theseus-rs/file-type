use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_440: FileFormat = FileFormat {
    id: 440,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Document",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
