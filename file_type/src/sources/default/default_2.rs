use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const DEFAULT_2: FileType = FileType {
    file_format: &FileFormat {
        id: 2,
        source_type: SourceType::Default,
        name: "Text",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
