use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1600: FileFormat = FileFormat {
    id: 1_600,
    source_type: SourceType::Pronom,
    name: "CSV Schema",
    extensions: &["csvs"],
    media_types: &["text/csv-schema"],
    internal_signatures: &[],
    related_formats: &[],
};
