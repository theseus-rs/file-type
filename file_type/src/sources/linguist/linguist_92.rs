use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_92: FileType = FileType {
    file_format: &FileFormat {
        id: 92,
        source_type: SourceType::Linguist,
        name: "E",
        extensions: &["e"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
