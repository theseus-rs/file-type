use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_342: FileType = FileType {
    file_format: &FileFormat {
        id: 342,
        source_type: SourceType::Linguist,
        name: "Scaml",
        extensions: &["scaml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
