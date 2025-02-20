use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
