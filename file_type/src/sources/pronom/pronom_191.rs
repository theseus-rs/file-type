use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
