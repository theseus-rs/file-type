use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_809: FileType = FileType {
    file_format: &FileFormat {
        id: 809,
        source_type: SourceType::Pronom,
        name: "Java Language Source Code File",
        extensions: &["java"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
