use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_491: FileType = FileType {
    file_format: &FileFormat {
        id: 491,
        source_type: SourceType::Pronom,
        name: "InterBase Database",
        extensions: &["gdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
