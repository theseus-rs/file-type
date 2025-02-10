use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_105187618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_187_618,
        source_type: SourceType::Linguist,
        name: "OASv2-yaml",
        extensions: &["yaml", "yml"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
