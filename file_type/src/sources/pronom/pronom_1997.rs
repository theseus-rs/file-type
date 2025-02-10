use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1997: FileType = FileType {
    file_format: &FileFormat {
        id: 1_997,
        source_type: SourceType::Pronom,
        name: "Apple iWork Template",
        extensions: &["template"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
