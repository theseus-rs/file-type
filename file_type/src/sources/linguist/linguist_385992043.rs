use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_385992043: FileType = FileType {
    file_format: &FileFormat {
        id: 385_992_043,
        source_type: SourceType::Linguist,
        name: "Mermaid",
        extensions: &["mermaid", "mmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
