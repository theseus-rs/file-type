use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_439: FileFormat = FileFormat {
    id: 439,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Document",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
