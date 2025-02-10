use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_495: FileType = FileType {
    file_format: &FileFormat {
        id: 495,
        source_type: SourceType::Pronom,
        name: "Lotus 1-2-3 Spreadsheet Formatting File",
        extensions: &["fm3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
