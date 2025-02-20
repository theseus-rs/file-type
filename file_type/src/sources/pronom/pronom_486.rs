use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
