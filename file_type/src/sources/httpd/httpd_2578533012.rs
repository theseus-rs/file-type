use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2578533012: FileType = FileType {
    file_format: &FileFormat {
        id: 2_578_533_012,
        source_type: SourceType::Httpd,
        name: "kde kontour",
        extensions: &["kon"],
        media_types: &["application/vnd.kde.kontour"],
        signatures: &[],
        related_formats: &[],
    },
};
