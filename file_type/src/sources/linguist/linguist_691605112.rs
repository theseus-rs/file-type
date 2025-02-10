use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_691605112: FileType = FileType {
    file_format: &FileFormat {
        id: 691_605_112,
        source_type: SourceType::Linguist,
        name: "dircolors",
        extensions: &["dircolors"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
