use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2831585141: FileFormat = FileFormat {
    id: 2_831_585_141,
    source_type: SourceType::Httpd,
    name: "mads xml",
    extensions: &["mads"],
    media_types: &["application/mads+xml"],
    signatures: &[],
    related_formats: &[],
};
