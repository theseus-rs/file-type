use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_210: FileType = FileType {
    file_format: &FileFormat {
        id: 210,
        source_type: SourceType::Pronom,
        name: "Desktop Color Separation File",
        extensions: &["dcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
