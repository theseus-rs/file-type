use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_602521492: FileFormat = FileFormat {
    id: 602_521_492,
    source_type: SourceType::Httpd,
    name: "tei xml",
    extensions: &["tei", "teicorpus"],
    media_types: &["application/tei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
