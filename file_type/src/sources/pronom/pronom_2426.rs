use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2426: FileType = FileType {
    file_format: &FileFormat {
        id: 2_426,
        source_type: SourceType::Pronom,
        name: "R Program File",
        extensions: &["r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
