use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3019556387: FileFormat = FileFormat {
    id: 3_019_556_387,
    source_type: SourceType::Httpd,
    name: "thraud xml",
    extensions: &["tfi"],
    media_types: &["application/thraud+xml"],
    signatures: &[],
    related_formats: &[],
};
