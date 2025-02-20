use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_992: FileType = FileType {
    file_format: &FileFormat {
        id: 992,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
