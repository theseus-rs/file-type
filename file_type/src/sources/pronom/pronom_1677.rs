use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1677: FileType = FileType {
    file_format: &FileFormat {
        id: 1_677,
        source_type: SourceType::Pronom,
        name: "Notation3",
        extensions: &["n3"],
        media_types: &["text/n3"],
        signatures: &[],
        related_formats: &[],
    },
};
