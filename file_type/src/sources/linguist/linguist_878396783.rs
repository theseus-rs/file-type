use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_878396783: FileType = FileType {
    file_format: &FileFormat {
        id: 878_396_783,
        source_type: SourceType::Linguist,
        name: "Riot",
        extensions: &["riot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
