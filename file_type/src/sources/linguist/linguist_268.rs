use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_268: FileType = FileType {
    file_format: &FileFormat {
        id: 268,
        source_type: SourceType::Linguist,
        name: "Ox",
        extensions: &["ox", "oxh", "oxo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
