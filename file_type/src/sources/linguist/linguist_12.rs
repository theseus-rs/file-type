use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_12: FileType = FileType {
    file_format: &FileFormat {
        id: 12,
        source_type: SourceType::Linguist,
        name: "Agda",
        extensions: &["agda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
