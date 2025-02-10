use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1764878474: FileType = FileType {
    file_format: &FileFormat {
        id: 1_764_878_474,
        source_type: SourceType::Httpd,
        name: "dwf",
        extensions: &["dwf"],
        media_types: &["model/vnd.dwf"],
        signatures: &[],
        related_formats: &[],
    },
};
