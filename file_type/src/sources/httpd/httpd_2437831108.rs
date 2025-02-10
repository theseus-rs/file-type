use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2437831108: FileType = FileType {
    file_format: &FileFormat {
        id: 2_437_831_108,
        source_type: SourceType::Httpd,
        name: "kde karbon",
        extensions: &["karbon"],
        media_types: &["application/vnd.kde.karbon"],
        signatures: &[],
        related_formats: &[],
    },
};
