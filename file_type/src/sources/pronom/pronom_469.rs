use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_469: FileType = FileType {
    file_format: &FileFormat {
        id: 469,
        source_type: SourceType::Pronom,
        name: "dBASE Text Memo",
        extensions: &["dbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
