use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_144: FileType = FileType {
    file_format: &FileFormat {
        id: 144,
        source_type: SourceType::Pronom,
        name: "Pocket Word Template",
        extensions: &["pwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
