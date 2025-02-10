use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_418: FileType = FileType {
    file_format: &FileFormat {
        id: 418,
        source_type: SourceType::Linguist,
        name: "ooc",
        extensions: &["ooc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
