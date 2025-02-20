use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1049349370: FileType = FileType {
    file_format: &FileFormat {
        id: 1_049_349_370,
        source_type: SourceType::Httpd,
        name: "atomsvc xml",
        extensions: &["atomsvc"],
        media_types: &["application/atomsvc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
