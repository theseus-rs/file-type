use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_71: FileType = FileType {
    file_format: &FileFormat {
        id: 71,
        source_type: SourceType::Linguist,
        name: "Creole",
        extensions: &["creole"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
