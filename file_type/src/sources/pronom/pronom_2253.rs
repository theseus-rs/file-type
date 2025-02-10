use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2253: FileType = FileType {
    file_format: &FileFormat {
        id: 2_253,
        source_type: SourceType::Pronom,
        name: "Minitab Worksheet",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
