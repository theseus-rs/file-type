use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_131750475: FileType = FileType {
    file_format: &FileFormat {
        id: 131_750_475,
        source_type: SourceType::Linguist,
        name: "Git Commit",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
