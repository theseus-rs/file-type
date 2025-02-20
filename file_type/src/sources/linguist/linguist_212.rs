use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_212: FileType = FileType {
    file_format: &FileFormat {
        id: 212,
        source_type: SourceType::Linguist,
        name: "LoomScript",
        extensions: &["ls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
