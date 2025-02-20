use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_439: FileType = FileType {
    file_format: &FileFormat {
        id: 439,
        source_type: SourceType::Pronom,
        name: "IBM DisplayWrite Document",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
