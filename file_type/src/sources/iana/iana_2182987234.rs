use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2182987234: FileFormat = FileFormat {
    id: 2_182_987_234,
    source_type: SourceType::Iana,
    name: "json",
    extensions: &[],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
