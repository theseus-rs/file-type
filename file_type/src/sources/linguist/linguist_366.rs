use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_366: FileType = FileType {
    file_format: &FileFormat {
        id: 366,
        source_type: SourceType::Linguist,
        name: "TXL",
        extensions: &["txl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
