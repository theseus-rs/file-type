use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
