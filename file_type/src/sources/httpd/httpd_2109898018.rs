use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2109898018: FileFormat = FileFormat {
    id: 2_109_898_018,
    source_type: SourceType::Httpd,
    name: "woff2",
    extensions: &["woff2"],
    media_types: &["font/woff2"],
    signatures: &[],
    related_formats: &[],
};
