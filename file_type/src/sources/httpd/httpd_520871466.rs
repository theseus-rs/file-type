use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_520871466: FileType = FileType {
    file_format: &FileFormat {
        id: 520_871_466,
        source_type: SourceType::Httpd,
        name: "pkix cert",
        extensions: &["cer"],
        media_types: &["application/pkix-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
