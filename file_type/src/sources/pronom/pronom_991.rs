use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_991: FileType = FileType {
    file_format: &FileFormat {
        id: 991,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
