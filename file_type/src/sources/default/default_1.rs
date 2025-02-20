use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const DEFAULT_1: FileType = FileType {
    file_format: &FileFormat {
        id: 1,
        source_type: SourceType::Default,
        name: "Binary",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
