use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2572911062: FileType = FileType {
    file_format: &FileFormat {
        id: 2_572_911_062,
        source_type: SourceType::Httpd,
        name: "kde kivio",
        extensions: &["flw"],
        media_types: &["application/vnd.kde.kivio"],
        signatures: &[],
        related_formats: &[],
    },
};
