use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_39470048: FileFormat = FileFormat {
    id: 39_470_048,
    source_type: SourceType::Httpd,
    name: "pdf",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
