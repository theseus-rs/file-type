use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const CUSTOM_4: FileType = FileType {
    file_format: &FileFormat {
        id: 4,
        source_type: SourceType::Custom,
        name: "JSON Lines",
        extensions: &["jsonl"],
        media_types: &["application/jsonl"],
        signatures: &[],
        related_formats: &[],
    },
};
