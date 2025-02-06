use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1600: FileFormat = FileFormat {
    id: 1_600,
    source_type: SourceType::Pronom,
    name: "CSV Schema",
    extensions: &["csvs"],
    media_types: &["text/csv-schema"],
    signatures: &[],
    related_formats: &[],
};
