use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_278: FileType = FileType {
    file_format: &FileFormat {
        id: 278,
        source_type: SourceType::Linguist,
        name: "Parrot",
        extensions: &["parrot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
