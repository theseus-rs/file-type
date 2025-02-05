use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2755564633: FileFormat = FileFormat {
    id: 2_755_564_633,
    source_type: SourceType::Iana,
    name: "shaclc",
    extensions: &[],
    media_types: &["text/shaclc"],
    signatures: &[],
    related_formats: &[],
};
