use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1322499123: FileFormat = FileFormat {
    id: 1_322_499_123,
    source_type: SourceType::Httpd,
    name: "xenc xml",
    extensions: &["xenc"],
    media_types: &["application/xenc+xml"],
    signatures: &[],
    related_formats: &[],
};
