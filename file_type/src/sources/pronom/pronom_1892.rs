use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1892: FileType = FileType {
    file_format: &FileFormat {
        id: 1_892,
        source_type: SourceType::Pronom,
        name: "Hangul Word Processor Document",
        extensions: &["hwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
