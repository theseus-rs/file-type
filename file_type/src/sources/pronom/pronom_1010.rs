use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1010: FileType = FileType {
    file_format: &FileFormat {
        id: 1_010,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor Macintosh",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
