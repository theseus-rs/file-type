use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1445: FileType = FileType {
    file_format: &FileFormat {
        id: 1_445,
        source_type: SourceType::Pronom,
        name: "Apple iWork Keynote",
        extensions: &["key"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
