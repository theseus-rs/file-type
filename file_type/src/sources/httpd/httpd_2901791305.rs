use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2901791305: FileType = FileType {
    file_format: &FileFormat {
        id: 2_901_791_305,
        source_type: SourceType::Httpd,
        name: "ms wmx",
        extensions: &["wmx"],
        media_types: &["video/x-ms-wmx"],
        signatures: &[],
        related_formats: &[],
    },
};
