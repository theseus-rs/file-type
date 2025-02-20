use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_15: FileType = FileType {
    file_format: &FileFormat {
        id: 15,
        source_type: SourceType::Pronom,
        name: "Works for Macintosh Document",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
