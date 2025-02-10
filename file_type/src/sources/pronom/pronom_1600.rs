use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1600: FileType = FileType {
    file_format: &FileFormat {
        id: 1_600,
        source_type: SourceType::Pronom,
        name: "CSV Schema",
        extensions: &["csvs"],
        media_types: &["text/csv-schema"],
        signatures: &[],
        related_formats: &[],
    },
};
