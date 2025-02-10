use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2255: FileType = FileType {
    file_format: &FileFormat {
        id: 2_255,
        source_type: SourceType::Pronom,
        name: "Minitab Worksheet",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
