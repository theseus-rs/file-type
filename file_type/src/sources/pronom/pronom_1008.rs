use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1008: FileType = FileType {
    file_format: &FileFormat {
        id: 1_008,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet for Macintosh",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
