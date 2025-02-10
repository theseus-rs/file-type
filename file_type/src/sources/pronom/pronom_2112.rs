use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2112: FileType = FileType {
    file_format: &FileFormat {
        id: 2_112,
        source_type: SourceType::Pronom,
        name: "602Tab Spreadsheet",
        extensions: &["wls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
