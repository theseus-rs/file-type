use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_913317735: FileType = FileType {
    file_format: &FileFormat {
        id: 913_317_735,
        source_type: SourceType::Httpd,
        name: "doom",
        extensions: &["wad"],
        media_types: &["application/x-doom"],
        signatures: &[],
        related_formats: &[],
    },
};
