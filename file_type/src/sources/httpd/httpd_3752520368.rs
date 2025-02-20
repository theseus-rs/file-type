use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3752520368: FileType = FileType {
    file_format: &FileFormat {
        id: 3_752_520_368,
        source_type: SourceType::Httpd,
        name: "pkcs7 signature",
        extensions: &["p7s"],
        media_types: &["application/pkcs7-signature"],
        signatures: &[],
        related_formats: &[],
    },
};
