use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1618: FileType = FileType {
    file_format: &FileFormat {
        id: 1_618,
        source_type: SourceType::Pronom,
        name: "YAML",
        extensions: &["yaml", "yml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
