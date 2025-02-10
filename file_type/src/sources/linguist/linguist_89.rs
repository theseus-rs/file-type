use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_89: FileType = FileType {
    file_format: &FileFormat {
        id: 89,
        source_type: SourceType::Linguist,
        name: "Dockerfile",
        extensions: &["containerfile", "dockerfile"],
        media_types: &["text/x-dockerfile"],
        signatures: &[],
        related_formats: &[],
    },
};
