use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2812998300: FileType = FileType {
    file_format: &FileFormat {
        id: 2_812_998_300,
        source_type: SourceType::Httpd,
        name: "x509 ca cert",
        extensions: &["der", "crt"],
        media_types: &["application/x-x509-ca-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
