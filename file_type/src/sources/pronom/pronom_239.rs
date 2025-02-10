use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_239: FileType = FileType {
    file_format: &FileFormat {
        id: 239,
        source_type: SourceType::Pronom,
        name: "Adobe PhotoDeluxe",
        extensions: &["pdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
