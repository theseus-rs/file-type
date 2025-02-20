use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1001: FileType = FileType {
    file_format: &FileFormat {
        id: 1_001,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
