use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_963: FileType = FileType {
    file_format: &FileFormat {
        id: 963,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor 3-4 for Windows",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
