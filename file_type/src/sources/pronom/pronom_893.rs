use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_893: FileType = FileType {
    file_format: &FileFormat {
        id: 893,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor for DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
