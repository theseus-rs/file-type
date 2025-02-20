use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_4896465: FileType = FileType {
    file_format: &FileFormat {
        id: 4_896_465,
        source_type: SourceType::Linguist,
        name: "MiniYAML",
        extensions: &["yaml", "yml"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
