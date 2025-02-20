use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1689: FileType = FileType {
    file_format: &FileFormat {
        id: 1_689,
        source_type: SourceType::Pronom,
        name: "BASIC File",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
