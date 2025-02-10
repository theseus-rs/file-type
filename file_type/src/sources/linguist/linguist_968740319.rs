use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_968740319: FileType = FileType {
    file_format: &FileFormat {
        id: 968_740_319,
        source_type: SourceType::Linguist,
        name: "Mint",
        extensions: &["mint"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
