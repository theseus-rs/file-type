use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_620599567: FileType = FileType {
    file_format: &FileFormat {
        id: 620_599_567,
        source_type: SourceType::Linguist,
        name: "Cairo",
        extensions: &["cairo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
