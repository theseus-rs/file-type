use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
