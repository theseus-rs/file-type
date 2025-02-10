use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_434: FileType = FileType {
    file_format: &FileFormat {
        id: 434,
        source_type: SourceType::Pronom,
        name: "8-bit ASCII Text",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
