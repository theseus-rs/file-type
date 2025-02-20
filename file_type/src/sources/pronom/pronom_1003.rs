use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1003: FileType = FileType {
    file_format: &FileFormat {
        id: 1_003,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
