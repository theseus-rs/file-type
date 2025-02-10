use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3186144488: FileType = FileType {
    file_format: &FileFormat {
        id: 3_186_144_488,
        source_type: SourceType::Httpd,
        name: "plain",
        extensions: &["txt", "text", "conf", "def", "list", "log", "in"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
