use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
