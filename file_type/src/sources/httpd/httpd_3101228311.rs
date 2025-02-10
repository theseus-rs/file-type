use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3101228311: FileType = FileType {
    file_format: &FileFormat {
        id: 3_101_228_311,
        source_type: SourceType::Httpd,
        name: "lost xml",
        extensions: &["lostxml"],
        media_types: &["application/lost+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
