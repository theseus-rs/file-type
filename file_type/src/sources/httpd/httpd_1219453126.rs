use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1219453126: FileFormat = FileFormat {
    id: 1_219_453_126,
    source_type: SourceType::Httpd,
    name: "dart",
    extensions: &["dart"],
    media_types: &["application/vnd.dart"],
    signatures: &[],
    related_formats: &[],
};
