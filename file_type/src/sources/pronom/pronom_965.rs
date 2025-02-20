use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_965: FileType = FileType {
    file_format: &FileFormat {
        id: 965,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
