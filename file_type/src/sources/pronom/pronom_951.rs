use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_951: FileType = FileType {
    file_format: &FileFormat {
        id: 951,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
