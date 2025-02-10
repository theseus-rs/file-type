use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1764878474: FileType = FileType {
    file_format: &FileFormat {
        id: 1_764_878_474,
        source_type: SourceType::Iana,
        name: "vnd.dwf",
        extensions: &[],
        media_types: &["model/vnd.dwf"],
        signatures: &[],
        related_formats: &[],
    },
};
