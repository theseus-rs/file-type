use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3889997362: FileType = FileType {
    file_format: &FileFormat {
        id: 3_889_997_362,
        source_type: SourceType::Httpd,
        name: "jpm",
        extensions: &["jpm", "jpgm"],
        media_types: &["video/jpm"],
        signatures: &[],
        related_formats: &[],
    },
};
