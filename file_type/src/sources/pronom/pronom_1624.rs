use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1624: FileType = FileType {
    file_format: &FileFormat {
        id: 1_624,
        source_type: SourceType::Pronom,
        name: "Apple iWork Pages",
        extensions: &["pages"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
