use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_977: FileFormat = FileFormat {
    id: 977,
    source_type: SourceType::Pronom,
    name: "Structured Data Exchange Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
