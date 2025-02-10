use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_962: FileType = FileType {
    file_format: &FileFormat {
        id: 962,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
