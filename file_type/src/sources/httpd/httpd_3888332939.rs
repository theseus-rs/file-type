use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3888332939: FileType = FileType {
    file_format: &FileFormat {
        id: 3_888_332_939,
        source_type: SourceType::Httpd,
        name: "pkcs7 certificates",
        extensions: &["p7b", "spc"],
        media_types: &["application/x-pkcs7-certificates"],
        signatures: &[],
        related_formats: &[],
    },
};
