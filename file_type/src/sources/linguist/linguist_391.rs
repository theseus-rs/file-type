use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_391: FileType = FileType {
    file_format: &FileFormat {
        id: 391,
        source_type: SourceType::Linguist,
        name: "Vue",
        extensions: &["vue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
