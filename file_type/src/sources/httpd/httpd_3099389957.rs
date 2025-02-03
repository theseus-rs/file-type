use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3099389957: FileFormat = FileFormat {
    id: 3_099_389_957,
    source_type: SourceType::Httpd,
    name: "abiword",
    extensions: &["abw"],
    media_types: &["application/x-abiword"],
    internal_signatures: &[],
    related_formats: &[],
};
