use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_192: FileType = FileType {
    file_format: &FileFormat {
        id: 192,
        source_type: SourceType::Pronom,
        name: "Speller Exclude Dictionary",
        extensions: &["dic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
