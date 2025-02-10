use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_965696054: FileType = FileType {
    file_format: &FileFormat {
        id: 965_696_054,
        source_type: SourceType::Linguist,
        name: "TextGrid",
        extensions: &["TextGrid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
