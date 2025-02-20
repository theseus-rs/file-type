use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_170: FileType = FileType {
    file_format: &FileFormat {
        id: 170,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
