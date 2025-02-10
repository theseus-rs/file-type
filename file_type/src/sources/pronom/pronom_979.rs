use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_979: FileType = FileType {
    file_format: &FileFormat {
        id: 979,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
