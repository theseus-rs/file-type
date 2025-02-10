use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1681: FileType = FileType {
    file_format: &FileFormat {
        id: 1_681,
        source_type: SourceType::Pronom,
        name: "Corel Presentation",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
