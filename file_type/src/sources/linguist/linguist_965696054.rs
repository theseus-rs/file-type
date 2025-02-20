use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
