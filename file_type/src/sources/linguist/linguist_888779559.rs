use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_888779559: FileType = FileType {
    file_format: &FileFormat {
        id: 888_779_559,
        source_type: SourceType::Linguist,
        name: "Whiley",
        extensions: &["whiley"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
