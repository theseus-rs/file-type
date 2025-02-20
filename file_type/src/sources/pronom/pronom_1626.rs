use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1626: FileType = FileType {
    file_format: &FileFormat {
        id: 1_626,
        source_type: SourceType::Pronom,
        name: "Apple iWork Numbers",
        extensions: &["numbers"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
