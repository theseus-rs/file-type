use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1946098896: FileFormat = FileFormat {
    id: 1_946_098_896,
    source_type: SourceType::Httpd,
    name: "yamaha hv script",
    extensions: &["hvs"],
    media_types: &["application/vnd.yamaha.hv-script"],
    internal_signatures: &[],
    related_formats: &[],
};
