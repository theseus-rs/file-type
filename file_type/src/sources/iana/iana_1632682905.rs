use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1632682905: FileFormat = FileFormat {
    id: 1_632_682_905,
    source_type: SourceType::Iana,
    name: "expect-ct-report+json",
    extensions: &[],
    media_types: &["application/expect-ct-report+json"],
    signatures: &[],
    related_formats: &[],
};
