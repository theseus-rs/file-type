use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_345: FileType = FileType {
    file_format: &FileFormat {
        id: 345,
        source_type: SourceType::Linguist,
        name: "Self",
        extensions: &["self"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
