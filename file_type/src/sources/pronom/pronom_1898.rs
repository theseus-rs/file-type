use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1898: FileType = FileType {
    file_format: &FileFormat {
        id: 1_898,
        source_type: SourceType::Pronom,
        name: "Exclude File",
        extensions: &["exclude"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
