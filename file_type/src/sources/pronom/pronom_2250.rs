use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2250: FileType = FileType {
    file_format: &FileFormat {
        id: 2_250,
        source_type: SourceType::Pronom,
        name: "Minitab Worksheet",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
