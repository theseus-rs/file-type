use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1684: FileType = FileType {
    file_format: &FileFormat {
        id: 1_684,
        source_type: SourceType::Pronom,
        name: "JSON-LD",
        extensions: &["jsonld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
