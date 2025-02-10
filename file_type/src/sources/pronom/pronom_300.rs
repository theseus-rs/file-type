use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_300: FileType = FileType {
    file_format: &FileFormat {
        id: 300,
        source_type: SourceType::Pronom,
        name: "Quicken Data File",
        extensions: &["abd", "qdf", "qel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
