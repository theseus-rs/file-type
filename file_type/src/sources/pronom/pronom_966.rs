use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_966: FileType = FileType {
    file_format: &FileFormat {
        id: 966,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
