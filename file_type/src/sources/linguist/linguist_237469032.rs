use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_237469032: FileType = FileType {
    file_format: &FileFormat {
        id: 237_469_032,
        source_type: SourceType::Linguist,
        name: "Solidity",
        extensions: &["sol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
