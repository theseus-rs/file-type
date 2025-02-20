use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_59: FileType = FileType {
    file_format: &FileFormat {
        id: 59,
        source_type: SourceType::Linguist,
        name: "Clarion",
        extensions: &["clw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
