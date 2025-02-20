use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1500: FileType = FileType {
    file_format: &FileFormat {
        id: 1_500,
        source_type: SourceType::Pronom,
        name: "Processing Development Environment",
        extensions: &["pde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
