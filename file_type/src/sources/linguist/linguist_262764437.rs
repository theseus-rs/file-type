use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_262764437: FileType = FileType {
    file_format: &FileFormat {
        id: 262_764_437,
        source_type: SourceType::Linguist,
        name: "PostCSS",
        extensions: &["pcss", "postcss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
