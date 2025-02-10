use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_70821616: FileType = FileType {
    file_format: &FileFormat {
        id: 70_821_616,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
