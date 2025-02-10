use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_70571009: FileType = FileType {
    file_format: &FileFormat {
        id: 70_571_009,
        source_type: SourceType::Httpd,
        name: "dssc der",
        extensions: &["dssc"],
        media_types: &["application/dssc+der"],
        signatures: &[],
        related_formats: &[],
    },
};
