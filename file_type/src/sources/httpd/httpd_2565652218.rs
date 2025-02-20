use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2565652218: FileType = FileType {
    file_format: &FileFormat {
        id: 2_565_652_218,
        source_type: SourceType::Httpd,
        name: "previewsystems box",
        extensions: &["box"],
        media_types: &["application/vnd.previewsystems.box"],
        signatures: &[],
        related_formats: &[],
    },
};
