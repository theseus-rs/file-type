use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1690: FileType = FileType {
    file_format: &FileFormat {
        id: 1_690,
        source_type: SourceType::Pronom,
        name: "HTML Components",
        extensions: &["htc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
