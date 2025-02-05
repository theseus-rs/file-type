use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_673108048: FileFormat = FileFormat {
    id: 673_108_048,
    source_type: SourceType::Httpd,
    name: "cdmi domain",
    extensions: &["cdmid"],
    media_types: &["application/cdmi-domain"],
    signatures: &[],
    related_formats: &[],
};
