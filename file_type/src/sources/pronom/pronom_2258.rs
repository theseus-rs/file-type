use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2258: FileType = FileType {
    file_format: &FileFormat {
        id: 2_258,
        source_type: SourceType::Pronom,
        name: "Apple iWork Numbers",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
