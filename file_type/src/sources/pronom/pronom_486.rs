use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_486: FileFormat = FileFormat {
    id: 486,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
