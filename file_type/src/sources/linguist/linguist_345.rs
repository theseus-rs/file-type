use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
