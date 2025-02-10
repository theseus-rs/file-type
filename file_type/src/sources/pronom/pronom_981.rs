use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_981: FileType = FileType {
    file_format: &FileFormat {
        id: 981,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
