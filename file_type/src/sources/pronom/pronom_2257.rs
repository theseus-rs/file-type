use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2257: FileType = FileType {
    file_format: &FileFormat {
        id: 2_257,
        source_type: SourceType::Pronom,
        name: "Apple iWork Pages",
        extensions: &["pages"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
