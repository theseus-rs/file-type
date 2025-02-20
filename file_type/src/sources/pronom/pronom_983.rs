use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_983: FileType = FileType {
    file_format: &FileFormat {
        id: 983,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
