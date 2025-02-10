use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_158: FileType = FileType {
    file_format: &FileFormat {
        id: 158,
        source_type: SourceType::Pronom,
        name: "Scalable Vector Graphics Compressed",
        extensions: &["svgz"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
