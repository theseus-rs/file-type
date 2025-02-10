use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_191: FileType = FileType {
    file_format: &FileFormat {
        id: 191,
        source_type: SourceType::Pronom,
        name: "Speller Custom Dictionary",
        extensions: &["dic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
