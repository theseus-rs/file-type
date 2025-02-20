use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2251: FileType = FileType {
    file_format: &FileFormat {
        id: 2_251,
        source_type: SourceType::Pronom,
        name: "Minitab Worksheet",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
