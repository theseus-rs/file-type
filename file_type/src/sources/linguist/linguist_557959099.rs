use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_557959099: FileType = FileType {
    file_format: &FileFormat {
        id: 557_959_099,
        source_type: SourceType::Linguist,
        name: "OpenAPI Specification v3",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
