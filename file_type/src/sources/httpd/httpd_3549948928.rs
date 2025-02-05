use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3549948928: FileFormat = FileFormat {
    id: 3_549_948_928,
    source_type: SourceType::Httpd,
    name: "pls xml",
    extensions: &["pls"],
    media_types: &["application/pls+xml"],
    signatures: &[],
    related_formats: &[],
};
