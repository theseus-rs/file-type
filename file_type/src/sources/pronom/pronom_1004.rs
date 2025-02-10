use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1004: FileType = FileType {
    file_format: &FileFormat {
        id: 1_004,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
