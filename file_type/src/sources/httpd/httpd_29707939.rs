use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_29707939: FileFormat = FileFormat {
    id: 29_707_939,
    source_type: SourceType::Httpd,
    name: "woff",
    extensions: &["woff"],
    media_types: &["font/woff"],
    signatures: &[],
    related_formats: &[],
};
