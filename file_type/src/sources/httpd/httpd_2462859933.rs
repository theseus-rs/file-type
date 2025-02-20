use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2462859933: FileType = FileType {
    file_format: &FileFormat {
        id: 2_462_859_933,
        source_type: SourceType::Httpd,
        name: "ms pki seccat",
        extensions: &["cat"],
        media_types: &["application/vnd.ms-pki.seccat"],
        signatures: &[],
        related_formats: &[],
    },
};
