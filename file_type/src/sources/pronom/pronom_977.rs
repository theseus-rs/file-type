use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_977: FileFormat = FileFormat {
    id: 977,
    source_type: SourceType::Pronom,
    name: "Structured Data Exchange Format",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
