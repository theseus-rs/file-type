use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_800: FileFormat = FileFormat {
    id: 1_600,
    puid: "fmt/800",
    name: "CSV Schema",
    extensions: &["csvs"],
    media_types: &["text/csv-schema"],
    internal_signatures: &[],
    related_formats: &[],
};
