use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_292: FileType = FileType {
    file_format: &FileFormat {
        id: 292,
        source_type: SourceType::Pronom,
        name: "XYWrite Document",
        extensions: &["xy3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
