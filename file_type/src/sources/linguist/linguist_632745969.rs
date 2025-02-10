use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_632745969: FileType = FileType {
    file_format: &FileFormat {
        id: 632_745_969,
        source_type: SourceType::Linguist,
        name: "Wollok",
        extensions: &["wlk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
