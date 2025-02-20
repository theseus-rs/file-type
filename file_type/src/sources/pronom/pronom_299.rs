use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_299: FileType = FileType {
    file_format: &FileFormat {
        id: 299,
        source_type: SourceType::Pronom,
        name: "Lotus 1-2-3 Worksheet",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
