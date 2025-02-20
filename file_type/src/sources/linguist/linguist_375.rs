use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_375: FileType = FileType {
    file_format: &FileFormat {
        id: 375,
        source_type: SourceType::Linguist,
        name: "Turing",
        extensions: &["t", "tu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
