use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_45: FileType = FileType {
    file_format: &FileFormat {
        id: 45,
        source_type: SourceType::Pronom,
        name: "Comma Separated Values",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[],
        related_formats: &[],
    },
};
