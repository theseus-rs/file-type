use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1000: FileType = FileType {
    file_format: &FileFormat {
        id: 1_000,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
