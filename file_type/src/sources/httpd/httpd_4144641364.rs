use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4144641364: FileFormat = FileFormat {
    id: 4_144_641_364,
    source_type: SourceType::Httpd,
    name: "xliff xml",
    extensions: &["xlf"],
    media_types: &["application/x-xliff+xml"],
    signatures: &[],
    related_formats: &[],
};
