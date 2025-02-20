use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1011: FileType = FileType {
    file_format: &FileFormat {
        id: 1_011,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor Macintosh",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
