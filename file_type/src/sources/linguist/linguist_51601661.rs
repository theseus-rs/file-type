use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_51601661: FileType = FileType {
    file_format: &FileFormat {
        id: 51_601_661,
        source_type: SourceType::Linguist,
        name: "Rich Text Format",
        extensions: &["rtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
