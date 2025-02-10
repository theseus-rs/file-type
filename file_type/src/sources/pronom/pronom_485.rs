use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_485: FileType = FileType {
    file_format: &FileFormat {
        id: 485,
        source_type: SourceType::Pronom,
        name: "Framework Database",
        extensions: &["fw3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
