use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1051507719: FileType = FileType {
    file_format: &FileFormat {
        id: 1_051_507_719,
        source_type: SourceType::Httpd,
        name: "flv",
        extensions: &["flv"],
        media_types: &["video/x-flv"],
        signatures: &[],
        related_formats: &[],
    },
};
