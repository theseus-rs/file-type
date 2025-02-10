use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_996: FileType = FileType {
    file_format: &FileFormat {
        id: 996,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor 5-6",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
