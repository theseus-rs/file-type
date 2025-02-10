use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2030495307: FileType = FileType {
    file_format: &FileFormat {
        id: 2_030_495_307,
        source_type: SourceType::Httpd,
        name: "apple installer xml",
        extensions: &["mpkg"],
        media_types: &["application/vnd.apple.installer+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
