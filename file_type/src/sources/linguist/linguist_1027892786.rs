use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1027892786: FileType = FileType {
    file_format: &FileFormat {
        id: 1_027_892_786,
        source_type: SourceType::Linguist,
        name: "Smithy",
        extensions: &["smithy"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
