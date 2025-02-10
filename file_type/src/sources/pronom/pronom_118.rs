use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_118: FileType = FileType {
    file_format: &FileFormat {
        id: 118,
        source_type: SourceType::Pronom,
        name: "CorelDraw Pattern",
        extensions: &["pat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
