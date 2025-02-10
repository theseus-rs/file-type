use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1875755222: FileType = FileType {
    file_format: &FileFormat {
        id: 1_875_755_222,
        source_type: SourceType::Httpd,
        name: "h263",
        extensions: &["h263"],
        media_types: &["video/h263"],
        signatures: &[],
        related_formats: &[],
    },
};
