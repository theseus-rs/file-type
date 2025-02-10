use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_486: FileType = FileType {
    file_format: &FileFormat {
        id: 486,
        source_type: SourceType::Pronom,
        name: "Framework Database",
        extensions: &["fw4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
