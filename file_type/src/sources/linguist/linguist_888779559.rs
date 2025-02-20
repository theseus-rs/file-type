use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
