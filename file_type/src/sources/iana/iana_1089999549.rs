use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1089999549: FileFormat = FileFormat {
    id: 1_089_999_549,
    source_type: SourceType::Iana,
    name: "csv-schema",
    extensions: &[],
    media_types: &["text/csv-schema"],
    internal_signatures: &[],
    related_formats: &[],
};
