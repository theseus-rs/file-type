use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_239: FileType = FileType {
    file_format: &FileFormat {
        id: 239,
        source_type: SourceType::Linguist,
        name: "Myghty",
        extensions: &["myt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
