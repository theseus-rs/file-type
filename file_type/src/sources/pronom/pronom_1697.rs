use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1697: FileType = FileType {
    file_format: &FileFormat {
        id: 1_697,
        source_type: SourceType::Pronom,
        name: "i2 Analysts Notebook",
        extensions: &["anb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
