use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_950: FileType = FileType {
    file_format: &FileFormat {
        id: 950,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
