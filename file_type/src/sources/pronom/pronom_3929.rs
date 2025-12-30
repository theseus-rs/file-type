use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_3929: FileType = FileType {
    file_format: &FileFormat {
        id: 3_929,
        source_type: SourceType::Pronom,
        name: "JSON Lines Text Format",
        extensions: &["jsonl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
