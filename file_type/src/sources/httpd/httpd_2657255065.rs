use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2657255065: FileType = FileType {
    file_format: &FileFormat {
        id: 2_657_255_065,
        source_type: SourceType::Httpd,
        name: "icon",
        extensions: &["ico"],
        media_types: &["image/x-icon"],
        signatures: &[],
        related_formats: &[],
    },
};
