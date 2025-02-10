use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1009: FileType = FileType {
    file_format: &FileFormat {
        id: 1_009,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Macintosh",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
