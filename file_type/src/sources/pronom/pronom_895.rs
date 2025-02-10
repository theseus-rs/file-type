use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_895: FileType = FileType {
    file_format: &FileFormat {
        id: 895,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for DOS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
