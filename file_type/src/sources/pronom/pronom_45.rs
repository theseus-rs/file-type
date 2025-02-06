use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_45: FileFormat = FileFormat {
    id: 45,
    source_type: SourceType::Pronom,
    name: "Comma Separated Values",
    extensions: &["csv"],
    media_types: &["text/csv"],
    signatures: &[],
    related_formats: &[],
};
