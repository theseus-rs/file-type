use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
