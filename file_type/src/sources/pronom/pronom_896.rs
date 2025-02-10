use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_896: FileType = FileType {
    file_format: &FileFormat {
        id: 896,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
