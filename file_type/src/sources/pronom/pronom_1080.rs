use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1080: FileType = FileType {
    file_format: &FileFormat {
        id: 1_080,
        source_type: SourceType::Pronom,
        name: "Dreamweaver Lock File",
        extensions: &["lck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
