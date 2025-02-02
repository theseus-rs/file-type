use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_45: FileFormat = FileFormat {
    id: 45,
    source_type: SourceType::Pronom,
    name: "Comma Separated Values",
    extensions: &["csv"],
    media_types: &["text/csv"],
    internal_signatures: &[],
    related_formats: &[],
};
