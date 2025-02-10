use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_370: FileType = FileType {
    file_format: &FileFormat {
        id: 370,
        source_type: SourceType::Linguist,
        name: "Tea",
        extensions: &["tea"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
