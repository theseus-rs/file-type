use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
