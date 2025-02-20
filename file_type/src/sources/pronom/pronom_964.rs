use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_964: FileType = FileType {
    file_format: &FileFormat {
        id: 964,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
