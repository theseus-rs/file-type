use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_291: FileType = FileType {
    file_format: &FileFormat {
        id: 291,
        source_type: SourceType::Pronom,
        name: "XYWrite Document",
        extensions: &["xy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
