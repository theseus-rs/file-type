use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2618: FileType = FileType {
    file_format: &FileFormat {
        id: 2_618,
        source_type: SourceType::Pronom,
        name: "C Source Code File",
        extensions: &["c"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
