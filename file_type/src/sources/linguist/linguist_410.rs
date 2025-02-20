use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_410: FileType = FileType {
    file_format: &FileFormat {
        id: 410,
        source_type: SourceType::Linguist,
        name: "Zephir",
        extensions: &["zep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
