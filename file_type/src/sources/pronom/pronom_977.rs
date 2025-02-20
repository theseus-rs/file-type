use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_977: FileType = FileType {
    file_format: &FileFormat {
        id: 977,
        source_type: SourceType::Pronom,
        name: "Structured Data Exchange Format",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
