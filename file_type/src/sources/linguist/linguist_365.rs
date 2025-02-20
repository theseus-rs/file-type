use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_365: FileType = FileType {
    file_format: &FileFormat {
        id: 365,
        source_type: SourceType::Linguist,
        name: "TOML",
        extensions: &["toml"],
        media_types: &["text/x-toml"],
        signatures: &[],
        related_formats: &[],
    },
};
