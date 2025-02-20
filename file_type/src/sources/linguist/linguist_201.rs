use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_201: FileType = FileType {
    file_format: &FileFormat {
        id: 201,
        source_type: SourceType::Linguist,
        name: "Limbo",
        extensions: &["b", "m"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
