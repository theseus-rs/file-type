use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_935: FileType = FileType {
    file_format: &FileFormat {
        id: 935,
        source_type: SourceType::Pronom,
        name: "Sound Designer II Audio File",
        extensions: &["sd2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
